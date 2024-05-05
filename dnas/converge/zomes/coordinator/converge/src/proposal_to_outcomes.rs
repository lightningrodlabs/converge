use hdk::prelude::*;
use converge_integrity::*;
#[derive(Serialize, Deserialize, Debug)]
pub struct AddOutcomeForProposalInput {
    pub base_proposal_hash: ActionHash,
    pub target_outcome_hash: ActionHash,
}
#[hdk_extern]
pub fn add_outcome_for_proposal(input: AddOutcomeForProposalInput) -> ExternResult<()> {
    create_link(
        input.base_proposal_hash.clone(),
        input.target_outcome_hash.clone(),
        LinkTypes::ProposalToOutcomes,
        (),
    )?;
    Ok(())
}
#[hdk_extern]
pub fn get_outcomes_for_proposal(proposal_hash: ActionHash) -> ExternResult<Vec<Link>> {
    get_links(
        GetLinksInputBuilder::try_new(proposal_hash, LinkTypes::ProposalToOutcomes)?
            .build(),
    )
}
