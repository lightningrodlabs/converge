use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Outcome {
    pub title: String,
    pub description: String,
    pub outcome_attachment: String,
    pub proposal: Option<ActionHash>,
}
pub fn validate_create_outcome(
    _action: EntryCreationAction,
    _outcome: Outcome,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_outcome(
    _action: Update,
    _outcome: Outcome,
    _original_action: EntryCreationAction,
    _original_outcome: Outcome,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from("Outcomes cannot be updated")))
}
pub fn validate_delete_outcome(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_outcome: Outcome,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_create_link_all_outcomes(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::try_from(target_address)
        .map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into())))
        .unwrap();
    let record = must_get_valid_record(action_hash)?;
    let _outcome: crate::Outcome = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_all_outcomes(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("AllOutcomes links cannot be deleted"),
        ),
    )
}
