pub mod criterion_to_criterion_comments;
pub mod criterion_comment;
pub mod criterion_to_criteria;
pub mod deliberator_to_deliberations;
pub mod all_proposals;
pub mod all_criteria;
pub mod objector_to_criteria;
pub mod proposal_to_criteria;
pub mod supporter_to_criteria;
pub mod deliberation_to_proposals;
pub mod deliberation_to_criteria;
pub mod all_deliberations;
pub mod proposal;
pub mod criterion;
pub mod deliberation;
use hdk::prelude::{*, tracing::field::debug};
use converge_integrity::*;
use serde::de;
// use hc_zome_profiles_integrity::LinkTypes as ProfileLinkTypes;
#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    #[cfg(any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "dragonfly",
        target_os = "openbsd",
        target_os = "netbsd"
    ))]
    std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");

    let mut functions: BTreeSet<(ZomeName, FunctionName)> = BTreeSet::new();
    functions.insert((zome_info()?.name, FunctionName(String::from("new_activity_receiver"))));
    let functions = GrantedFunctions::Listed(functions);
    let access = CapAccess::Unrestricted;
    let capability_grant = CapGrantEntry {
        functions,
        access,
        tag: String::from("unrestricted"),
    };
    create_cap_grant(capability_grant)?;

    Ok(InitCallbackResult::Pass)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ActivityPayload {
    deliberation_hash: ActionHash,
    message: String,
}

#[hdk_extern]
pub fn new_activity_sender(data: ActivityPayload) -> ExternResult<InitCallbackResult> {
    // let all_agents: Vec<AgentPubKey> = vec![];

    let zome_call_response = call(
        CallTargetCell::Local,
        ZomeName::from(String::from("converge")),
        FunctionName(String::from("get_deliberators_for_deliberation")),
        None,
        data.clone().deliberation_hash,
    )?;
    debug!("zome_call_response: {:?}", zome_call_response);


    match zome_call_response {
        ZomeCallResponse::Ok(result) => {
            debug!("result: {:?}", result);
            let all_agents: Vec<AgentPubKey> = result.decode().ok().unwrap();
            debug!("all_agents: {:?}", all_agents);
            for agent in all_agents {
                if (agent != agent_info()?.agent_latest_pubkey.into()) {
                    let zome_call_response = call_remote(
                        agent.clone(),
                        "converge",
                        FunctionName(String::from("new_activity_receiver")),
                        None,
                        data.clone(),
                    )?;
                    debug!("zome_call_response: {:?}", zome_call_response);
                }
            }
        }
        ZomeCallResponse::NetworkError(err) => {
            debug!("network error: {:?}", err);
        }
        ZomeCallResponse::Unauthorized(a,b,c,d,e) => {
            debug!("unauthorized: {:?}", a);
        }
        _ => {
            debug!("error: {:?}", zome_call_response);
        },
    }
    Ok(InitCallbackResult::Pass)
}

#[hdk_extern]
pub fn new_activity_receiver(data: ActivityPayload) -> ExternResult<()> {
    emit_signal(data.clone())?;
    debug!("data: {:?}", data);
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Signal {
    LinkCreated { action: SignedActionHashed, link_type: LinkTypes },
    LinkDeleted { action: SignedActionHashed, link_type: LinkTypes },
    EntryCreated { action: SignedActionHashed, app_entry: EntryTypes },
    EntryUpdated {
        action: SignedActionHashed,
        app_entry: EntryTypes,
        original_app_entry: EntryTypes,
    },
    EntryDeleted { action: SignedActionHashed, original_app_entry: EntryTypes },
}
#[hdk_extern(infallible)]
pub fn post_commit(committed_actions: Vec<SignedActionHashed>) {
    for action in committed_actions {
        if let Err(err) = signal_action(action) {
            error!("Error signaling new action: {:?}", err);
        }
    }
}
fn signal_action(action: SignedActionHashed) -> ExternResult<()> {
    match action.hashed.content.clone() {
        Action::CreateLink(create_link) => {
            if let Ok(Some(link_type))
                = LinkTypes::from_type(create_link.zome_index, create_link.link_type) {
                emit_signal(Signal::LinkCreated {
                    action,
                    link_type,
                })?;
            }
            Ok(())
        }
        Action::DeleteLink(delete_link) => {
            let record = get(
                    delete_link.link_add_address.clone(),
                    GetOptions::default(),
                )?
                .ok_or(
                    wasm_error!(
                        WasmErrorInner::Guest("Failed to fetch CreateLink action"
                        .to_string())
                    ),
                )?;
            match record.action() {
                Action::CreateLink(create_link) => {
                    if let Ok(Some(link_type))
                        = LinkTypes::from_type(
                            create_link.zome_index,
                            create_link.link_type,
                        ) {
                        emit_signal(Signal::LinkDeleted {
                            action,
                            link_type,
                        })?;
                    }
                    Ok(())
                }
                _ => {
                    return Err(
                        wasm_error!(
                            WasmErrorInner::Guest("Create Link should exist".to_string())
                        ),
                    );
                }
            }
        }
        Action::Create(_create) => {
            if let Ok(Some(app_entry)) = get_entry_for_action(&action.hashed.hash) {
                emit_signal(Signal::EntryCreated {
                    action,
                    app_entry,
                })?;
            }
            Ok(())
        }
        Action::Update(update) => {
            if let Ok(Some(app_entry)) = get_entry_for_action(&action.hashed.hash) {
                if let Ok(Some(original_app_entry))
                    = get_entry_for_action(&update.original_action_address) {
                    emit_signal(Signal::EntryUpdated {
                        action,
                        app_entry,
                        original_app_entry,
                    })?;
                }
            }
            Ok(())
        }
        Action::Delete(delete) => {
            if let Ok(Some(original_app_entry))
                = get_entry_for_action(&delete.deletes_address) {
                emit_signal(Signal::EntryDeleted {
                    action,
                    original_app_entry,
                })?;
            }
            Ok(())
        }
        _ => Ok(()),
    }
}
fn get_entry_for_action(action_hash: &ActionHash) -> ExternResult<Option<EntryTypes>> {
    let record = match get_details(action_hash.clone(), GetOptions::default())? {
        Some(Details::Record(record_details)) => record_details.record,
        _ => {
            return Ok(None);
        }
    };
    let entry = match record.entry().as_option() {
        Some(entry) => entry,
        None => {
            return Ok(None);
        }
    };
    let (zome_index, entry_index) = match record.action().entry_type() {
        Some(EntryType::App(AppEntryDef { zome_index, entry_index, .. })) => {
            (zome_index, entry_index)
        }
        _ => {
            return Ok(None);
        }
    };
    Ok(
        EntryTypes::deserialize_from_type(
            zome_index.clone(),
            entry_index.clone(),
            entry,
        )?,
    )
}
