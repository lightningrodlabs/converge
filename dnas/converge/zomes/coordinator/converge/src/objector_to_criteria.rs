use std::ptr::null;

use hdk::prelude::{*, tracing::field::debug};
use converge_integrity::*;
#[derive(Serialize, Deserialize, Debug)]
pub struct AddCriterionForObjectorInput {
    pub base_objector: AgentPubKey,
    pub target_criterion_hash: ActionHash,
    pub comment: String,
}
#[hdk_extern]
pub fn add_criterion_for_objector(
    input: AddCriterionForObjectorInput,
) -> ExternResult<ActionHash> {
    let tag_str = input.comment;
    let tag_bytes = tag_str.as_bytes().to_vec();
    let tag = LinkTag(tag_bytes);
    let link = create_link(
        input.target_criterion_hash,
        input.base_objector,
        LinkTypes::CriterionToObjectors,
        tag,
    )?;
    Ok(link)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Objection {
    pub base_objector: Option<AgentPubKey>,
    pub target_criterion_hash: Option<ActionHash>,
    pub comment: Option<String>,
}

#[hdk_extern]
pub fn get_objection_link(link_hash: ActionHash) -> ExternResult<Objection> {
    debug!("link_hash: {:?}", link_hash);
    let link: Option<Record> = get(link_hash, GetOptions::default())?;
    debug!("record: {:?}", link.clone());
    let mut tag: String = "".to_string();
    let mut objector: AgentPubKey;
    let mut criterion_hash: ActionHash;
    debug!("criterion hash");

    if let Some(l) = link.clone() {
        let o: CreateLink = l.signed_action.hashed.content.try_into().unwrap();
        tag = String::from_utf8(o.tag.0).unwrap();
        objector = AgentPubKey::from(EntryHash::from(o.base_address));
        criterion_hash = ActionHash::from(o.target_address);
        let objection = Objection {
            base_objector: Some(objector.clone()),
            target_criterion_hash: Some(criterion_hash.clone()),
            comment: Some(tag),
        };
        debug!("------------------------------------------: {:?}", objection.clone());
        Ok(objection)
    } else {
        let objection = Objection {
            base_objector: None,
            target_criterion_hash: None,
            comment: None,
        };
        debug!("none: {:?}", objection.clone());
        Ok(objection)
    }
}

#[hdk_extern]
pub fn get_criteria_for_objector(objector: AgentPubKey) -> ExternResult<Vec<Record>> {
    let links = get_links(objector, LinkTypes::ObjectorToCriteria, None)?;
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
pub struct AgentPubKeyWithTag {
    pub agent: AgentPubKey,
    pub tag: String,
    pub objection_hash: ActionHash,
}
#[hdk_extern]
pub fn get_objectors_for_criterion(
    criterion_hash: ActionHash,
) -> ExternResult<Vec<AgentPubKeyWithTag>> {
    let links = get_links(criterion_hash, LinkTypes::CriterionToObjectors, None)?;
    let agents: Vec<AgentPubKeyWithTag> = links
        .into_iter()
        .map(|link| {
            let tag = link.tag;
            let tag_str = String::from_utf8(tag.0).unwrap();
            let agent = AgentPubKey::from(EntryHash::from(link.target));
            let agent_with_tag = AgentPubKeyWithTag {
                agent: agent.clone(),
                tag: tag_str,
                objection_hash: link.create_link_hash
            };
            agent_with_tag
        })
        .collect();
    Ok(agents)
}
#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveCriterionForObjectorInput {
    pub base_objector: AgentPubKey,
    pub target_criterion_hash: ActionHash,
}
#[hdk_extern]
pub fn remove_criterion_for_objector(
    input: RemoveCriterionForObjectorInput,
) -> ExternResult<()> {
    let links = get_links(
        input.base_objector.clone(),
        LinkTypes::ObjectorToCriteria,
        None,
    )?;
    for link in links {
        if ActionHash::from(link.target.clone()).eq(&input.target_criterion_hash) {
            delete_link(link.create_link_hash)?;
        }
    }
    let links = get_links(
        input.target_criterion_hash.clone(),
        LinkTypes::CriterionToObjectors,
        None,
    )?;
    for link in links {
        if AgentPubKey::from(EntryHash::from(link.target.clone()))
            .eq(&input.base_objector)
        {
            delete_link(link.create_link_hash)?;
        }
    }
    Ok(())
}
