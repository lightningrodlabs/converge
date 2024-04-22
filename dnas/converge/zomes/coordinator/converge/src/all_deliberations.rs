use hdk::prelude::*;
use converge_integrity::*;
use zome_utils::*;
#[hdk_extern]
pub fn get_all_deliberations(_: ()) -> ExternResult<Vec<Record>> {
    let path = Path::from("all_deliberations");
    let links = get_links(
        link_input(path.path_entry_hash()?, LinkTypes::AllDeliberations, None),
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
#[hdk_extern]
pub fn search_all_deliberations(query: String) -> ExternResult<Vec<ActionHash>> {
    let path = Path::from("all_deliberations");
    let links = get_links(
        link_input(path.path_entry_hash()?, LinkTypes::AllDeliberations, None),
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
    let mut output: Vec<ActionHash> = vec![];
    for item in records.iter() {
        emit_signal(item.clone())?;
        let deliberation: Deliberation = item
            .entry()
            .to_app_option()
            .map_err(|err| wasm_error!(err))?
            .ok_or(
                wasm_error!(
                    WasmErrorInner::Guest("Could not deserialize record to FacetGroup."
                    .into(),)
                ),
            )?;
        if deliberation.title.contains(&query)
            || deliberation.description.contains(&query)
        {
            output.push(item.signed_action.as_hash().to_owned());
        }
    }
    Ok(output)
}
