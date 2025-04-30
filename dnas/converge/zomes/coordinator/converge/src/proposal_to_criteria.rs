use hdk::prelude::*;
use converge_integrity::*;
use crate::utils::link_input;
#[derive(Serialize, Deserialize, Debug)]
pub struct AddCriterionForProposalInput {
    pub base_proposal_hash: ActionHash,
    pub target_criterion_hash: ActionHash,
    pub percentage: String,
}
#[hdk_extern]
pub fn add_criterion_for_proposal(
    input: AddCriterionForProposalInput,
) -> ExternResult<()> {
    let tag_str = input.percentage;
    let tag_bytes = tag_str.as_bytes().to_vec();
    let tag = LinkTag(tag_bytes);
    create_link(
        input.base_proposal_hash.clone(),
        input.target_criterion_hash.clone(),
        LinkTypes::ProposalToCriteria,
        tag,
    )?;
    Ok(())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Rating {
    pub agent: AgentPubKey,
    pub criterion: ActionHash,
    pub tag: String,
}
#[hdk_extern]
pub fn get_ratings_for_proposal(proposal_hash: ActionHash) -> ExternResult<Vec<Rating>> {
    let links = get_links(
        link_input(proposal_hash, LinkTypes::ProposalToCriteria, None),
    )?;
    let output: Vec<Rating> = links
        .into_iter()
        .map(|link| {
            let tag = link.tag;
            let tag_str = String::from_utf8(tag.0).unwrap();
            let agent = AgentPubKey::from(EntryHash::from(link.author));
            let agent_with_tag = Rating {
                agent: agent.clone(),
                criterion: ActionHash::try_from(link.target)
                    .map_err(|_| {
                        wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))
                    })
                    .unwrap(),
                tag: tag_str,
            };
            agent_with_tag
        })
        .collect();
    Ok(output)
}
#[hdk_extern]
pub fn get_criteria_for_proposal(
    proposal_hash: ActionHash,
) -> ExternResult<Vec<Record>> {
    let links = get_links(
        link_input(proposal_hash, LinkTypes::ProposalToCriteria, None),
    )?;
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
pub struct RemoveCriterionForProposalInput {
    pub base_proposal_hash: ActionHash,
    pub target_criterion_hash: ActionHash,
}
#[hdk_extern]
pub fn remove_criterion_for_proposal(
    input: RemoveCriterionForProposalInput,
) -> ExternResult<()> {
    let links = get_links(
        link_input(input.base_proposal_hash.clone(), LinkTypes::ProposalToCriteria, None),
    )?;
    for link in links {
        let me: AgentPubKey = agent_info()?.agent_initial_pubkey.into();
        if link.author == me {
            if ActionHash::try_from(link.target.clone())
                .map_err(|_| {
                    wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))
                })
                .unwrap()
                .eq(&input.target_criterion_hash)
            {
                delete_link(link.create_link_hash)?;
            }
        }
    }
    Ok(())
}
