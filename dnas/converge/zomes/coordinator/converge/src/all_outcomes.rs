use hdk::prelude::*;
use converge_integrity::*;
use crate::utils::link_input;
#[hdk_extern]
pub fn get_all_outcomes(_: ()) -> ExternResult<Vec<Record>> {
    let path = Path::from("all_outcomes");
    let links = get_links(
        link_input(path.path_entry_hash()?, LinkTypes::AllOutcomes, None),
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
    let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;
    let records: Vec<Record> = records.into_iter().filter_map(|r| r).collect();
    Ok(records)
}
