use hdk::prelude::*;
use converge_integrity::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddEvaluatorForProposalInput {
    pub base_proposal_hash: ActionHash,
    pub target_evaluator: AgentPubKey,
    pub tag: String,
}

#[hdk_extern]
pub fn add_evaluator_for_proposal(
    input: AddEvaluatorForProposalInput,
) -> ExternResult<()> {
    let links = get_links(
        GetLinksInputBuilder::try_new(
                input.base_proposal_hash.clone(),
                LinkTypes::ProposalToEvaluators,
            )?
            .build(),
    )?;
    for link in links {
        if AgentPubKey::from(
            link
                .target
                .clone()
                .into_entry_hash()
                .ok_or(
                    wasm_error!(
                        WasmErrorInner::Guest("No entry_hash associated with link"
                        .to_string())
                    ),
                )?,
        ) == input.target_evaluator
        {
            delete_link(link.create_link_hash)?;
        }
    }

    let tag_str = input.tag;
    let tag_bytes = tag_str.as_bytes().to_vec();
    let tag = LinkTag(tag_bytes);
    create_link(
        input.base_proposal_hash.clone(),
        input.target_evaluator.clone(),
        LinkTypes::ProposalToEvaluators,
        tag,
    )?;
    Ok(())
}

#[hdk_extern]
pub fn get_evaluators_for_proposal(
    proposal_hash: ActionHash,
) -> ExternResult<Vec<Link>> {
    get_links(
        GetLinksInputBuilder::try_new(proposal_hash, LinkTypes::ProposalToEvaluators)?
            .build(),
    )
}

#[hdk_extern]
pub fn get_deleted_evaluators_for_proposal(
    proposal_hash: ActionHash,
) -> ExternResult<Vec<(SignedActionHashed, Vec<SignedActionHashed>)>> {
    let details = get_link_details(
        proposal_hash,
        LinkTypes::ProposalToEvaluators,
        None,
        GetOptions::default(),
    )?;
    Ok(
        details
            .into_inner()
            .into_iter()
            .filter(|(_link, deletes)| !deletes.is_empty())
            .collect(),
    )
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveEvaluatorForProposalInput {
    pub base_proposal_hash: ActionHash,
    pub target_evaluator: AgentPubKey,
}

#[hdk_extern]
pub fn remove_evaluator_for_proposal(
    input: RemoveEvaluatorForProposalInput,
) -> ExternResult<()> {
    let links = get_links(
        GetLinksInputBuilder::try_new(
                input.base_proposal_hash.clone(),
                LinkTypes::ProposalToEvaluators,
            )?
            .build(),
    )?;
    for link in links {
        if AgentPubKey::from(
            link
                .target
                .clone()
                .into_entry_hash()
                .ok_or(
                    wasm_error!(
                        WasmErrorInner::Guest("No entry_hash associated with link"
                        .to_string())
                    ),
                )?,
        ) == input.target_evaluator
        {
            delete_link(link.create_link_hash)?;
        }
    }
    Ok(())
}
