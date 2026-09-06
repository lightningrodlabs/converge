use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Proposal {
    pub title: String,
    pub description: String,
    pub attachments: Option<Vec<String>>,
}
pub fn validate_create_proposal(
    _action: TypedAction<EntryCreationData>,
    _proposal: Proposal,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_proposal(
    _action: TypedAction<UpdateData>,
    _proposal: Proposal,
    _original_action: TypedAction<EntryCreationData>,
    _original_proposal: Proposal,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from("Proposals cannot be updated")))
}
pub fn validate_delete_proposal(
    _action: TypedAction<DeleteData>,
    _original_action: TypedAction<EntryCreationData>,
    _original_proposal: Proposal,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_create_link_all_proposals(
    _action: TypedAction<CreateLinkData>,
    _base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::try_from(target_address)
        .map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into())))
        .unwrap();
    let record = must_get_valid_record(action_hash)?;
    let _proposal: crate::Proposal = record
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
pub fn validate_delete_link_all_proposals(
    _action: TypedAction<DeleteLinkData>,
    _original_action: TypedAction<CreateLinkData>,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("AllProposals links cannot be deleted"),
        ),
    )
}
