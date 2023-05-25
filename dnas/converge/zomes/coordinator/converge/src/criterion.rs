use hdk::prelude::*;
use converge_integrity::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCriterionInput {
    criterion: Criterion,
    deliberation: ActionHash,
}

#[hdk_extern]
pub fn create_criterion(create_criterion_input: CreateCriterionInput) -> ExternResult<Record> {
    let CreateCriterionInput {
        criterion,
        deliberation,
    } = create_criterion_input;
    let criterion_hash = create_entry(&EntryTypes::Criterion(criterion.clone()))?;
    let record = get(criterion_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Criterion"))
            ),
        )?;
    create_link(
        deliberation.clone(),
        criterion_hash.clone(),
        LinkTypes::DeliberationToCriteria,
        (),
    )?;
    create_link(
        criterion_hash.clone(),
        deliberation.clone(),
        LinkTypes::CriterionToDeliberations,
        (),
    )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_criterion(criterion_hash: ActionHash) -> ExternResult<Option<Record>> {
    get(criterion_hash, GetOptions::default())
}
#[hdk_extern]
pub fn delete_criterion(
    original_criterion_hash: ActionHash,
) -> ExternResult<ActionHash> {
    delete_entry(original_criterion_hash)
}
