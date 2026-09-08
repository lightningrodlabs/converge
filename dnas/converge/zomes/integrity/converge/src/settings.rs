use hdi::prelude::*;
use crate::UnitEntryTypes;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct Settings {
    pub discussion_app: String,
}
pub fn validate_create_settings(
    _action: TypedAction<EntryCreationData>,
    _settings: Settings,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_settings(
    _action: TypedAction<UpdateData>,
    _settings: Settings,
    _original_action: TypedAction<EntryCreationData>,
    _original_settings: Settings,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_settings(
    _action: TypedAction<DeleteData>,
    _original_action: TypedAction<EntryCreationData>,
    _original_settings: Settings,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
/// Check that `action_hash` refers to an entry-creation action for `Settings`.
///
/// `Settings` is a PRIVATE entry (see `EntryTypes` in `lib.rs`). A remote
/// validation authority receives the *action* but never the entry content, so
/// decoding the entry here — which is what this validator used to do — succeeds
/// only on the author's own machine. Everywhere else `to_app_option()` returned
/// `None`, the `ok_or` fired, and the link failed validation, so a
/// `SettingsUpdates` link never integrated and `get_settings` kept returning the
/// original for every agent except its author.
///
/// The action's declared entry type travels with the action, so checking that
/// validates the same property — "this link references a Settings entry" — on
/// every authority rather than only on the author's.
fn validate_references_settings(
    action_hash: ActionHash,
) -> ExternResult<ValidateCallbackResult> {
    let record = must_get_valid_record(action_hash)?;
    let entry_type = match record.action().entry_type() {
        Some(entry_type) => entry_type.clone(),
        None => {
            return Ok(
                ValidateCallbackResult::Invalid(
                    String::from("SettingsUpdates links must reference an entry-creation action"),
                ),
            );
        }
    };
    let expected: EntryType = UnitEntryTypes::Settings.try_into()?;
    if entry_type != expected {
        return Ok(
            ValidateCallbackResult::Invalid(
                String::from("SettingsUpdates links must reference a Settings entry"),
            ),
        );
    }
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_create_link_settings_updates(
    _action: TypedAction<CreateLinkData>,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let base_action_hash = base_address
        .into_action_hash()
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("No action hash associated with link"))
            ),
        )?;
    match validate_references_settings(base_action_hash)? {
        ValidateCallbackResult::Valid => {}
        invalid => return Ok(invalid),
    }
    let target_action_hash = target_address
        .into_action_hash()
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("No action hash associated with link"))
            ),
        )?;
    match validate_references_settings(target_action_hash)? {
        ValidateCallbackResult::Valid => {}
        invalid => return Ok(invalid),
    }
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_settings_updates(
    _action: TypedAction<DeleteLinkData>,
    _original_action: TypedAction<CreateLinkData>,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("SettingsUpdates links cannot be deleted"),
        ),
    )
}
