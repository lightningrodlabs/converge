use hdk::prelude::*;
use converge_integrity::*;
use zome_utils::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddProposalForDeliberationInput {
    pub base_deliberation_hash: ActionHash,
    pub target_proposal_hash: ActionHash,
}
#[hdk_extern]
pub fn add_proposal_for_deliberation(
    input: AddProposalForDeliberationInput,
) -> ExternResult<()> {
    create_link(
        input.base_deliberation_hash.clone(),
        input.target_proposal_hash.clone(),
        LinkTypes::DeliberationToProposals,
        (),
    )?;
    create_link(
        input.target_proposal_hash,
        input.base_deliberation_hash,
        LinkTypes::ProposalToDeliberations,
        (),
    )?;
    Ok(())
}
#[hdk_extern]
pub fn get_proposals_for_deliberation(
    deliberation_hash: ActionHash,
) -> ExternResult<Vec<Record>> {
    let links = get_links(link_input(deliberation_hash, LinkTypes::DeliberationToProposals, None))?;
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
#[hdk_extern]
pub fn get_deliberations_for_proposal(
    proposal_hash: ActionHash,
) -> ExternResult<Vec<Record>> {
    let links = get_links(link_input(proposal_hash, LinkTypes::ProposalToDeliberations, None))?;
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
pub struct RemoveProposalForDeliberationInput {
    pub base_deliberation_hash: ActionHash,
    pub target_proposal_hash: ActionHash,
}
#[hdk_extern]
pub fn remove_proposal_for_deliberation(
    input: RemoveProposalForDeliberationInput,
) -> ExternResult<()> {
    let links = get_links(link_input(
        input.base_deliberation_hash.clone(),
        LinkTypes::DeliberationToProposals,
        None,
    ))?;
    for link in links {
        if ActionHash::try_from(link.target.clone())
            .map_err(|_| {
                wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))
            })
            .unwrap()
            .eq(&input.target_proposal_hash)
        {
            delete_link(link.create_link_hash)?;
        }
    }
    let links = get_links(link_input(
        input.target_proposal_hash.clone(),
        LinkTypes::ProposalToDeliberations,
        None,
    ))?;
    for link in links {
        if ActionHash::try_from(link.target.clone())
            .map_err(|_| {
                wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))
            })
            .unwrap()
            .eq(&input.base_deliberation_hash)
        {
            delete_link(link.create_link_hash)?;
        }
    }
    Ok(())
}
