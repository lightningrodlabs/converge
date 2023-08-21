use hdk::prelude::*;
use converge_integrity::*;
#[derive(Serialize, Deserialize, Debug)]
pub struct AddDeliberationForDeliberatorInput {
    pub base_deliberator: AgentPubKey,
    pub target_deliberation_hash: ActionHash,
}
#[hdk_extern]
pub fn add_deliberation_for_deliberator(
    input: AddDeliberationForDeliberatorInput,
) -> ExternResult<()> {
    create_link(
        input.base_deliberator.clone(),
        input.target_deliberation_hash.clone(),
        LinkTypes::DeliberatorToDeliberations,
        (),
    )?;
    create_link(
        input.target_deliberation_hash,
        input.base_deliberator,
        LinkTypes::DeliberationToDeliberators,
        (),
    )?;
    Ok(())
}
#[hdk_extern]
pub fn get_deliberations_for_deliberator(
    deliberator: AgentPubKey,
) -> ExternResult<Vec<Record>> {
    let links = get_links(deliberator, LinkTypes::DeliberatorToDeliberations, None)?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(
            ActionHash::try_from(link.target).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap().into(),
            GetOptions::default(),
        ))
        .collect();
    let records: Vec<Record> = HDK
        .with(|hdk| hdk.borrow().get(get_input))?
        .into_iter()
        .filter_map(|r| r)
        .collect();
    Ok(records)
}
#[hdk_extern]
pub fn get_deliberators_for_deliberation(
    deliberation_hash: ActionHash,
) -> ExternResult<Vec<AgentPubKey>> {
    let links = get_links(
        deliberation_hash,
        LinkTypes::DeliberationToDeliberators,
        None,
    )?;
    let agents: Vec<AgentPubKey> = links
        .into_iter()
        .map(|link| AgentPubKey::from(EntryHash::try_from(link.target).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))).unwrap()))
        .collect();
    Ok(agents)
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveDeliberationForDeliberatorInput {
    pub base_deliberator: AgentPubKey,
    pub target_deliberation_hash: ActionHash,
}
#[hdk_extern]
pub fn remove_deliberation_for_deliberator(
    input: RemoveDeliberationForDeliberatorInput,
) -> ExternResult<()> {
    let links = get_links(
        input.base_deliberator.clone(),
        LinkTypes::DeliberatorToDeliberations,
        None,
    )?;
    for link in links {
        if ActionHash::try_from(link.target.clone()).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap().eq(&input.target_deliberation_hash) {
            delete_link(link.create_link_hash)?;
        }
    }
    let links = get_links(
        input.target_deliberation_hash.clone(),
        LinkTypes::DeliberationToDeliberators,
        None,
    )?;
    for link in links {
        if AgentPubKey::from(EntryHash::try_from(link.target.clone()).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))).unwrap())
            .eq(&input.base_deliberator)
        {
            delete_link(link.create_link_hash)?;
        }
    }
    Ok(())
}
