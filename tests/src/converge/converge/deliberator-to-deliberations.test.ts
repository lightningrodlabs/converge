import { assert, test } from "vitest";

import { runScenario, pause, CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource,  fakeActionHash, fakeAgentPubKey, fakeEntryHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';

import { createDeliberation } from './common.js';

test('link a Deliberator to a Deliberation', async () => {
  await runScenario(async scenario => {
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + '/../workdir/converge.happ';

    // Set up the app to be installed 
    const appSource = { appBundleSource: { path: testAppPath } };

    // Add 2 players with the test app to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    const baseAddress = alice.agentPubKey;
    const targetRecord = await createDeliberation(alice.cells[0]);
    const targetAddress = targetRecord.signed_action.hashed.hash;

    // Bob gets the links, should be empty
    let linksOutput: Record[] = await bob.cells[0].callZome({
      zome_name: "converge",
      fn_name: "get_deliberations_for_deliberator",
      payload: baseAddress
    });
    assert.equal(linksOutput.length, 0);

    // Alice creates a link from Deliberator to Deliberation
    await alice.cells[0].callZome({
      zome_name: "converge",
      fn_name: "add_deliberation_for_deliberator",
      payload: {
        base_deliberator: baseAddress,
        target_deliberation_hash: targetAddress
      }
    });
    
    await pause(1200);
    
    // Bob gets the links again
    linksOutput = await bob.cells[0].callZome({
      zome_name: "converge",
      fn_name: "get_deliberations_for_deliberator",
      payload: baseAddress
    });
    assert.equal(linksOutput.length, 1);
    assert.deepEqual(targetRecord, linksOutput[0]);


    // Bob gets the links in the inverse direction
    linksOutput = await bob.cells[0].callZome({
      zome_name: "converge",
      fn_name: "get_deliberators_for_deliberation",
      payload: targetAddress
    });
    assert.equal(linksOutput.length, 1);

    await alice.cells[0].callZome({
      zome_name: "converge",
      fn_name: "remove_deliberation_for_deliberator",
      payload: {
        base_deliberator: baseAddress,
        target_deliberation_hash: targetAddress
      }
    });
    
    await pause(1200);

    // Bob gets the links again
    linksOutput = await bob.cells[0].callZome({
      zome_name: "converge",
      fn_name: "get_deliberations_for_deliberator",
      payload: baseAddress
    });
    assert.equal(linksOutput.length, 0);

    // Bob gets the links in the inverse direction
    linksOutput = await bob.cells[0].callZome({
      zome_name: "converge",
      fn_name: "get_deliberators_for_deliberation",
      payload: targetAddress
    });
    assert.equal(linksOutput.length, 0);

  });
});
