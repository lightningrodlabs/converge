use hdk::prelude::*;
use converge_integrity::*;
#[derive(Serialize, Deserialize, Debug)]
pub struct AddCriterionForSupporterInput {
    pub base_supporter: AgentPubKey,
    pub target_criterion_hash: ActionHash,
    pub tag: String,
}
#[hdk_extern]
pub fn add_criterion_for_supporter(
    input: AddCriterionForSupporterInput,
) -> ExternResult<()> {

    // remove previous objections
    let links = get_links(
        input.base_supporter.clone(),
        LinkTypes::ObjectorToCriteria,
        None,
    )?;
    for link in links {
        if ActionHash::try_from(link.target.clone()).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap().eq(&input.target_criterion_hash) {
            delete_link(link.create_link_hash)?;
        }
    }
    let links = get_links(
        input.target_criterion_hash.clone(),
        LinkTypes::CriterionToObjectors,
        None,
    )?;
    for link in links {
        if AgentPubKey::from(EntryHash::try_from(link.target.clone()).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap())
            .eq(&input.base_supporter)
        {
            delete_link(link.create_link_hash)?;
        }
    }

    let tag_str = input.tag;
    let tag_bytes = tag_str.as_bytes().to_vec();
    let tag = LinkTag(tag_bytes);
    create_link(
        input.target_criterion_hash,
        input.base_supporter,
        LinkTypes::CriterionToSupporters,
        tag,
    )?;
    Ok(())
}
#[hdk_extern]
pub fn get_criteria_for_supporter(supporter: AgentPubKey) -> ExternResult<Vec<Record>> {
    let links = get_links(supporter, LinkTypes::SupporterToCriteria, None)?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(
            ActionHash::try_from(link.target).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap().into(),
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
pub struct AgentPubKeyWithTag {
    pub agent: AgentPubKey,
    pub tag: String,
}
#[hdk_extern]
pub fn get_supporters_for_criterion(
    criterion_hash: ActionHash,
) -> ExternResult<Vec<AgentPubKeyWithTag>> {
    let links = get_links(criterion_hash, LinkTypes::CriterionToSupporters, None)?;
    let agents: Vec<AgentPubKeyWithTag> = links
        .into_iter()
        .map(|link| {
            let tag = link.tag;
            let tag_str = String::from_utf8(tag.0).unwrap();
            let agent = AgentPubKey::from(EntryHash::try_from(link.target).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))).unwrap());
            let agent_with_tag = AgentPubKeyWithTag {
                agent: agent.clone(),
                tag: tag_str,
            };
            agent_with_tag
        })
        .collect();
    Ok(agents)
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveCriterionForSupporterInput {
    pub base_supporter: AgentPubKey,
    pub target_criterion_hash: ActionHash,
}
#[hdk_extern]
pub fn remove_criterion_for_supporter(
    input: RemoveCriterionForSupporterInput,
) -> ExternResult<()> {
    let links = get_links(
        input.base_supporter.clone(),
        LinkTypes::SupporterToCriteria,
        None,
    )?;
    for link in links {
        if ActionHash::try_from(link.target.clone()).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))).unwrap().eq(&input.target_criterion_hash) {
            delete_link(link.create_link_hash)?;
        }
    }
    let links = get_links(
        input.target_criterion_hash.clone(),
        LinkTypes::CriterionToSupporters,
        None,
    )?;
    for link in links {
        if AgentPubKey::from(EntryHash::try_from(link.target.clone()).map_err(|_| wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))).unwrap())
            .eq(&input.base_supporter)
        {
            delete_link(link.create_link_hash)?;
        }
    }
    Ok(())
}
