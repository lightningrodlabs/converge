import { assert, test } from "vitest";

import { runScenario, pause, CallableCell } from '@holochain/tryorama';
import { NewEntryAction, ActionHash, Record, AppBundleSource, fakeDnaHash, fakeActionHash, fakeAgentPubKey, fakeEntryHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';

import { createCriterionComment, sampleCriterionComment } from './common.js';

test('create CriterionComment', async () => {
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

    // Alice creates a CriterionComment
    const record: Record = await createCriterionComment(alice.cells[0]);
    assert.ok(record);
  });
});

test('create and read CriterionComment', async () => {
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

    const sample = await sampleCriterionComment(alice.cells[0]);

    // Alice creates a CriterionComment
    const record: Record = await createCriterionComment(alice.cells[0], sample);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    await pause(1200);

    // Bob gets the created CriterionComment
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "converge",
      fn_name: "get_criterion_comment",
      payload: record.signed_action.hashed.hash,
    });
    assert.deepEqual(sample, decode((createReadOutput.entry as any).Present.entry) as any);
  });
});

test('create and update CriterionComment', async () => {
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

    // Alice creates a CriterionComment
    const record: Record = await createCriterionComment(alice.cells[0]);
    assert.ok(record);
        
    const originalActionHash = record.signed_action.hashed.hash;
 
    // Alice updates the CriterionComment
    let contentUpdate: any = await sampleCriterionComment(alice.cells[0]);
    let updateInput = {
      original_criterion_comment_hash: originalActionHash,
      previous_criterion_comment_hash: originalActionHash,
      updated_criterion_comment: contentUpdate,
    };

    let updatedRecord: Record = await alice.cells[0].callZome({
      zome_name: "converge",
      fn_name: "update_criterion_comment",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await pause(1200);
        
    // Bob gets the updated CriterionComment
    const readUpdatedOutput0: Record = await bob.cells[0].callZome({
      zome_name: "converge",
      fn_name: "get_criterion_comment",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput0.entry as any).Present.entry) as any);

    // Alice updates the CriterionComment again
    contentUpdate = await sampleCriterionComment(alice.cells[0]);
    updateInput = { 
      original_criterion_comment_hash: originalActionHash,
      previous_criterion_comment_hash: updatedRecord.signed_action.hashed.hash,
      updated_criterion_comment: contentUpdate,
    };

    updatedRecord = await alice.cells[0].callZome({
      zome_name: "converge",
      fn_name: "update_criterion_comment",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await pause(1200);
        
    // Bob gets the updated CriterionComment
    const readUpdatedOutput1: Record = await bob.cells[0].callZome({
      zome_name: "converge",
      fn_name: "get_criterion_comment",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput1.entry as any).Present.entry) as any);
  });
});

test('create and delete CriterionComment', async () => {
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

    // Alice creates a CriterionComment
    const record: Record = await createCriterionComment(alice.cells[0]);
    assert.ok(record);
        
    // Alice deletes the CriterionComment
    const deleteActionHash = await alice.cells[0].callZome({
      zome_name: "converge",
      fn_name: "delete_criterion_comment",
      payload: record.signed_action.hashed.hash,
    });
    assert.ok(deleteActionHash);

    // Wait for the entry deletion to be propagated to the other node.
    await pause(1200);
        
    // Bob tries to get the deleted CriterionComment
    const readDeletedOutput = await bob.cells[0].callZome({
      zome_name: "converge",
      fn_name: "get_criterion_comment",
      payload: record.signed_action.hashed.hash,
    });
    assert.notOk(readDeletedOutput);
  });
});
