pub mod proposal_to_outcomes;
pub use proposal_to_outcomes::*;
pub mod settings;
pub use settings::*;
pub mod criterion_to_criterion_comments;
pub use criterion_to_criterion_comments::*;
pub mod criterion_comment;
pub use criterion_comment::*;
pub mod criterion_to_criteria;
pub use criterion_to_criteria::*;
pub mod deliberator_to_deliberations;
pub use deliberator_to_deliberations::*;
pub mod objector_to_criteria;
pub use objector_to_criteria::*;
pub mod proposal_to_criteria;
pub use proposal_to_criteria::*;
pub mod supporter_to_criteria;
pub use supporter_to_criteria::*;
pub mod deliberation_to_proposals;
pub use deliberation_to_proposals::*;
pub mod deliberation_to_outcomes;
pub use deliberation_to_outcomes::*;
pub mod deliberation_to_criteria;
pub use deliberation_to_criteria::*;
pub mod proposal;
pub use proposal::*;
pub mod outcome;
pub use outcome::*;
pub mod criterion;
pub use criterion::*;
pub mod deliberation;
pub use deliberation::*;
use hdi::prelude::*;
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[hdk_entry_types]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    Deliberation(Deliberation),
    Criterion(Criterion),
    Proposal(Proposal),
    Outcome(Outcome),
    CriterionComment(CriterionComment),
    #[entry_type(name = "Settings", visibility = "private")]
    Settings(Settings),
}
#[derive(Serialize, Deserialize)]
#[hdk_link_types]
pub enum LinkTypes {
    DeliberationUpdates,
    AllDeliberations,
    DeliberationToCriteria,
    CriterionToDeliberations,
    DeliberationToProposals,
    DeliberationToOutcomes,
    ProposalToDeliberations,
    OutcomeToDeliberations,
    SupporterToCriteria,
    CriterionToSupporters,
    ProposalToCriteria,
    ObjectorToCriteria,
    CriterionToObjectors,
    AllCriteria,
    AllProposals,
    AllOutcomes,
    DeliberatorToDeliberations,
    DeliberationToDeliberators,
    CriterionToCriteria,
    CriterionCommentUpdates,
    AllCriterionComments,
    CriterionToCriterionComments,
    SettingsUpdates,
    ProposalToOutcomes,
}
#[hdk_extern]
pub fn genesis_self_check(
    _data: GenesisSelfCheckData,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_agent_joining(
    _agent_pub_key: AgentPubKey,
    _membrane_proof: &Option<MembraneProof>,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    match op.flattened::<EntryTypes, LinkTypes>()? {
        FlatOp::StoreEntry(store_entry) => {
            match store_entry {
                OpEntry::CreateEntry { app_entry, action } => {
                    match app_entry {
                        EntryTypes::Deliberation(deliberation) => {
                            validate_create_deliberation(
                                EntryCreationAction::Create(action),
                                deliberation,
                            )
                        }
                        EntryTypes::Criterion(criterion) => {
                            validate_create_criterion(
                                EntryCreationAction::Create(action),
                                criterion,
                            )
                        }
                        EntryTypes::Proposal(proposal) => {
                            validate_create_proposal(
                                EntryCreationAction::Create(action),
                                proposal,
                            )
                        }
                        EntryTypes::Outcome(outcome) => {
                            validate_create_outcome(
                                EntryCreationAction::Create(action),
                                outcome,
                            )
                        }
                        EntryTypes::CriterionComment(criterion_comment) => {
                            validate_create_criterion_comment(
                                EntryCreationAction::Create(action),
                                criterion_comment,
                            )
                        }
                        EntryTypes::Settings(settings) => {
                            validate_create_settings(
                                EntryCreationAction::Create(action),
                                settings,
                            )
                        }
                    }
                }
                OpEntry::UpdateEntry { app_entry, action, .. } => {
                    match app_entry {
                        EntryTypes::Deliberation(deliberation) => {
                            validate_create_deliberation(
                                EntryCreationAction::Update(action),
                                deliberation,
                            )
                        }
                        EntryTypes::Criterion(criterion) => {
                            validate_create_criterion(
                                EntryCreationAction::Update(action),
                                criterion,
                            )
                        }
                        EntryTypes::Proposal(proposal) => {
                            validate_create_proposal(
                                EntryCreationAction::Update(action),
                                proposal,
                            )
                        }
                        EntryTypes::Outcome(outcome) => {
                            validate_create_outcome(
                                EntryCreationAction::Update(action),
                                outcome,
                            )
                        }
                        EntryTypes::CriterionComment(criterion_comment) => {
                            validate_create_criterion_comment(
                                EntryCreationAction::Update(action),
                                criterion_comment,
                            )
                        }
                        EntryTypes::Settings(settings) => {
                            validate_create_settings(
                                EntryCreationAction::Update(action),
                                settings,
                            )
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        FlatOp::RegisterUpdate(update_entry) => {
            match update_entry {
                OpUpdate::Entry {
                    original_action,
                    original_app_entry,
                    app_entry,
                    action,
                } => {
                    match (app_entry, original_app_entry) {
                        (
                            EntryTypes::Settings(settings),
                            EntryTypes::Settings(original_settings),
                        ) => {
                            validate_update_settings(
                                action,
                                settings,
                                original_action,
                                original_settings,
                            )
                        }
                        (
                            EntryTypes::CriterionComment(criterion_comment),
                            EntryTypes::CriterionComment(original_criterion_comment),
                        ) => {
                            validate_update_criterion_comment(
                                action,
                                criterion_comment,
                                original_action,
                                original_criterion_comment,
                            )
                        }
                        (
                            EntryTypes::Proposal(proposal),
                            EntryTypes::Proposal(original_proposal),
                        ) => {
                            validate_update_proposal(
                                action,
                                proposal,
                                original_action,
                                original_proposal,
                            )
                        }
                        (
                            EntryTypes::Outcome(outcome),
                            EntryTypes::Outcome(original_outcome),
                        ) => {
                            validate_update_outcome(
                                action,
                                outcome,
                                original_action,
                                original_outcome,
                            )
                        }
                        (
                            EntryTypes::Criterion(criterion),
                            EntryTypes::Criterion(original_criterion),
                        ) => {
                            validate_update_criterion(
                                action,
                                criterion,
                                original_action,
                                original_criterion,
                            )
                        }
                        (
                            EntryTypes::Deliberation(deliberation),
                            EntryTypes::Deliberation(original_deliberation),
                        ) => {
                            validate_update_deliberation(
                                action,
                                deliberation,
                                original_action,
                                original_deliberation,
                            )
                        }
                        _ => {
                            Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original and updated entry types must be the same"
                                        .to_string(),
                                ),
                            )
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        FlatOp::RegisterDelete(delete_entry) => {
            match delete_entry {
                OpDelete::Entry { original_action, original_app_entry, action } => {
                    match original_app_entry {
                        EntryTypes::Deliberation(deliberation) => {
                            validate_delete_deliberation(
                                action,
                                original_action,
                                deliberation,
                            )
                        }
                        EntryTypes::Criterion(criterion) => {
                            validate_delete_criterion(action, original_action, criterion)
                        }
                        EntryTypes::Proposal(proposal) => {
                            validate_delete_proposal(action, original_action, proposal)
                        }
                        EntryTypes::Outcome(outcome) => {
                            validate_delete_outcome(action, original_action, outcome)
                        }
                        EntryTypes::CriterionComment(criterion_comment) => {
                            validate_delete_criterion_comment(
                                action,
                                original_action,
                                criterion_comment,
                            )
                        }
                        EntryTypes::Settings(settings) => {
                            validate_delete_settings(action, original_action, settings)
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        FlatOp::RegisterCreateLink {
            link_type,
            base_address,
            target_address,
            tag,
            action,
        } => {
            match link_type {
                LinkTypes::DeliberationUpdates => {
                    validate_create_link_deliberation_updates(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllDeliberations => {
                    validate_create_link_all_deliberations(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::DeliberationToCriteria => {
                    validate_create_link_deliberation_to_criteria(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::CriterionToDeliberations => {
                    validate_create_link_criterion_to_deliberations(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::DeliberationToProposals => {
                    validate_create_link_deliberation_to_proposals(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::ProposalToDeliberations => {
                    validate_create_link_proposal_to_deliberations(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::DeliberationToOutcomes => {
                    validate_create_link_deliberation_to_outcomes(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::OutcomeToDeliberations => {
                    validate_create_link_outcome_to_deliberations(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::SupporterToCriteria => {
                    validate_create_link_supporter_to_criteria(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::CriterionToSupporters => {
                    validate_create_link_criterion_to_supporters(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::ProposalToCriteria => {
                    validate_create_link_proposal_to_criteria(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::ObjectorToCriteria => {
                    validate_create_link_objector_to_criteria(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::CriterionToObjectors => {
                    validate_create_link_criterion_to_objectors(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllCriteria => {
                    validate_create_link_all_criteria(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllProposals => {
                    validate_create_link_all_proposals(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllOutcomes => {
                    validate_create_link_all_outcomes(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::DeliberatorToDeliberations => {
                    validate_create_link_deliberator_to_deliberations(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::DeliberationToDeliberators => {
                    validate_create_link_deliberation_to_deliberators(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::CriterionToCriteria => {
                    validate_create_link_criterion_to_criteria(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::CriterionCommentUpdates => {
                    validate_create_link_criterion_comment_updates(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllCriterionComments => {
                    validate_create_link_all_criterion_comments(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::CriterionToCriterionComments => {
                    validate_create_link_criterion_to_criterion_comments(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::SettingsUpdates => {
                    validate_create_link_settings_updates(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::ProposalToOutcomes => {
                    validate_create_link_proposal_to_outcomes(
                        action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
            }
        }
        FlatOp::RegisterDeleteLink {
            link_type,
            base_address,
            target_address,
            tag,
            original_action,
            action,
        } => {
            match link_type {
                LinkTypes::DeliberationUpdates => {
                    validate_delete_link_deliberation_updates(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllDeliberations => {
                    validate_delete_link_all_deliberations(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::DeliberationToCriteria => {
                    validate_delete_link_deliberation_to_criteria(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::CriterionToDeliberations => {
                    validate_delete_link_criterion_to_deliberations(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::DeliberationToProposals => {
                    validate_delete_link_deliberation_to_proposals(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::ProposalToDeliberations => {
                    validate_delete_link_proposal_to_deliberations(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::DeliberationToOutcomes => {
                    validate_delete_link_deliberation_to_outcomes(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::OutcomeToDeliberations => {
                    validate_delete_link_outcome_to_deliberations(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::SupporterToCriteria => {
                    validate_delete_link_supporter_to_criteria(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::CriterionToSupporters => {
                    validate_delete_link_criterion_to_supporters(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::ProposalToCriteria => {
                    validate_delete_link_proposal_to_criteria(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::ObjectorToCriteria => {
                    validate_delete_link_objector_to_criteria(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::CriterionToObjectors => {
                    validate_delete_link_criterion_to_objectors(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllCriteria => {
                    validate_delete_link_all_criteria(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllProposals => {
                    validate_delete_link_all_proposals(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllOutcomes => {
                    validate_delete_link_all_outcomes(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::DeliberatorToDeliberations => {
                    validate_delete_link_deliberator_to_deliberations(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::DeliberationToDeliberators => {
                    validate_delete_link_deliberation_to_deliberators(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::CriterionToCriteria => {
                    validate_delete_link_criterion_to_criteria(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::CriterionCommentUpdates => {
                    validate_delete_link_criterion_comment_updates(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::AllCriterionComments => {
                    validate_delete_link_all_criterion_comments(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::CriterionToCriterionComments => {
                    validate_delete_link_criterion_to_criterion_comments(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::SettingsUpdates => {
                    validate_delete_link_settings_updates(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
                LinkTypes::ProposalToOutcomes => {
                    validate_delete_link_proposal_to_outcomes(
                        action,
                        original_action,
                        base_address,
                        target_address,
                        tag,
                    )
                }
            }
        }
        FlatOp::StoreRecord(store_record) => {
            match store_record {
                OpRecord::CreateEntry { app_entry, action } => {
                    match app_entry {
                        EntryTypes::Deliberation(deliberation) => {
                            validate_create_deliberation(
                                EntryCreationAction::Create(action),
                                deliberation,
                            )
                        }
                        EntryTypes::Criterion(criterion) => {
                            validate_create_criterion(
                                EntryCreationAction::Create(action),
                                criterion,
                            )
                        }
                        EntryTypes::Proposal(proposal) => {
                            validate_create_proposal(
                                EntryCreationAction::Create(action),
                                proposal,
                            )
                        }
                        EntryTypes::Outcome(outcome) => {
                            validate_create_outcome(
                                EntryCreationAction::Create(action),
                                outcome,
                            )
                        }
                        EntryTypes::CriterionComment(criterion_comment) => {
                            validate_create_criterion_comment(
                                EntryCreationAction::Create(action),
                                criterion_comment,
                            )
                        }
                        EntryTypes::Settings(settings) => {
                            validate_create_settings(
                                EntryCreationAction::Create(action),
                                settings,
                            )
                        }
                    }
                }
                OpRecord::UpdateEntry {
                    original_action_hash,
                    app_entry,
                    action,
                    ..
                } => {
                    let original_record = must_get_valid_record(original_action_hash)?;
                    let original_action = original_record.action().clone();
                    let original_action = match original_action {
                        Action::Create(create) => EntryCreationAction::Create(create),
                        Action::Update(update) => EntryCreationAction::Update(update),
                        _ => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original action for an update must be a Create or Update action"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    match app_entry {
                        EntryTypes::Deliberation(deliberation) => {
                            let result = validate_create_deliberation(
                                EntryCreationAction::Update(action.clone()),
                                deliberation.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_deliberation: Option<Deliberation> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_deliberation = match original_deliberation {
                                    Some(deliberation) => deliberation,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_deliberation(
                                    action,
                                    deliberation,
                                    original_action,
                                    original_deliberation,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::Criterion(criterion) => {
                            let result = validate_create_criterion(
                                EntryCreationAction::Update(action.clone()),
                                criterion.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_criterion: Option<Criterion> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_criterion = match original_criterion {
                                    Some(criterion) => criterion,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_criterion(
                                    action,
                                    criterion,
                                    original_action,
                                    original_criterion,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::Proposal(proposal) => {
                            let result = validate_create_proposal(
                                EntryCreationAction::Update(action.clone()),
                                proposal.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_proposal: Option<Proposal> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_proposal = match original_proposal {
                                    Some(proposal) => proposal,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_proposal(
                                    action,
                                    proposal,
                                    original_action,
                                    original_proposal,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::Outcome(outcome) => {
                            let result = validate_create_outcome(
                                EntryCreationAction::Update(action.clone()),
                                outcome.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_outcome: Option<Outcome> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_outcome = match original_outcome {
                                    Some(outcome) => outcome,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_outcome(
                                    action,
                                    outcome,
                                    original_action,
                                    original_outcome,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::CriterionComment(criterion_comment) => {
                            let result = validate_create_criterion_comment(
                                EntryCreationAction::Update(action.clone()),
                                criterion_comment.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_criterion_comment: Option<CriterionComment> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_criterion_comment = match original_criterion_comment {
                                    Some(criterion_comment) => criterion_comment,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_criterion_comment(
                                    action,
                                    criterion_comment,
                                    original_action,
                                    original_criterion_comment,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                        EntryTypes::Settings(settings) => {
                            let result = validate_create_settings(
                                EntryCreationAction::Update(action.clone()),
                                settings.clone(),
                            )?;
                            if let ValidateCallbackResult::Valid = result {
                                let original_settings: Option<Settings> = original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                                let original_settings = match original_settings {
                                    Some(settings) => settings,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                                validate_update_settings(
                                    action,
                                    settings,
                                    original_action,
                                    original_settings,
                                )
                            } else {
                                Ok(result)
                            }
                        }
                    }
                }
                OpRecord::DeleteEntry { original_action_hash, action, .. } => {
                    let original_record = must_get_valid_record(original_action_hash)?;
                    let original_action = original_record.action().clone();
                    let original_action = match original_action {
                        Action::Create(create) => EntryCreationAction::Create(create),
                        Action::Update(update) => EntryCreationAction::Update(update),
                        _ => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original action for a delete must be a Create or Update action"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    let app_entry_type = match original_action.entry_type() {
                        EntryType::App(app_entry_type) => app_entry_type,
                        _ => {
                            return Ok(ValidateCallbackResult::Valid);
                        }
                    };
                    let entry = match original_record.entry().as_option() {
                        Some(entry) => entry,
                        None => {
                            if original_action.entry_type().visibility().is_public() {
                                return Ok(
                                    ValidateCallbackResult::Invalid(
                                        "Original record for a delete of a public entry must contain an entry"
                                            .to_string(),
                                    ),
                                );
                            } else {
                                return Ok(ValidateCallbackResult::Valid);
                            }
                        }
                    };
                    let original_app_entry = match EntryTypes::deserialize_from_type(
                        app_entry_type.zome_index.clone(),
                        app_entry_type.entry_index.clone(),
                        &entry,
                    )? {
                        Some(app_entry) => app_entry,
                        None => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original app entry must be one of the defined entry types for this zome"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    match original_app_entry {
                        EntryTypes::Deliberation(original_deliberation) => {
                            validate_delete_deliberation(
                                action,
                                original_action,
                                original_deliberation,
                            )
                        }
                        EntryTypes::Criterion(original_criterion) => {
                            validate_delete_criterion(
                                action,
                                original_action,
                                original_criterion,
                            )
                        }
                        EntryTypes::Proposal(original_proposal) => {
                            validate_delete_proposal(
                                action,
                                original_action,
                                original_proposal,
                            )
                        }
                        EntryTypes::Outcome(original_outcome) => {
                            validate_delete_outcome(
                                action,
                                original_action,
                                original_outcome,
                            )
                        }
                        EntryTypes::CriterionComment(original_criterion_comment) => {
                            validate_delete_criterion_comment(
                                action,
                                original_action,
                                original_criterion_comment,
                            )
                        }
                        EntryTypes::Settings(original_settings) => {
                            validate_delete_settings(
                                action,
                                original_action,
                                original_settings,
                            )
                        }
                    }
                }
                OpRecord::CreateLink {
                    base_address,
                    target_address,
                    tag,
                    link_type,
                    action,
                } => {
                    match link_type {
                        LinkTypes::DeliberationUpdates => {
                            validate_create_link_deliberation_updates(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::AllDeliberations => {
                            validate_create_link_all_deliberations(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::DeliberationToCriteria => {
                            validate_create_link_deliberation_to_criteria(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::CriterionToDeliberations => {
                            validate_create_link_criterion_to_deliberations(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::DeliberationToProposals => {
                            validate_create_link_deliberation_to_proposals(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::ProposalToDeliberations => {
                            validate_create_link_proposal_to_deliberations(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::DeliberationToOutcomes => {
                            validate_create_link_deliberation_to_outcomes(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::OutcomeToDeliberations => {
                            validate_create_link_outcome_to_deliberations(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::SupporterToCriteria => {
                            validate_create_link_supporter_to_criteria(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::CriterionToSupporters => {
                            validate_create_link_criterion_to_supporters(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::ProposalToCriteria => {
                            validate_create_link_proposal_to_criteria(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::ObjectorToCriteria => {
                            validate_create_link_objector_to_criteria(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::CriterionToObjectors => {
                            validate_create_link_criterion_to_objectors(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::AllCriteria => {
                            validate_create_link_all_criteria(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::AllProposals => {
                            validate_create_link_all_proposals(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::AllOutcomes => {
                            validate_create_link_all_outcomes(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::DeliberatorToDeliberations => {
                            validate_create_link_deliberator_to_deliberations(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::DeliberationToDeliberators => {
                            validate_create_link_deliberation_to_deliberators(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::CriterionToCriteria => {
                            validate_create_link_criterion_to_criteria(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::CriterionCommentUpdates => {
                            validate_create_link_criterion_comment_updates(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::AllCriterionComments => {
                            validate_create_link_all_criterion_comments(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::CriterionToCriterionComments => {
                            validate_create_link_criterion_to_criterion_comments(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::SettingsUpdates => {
                            validate_create_link_settings_updates(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                        LinkTypes::ProposalToOutcomes => {
                            validate_create_link_proposal_to_outcomes(
                                action,
                                base_address,
                                target_address,
                                tag,
                            )
                        }
                    }
                }
                OpRecord::DeleteLink { original_action_hash, base_address, action } => {
                    let record = must_get_valid_record(original_action_hash)?;
                    let create_link = match record.action() {
                        Action::CreateLink(create_link) => create_link.clone(),
                        _ => {
                            return Ok(
                                ValidateCallbackResult::Invalid(
                                    "The action that a DeleteLink deletes must be a CreateLink"
                                        .to_string(),
                                ),
                            );
                        }
                    };
                    let link_type = match LinkTypes::from_type(
                        create_link.zome_index.clone(),
                        create_link.link_type.clone(),
                    )? {
                        Some(lt) => lt,
                        None => {
                            return Ok(ValidateCallbackResult::Valid);
                        }
                    };
                    match link_type {
                        LinkTypes::DeliberationUpdates => {
                            validate_delete_link_deliberation_updates(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::AllDeliberations => {
                            validate_delete_link_all_deliberations(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::DeliberationToCriteria => {
                            validate_delete_link_deliberation_to_criteria(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::CriterionToDeliberations => {
                            validate_delete_link_criterion_to_deliberations(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::DeliberationToProposals => {
                            validate_delete_link_deliberation_to_proposals(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ProposalToDeliberations => {
                            validate_delete_link_proposal_to_deliberations(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::DeliberationToOutcomes => {
                            validate_delete_link_deliberation_to_outcomes(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::OutcomeToDeliberations => {
                            validate_delete_link_outcome_to_deliberations(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::SupporterToCriteria => {
                            validate_delete_link_supporter_to_criteria(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::CriterionToSupporters => {
                            validate_delete_link_criterion_to_supporters(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ProposalToCriteria => {
                            validate_delete_link_proposal_to_criteria(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ObjectorToCriteria => {
                            validate_delete_link_objector_to_criteria(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::CriterionToObjectors => {
                            validate_delete_link_criterion_to_objectors(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::AllCriteria => {
                            validate_delete_link_all_criteria(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::AllProposals => {
                            validate_delete_link_all_proposals(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::AllOutcomes => {
                            validate_delete_link_all_outcomes(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::DeliberatorToDeliberations => {
                            validate_delete_link_deliberator_to_deliberations(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::DeliberationToDeliberators => {
                            validate_delete_link_deliberation_to_deliberators(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::CriterionToCriteria => {
                            validate_delete_link_criterion_to_criteria(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::CriterionCommentUpdates => {
                            validate_delete_link_criterion_comment_updates(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::AllCriterionComments => {
                            validate_delete_link_all_criterion_comments(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::CriterionToCriterionComments => {
                            validate_delete_link_criterion_to_criterion_comments(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::SettingsUpdates => {
                            validate_delete_link_settings_updates(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                        LinkTypes::ProposalToOutcomes => {
                            validate_delete_link_proposal_to_outcomes(
                                action,
                                create_link.clone(),
                                base_address,
                                create_link.target_address,
                                create_link.tag,
                            )
                        }
                    }
                }
                OpRecord::CreatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CreateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CreateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::Dna { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::OpenChain { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::CloseChain { .. } => Ok(ValidateCallbackResult::Valid),
                OpRecord::InitZomesComplete { .. } => Ok(ValidateCallbackResult::Valid),
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
        FlatOp::RegisterAgentActivity(agent_activity) => {
            match agent_activity {
                OpActivity::CreateAgent { agent, action } => {
                    let previous_action = must_get_action(action.prev_action)?;
                    match previous_action.action() {
                        Action::AgentValidationPkg(
                            AgentValidationPkg { membrane_proof, .. },
                        ) => validate_agent_joining(agent, membrane_proof),
                        _ => {
                            Ok(
                                ValidateCallbackResult::Invalid(
                                    "The previous action for a `CreateAgent` action must be an `AgentValidationPkg`"
                                        .to_string(),
                                ),
                            )
                        }
                    }
                }
                _ => Ok(ValidateCallbackResult::Valid),
            }
        }
    }
}
