use hdk::prelude::*;
use converge_integrity::*;
#[hdk_extern]
pub fn get_all_proposals(_: ()) -> ExternResult<Vec<Record>> {
    let path = Path::from("all_proposals");
    let links = get_links(path.path_entry_hash()?, LinkTypes::AllProposals, None)?;
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
