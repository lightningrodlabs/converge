use hdk::prelude::*;
use converge_integrity::*;
#[hdk_extern]
pub fn create_viewed(viewed: Viewed) -> ExternResult<Record> {
    let viewed_hash = create_entry(&EntryTypes::Viewed(viewed.clone()))?;
    let record = get(viewed_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Viewed"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_viewed(viewed_hash: ActionHash) -> ExternResult<Option<Record>> {
    let Some(details) = get_details(viewed_hash, GetOptions::default())? else {
        return Ok(None);
    };
    match details {
        Details::Record(details) => Ok(Some(details.record)),
        _ => {
            Err(
                wasm_error!(
                    WasmErrorInner::Guest(String::from("Malformed get details response"))
                ),
            )
        }
    }
}

#[hdk_extern]
pub fn get_all_viewed() -> ExternResult<Vec<Viewed>> {
    emit_signal("viewed below")?;

    let viewed_entry_type: EntryType = UnitEntryTypes::Viewed.try_into()?;
    let filter = ChainQueryFilter::new()
        .entry_type(viewed_entry_type)
        .include_entries(true);
    let all_records = query(filter)?;

    emit_signal(all_records.clone())?;

    let all_viewed: Vec<Viewed> = all_records
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

#[hdk_extern]
pub fn delete_viewed(original_viewed_hash: ActionHash) -> ExternResult<ActionHash> {
    let details = get_details(original_viewed_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("{pascal_entry_def_name} not found"))
            ),
        )?;
    let record = match details {
        Details::Record(details) => Ok(details.record),
        _ => {
            Err(
                wasm_error!(
                    WasmErrorInner::Guest(String::from("Malformed get details response"))
                ),
            )
        }
    }?;
    delete_entry(original_viewed_hash)
}
#[hdk_extern]
pub fn get_all_deletes_for_viewed(
    original_viewed_hash: ActionHash,
) -> ExternResult<Option<Vec<SignedActionHashed>>> {
    let Some(details) = get_details(original_viewed_hash, GetOptions::default())? else {
        return Ok(None);
    };
    match details {
        Details::Entry(_) => {
            Err(wasm_error!(WasmErrorInner::Guest("Malformed details".into())))
        }
        Details::Record(record_details) => Ok(Some(record_details.deletes)),
    }
}
#[hdk_extern]
pub fn get_oldest_delete_for_viewed(
    original_viewed_hash: ActionHash,
) -> ExternResult<Option<SignedActionHashed>> {
    let Some(mut deletes) = get_all_deletes_for_viewed(original_viewed_hash)? else {
        return Ok(None);
    };
    deletes
        .sort_by(|delete_a, delete_b| {
            delete_a.action().timestamp().cmp(&delete_b.action().timestamp())
        });
    Ok(deletes.first().cloned())
}
