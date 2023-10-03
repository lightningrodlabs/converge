use hdi::prelude::*;
pub fn validate_create_link_criterion_to_criterion_comments(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // Check the entry type for the given action hash
    let action_hash = ActionHash::try_from(base_address).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap();
    let record = must_get_valid_record(action_hash)?;
    let _criterion: crate::Criterion = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    // Check the entry type for the given action hash
    let action_hash = ActionHash::try_from(target_address).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap();
    let record = must_get_valid_record(action_hash)?;
    let _criterion_comment: crate::CriterionComment = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_criterion_to_criterion_comments(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}
