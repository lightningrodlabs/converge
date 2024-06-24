<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { Record, EntryHash, ActionHash, AgentPubKey, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../../contexts';
import DeliberationDetail from './DeliberationDetail.svelte';
import type { ConvergeSignal } from '../types';
import DeliberationListItem from './DeliberationListItem.svelte';
import { view, viewHash, navigate } from '../../../store.js';
import { joinDeliberation, leaveDeliberation } from '../../../publish';
import { refetchDeliberations } from '../../../refetch';
import { allDeliberations } from '../../../store.js';
import { decodeHashFromBase64, encodeHashToBase64 } from '@holochain/client';

export let deliberator: AgentPubKey;
let completedHashes: Array<ActionHash> = [];

let deliberations = [];
allDeliberations.subscribe(value => {
  deliberations = value.filter(d => d.deliberators.filter(d => encodeHashToBase64(d.deliberator) === encodeHashToBase64(deliberator)).length > 0);
  completedHashes = deliberations.filter(d => d.deliberators.filter(d => d.completed).length > 0).map(d => d.action_hash);
});

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let loading = true;
let error: any = undefined;

$: completedHashes, loading, error;

async function completeDeliberation(deliberationHash: ActionHash) {
  completedHashes = [...completedHashes, deliberationHash];
  // hashes = hashes.filter(h => h !== deliberationHash);
  await leaveDeliberation(deliberationHash, client);
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
  // hashes = [...hashes, deliberationHash];
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
    await joinDeliberation(deliberationHash, client);
    // await fetchDeliberations();
  } catch (e: any) {
    console.log("error", e)
  }
}

onMount(async () => {
  if (deliberator === undefined) {
    throw new Error(`The deliberator input is required for the DeliberationsForDeliberator element`);
  }

  await refetchDeliberations(client)
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
{:else if deliberations.length === 0 && completedHashes.length === 0}
<span>No deliberations found for this deliberator.</span>
{:else if deliberations.length > 0}
<div style="display: flex; flex-direction: column">
  {#each deliberations as deliberation}
    {@const completed = completedHashes.includes(deliberation.action_hash)}
      {#if !completed}
      <div style="margin-bottom: 8px; display: flex; justify-content: space-between;">
        <!-- <DeliberationDetail deliberationHash={hash}></DeliberationDetail> -->
        <span on:click={() => navigate('deliberation', deliberation.action_hash)} style="width: 100%;">
          <DeliberationListItem {deliberation}></DeliberationListItem>
        </span>
        <button class="complete-button" on:click={() => completeDeliberation(deliberation.action_hash)}>Mark complete</button>
      </div>
    {/if}
  {/each}
</div>

<!-- list all deliberations with hashes in completedHashes -->
{#if completedHashes.length > 0}
  <h3>Completed deliberations</h3>
  {#each completedHashes as hash}
    {@const deliberation = deliberations.find(d => d.action_hash === hash)}
    {#if deliberation}
    <div style="margin-bottom: 8px; display: flex; justify-content: space-between;">
        <span on:click={() => navigate('deliberation', deliberation.action_hash)} style="width: 100%;">
          <DeliberationListItem {deliberation}></DeliberationListItem>
        </span>
        <button class="complete-button" on:click={() => rejoinDeliberation(deliberation.action_hash)}>Rejoin</button>
      </div>
    {/if}
  {/each}
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