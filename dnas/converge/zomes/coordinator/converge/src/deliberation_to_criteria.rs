use hdk::prelude::*;
use converge_integrity::*;
use crate::utils::link_input;
#[derive(Serialize, Deserialize, Debug)]
pub struct AddCriterionForDeliberationInput {
    pub base_deliberation_hash: ActionHash,
    pub target_criterion_hash: ActionHash,
}
#[hdk_extern]
pub fn add_criterion_for_deliberation(
    input: AddCriterionForDeliberationInput,
) -> ExternResult<()> {
    create_link(
        input.base_deliberation_hash.clone(),
        input.target_criterion_hash.clone(),
        LinkTypes::DeliberationToCriteria,
        (),
    )?;
    create_link(
        input.target_criterion_hash,
        input.base_deliberation_hash,
        LinkTypes::CriterionToDeliberations,
        (),
    )?;
    Ok(())
}
#[hdk_extern]
pub fn get_criteria_for_deliberation(
    deliberation_hash: ActionHash,
) -> ExternResult<Vec<Record>> {
    let links = get_links(
        link_input(deliberation_hash, LinkTypes::DeliberationToCriteria, None),
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
pub fn get_deliberations_for_criterion(
    criterion_hash: ActionHash,
) -> ExternResult<Vec<Record>> {
    let links = get_links(
        link_input(criterion_hash, LinkTypes::CriterionToDeliberations, None),
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
pub struct RemoveCriterionForDeliberationInput {
    pub base_deliberation_hash: ActionHash,
    pub target_criterion_hash: ActionHash,
}
#[hdk_extern]
pub fn remove_criterion_for_deliberation(
    input: RemoveCriterionForDeliberationInput,
) -> ExternResult<()> {
    let links = get_links(
        link_input(
            input.base_deliberation_hash.clone(),
            LinkTypes::DeliberationToCriteria,
            None,
        ),
    )?;
    for link in links {
        if ActionHash::try_from(link.target.clone())
            .map_err(|_| {
                wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))
            })
            .unwrap()
            .eq(&input.target_criterion_hash)
        {
            delete_link(link.create_link_hash)?;
        }
    }
    let links = get_links(
        link_input(
            input.target_criterion_hash.clone(),
            LinkTypes::CriterionToDeliberations,
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
