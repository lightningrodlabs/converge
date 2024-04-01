use hdk::prelude::*;
use converge_integrity::*;
use zome_utils::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddCriterionCommentForCriterionInput {
    pub base_criterion_hash: ActionHash,
    pub target_criterion_comment_hash: ActionHash,
}
#[hdk_extern]
pub fn add_criterion_comment_for_criterion(
    input: AddCriterionCommentForCriterionInput,
) -> ExternResult<()> {
    create_link(
        input.base_criterion_hash.clone(),
        input.target_criterion_comment_hash.clone(),
        LinkTypes::CriterionToCriterionComments,
        (),
    )?;
    Ok(())
}
#[hdk_extern]
pub fn get_criterion_comments_for_criterion(
    criterion_hash: ActionHash,
) -> ExternResult<Vec<Record>> {
    let links = get_links(link_input(
        criterion_hash,
        LinkTypes::CriterionToCriterionComments,
        None,
    ))?;
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
    let records: Vec<Record> = HDK
        .with(|hdk| hdk.borrow().get(get_input))?
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
pub fn remove_criterion_comment_for_criterion(
    input: RemoveCriterionCommentForCriterionInput,
) -> ExternResult<()> {
    let links = get_links(link_input(
        input.base_criterion_hash.clone(),
        LinkTypes::CriterionToCriterionComments,
        None,
    ))?;
    for link in links {
        if ActionHash::try_from(link.target.clone())
            .map_err(|_| {
                wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))
            })
            .unwrap()
            .eq(&input.target_criterion_comment_hash)
        {
            delete_link(link.create_link_hash)?;
        }
    }
    Ok(())
}
