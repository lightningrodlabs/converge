import type { Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import type { Proposal, CreateProposalInput } from './converge/converge/types';
import { addDeliberation, addSomeProposals, addSomeEvaluations, addProposalHashesToDeliberation } from './store';
import { decode } from "@msgpack/msgpack";
import { encodeHashToBase64, decodeHashFromBase64 } from "@holochain/client";

export async function createDeliberation(deliberationEntry, client) {  
  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'create_deliberation',
      payload: deliberationEntry,
    });

    await joinDeliberation(record.signed_action.hashed.hash, client)

    await addDeliberation({
      action_hash: record.signed_action.hashed.hash,
      deliberation: deliberationEntry,
      deliberators: [{completed: false, deliberator: client.myPubKey}],
      criteria: [],
      outcomes: [],
      proposals: []
    })
    return record.signed_action.hashed.hash
  } catch (e) {
    console.log("error", e)
  }
}

export async function joinDeliberation(deliberationHash, client) {
  await client.callZome({
    cap_secret: null,
    role_name: 'converge',
    zome_name: 'converge',
    fn_name: 'add_deliberation_for_deliberator',
    payload: {
      base_deliberator: client.myPubKey,
      target_deliberation_hash: deliberationHash
    },
  });
}

export async function leaveDeliberation(deliberationHash: ActionHash, client) {
  try {
    await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'remove_deliberation_for_deliberator',
      payload: {
        base_deliberator: client.myPubKey,
        target_deliberation_hash: deliberationHash
      },
    });
  } catch (e: any) {
    console.log("error", e)
  }
}

export async function addSimpleEvaluation(evaluation, client) {
  try {
    console.log("evaluation", evaluation)
    await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'add_evaluator_for_proposal',
      payload: evaluation,
    });
    console.log("evaluation", evaluation)
    addSomeEvaluations([{
      base_proposal_hash: encodeHashToBase64(evaluation.base_proposal_hash),
      target_evaluator: encodeHashToBase64(evaluation.target_evaluator),
      tag: evaluation.tag
    }])
  } catch (e: any) {
    console.log("error", e)
  }
}

export async function createProposal(createProposalInput: CreateProposalInput, client) {  
  try {
    let record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'create_proposal',
      payload: createProposalInput,
    });
    console.log(decode((record.entry as any).Present.entry) as Proposal)
    addSomeProposals([{...decode((record.entry as any).Present.entry) as Proposal,
      action_hash: record.signed_action.hashed.hash,
    }])
    addProposalHashesToDeliberation(createProposalInput.deliberation, [record.signed_action.hashed.hash])
    return record.signed_action.hashed.hash
  } catch (e) {
    console.log("error", e)
  }
}
