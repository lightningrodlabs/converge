use hdk::prelude::*;
use converge_integrity::*;
use zome_utils::*;
#[hdk_extern]
pub fn create_deliberation(deliberation: Deliberation) -> ExternResult<Record> {
    debug!("create_deliberation: {:?}", deliberation);
    let deliberation_hash = create_entry(
        &EntryTypes::Deliberation(deliberation.clone()),
    )?;
    let record = get(deliberation_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Deliberation"))
            ),
        )?;
    let path = Path::from("all_deliberations");
    create_link(
        path.path_entry_hash()?,
        deliberation_hash.clone(),
        LinkTypes::AllDeliberations,
        (),
    )?;
    Ok(record)
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RecordWithLinks {
    pub record: Record,
    pub criteria: Vec<Link>,
    pub proposals: Vec<Link>,
    pub outcomes: Vec<Link>,
}
#[hdk_extern]
pub fn get_deliberation(
    original_deliberation_hash: ActionHash,
) -> ExternResult<Option<RecordWithLinks>> {
    let links = get_links(
        link_input(
            original_deliberation_hash.clone(),
            LinkTypes::DeliberationUpdates,
            None,
        ),
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_deliberation_hash = match latest_link {
        Some(link) => {
            ActionHash::try_from(link.target.clone())
                .map_err(|_| {
                    wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))
                })
                .unwrap()
        }
        None => original_deliberation_hash.clone(),
    };
    let deliberation = get(latest_deliberation_hash.clone(), GetOptions::default());
    let criteria = get_links(
        link_input(
            latest_deliberation_hash.clone(),
            LinkTypes::DeliberationToCriteria,
            None,
        ),
    )?;
    let proposals = get_links(
        link_input(
            latest_deliberation_hash.clone(),
            LinkTypes::DeliberationToProposals,
            None,
        ),
    )?;
    let outcomes = get_links(
        link_input(
            latest_deliberation_hash.clone(),
            LinkTypes::DeliberationToOutcomes,
            None,
        ),
    )?;
    return Ok(
        Some(RecordWithLinks {
            record: deliberation
                .unwrap()
                .ok_or(
                    wasm_error!(
                        WasmErrorInner::Guest(String::from("Could not find the Deliberation"))
                    ),
                )?,
            criteria,
            proposals,
            outcomes,
        }),
    );
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateDeliberationInput {
    pub original_deliberation_hash: ActionHash,
    pub previous_deliberation_hash: ActionHash,
    pub updated_deliberation: Deliberation,
}
#[hdk_extern]
pub fn update_deliberation(input: UpdateDeliberationInput) -> ExternResult<Record> {
    let updated_deliberation_hash = update_entry(
        input.previous_deliberation_hash.clone(),
        &input.updated_deliberation,
    )?;
    create_link(
        input.original_deliberation_hash.clone(),
        updated_deliberation_hash.clone(),
        LinkTypes::DeliberationUpdates,
        (),
    )?;
    let record = get(updated_deliberation_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated Deliberation"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_deliberation(
    original_deliberation_hash: ActionHash,
) -> ExternResult<ActionHash> {
    delete_entry(original_deliberation_hash)
}
