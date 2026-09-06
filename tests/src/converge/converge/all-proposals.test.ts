import { assert, test } from "vitest";

import { dhtSync, runScenario, CallableCell } from '@holochain-open-dev/tryorama';
import { ActionHash, Record, fakeActionHash, fakeAgentPubKey, fakeEntryHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';

import { createProposal } from './common.js';

test('create a Proposal and get all proposals', async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/converge.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { type: 'path' as const, value: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    // Bob gets all proposals
    let collectionOutput: Record[] = await bob.cells[0].callZome({
      zome_name: "converge",
      fn_name: "get_all_proposals",
      payload: null
    });
    assert.equal(collectionOutput.length, 0);

    // Alice creates a Proposal
    const createdRecord: Record = await createProposal(alice.cells[0]);
    assert.ok(createdRecord);
    
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);
    
    // Bob gets all proposals again
    collectionOutput = await bob.cells[0].callZome({
      zome_name: "converge",
      fn_name: "get_all_proposals",
      payload: null
    });
    assert.equal(collectionOutput.length, 1);
    assert.deepEqual(createdRecord, collectionOutput[0]);    
  });
});

