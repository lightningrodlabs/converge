use hdk::prelude::*;
use converge_integrity::*;
use crate::utils::link_input;
#[derive(Serialize, Deserialize, Debug)]
pub struct AddOutcomeForDeliberationInput {
    pub base_deliberation_hash: ActionHash,
    pub target_outcome_hash: ActionHash,
}
#[hdk_extern]
pub fn add_outcome_for_deliberation(
    input: AddOutcomeForDeliberationInput,
) -> ExternResult<()> {
    create_link(
        input.base_deliberation_hash.clone(),
        input.target_outcome_hash.clone(),
        LinkTypes::DeliberationToOutcomes,
        (),
    )?;
    create_link(
        input.target_outcome_hash,
        input.base_deliberation_hash,
        LinkTypes::OutcomeToDeliberations,
        (),
    )?;
    Ok(())
}
#[hdk_extern]
pub fn get_outcomes_for_deliberation(
    deliberation_hash: ActionHash,
) -> ExternResult<Vec<Record>> {
    let links = get_links(
        link_input(deliberation_hash, LinkTypes::DeliberationToOutcomes, None),
    )?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(
            ActionHash::try_from(link.target)
                .map_err(|_| {
                    wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))
                })
                .unwrap()
                .into(),
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
pub fn get_deliberations_for_outcome(
    outcome_hash: ActionHash,
) -> ExternResult<Vec<Record>> {
    let links = get_links(
        link_input(outcome_hash, LinkTypes::OutcomeToDeliberations, None),
    )?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(
            ActionHash::try_from(link.target)
                .map_err(|_| {
                    wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))
                })
                .unwrap()
                .into(),
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
#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveOutcomeForDeliberationInput {
    pub base_deliberation_hash: ActionHash,
    pub target_outcome_hash: ActionHash,
}
#[hdk_extern]
pub fn remove_outcome_for_deliberation(
    input: RemoveOutcomeForDeliberationInput,
) -> ExternResult<()> {
    let links = get_links(
        link_input(
            input.base_deliberation_hash.clone(),
            LinkTypes::DeliberationToOutcomes,
            None,
        ),
    )?;
    for link in links {
        if ActionHash::try_from(link.target.clone())
            .map_err(|_| {
                wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))
            })
            .unwrap()
            .eq(&input.target_outcome_hash)
        {
            delete_link(link.create_link_hash)?;
        }
    }
    let links = get_links(
        link_input(
            input.target_outcome_hash.clone(),
            LinkTypes::OutcomeToDeliberations,
            None,
        ),
    )?;
    for link in links {
        if ActionHash::try_from(link.target.clone())
            .map_err(|_| {
                wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))
            })
            .unwrap()
            .eq(&input.base_deliberation_hash)
        {
            delete_link(link.create_link_hash)?;
        }
    }
    Ok(())
}
