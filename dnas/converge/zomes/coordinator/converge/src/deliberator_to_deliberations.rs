use hdk::prelude::*;
use converge_integrity::*;
use zome_utils::*;
#[derive(Serialize, Deserialize, Debug)]
pub struct AddDeliberationForDeliberatorInput {
    pub base_deliberator: AgentPubKey,
    pub target_deliberation_hash: ActionHash,
}
#[hdk_extern]
pub fn add_deliberation_for_deliberator(
    input: AddDeliberationForDeliberatorInput,
) -> ExternResult<()> {
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
    let my_pub_key = agent_info()?.agent_latest_pubkey;

    // let links = get_links(
    //     link_input(
    //         my_pub_key.clone(),
    //         LinkTypes::DeliberatorToDeliberations,
    //         None,
    //     ),
    // )?;
    // for link in links {
    //     if ActionHash::try_from(link.target.clone())
    //         .map_err(|_| {
    //             wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))
    //         })
    //         .unwrap()
    //         .eq(&deliberation_hash)
    //     {
    //         delete_link(link.create_link_hash)?;
    //     }
    // }
    // let links = get_links(
    //     link_input(
    //         deliberation_hash.clone(),
    //         LinkTypes::DeliberationToDeliberators,
    //         None,
    //     ),
    // )?;
    // for link in links {
    //     if AgentPubKey::from(
    //             EntryHash::try_from(link.target.clone())
    //                 .map_err(|_| {
    //                     wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))
    //                 })
    //                 .unwrap(),
    //         )
    //         .eq(&my_pub_key)
    //     {
    //         delete_link(link.create_link_hash)?;
    //     }
    // }

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
    let my_pub_key = agent_info()?.agent_latest_pubkey;

    let links = get_links(
        link_input(
            my_pub_key.clone(),
            LinkTypes::DeliberatorToDeliberations,
            None,
        ),
    )?;
    for link in links {
        if ActionHash::try_from(link.target.clone())
            .map_err(|_| {
                wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))
            })
            .unwrap()
            .eq(&deliberation_hash)
        {
            delete_link(link.create_link_hash)?;
        }
    }
    let links = get_links(
        link_input(
            deliberation_hash.clone(),
            LinkTypes::DeliberationToDeliberators,
            None,
        ),
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
            delete_link(link.create_link_hash)?;
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
        link_input(deliberator, LinkTypes::DeliberatorToDeliberations, None),
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
        link_input(deliberation_hash.clone(), LinkTypes::DeliberationToDeliberators, None),
    )?;
    let output: Vec<DeliberatorsWithCompleted> = links
        .into_iter()
        .map(|link| {
            let tag = link.tag;
            let tag_str = String::from_utf8(tag.0).unwrap();
            let agent_pub_key = AgentPubKey::from(
                EntryHash::try_from(link.target)
                .map_err(|_| {
                    wasm_error!(WasmErrorInner::Guest("Expected entryhash".into()))
                })
                .unwrap(),
            );
            DeliberatorsWithCompleted {
                deliberator: agent_pub_key,
                completed: tag_str == "completed",
            }
        }).collect();
    Ok(output as Vec<DeliberatorsWithCompleted>)
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
        link_input(
            input.base_deliberator.clone(),
            LinkTypes::DeliberatorToDeliberations,
            None,
        ),
    )?;
    for link in links {
        if ActionHash::try_from(link.target.clone())
            .map_err(|_| {
                wasm_error!(WasmErrorInner::Guest("Expected actionhash".into()))
            })
            .unwrap()
            .eq(&input.target_deliberation_hash)
        {
            delete_link(link.create_link_hash)?;
        }
    }
    let links = get_links(
        link_input(
            input.target_deliberation_hash.clone(),
            LinkTypes::DeliberationToDeliberators,
            None,
        ),
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
            delete_link(link.create_link_hash)?;
        }
    }
    Ok(())
}
