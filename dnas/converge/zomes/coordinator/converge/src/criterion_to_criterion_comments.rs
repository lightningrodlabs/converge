use hdk::prelude::*;
use converge_integrity::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddCriterionCommentForCriterionInput {
    pub base_criterion_hash: ActionHash,
    pub target_criterion_comment_hash: ActionHash,
}
#[hdk_extern]
pub fn add_criterion_comment_for_criterion(input: AddCriterionCommentForCriterionInput) -> ExternResult<()> {
    create_link(input.base_criterion_hash.clone(), input.target_criterion_comment_hash.clone(), LinkTypes::CriterionToCriterionComments, ())?;
    

    Ok(())    
}

#[hdk_extern]
pub fn get_criterion_comments_for_criterion(criterion_hash: ActionHash) -> ExternResult<Vec<Record>> {
    let links = get_links(criterion_hash, LinkTypes::CriterionToCriterionComments, None)?;
    
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

        
#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveCriterionCommentForCriterionInput {
    pub base_criterion_hash: ActionHash,
    pub target_criterion_comment_hash: ActionHash,
}
#[hdk_extern]
pub fn remove_criterion_comment_for_criterion(input: RemoveCriterionCommentForCriterionInput ) -> ExternResult<()> {
    let links = get_links(input.base_criterion_hash.clone(), LinkTypes::CriterionToCriterionComments, None)?;
    
    for link in links {
        if ActionHash::from(link.target.clone()).eq(&input.target_criterion_comment_hash) {
            delete_link(link.create_link_hash)?;
        }
    }
    

    Ok(())        
}