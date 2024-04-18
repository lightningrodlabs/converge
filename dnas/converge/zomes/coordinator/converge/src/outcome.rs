use hdk::prelude::*;
use converge_integrity::*;
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOutcomeInput {
    outcome: Outcome,
    deliberation: ActionHash,
}
#[hdk_extern]
pub fn create_outcome(
    create_outcome_input: CreateOutcomeInput,
) -> ExternResult<Record> {
    let CreateOutcomeInput { outcome, deliberation } = create_outcome_input;
    let outcome_hash = create_entry(&EntryTypes::Outcome(outcome.clone()))?;
    let record = get(outcome_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Outcome"))
            ),
        )?;
    create_link(
        deliberation.clone(),
        outcome_hash.clone(),
        LinkTypes::DeliberationToOutcomes,
        (),
    )?;
    create_link(outcome_hash, deliberation, LinkTypes::OutcomeToDeliberations, ())?;
    Ok(record)
}
#[hdk_extern]
pub fn get_outcome(outcome_hash: ActionHash) -> ExternResult<Option<Record>> {
    get(outcome_hash, GetOptions::default())
}
#[hdk_extern]
pub fn delete_outcome(original_outcome_hash: ActionHash) -> ExternResult<ActionHash> {
    delete_entry(original_outcome_hash)
}
