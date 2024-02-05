use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct CriterionComment {
    pub comment: String,
    pub author: AgentPubKey,
    pub created: Timestamp,
    pub comment_reference: Option<ActionHash>,
    pub objection_reference: Option<ActionHash>,
    pub alternative_reference: Option<ActionHash>,
}
pub fn validate_create_criterion_comment(
    _action: EntryCreationAction,
    _criterion_comment: CriterionComment,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_criterion_comment(
    _action: Update,
    _criterion_comment: CriterionComment,
    _original_action: EntryCreationAction,
    _original_criterion_comment: CriterionComment,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_criterion_comment(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_criterion_comment: CriterionComment,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_create_link_criterion_comment_updates(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::try_from(base_address)
        .map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into())))
        .unwrap();
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
    let action_hash = ActionHash::try_from(target_address)
        .map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into())))
        .unwrap();
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
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_criterion_comment_updates(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("CriterionCommentUpdates links cannot be deleted"),
        ),
    )
}
pub fn validate_create_link_all_criterion_comments(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::try_from(target_address)
        .map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into())))
        .unwrap();
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
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_all_criterion_comments(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("AllCriterionComments links cannot be deleted"),
        ),
    )
}
