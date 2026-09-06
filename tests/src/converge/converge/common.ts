import { CallableCell } from '@holochain-open-dev/tryorama';
import { ActionHash, Record, fakeActionHash, fakeAgentPubKey, fakeEntryHash, fakeDnaHash } from '@holochain/client';



export async function sampleDeliberation(cell: CallableCell, partialDeliberation = {}) {
    return {
        ...{
	  title: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  settings: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialDeliberation
    };
}

export async function createDeliberation(cell: CallableCell, deliberation = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "converge",
      fn_name: "create_deliberation",
      payload: deliberation || await sampleDeliberation(cell),
    });
}



export async function sampleCriterion(cell: CallableCell, partialCriterion = {}) {
    return {
        ...{
	  title: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialCriterion
    };
}

// `create_criterion` takes `CreateCriterionInput { criterion, deliberation }`, not a
// bare Criterion. The scaffolded helper sent the bare entry, which fails to
// deserialize in the coordinator. A Criterion is always created against a
// Deliberation, so create one when the caller does not supply a hash.
export async function createCriterion(
    cell: CallableCell,
    criterion = undefined,
    deliberationHash: ActionHash | undefined = undefined,
): Promise<Record> {
    const deliberation = deliberationHash
        ?? (await createDeliberation(cell)).signed_action.hashed.hash;
    return cell.callZome({
      zome_name: "converge",
      fn_name: "create_criterion",
      payload: {
        criterion: criterion || await sampleCriterion(cell),
        deliberation,
      },
    });
}



export async function sampleProposal(cell: CallableCell, partialProposal = {}) {
    return {
        ...{
	  title: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  description: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialProposal
    };
}

// `create_proposal` takes `CreateProposalInput { proposal, deliberation }`; same
// scaffold mismatch as createCriterion above.
export async function createProposal(
    cell: CallableCell,
    proposal = undefined,
    deliberationHash: ActionHash | undefined = undefined,
): Promise<Record> {
    const deliberation = deliberationHash
        ?? (await createDeliberation(cell)).signed_action.hashed.hash;
    return cell.callZome({
      zome_name: "converge",
      fn_name: "create_proposal",
      payload: {
        proposal: proposal || await sampleProposal(cell),
        deliberation,
      },
    });
}



export async function sampleCriterionComment(cell: CallableCell, partialCriterionComment = {}) {
    return {
        ...{
	  comment: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
	  comment_reference: (await fakeActionHash()),
	  objection_reference: (await fakeActionHash()),
	  alternative_reference: (await fakeActionHash()),
	  author: (await fakeAgentPubKey()),
	  created: 1674053334548000,
        },
        ...partialCriterionComment
    };
}

// `create_criterion_comment` takes
// `CreateCriterionCommentInput { criterion_comment, criterion_hash }`; same
// scaffold mismatch. A comment always hangs off a Criterion.
export async function createCriterionComment(
    cell: CallableCell,
    criterionComment = undefined,
    criterionHash: ActionHash | undefined = undefined,
): Promise<Record> {
    const criterion_hash = criterionHash
        ?? (await createCriterion(cell)).signed_action.hashed.hash;
    return cell.callZome({
      zome_name: "converge",
      fn_name: "create_criterion_comment",
      payload: {
        criterion_comment: criterionComment || await sampleCriterionComment(cell),
        criterion_hash,
      },
    });
}



export async function sampleSettings(cell: CallableCell, partialSettings = {}) {
    return {
        ...{
	  discussion_app: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        },
        ...partialSettings
    };
}

export async function createSettings(cell: CallableCell, settings = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "converge",
      fn_name: "create_settings",
      payload: settings || await sampleSettings(cell),
    });
}



export async function sampleViewed(cell: CallableCell, partialViewed = {}) {
    return {
        ...{
	  viewed_hash: (await fakeActionHash()),
	  viewed_date: 1674053334548000,
        },
        ...partialViewed
    };
}

export async function createViewed(cell: CallableCell, viewed = undefined): Promise<Record> {
    return cell.callZome({
      zome_name: "converge",
      fn_name: "create_viewed",
      payload: viewed || await sampleViewed(cell),
    });
}

