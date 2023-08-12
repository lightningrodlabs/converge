use hdk::prelude::*;
use converge_integrity::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddCriterionForCriterionInput {
    pub base_criterion_hash: ActionHash,
    pub target_criterion_hash: ActionHash,
}
#[hdk_extern]
pub fn add_criterion_for_criterion(input: AddCriterionForCriterionInput) -> ExternResult<()> {
    create_link(input.base_criterion_hash.clone(), input.target_criterion_hash.clone(), LinkTypes::CriterionToCriteria, ())?;
    

    Ok(())    
}

#[hdk_extern]
pub fn get_criteria_for_criterion(criterion_hash: ActionHash) -> ExternResult<Vec<Record>> {
    let links = get_links(criterion_hash, LinkTypes::CriterionToCriteria, None)?;
    
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(ActionHash::from(link.target).into(), GetOptions::default()))
        .collect();

    // Get the records to filter out the deleted ones
    let records: Vec<Record> = HDK.with(|hdk| hdk.borrow().get(get_input))?
        .into_iter()
        .filter_map(|r| r)
        .collect();

    Ok(records)
}

        
