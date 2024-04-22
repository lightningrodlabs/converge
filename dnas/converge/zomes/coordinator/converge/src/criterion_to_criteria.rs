use hdk::prelude::*;
use converge_integrity::*;
use zome_utils::*;
use std::collections::HashSet;
#[derive(Serialize, Deserialize, Debug)]
pub struct AddCriterionForCriterionInput {
    pub base_criterion_hash: ActionHash,
    pub target_criterion_hash: ActionHash,
}
#[hdk_extern]
pub fn add_criterion_for_criterion(
    input: AddCriterionForCriterionInput,
) -> ExternResult<ActionHash> {
    let link = create_link(
        input.base_criterion_hash.clone(),
        input.target_criterion_hash.clone(),
        LinkTypes::CriterionToCriteria,
        (),
    )?;
    Ok(link)
}
#[hdk_extern]
pub fn get_criteria_for_criterion(
    criterion_hash: ActionHash,
) -> ExternResult<Vec<Record>> {
    let links = get_links(
        link_input(criterion_hash, LinkTypes::CriterionToCriteria, None),
    )?;
    let mut seen_targets = HashSet::new();
    let get_input: Vec<GetInput> = links
        .into_iter()
        .filter(|link| seen_targets.insert(link.target.clone()))
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
    let records: Vec<Record> = HDK
        .with(|hdk| hdk.borrow().get(get_input))?
        .into_iter()
        .filter_map(|r| r)
        .collect();
    Ok(records)
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Alternative {
    pub base_criterion_hash: Option<ActionHash>,
    pub target_criterion_hash: Option<ActionHash>,
}
#[hdk_extern]
pub fn get_alternative_link(link_hash: ActionHash) -> ExternResult<Alternative> {
    debug!("link_hash: {:?}", link_hash);
    let link: Option<Record> = get(link_hash, GetOptions::default())?;
    debug!("record: {:?}", link.clone());
    let mut tag: String = "".to_string();
    let mut base_criterion_hash: ActionHash;
    let mut criterion_hash: ActionHash;
    debug!("criterion hash");
    if let Some(l) = link.clone() {
        let o: CreateLink = l.signed_action.hashed.content.try_into().unwrap();
        tag = String::from_utf8(o.tag.0).unwrap();
        base_criterion_hash = ActionHash::try_from(o.base_address)
            .map_err(|_| {
                wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))
            })
            .unwrap();
        criterion_hash = ActionHash::try_from(o.target_address)
            .map_err(|_| {
                wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))
            })
            .unwrap();
        let objection = Alternative {
            base_criterion_hash: Some(base_criterion_hash.clone()),
            target_criterion_hash: Some(criterion_hash.clone()),
        };
        debug!("------------------------------------------: {:?}", objection.clone());
        Ok(objection)
    } else {
        let objection = Alternative {
            base_criterion_hash: None,
            target_criterion_hash: None,
        };
        debug!("none: {:?}", objection.clone());
        Ok(objection)
    }
}
