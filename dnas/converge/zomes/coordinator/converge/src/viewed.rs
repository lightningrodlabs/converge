use hdk::prelude::*;
use converge_integrity::*;
#[hdk_extern]
pub fn create_viewed(viewed: Viewed) -> ExternResult<Record> {
    let viewed_hash = create_entry(&EntryTypes::Viewed(viewed.clone()))?;
    let record = get(viewed_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Viewed"))
            ),
        )?;
    let path = Path::from(format!("all_viewed_{}", agent_info()?.agent_latest_pubkey));
    create_link(path.path_entry_hash()?, viewed_hash.clone(), LinkTypes::AllViewed, ())?;
    Ok(record)
}
#[hdk_extern]
pub fn get_viewed(viewed_hash: ActionHash) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(viewed_hash, GetOptions::default())? else {
        return Ok(None);
    };
    match details {
        Details::Record(details) => Ok(Some(details.record)),
        _ => {
            Err(
                wasm_error!(
                    WasmErrorInner::Guest(String::from("Malformed get details response"))
                ),
            )
        }
    }
}
#[hdk_extern]
pub fn delete_viewed(original_viewed_hash: ActionHash) -> ExternResult<ActionHash> {
    let details = get_details(original_viewed_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("{pascal_entry_def_name} not found"))
            ),
        )?;
    let record = match details {
        Details::Record(details) => Ok(details.record),
        _ => {
            Err(
                wasm_error!(
                    WasmErrorInner::Guest(String::from("Malformed get details response"))
                ),
            )
        }
    }?;
    let path = Path::from(format!("all_viewed_{}", agent_info()?.agent_latest_pubkey));
    let links = get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllViewed)?
            .build(),
    )?;
    for link in links {
        if let Some(hash) = link.target.into_action_hash() {
            if hash.eq(&original_viewed_hash) {
                delete_link(link.create_link_hash)?;
            }
        }
    }
    delete_entry(original_viewed_hash)
}
#[hdk_extern]
pub fn get_all_deletes_for_viewed(
    original_viewed_hash: ActionHash,
) -> ExternResult<Option<Vec<SignedActionHashed>>> {
    let Some(details) = get_details(original_viewed_hash, GetOptions::default())? else {
        return Ok(None);
    };
    match details {
        Details::Entry(_) => {
            Err(wasm_error!(WasmErrorInner::Guest("Malformed details".into())))
        }
        Details::Record(record_details) => Ok(Some(record_details.deletes)),
    }
}
#[hdk_extern]
pub fn get_oldest_delete_for_viewed(
    original_viewed_hash: ActionHash,
) -> ExternResult<Option<SignedActionHashed>> {
    let Some(mut deletes) = get_all_deletes_for_viewed(original_viewed_hash)? else {
        return Ok(None);
    };
    deletes
        .sort_by(|delete_a, delete_b| {
            delete_a.action().timestamp().cmp(&delete_b.action().timestamp())
        });
    Ok(deletes.first().cloned())
}
