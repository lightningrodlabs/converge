use hdk::prelude::*;
use converge_integrity::*;
#[derive(Serialize, Deserialize, Debug)]
pub struct AddCriterionForObjectorInput {
    pub base_objector: AgentPubKey,
    pub target_criterion_hash: ActionHash,
}
#[hdk_extern]
pub fn add_criterion_for_objector(
    input: AddCriterionForObjectorInput,
) -> ExternResult<()> {
    create_link(
        input.base_objector.clone(),
        input.target_criterion_hash.clone(),
        LinkTypes::ObjectorToCriteria,
        (),
    )?;
    create_link(
        input.target_criterion_hash,
        input.base_objector,
        LinkTypes::CriterionToObjectors,
        (),
    )?;
    Ok(())
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
#[hdk_extern]
pub fn get_objectors_for_criterion(
    criterion_hash: ActionHash,
) -> ExternResult<Vec<AgentPubKey>> {
    let links = get_links(criterion_hash, LinkTypes::CriterionToObjectors, None)?;
    let agents: Vec<AgentPubKey> = links
        .into_iter()
        .map(|link| AgentPubKey::from(EntryHash::from(link.target)))
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
