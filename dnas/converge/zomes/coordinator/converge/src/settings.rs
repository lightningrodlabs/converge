use hdk::prelude::*;
use converge_integrity::*;
use zome_utils::*;
#[hdk_extern]
pub fn create_settings(settings: Settings) -> ExternResult<Record> {
    let settings_hash = create_entry(&EntryTypes::Settings(settings.clone()))?;
    let record = get(settings_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Settings"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_settings(original_settings_hash: ActionHash) -> ExternResult<Option<Record>> {
    let links = get_links(
        link_input(original_settings_hash.clone(), LinkTypes::SettingsUpdates, None),
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_settings_hash = match latest_link {
        Some(link) => {
            link.target
                .clone()
                .into_action_hash()
                .ok_or(
                    wasm_error!(
                        WasmErrorInner::Guest(String::from("No action hash associated with link"))
                    ),
                )?
        }
        None => original_settings_hash.clone(),
    };
    get(latest_settings_hash, GetOptions::default())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateSettingsInput {
    pub original_settings_hash: ActionHash,
    pub previous_settings_hash: ActionHash,
    pub updated_settings: Settings,
}
#[hdk_extern]
pub fn update_settings(input: UpdateSettingsInput) -> ExternResult<Record> {
    let updated_settings_hash = update_entry(
        input.previous_settings_hash.clone(),
        &input.updated_settings,
    )?;
    create_link(
        input.original_settings_hash.clone(),
        updated_settings_hash.clone(),
        LinkTypes::SettingsUpdates,
        (),
    )?;
    let record = get(updated_settings_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated Settings"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_settings(original_settings_hash: ActionHash) -> ExternResult<ActionHash> {
    delete_entry(original_settings_hash)
}
