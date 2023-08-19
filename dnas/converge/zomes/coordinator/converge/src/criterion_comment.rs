use hdk::prelude::*;
use converge_integrity::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCriterionCommentInput {
    pub criterion_comment: CriterionComment,
    pub criterion_hash: ActionHash,
}

#[hdk_extern]
pub fn create_criterion_comment(
    input: CreateCriterionCommentInput,
) -> ExternResult<Record> {
    let criterion_comment_hash = create_entry(
        &EntryTypes::CriterionComment(input.criterion_comment.clone()),
    )?;
    let record = get(criterion_comment_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created CriterionComment"))
            ),
        )?;
    // let path = Path::from("all_criterion_comments");
    // create_link(
    //     path.path_entry_hash()?,
    //     criterion_comment_hash.clone(),
    //     LinkTypes::AllCriterionComments,
    //     (),
    // )?;
    create_link(
        input.criterion_hash,
        criterion_comment_hash,
        LinkTypes::CriterionToCriterionComments, 
        ()
    )?;

    Ok(record)
}
#[hdk_extern]
pub fn get_criterion_comment(
    original_criterion_comment_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let links = get_links(
        original_criterion_comment_hash.clone(),
        LinkTypes::CriterionCommentUpdates,
        None,
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_a.timestamp.cmp(&link_b.timestamp));
    let latest_criterion_comment_hash = match latest_link {
        Some(link) => ActionHash::from(link.target.clone()),
        None => original_criterion_comment_hash.clone(),
    };
    get(latest_criterion_comment_hash, GetOptions::default())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateCriterionCommentInput {
    pub original_criterion_comment_hash: ActionHash,
    pub previous_criterion_comment_hash: ActionHash,
    pub updated_criterion_comment: CriterionComment,
}
#[hdk_extern]
pub fn update_criterion_comment(
    input: UpdateCriterionCommentInput,
) -> ExternResult<Record> {
    let updated_criterion_comment_hash = update_entry(
        input.previous_criterion_comment_hash.clone(),
        &input.updated_criterion_comment,
    )?;
    create_link(
        input.original_criterion_comment_hash.clone(),
        updated_criterion_comment_hash.clone(),
        LinkTypes::CriterionCommentUpdates,
        (),
    )?;
    let record = get(updated_criterion_comment_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated CriterionComment"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn delete_criterion_comment(
    original_criterion_comment_hash: ActionHash,
) -> ExternResult<ActionHash> {
    delete_entry(original_criterion_comment_hash)
}
