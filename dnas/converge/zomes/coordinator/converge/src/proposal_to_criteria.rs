use hdk::prelude::*;
use converge_integrity::*;
#[derive(Serialize, Deserialize, Debug)]
pub struct AddCriterionForProposalInput {
    pub base_proposal_hash: ActionHash,
    pub target_criterion_hash: ActionHash,
}
#[hdk_extern]
pub fn add_criterion_for_proposal(
    input: AddCriterionForProposalInput,
) -> ExternResult<()> {
    create_link(
        input.base_proposal_hash.clone(),
        input.target_criterion_hash.clone(),
        LinkTypes::ProposalToCriteria,
        (),
    )?;
    Ok(())
}
#[hdk_extern]
pub fn get_criteria_for_proposal(
    proposal_hash: ActionHash,
) -> ExternResult<Vec<Record>> {
    let links = get_links(proposal_hash, LinkTypes::ProposalToCriteria, None)?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(
            ActionHash::from(link.target).into(),
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
pub struct RemoveCriterionForProposalInput {
    pub base_proposal_hash: ActionHash,
    pub target_criterion_hash: ActionHash,
}
#[hdk_extern]
pub fn remove_criterion_for_proposal(
    input: RemoveCriterionForProposalInput,
) -> ExternResult<()> {
    let links = get_links(
        input.base_proposal_hash.clone(),
        LinkTypes::ProposalToCriteria,
        None,
    )?;
    for link in links {
        if ActionHash::from(link.target.clone()).eq(&input.target_criterion_hash) {
            delete_link(link.create_link_hash)?;
        }
    }
    Ok(())
}
