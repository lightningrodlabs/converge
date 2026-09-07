import { assert, test } from "vitest";

import { dhtSync, runScenario } from "@holochain-open-dev/tryorama";
import { ActionHash, Record } from "@holochain/client";

import {
  createCriterion,
  createCriterionComment,
  createDeliberation,
  createSettings,
  sampleCriterionComment,
  sampleDeliberation,
  sampleSettings,
} from "./common.js";

/**
 * Coverage for the `converge_integrity` validate callback across the op arms this
 * DNA actually reaches. Written for the 0.7 upgrade, where every arm of `validate`
 * was rewritten for hdi 0.8 and a silent behaviour change would be frozen into the
 * DNA hash.
 *
 * Which arms this drives, and how that was checked (mutation testing, see
 * recipe R10): mutating `validate_create_deliberation` to return
 * `Invalid("...")` makes the create step fail; mutating
 * `validate_create_link_deliberation_to_criteria` makes the criterion step fail.
 * Both probes were run and both flipped. Re-run them if the integrity zome or
 * `dnas/converge/workdir/dna.yaml` changes.
 *
 * Arms reached:
 *   - `FlatOp::CreateRecord` / `OpRecord::CreateEntry`  (author, Deliberation + Criterion)
 *   - `FlatOp::CreateEntry`  / `OpEntry::CreateEntry`   (entry authority, both agents)
 *   - `FlatOp::CreateRecord` / `OpRecord::CreateLink`   (author, 2 link types)
 *   - `FlatOp::Link(OpLink::CreateLink)`                (link authority, both agents)
 *   - `FlatOp::Update`  / `OpUpdate::Entry`             (entry authority — see below)
 *   - `FlatOp::Delete`  / `OpDelete`                    (entry authority — see below)
 *   - `FlatOp::AgentActivity`                           (every action on every chain)
 *
 * Not reached: the delete-link arms (no coordinator extern deletes a link that
 * has a `LinkTypes` this zome recognises through this path) and the private
 * entry types.
 */
test("integrity zome accepts a Deliberation and its Criterion links on both agents", async () => {
  await runScenario(async (scenario) => {
    const testAppPath = process.cwd() + "/../workdir/converge.happ";
    const appSource = { appBundleSource: { type: "path" as const, value: testAppPath } };
    const [alice, bob] = await scenario.addPlayersWithApps([appSource, appSource]);
    await scenario.shareAllAgents();

    const sample = await sampleDeliberation(alice.cells[0]);
    const deliberation: Record = await createDeliberation(alice.cells[0], sample);
    assert.ok(deliberation);

    // Gate G2b, empirically: the entry must be attributed to `converge_integrity`,
    // which is zome 0 in `dnas/converge/workdir/dna.yaml`'s `integrity:` list.
    // A wrong `dependencies:` line in that manifest would silently make this a
    // different index and route every op to the other integrity zome.
    const entryType = (deliberation.signed_action.hashed.content as any).data.entry_type;
    assert.equal(entryType.App.zome_index, 0, "entries must be attributed to converge_integrity");

    const deliberationHash: ActionHash = deliberation.signed_action.hashed.hash;

    // Sync BEFORE creating the criterion. The link validators call
    // `must_get_valid_record` on base and target, so the link ops sit in
    // awaiting-deps on Bob until he has integrated the deliberation; creating
    // both back to back and syncing once does not converge inside the default
    // 60 s.
    await dhtSync([alice, bob], alice.cells[0].cell_id[0], 500, 120_000);

    // Creates the entry plus the DeliberationToCriteria and CriterionToDeliberations
    // links, whose validators both `must_get_valid_record` base and target.
    const criterion: Record = await createCriterion(alice.cells[0], undefined, deliberationHash);
    assert.ok(criterion);

    await dhtSync([alice, bob], alice.cells[0].cell_id[0], 500, 120_000);

    // Bob is an authority for these ops too; reading them back proves his
    // conductor integrated them, i.e. his copy of `validate` returned Valid.
    // `get_deliberation` returns `RecordWithLinks`, not a bare Record.
    const bobDeliberation: any = await bob.cells[0].callZome({
      zome_name: "converge",
      fn_name: "get_deliberation",
      payload: deliberationHash,
    });
    assert.ok(bobDeliberation);
    assert.ok(bobDeliberation.record);

    const bobCriteria: Record[] = await bob.cells[0].callZome({
      zome_name: "converge",
      fn_name: "get_criteria_for_deliberation",
      payload: deliberationHash,
    });
    assert.equal(bobCriteria.length, 1);
  });
}, 300_000);

/**
 * Update and delete dispatch, added 2026-09-07.
 *
 * Until then `validate` carried the scaffold's blanket
 * `FlatOp::Update => Invalid("Entry cannot be updated")` and
 * `FlatOp::Delete => Invalid("Entry cannot be deleted")`, which overrode all
 * fourteen per-type `validate_update_*` / `validate_delete_*` functions and made
 * ten coordinator externs and three `Edit*.svelte` components unreachable. Both
 * arms now dispatch to the per-type validators, so each type's declared rule is
 * what applies:
 *
 *   Deliberation / CriterionComment / Settings  `validate_update_*` -> Valid
 *   Criterion / Proposal / Outcome / Viewed     `validate_update_*` -> Invalid,
 *                                               with their own messages
 *   all seven                                   `validate_delete_*` -> Valid
 *
 * The four rejecting types have no `update_*` coordinator extern, so their
 * `Invalid` messages are not reachable from this suite; they were proven by
 * building a throwaway `update_criterion` extern, which returned
 * `Validation failed while committing: Criteria cannot be updated` rather than
 * the old blanket string. That extern is deliberately not committed.
 */
test("integrity zome allows the updates and deletes its per-type validators allow", async () => {
  await runScenario(async (scenario) => {
    const testAppPath = process.cwd() + "/../workdir/converge.happ";
    const appSource = { appBundleSource: { type: "path" as const, value: testAppPath } };
    const [alice] = await scenario.addPlayersWithApps([appSource]);
    const cell = alice.cells[0];

    // --- Deliberation: validate_update_deliberation -> Valid --------------------
    const deliberation: Record = await createDeliberation(cell);
    const deliberationHash: ActionHash = deliberation.signed_action.hashed.hash;

    const updatedDeliberation: Record = await cell.callZome({
      zome_name: "converge",
      fn_name: "update_deliberation",
      payload: {
        original_deliberation_hash: deliberationHash,
        previous_deliberation_hash: deliberationHash,
        updated_deliberation: await sampleDeliberation(cell, { title: "changed" }),
      },
    });
    assert.ok(updatedDeliberation, "update_deliberation must be accepted");

    // --- Settings (private entry): validate_update_settings -> Valid ------------
    const settings: Record = await createSettings(cell);
    const settingsHash: ActionHash = settings.signed_action.hashed.hash;

    const updatedSettings: Record = await cell.callZome({
      zome_name: "converge",
      fn_name: "update_settings",
      payload: {
        original_settings_hash: settingsHash,
        previous_settings_hash: settingsHash,
        updated_settings: await sampleSettings(cell, { discussion_app: "changed" }),
      },
    });
    assert.ok(updatedSettings, "update_settings must be accepted");

    // --- CriterionComment: validate_update_criterion_comment -> Valid -----------
    const criterion: Record = await createCriterion(cell, undefined, deliberationHash);
    const criterionHash: ActionHash = criterion.signed_action.hashed.hash;

    const comment: Record = await createCriterionComment(cell, undefined, criterionHash);
    const commentHash: ActionHash = comment.signed_action.hashed.hash;

    const updatedComment: Record = await cell.callZome({
      zome_name: "converge",
      fn_name: "update_criterion_comment",
      payload: {
        original_criterion_comment_hash: commentHash,
        previous_criterion_comment_hash: commentHash,
        updated_criterion_comment: await sampleCriterionComment(cell, { comment: "changed" }),
      },
    });
    assert.ok(updatedComment, "update_criterion_comment must be accepted");

    // --- Deletes: all seven validate_delete_* return Valid ----------------------
    const deletedCriterion: ActionHash = await cell.callZome({
      zome_name: "converge",
      fn_name: "delete_criterion",
      payload: criterionHash,
    });
    assert.ok(deletedCriterion, "delete_criterion must be accepted");

    const deletedDeliberation: ActionHash = await cell.callZome({
      zome_name: "converge",
      fn_name: "delete_deliberation",
      payload: deliberationHash,
    });
    assert.ok(deletedDeliberation, "delete_deliberation must be accepted");
  });
}, 300_000);
