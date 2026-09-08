import { assert, test } from "vitest";

import { dhtSync, runScenario, CallableCell } from '@holochain-open-dev/tryorama';
import { ActionHash, Record, fakeDnaHash, fakeActionHash, fakeAgentPubKey, fakeEntryHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';

import { createSettings, sampleSettings } from './common.js';

test('create Settings', async () => {
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

    // Alice creates a Settings
    const record: Record = await createSettings(alice.cells[0]);
    assert.ok(record);
  });
});

test('create and read Settings', async () => {
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

    const sample = await sampleSettings(alice.cells[0]);

    // Alice creates a Settings
    const record: Record = await createSettings(alice.cells[0], sample);
    assert.ok(record);

    // Wait for the created entry to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);

    // Bob gets the created Settings
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "converge",
      fn_name: "get_settings",
      payload: record.signed_action.hashed.hash,
    });
    assert.deepEqual(sample, decode((createReadOutput.entry as any).Present.entry) as any);
  });
});

test('create and update Settings', async () => {
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

    // Alice creates a Settings
    const record: Record = await createSettings(alice.cells[0]);
    assert.ok(record);
        
    const originalActionHash = record.signed_action.hashed.hash;
 
    // Alice updates the Settings
    let contentUpdate: any = await sampleSettings(alice.cells[0]);
    let updateInput = {
      original_settings_hash: originalActionHash,
      previous_settings_hash: originalActionHash,
      updated_settings: contentUpdate,
    };

    let updatedRecord: Record = await alice.cells[0].callZome({
      zome_name: "converge",
      fn_name: "update_settings",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);
        
    // Bob gets the updated Settings
    const readUpdatedOutput0: Record = await bob.cells[0].callZome({
      zome_name: "converge",
      fn_name: "get_settings",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput0.entry as any).Present.entry) as any);

    // Alice updates the Settings again
    contentUpdate = await sampleSettings(alice.cells[0]);
    updateInput = { 
      original_settings_hash: originalActionHash,
      previous_settings_hash: updatedRecord.signed_action.hashed.hash,
      updated_settings: contentUpdate,
    };

    updatedRecord = await alice.cells[0].callZome({
      zome_name: "converge",
      fn_name: "update_settings",
      payload: updateInput,
    });
    assert.ok(updatedRecord);

    // Wait for the updated entry to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);
        
    // Bob gets the updated Settings
    const readUpdatedOutput1: Record = await bob.cells[0].callZome({
      zome_name: "converge",
      fn_name: "get_settings",
      payload: updatedRecord.signed_action.hashed.hash,
    });
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput1.entry as any).Present.entry) as any);
  });
});

test('create and delete Settings', async () => {
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

    // Alice creates a Settings
    const record: Record = await createSettings(alice.cells[0]);
    assert.ok(record);
        
    // Alice deletes the Settings
    const deleteActionHash = await alice.cells[0].callZome({
      zome_name: "converge",
      fn_name: "delete_settings",
      payload: record.signed_action.hashed.hash,
    });
    assert.ok(deleteActionHash);

    // Wait for the entry deletion to be propagated to the other node.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0]);
        
    // Bob tries to get the deleted Settings
    const readDeletedOutput = await bob.cells[0].callZome({
      zome_name: "converge",
      fn_name: "get_settings",
      payload: record.signed_action.hashed.hash,
    });
    assert.notOk(readDeletedOutput);
  });
});

// Regression test for the SettingsUpdates link failing to validate on a remote
// authority.
//
// Settings is a PRIVATE entry, so a remote validation authority receives the
// action but never the entry content. validate_create_link_settings_updates used
// to decode the entry, which only succeeds on the author's own machine, so the
// link never integrated anywhere else and get_settings kept resolving to the
// original for every agent except its author.
//
// The existing 'create and update Settings' test does not catch this because it
// reads back by the UPDATED action hash, which bypasses the link. This test reads
// by the ORIGINAL hash from the non-authoring agent, which is the only path that
// actually requires the link to have integrated. It asserts on the resolved
// action hash rather than the entry content, because a private entry's content
// is legitimately unavailable to Bob.
test('SettingsUpdates link resolves for a non-author (private entry)', async () => {
  await runScenario(async scenario => {
    const testAppPath = process.cwd() + '/../workdir/converge.happ';
    const appSource = { appBundleSource: { type: 'path' as const, value: testAppPath } };
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);
    await scenario.shareAllAgents();

    const originalRecord: Record = await createSettings(alice.cells[0]);
    const originalActionHash = originalRecord.signed_action.hashed.hash;

    const contentUpdate = await sampleSettings(alice.cells[0]);
    const updatedRecord: Record = await alice.cells[0].callZome({
      zome_name: "converge",
      fn_name: "update_settings",
      payload: {
        original_settings_hash: originalActionHash,
        previous_settings_hash: originalActionHash,
        updated_settings: contentUpdate,
      },
    });
    assert.ok(updatedRecord);

    // 60s (the default) is not reliably enough for this scenario on a loaded
    // machine: the same timeout also fires in 'create and read Settings', so it
    // is a property of the suite rather than of this test.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0], 1000, 180_000);

    // Bob resolves from the ORIGINAL hash. This only reaches the updated action
    // if the SettingsUpdates link validated and integrated on his authority.
    const bobResolved: Record | null = await bob.cells[0].callZome({
      zome_name: "converge",
      fn_name: "get_settings",
      payload: originalActionHash,
    });

    assert.ok(bobResolved, "Bob resolved nothing for the original Settings hash");
    assert.deepEqual(
      bobResolved.signed_action.hashed.hash,
      updatedRecord.signed_action.hashed.hash,
      "Bob resolved the ORIGINAL Settings, not the update — the SettingsUpdates link did not integrate on his authority",
    );
  });
});
