use hdk::prelude::*;
use converge_integrity::*;
#[hdk_extern]
pub fn get_all_viewed(_: ()) -> ExternResult<Vec<Viewed>> {
    let path = Path::from(format!("all_viewed_{}", agent_info()?.agent_latest_pubkey));
    let links = get_links(
        GetLinksInputBuilder::try_new(path.path_entry_hash()?, LinkTypes::AllViewed)?
            .build(),
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

    let all_viewed: Vec<Viewed> = records
        .into_iter()
        .map(|record| {
            let viewed: Viewed = record
                .entry
                .clone()
                .into_option()
                .ok_or(
                    wasm_error!(WasmErrorInner::Guest(
                        String::from("Could not find the Viewed entry")
                    )),
                )?
                .try_into()?;
            Ok(viewed)
        })
        .collect::<ExternResult<Vec<Viewed>>>()?;
    
    Ok(all_viewed)
}
