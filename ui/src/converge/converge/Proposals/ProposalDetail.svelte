<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
import { clientContext } from '../../../contexts';
import type { Proposal } from '../types';
import '@material/mwc-circular-progress';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-snackbar';
import '@material/mwc-icon-button';
import { view, viewHash, navigate } from '../../../store.js';
import RateCriteria from './RateCriteria.svelte';

const dispatch = createEventDispatcher();

export let proposalHash: ActionHash;
let deliberationHash: ActionHash | undefined;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let loading = true;
let error: any = undefined;

let record: Record | undefined;
let proposal: Proposal | undefined;

let allRatings;

let convergence;
let maxWeight;


let errorSnackbar: Snackbar;
  
$:  error, loading, record, proposal, allRatings;
$: if (proposalHash) {
  fetchProposal();
  fetchDeliberation();
}

onMount(async () => {
  if (proposalHash === undefined) {
    throw new Error(`The proposalHash input is required for the ProposalDetail element`);
  }
  await fetchProposal();
  await fetchDeliberation();
});

async function fetchProposal() {
  loading = true;
  error = undefined;
  record = undefined;
  proposal = undefined;
  
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'get_proposal',
      payload: proposalHash,
    });
    if (record) {
      proposal = decode((record.entry as any).Present.entry) as Proposal;
    }
  } catch (e) {
    error = e;
  }

  loading = false;
}

async function rateAlert() {
  console.log('rate-alert-4')
  dispatch('proposal-rated');
}

async function deleteProposal() {
  try {
    await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'delete_proposal',
      payload: proposalHash,
    });
    dispatch('proposal-deleted', { proposalHash: proposalHash });
  } catch (e: any) {
    errorSnackbar.labelText = `Error deleting the proposal: ${e.data.data}`;
    errorSnackbar.show();
  }
}

async function fetchDeliberation() {
  try {
    let records = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'get_deliberations_for_proposal',
      payload: proposalHash,
    });
    if (records && records.length) {
      deliberationHash = records[0].signed_action.hashed.hash
    }
  } catch (e: any) {
    error = e
  }
}
</script>

<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>

{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the proposal: {error.data.data}</span>
{:else}

<!-- <button on:click={()=>{navigate('deliberation', deliberationHash)}}>back</button> -->

<!-- <div style="display: flex; 
flex: 1;"> -->
<!-- {JSON.stringify(proposalHash.join(''))} -->
{JSON.stringify(convergence)}
<div class="outlined-item list-item-mini criterion-outer" style="width:96%">
  {#if convergence > 0 && maxWeight > 0}
  <div style="display: flex; flex-direction: column; font-size: .8em">
  <div class="vertical-progress-bar-container">
  <!-- <div class="vertical-progress-bar-container" style="height: 100px; border: 1px dotted black;"> -->
    {#each Array.from({ length: 35 * convergence / maxWeight }) as _, index}
      <div class="progress-line" style="opacity: {convergence / maxWeight}; background-color: rgb(254, 18, 18)"></div>
    {/each}
  </div>
  </div>
  {/if}
  <div class="two-sides">

  <div style="display: flex; flex: 1; flex-direction: column">
  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>{ proposal.title }</strong></span>
    <!-- <span style="white-space: pre-line">{ proposal.title }</span> -->
  </div>

  <div class="deliberation-section" style="display: flex; flex-direction: row; margin-bottom: 16px">
    <!-- <span style="margin-right: 4px"><strong>Description:</strong></span> -->
    <span style="white-space: pre-line">{ proposal.description }</span>
  </div>
</div>
</div>
</div>

  <!-- <div class="deliberation-section" style="display: flex; flex-direction: row; margin-bottom: 16px"> -->
    <!-- <span style="margin-right: 4px"><strong>Evaluate criteria</strong></span> -->
  <!-- </div> -->
  <div style="flex-direction: row; margin-bottom: 16px">
    <h2 style="margin-bottom: 0; margin-top: 30px;">Evaluate</h2>
    <span style="white-space: pre-line">
      <!-- {deliberationHash} -->
      {#if deliberationHash}
        <!-- {JSON.stringify(convergence)} -->
        <RateCriteria on:proposal-rated={rateAlert} bind:convergence bind:maxWeight deliberationHash={deliberationHash} proposalHash={proposalHash} />
      {/if}
    </span>
  </div>

{/if}

