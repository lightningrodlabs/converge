<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { Record, EntryHash, ActionHash, AgentPubKey, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../../contexts';
import DeliberationDetail from './DeliberationDetail.svelte';
import type { ConvergeSignal } from '../types';
    import DeliberationListItem from './DeliberationListItem.svelte';
    import { view, viewHash, navigate } from '../../../store.js';

export let deliberator: AgentPubKey;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;
let completedHashes: Array<ActionHash> | undefined;

let loading = true;
let error: any = undefined;

$: hashes, completedHashes, loading, error;



async function joinDeliberation(deliberationHash: ActionHash) {
  try {
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
  } catch (e: any) {
    console.log("error", e)
  }
}

async function leaveDeliberation(deliberationHash: ActionHash) {
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

async function completeDeliberation(deliberationHash: ActionHash) {
  completedHashes = [...completedHashes, deliberationHash];
  hashes = hashes.filter(h => h !== deliberationHash);
  await leaveDeliberation(deliberationHash);
  try {
    await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'add_completed_tag',
      payload: deliberationHash,
    });
  } catch (e: any) {
    console.log("error", e)
  }
}

async function rejoinDeliberation(deliberationHash: ActionHash) {
  completedHashes = completedHashes.filter(h => h !== deliberationHash);
  hashes = [...hashes, deliberationHash];
  await new Promise(r => setTimeout(r, 200));
  try {
    await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'remove_completed_tag',
      payload: deliberationHash,
    });
    await new Promise(r => setTimeout(r, 200));
    await joinDeliberation(deliberationHash);
    // await fetchDeliberations();
  } catch (e: any) {
    console.log("error", e)
  }
}

async function fetchDeliberations() {
  try {
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'get_deliberations_for_deliberator',
      payload: deliberator,
    });
    console.log("reco", records)
    hashes = records.uncompleted.map(r => r.signed_action.hashed.hash);
    completedHashes = records.completed.map(r => r.signed_action.hashed.hash);
  } catch (e) {
    error = e;
  }
}

onMount(async () => {
  if (deliberator === undefined) {
    throw new Error(`The deliberator input is required for the DeliberationsForDeliberator element`);
  }

  await fetchDeliberations()
  loading = false;

  // client.on('signal', signal => {
  //   if (signal.zome_name !== 'converge') return;
  //   const payload = signal.payload as ConvergeSignal;
  //   if (payload.type !== 'LinkCreated') return;
  //   if (payload.link_type !== 'DeliberatorToDeliberations') return;

  //   hashes = [...hashes, payload.action.hashed.content.target_address];
  // });
});

</script>

<br>
<h2>Deliberations I've joined</h2>

{#if loading }
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching deliberations: {error}.</span>
{:else if hashes.length === 0 && completedHashes.length === 0}
<span>No deliberations found for this deliberator.</span>
{:else}
<div style="display: flex; flex-direction: column">
  {#each hashes.reverse() as hash}
  <div style="margin-bottom: 8px; display: flex; justify-content: space-between;">
    <!-- <DeliberationDetail deliberationHash={hash}></DeliberationDetail> -->
      <span on:click={() => navigate('deliberation', hash)} style="width: 100%;">
        <DeliberationListItem deliberationHash={hash}></DeliberationListItem>
      </span>
      <button class="complete-button" on:click={() => completeDeliberation(hash)}>Mark complete</button>
    </div>
  {/each}
</div>

{#if completedHashes.length > 0}
<div style="display: flex; flex-direction: column">
  <h3>Completed</h3>
  {#each completedHashes.reverse() as hash}
    <div style="margin-bottom: 8px; display: flex; justify-content: space-between;">
      <!-- <DeliberationDetail deliberationHash={hash}></DeliberationDetail> -->
      <span on:click={() => navigate('deliberation', hash)} style="width: 100%;">
        <DeliberationListItem deliberationHash={hash}></DeliberationListItem>
      </span>
      <button class="complete-button" on:click={() => rejoinDeliberation(hash)}>Rejoin</button>
    </div>
  {/each}
</div>
{/if}
{/if}

<style>
  h3 {
    font-size: 14px;
    font-weight: normal;
    font-style: italic;
  }

  .complete-button {
    height: 84px;
    margin: 8px 0;
    border: 0;
    background-color: #b3b3b3;
    color: white;
    border-radius: 10px;
    cursor: pointer;
    width: 70px;
    font-weight: bold;
  }

  .complete-button:hover {
    background-color: #cccccc;
  }
</style>