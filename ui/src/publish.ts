import type { Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { addDeliberation } from './store';

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