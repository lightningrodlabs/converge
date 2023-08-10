use hdk::prelude::*;
use converge_integrity::*;
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateProposalInput {
    proposal: Proposal,
    deliberation: ActionHash,
}
#[hdk_extern]
pub fn create_proposal(
    create_proposal_input: CreateProposalInput,
) -> ExternResult<Record> {
    let CreateProposalInput { proposal, deliberation } = create_proposal_input;
    let proposal_hash = create_entry(&EntryTypes::Proposal(proposal.clone()))?;
    let record = get(proposal_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created Proposal"))
            ),
        )?;
    create_link(
        deliberation.clone(),
        proposal_hash.clone(),
        LinkTypes::DeliberationToProposals,
        (),
    )?;
    create_link(proposal_hash, deliberation, LinkTypes::ProposalToDeliberations, ())?;
    Ok(record)
}
#[hdk_extern]
pub fn get_proposal(proposal_hash: ActionHash) -> ExternResult<Option<Record>> {
    get(proposal_hash, GetOptions::default())
}
#[hdk_extern]
pub fn delete_proposal(original_proposal_hash: ActionHash) -> ExternResult<ActionHash> {
    delete_entry(original_proposal_hash)
}
