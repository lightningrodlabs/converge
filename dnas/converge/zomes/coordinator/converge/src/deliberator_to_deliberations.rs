use hdk::prelude::*;
use converge_integrity::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct AddDeliberationForDeliberatorInput {
    pub base_deliberator: AgentPubKey,
    pub target_deliberation_hash: ActionHash,
}
#[hdk_extern]
pub fn add_deliberation_for_deliberator(
    input: AddDeliberationForDeliberatorInput,
) -> ExternResult<()> {
    // first delete any existing links to avoid duplicates
    let links = get_links(
        LinkQuery::try_new(
            input.base_deliberator.clone(),
            LinkTypes::DeliberatorToDeliberations,
        )?, GetStrategy::Local
    )?;
    for link in links {
        if ActionHash::try_from(link.target.clone())
            .map_err(|_| {
                wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))
            })
            .unwrap()
            .eq(&input.target_deliberation_hash)
        {
            delete_link(link.create_link_hash, GetOptions::local())?;
        }
    }
    let links = get_links(
        LinkQuery::try_new(
            input.target_deliberation_hash.clone(),
            LinkTypes::DeliberationToDeliberators,
        )?, GetStrategy::Local
    )?;
    for link in links {
        if AgentPubKey::from(
                EntryHash::try_from(link.target.clone())
                    .map_err(|_| {
                        wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))
                    })
                    .unwrap(),
            )
            .eq(&input.base_deliberator)
        {
            delete_link(link.create_link_hash, GetOptions::local())?;
        }
    }
    create_link(
        input.base_deliberator.clone(),
        input.target_deliberation_hash.clone(),
        LinkTypes::DeliberatorToDeliberations,
        (),
    )?;
    create_link(
        input.target_deliberation_hash,
        input.base_deliberator,
        LinkTypes::DeliberationToDeliberators,
        (),
    )?;
    Ok(())
}

#[hdk_extern]
pub fn add_completed_tag(
    deliberation_hash: ActionHash,
) -> ExternResult<()> {
    let my_pub_key = agent_info()?.agent_initial_pubkey;

    // delete existing links to avoid duplicates
    let links = get_links(
        LinkQuery::try_new(
            my_pub_key.clone(),
            LinkTypes::DeliberatorToDeliberations,
        )?, GetStrategy::Local
    )?;
    for link in links {
        if ActionHash::try_from(link.target.clone())
            .map_err(|_| {
                wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))
            })
            .unwrap()
            .eq(&deliberation_hash)
        {
            delete_link(link.create_link_hash, GetOptions::local())?;
        }
    }
    let links = get_links(
        LinkQuery::try_new(
            deliberation_hash.clone(),
            LinkTypes::DeliberationToDeliberators,
        )?, GetStrategy::Local
    )?;
    for link in links {
        if AgentPubKey::from(
                EntryHash::try_from(link.target.clone())
                    .map_err(|_| {
                        wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))
                    })
                    .unwrap(),
            )
            .eq(&my_pub_key)
        {
            delete_link(link.create_link_hash, GetOptions::local())?;
        }
    }

    let tag_str = "completed".to_string();
    let tag_bytes = tag_str.as_bytes().to_vec();
    let tag = LinkTag(tag_bytes);
    create_link(
        deliberation_hash.clone(),
        my_pub_key.clone(),
        LinkTypes::DeliberationToDeliberators,
        tag.clone(),
    )?;
    create_link(
        my_pub_key,
        deliberation_hash,
        LinkTypes::DeliberatorToDeliberations,
        tag,
    )?;
    Ok(())
}

#[hdk_extern]
pub fn remove_completed_tag(
    deliberation_hash: ActionHash,
) -> ExternResult<()> {
    let my_pub_key = agent_info()?.agent_initial_pubkey;

    let links = get_links(
        LinkQuery::try_new(
            my_pub_key.clone(),
            LinkTypes::DeliberatorToDeliberations,
        )?, GetStrategy::Local
    )?;
    for link in links {
        if ActionHash::try_from(link.target.clone())
            .map_err(|_| {
                wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))
            })
            .unwrap()
            .eq(&deliberation_hash)
        {
            delete_link(link.create_link_hash, GetOptions::local())?;
        }
    }
    let links = get_links(
        LinkQuery::try_new(
            deliberation_hash.clone(),
            LinkTypes::DeliberationToDeliberators,
        )?, GetStrategy::Local
    )?;
    for link in links {
        if AgentPubKey::from(
                EntryHash::try_from(link.target.clone())
                    .map_err(|_| {
                        wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))
                    })
                    .unwrap(),
            )
            .eq(&my_pub_key)
        {
            delete_link(link.create_link_hash, GetOptions::local())?;
        }
    }

    // create_link(
    //     deliberation_hash.clone(),
    //     my_pub_key.clone(),
    //     LinkTypes::DeliberationToDeliberators,
    //     (),
    // )?;
    // create_link(
    //     my_pub_key,
    //     deliberation_hash,
    //     LinkTypes::DeliberatorToDeliberations,
    //     (),
    // )?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeliberationsWithCompleted {
    uncompleted: Vec<Record>,
    completed: Vec<Record>,
}

#[hdk_extern]
pub fn get_deliberations_for_deliberator(
    deliberator: AgentPubKey,
) -> ExternResult<DeliberationsWithCompleted> {
    let links: Vec<Link> = get_links(
        LinkQuery::try_new(
            deliberator,
            LinkTypes::DeliberatorToDeliberations,
        )?, GetStrategy::Local
    )?;

    let completed_links = links
        .iter()
        .filter(|link| {
            let tag = link.tag.clone();
            let tag_str = String::from_utf8(tag.0.clone()).unwrap();
            tag_str == "completed"
        })
        .collect::<Vec<&Link>>();

    let uncompleted_links: Vec<Link> = links.clone().into_iter()
        .filter(|link| !completed_links.contains(&link))
        .collect();

    let get_uncompleted_input: Vec<GetInput> = uncompleted_links
        .into_iter()
        .map(|link| GetInput::new(
            ActionHash::try_from(link.target.clone())
                .map_err(|_| {
                    wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))
                })
                .unwrap()
                .into(),
            GetOptions::default(),
        ))
        .collect();

    let get_completed_input: Vec<GetInput> = completed_links
        .into_iter()
        .map(|link| GetInput::new(
            ActionHash::try_from(link.target.clone())
                .map_err(|_| {
                    wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))
                })
                .unwrap()
                .into(),
            GetOptions::default(),
        ))
        .collect();

    let completed_records: Vec<Record> = HDK
        .with(|hdk| hdk.borrow().get(get_completed_input))?
        .into_iter()
        .filter_map(|r| r)
        .collect();

    let uncompleted_records: Vec<Record> = HDK
        .with(|hdk| hdk.borrow().get(get_uncompleted_input))?
        .into_iter()
        .filter_map(|r| r)
        .collect();

    Ok(
        DeliberationsWithCompleted {
            uncompleted: uncompleted_records,
            completed: completed_records,
        }
    )
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeliberatorsWithCompleted {
    pub deliberator: AgentPubKey,
    pub completed: bool,
}

#[hdk_extern]
pub fn get_deliberators_for_deliberation(
    deliberation_hash: ActionHash,
) -> ExternResult<Vec<DeliberatorsWithCompleted>> {
    let links = get_links(
        LinkQuery::try_new(
            deliberation_hash.clone(),
            LinkTypes::DeliberationToDeliberators,
        )?, GetStrategy::Local
    )?;
    
    // Deduplicate deliberators by agent key, keeping most recent entry
    let mut deliberators_map: std::collections::BTreeMap<Vec<u8>, (DeliberatorsWithCompleted, Timestamp)> = std::collections::BTreeMap::new();
    for link in links {
        let tag = link.tag;
        let tag_str = String::from_utf8(tag.0).unwrap();
        let agent_pub_key = AgentPubKey::from(
            EntryHash::try_from(link.target)
            .map_err(|_| {
                wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))
            })
            .unwrap(),
        );
        let agent_bytes = agent_pub_key.get_raw_39().to_vec();
        let deliberator_info = DeliberatorsWithCompleted {
            deliberator: agent_pub_key,
            completed: tag_str == "completed",
        };
        
        // Always keep the most recent entry for each agent
        let timestamp = link.timestamp;
        let should_insert = match deliberators_map.get(&agent_bytes) {
            Some((_, existing_timestamp)) => timestamp > *existing_timestamp,
            None => true,
        };
        
        if should_insert {
            deliberators_map.insert(agent_bytes, (deliberator_info, timestamp));
        }
    }
    
    let output: Vec<DeliberatorsWithCompleted> = deliberators_map
        .into_values()
        .map(|(info, _)| info)
        .collect();
    Ok(output)
}

// #[hdk_extern]
// pub fn get_completed_deliberators_for_deliberation(
//     deliberation_hash: ActionHash,
// ) -> ExternResult<Vec<AgentPubKey>> {
//     let tag_str = "completed".to_string();
//     let tag_bytes = tag_str.as_bytes().to_vec();
//     let tag = Some(LinkTag(tag_bytes));
//     let links = get_links(
//         link_input(deliberation_hash, LinkTypes::DeliberationToDeliberators, tag),
//     )?;

//     let agents: Vec<AgentPubKey> = links
//         .into_iter()
//         .map(|link| AgentPubKey::from(
//             EntryHash::try_from(link.target)
//                 .map_err(|_| {
//                     wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))
//                 })
//                 .unwrap(),
//         ))
//         .collect();
//     Ok(agents)
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct RemoveDeliberationForDeliberatorInput {
    pub base_deliberator: AgentPubKey,
    pub target_deliberation_hash: ActionHash,
}
#[hdk_extern]
pub fn remove_deliberation_for_deliberator(
    input: RemoveDeliberationForDeliberatorInput,
) -> ExternResult<()> {
    let links = get_links(
        LinkQuery::try_new(
            input.base_deliberator.clone(),
            LinkTypes::DeliberatorToDeliberations,
        )?, GetStrategy::Local
    )?;
    for link in links {
        if ActionHash::try_from(link.target.clone())
            .map_err(|_| {
                wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))
            })
            .unwrap()
            .eq(&input.target_deliberation_hash)
        {
            delete_link(link.create_link_hash, GetOptions::local())?;
        }
    }
    let links = get_links(
        LinkQuery::try_new(
            input.target_deliberation_hash.clone(),
            LinkTypes::DeliberationToDeliberators,
        )?, GetStrategy::Local
    )?;
    for link in links {
        if AgentPubKey::from(
                EntryHash::try_from(link.target.clone())
                    .map_err(|_| {
                        wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))
                    })
                    .unwrap(),
            )
            .eq(&input.base_deliberator)
        {
            delete_link(link.create_link_hash, GetOptions::local())?;
        }
    }
    Ok(())
}
