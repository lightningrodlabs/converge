<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Deliberation, ConvergeSignal } from './types';
import FaSort from 'svelte-icons/fa/FaSort.svelte';
import FaSearch from 'svelte-icons/fa/FaSearch.svelte';
import '@material/mwc-circular-progress';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-snackbar';
import '@material/mwc-icon-button';
import '@material/mwc-tab';
import '@material/mwc-tab-bar';
import EditDeliberation from './EditDeliberation.svelte'; 
import { view, viewHash, navigate } from '../../store.js';
import CreateCriterion from './CreateCriterion.svelte';
import AllCriteria from './AllCriteria.svelte';
import CreateProposal from './CreateProposal.svelte';
import AllProposals from './AllProposals.svelte';

const dispatch = createEventDispatcher();

export let deliberationHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let loading = true;
let error: any = undefined;

let record: Record | undefined;
let deliberation: Deliberation | undefined;
let deliberators: String[] | undefined;

let editing = false;
let criterionFormPopup = false;
let proposalFormPopup = false;

let errorSnackbar: Snackbar;
let activeTab = "criteria";
let criteriaCount = 0;
let proposalCount = 0;

let sortByOptions = [
  { value: "support", label: "Support" },
  { value: "objections", label: "Objections" },
];
let selectedSortBy = "support";

$: editing,  error, loading, record, deliberation, activeTab, criterionFormPopup, proposalFormPopup, criteriaCount, proposalCount;

function sortItems() {
  // items = items.slice().sort((a, b) => b[selectedSortBy] - a[selectedSortBy]);
}

onMount(async () => {
  if (deliberationHash === undefined) {
    throw new Error(`The deliberationHash input is required for the DeliberationDetail element`);
  }
  await fetchDeliberation();

  client.on('signal', signal => {
    if (signal.zome_name !== 'converge') return;
    const payload = signal.payload as ConvergeSignal;
    if (!['LinkCreated', 'LinkDeleted'].includes(payload.type)) return;
    if (!['DeliberatorToDeliberations'].includes(Object.keys(signal.payload['link_type'])[0])) return;
    fetchDeliberation();
  });
});

async function fetchDeliberation() {
  loading = true;
  error = undefined;
  record = undefined;
  deliberation = undefined;
  
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'get_deliberation',
      payload: deliberationHash,
    });
    if (record) {
      deliberation = decode((record.entry as any).Present.entry) as Deliberation;
    }
  } catch (e) {
    error = e;
  }

  try {
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'get_proposals_for_deliberation',
      payload: deliberationHash,
    });
    proposalCount = records.length;
  } catch (e) {
    error = e;
  }

  try {
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'get_deliberators_for_deliberation',
      payload: deliberationHash,
    });
    deliberators = records.map((record: AgentPubKey) => record.join(','));
  } catch (e) {
    error = e;
  }

  loading = false;
}

async function deleteDeliberation() {
  try {
    await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'delete_deliberation',
      payload: deliberationHash,
    });
    dispatch('deliberation-deleted', { deliberationHash: deliberationHash });
  } catch (e: any) {
    errorSnackbar.labelText = `Error deleting the deliberation: ${e.data.data}`;
    errorSnackbar.show();
  }
}

async function joinDeliberation() {
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
    dispatch('deliberation-joined', { deliberationHash: deliberationHash });
  } catch (e: any) {
    errorSnackbar.labelText = `Error joining the deliberation: ${e.data.data}`;
    errorSnackbar.show();
  }
}

async function leaveDeliberation() {
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
    dispatch('deliberation-left', { deliberationHash: deliberationHash });
    // navigate('')
  } catch (e: any) {
    errorSnackbar.labelText = `Error leaving the deliberation: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>

<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>

{#if loading || !deliberationHash}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the deliberation: {error.data.data}</span>
{:else if editing}
<EditDeliberation
  originalDeliberationHash={ deliberationHash}
  currentRecord={record}
  on:deliberation-updated={async () => {
    editing = false;
    await fetchDeliberation()
  } }
  on:edit-canceled={() => { editing = false; } }
></EditDeliberation>
{:else}

<div style="display: flex; flex-direction: column">
  <!-- <div style="display: flex; flex-direction: row">
    <span style="flex: 1"></span>
    <mwc-icon-button style="margin-left: 8px" icon="edit" on:click={() => { editing = true; } }></mwc-icon-button>
    <mwc-icon-button style="margin-left: 8px" icon="delete" on:click={() => deleteDeliberation()}></mwc-icon-button>
  </div> -->

  <div style="display: flex; flex-direction: row; margin-bottom: 0px">
    <h1>{ deliberation.title }</h1>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="white-space: pre-line">{ deliberation.description }</span>
  </div>

  <!-- <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <h1>{ deliberation.title }</h1>
  </div>

  <div class="deliberation-section">
    <div style="margin-right: 4px"><strong>Description</strong></div>
    <p style="white-space: pre-line">{ deliberation.description }</p>
  </div> -->
  
  <!-- <div class="deliberation-section" style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Settings:</strong></span>
    <span style="white-space: pre-line">{ deliberation.settings }</span>
  </div> -->
  
  <!-- <div class="deliberation-section"> -->
    <div style="display: flex; flex-direction: row; width: fit-content; margin-bottom: 6px;">
      
      {deliberators.length} 
      {#if deliberators.length == 1} participant{:else} participants{/if}
      
      &nbsp;|&nbsp;&nbsp;

      {#if deliberators.includes(client.myPubKey.join(','))}
        <div style="text-decoration: underline; cursor: pointer; width: fit-content; display:flex; flex-direction: column;" on:click={leaveDeliberation}>Leave</div>
      {:else}
        <div on:click={joinDeliberation}>Join</div>
      {/if}
    <!-- </div> -->
    <!-- <div style="display: flex; flex-direction: row; width: 100%;"> -->

    </div>

    
  <!-- </div> -->

  <div class="deliberation-section">
    <mwc-tab-bar>
      <mwc-tab on:click={() => {activeTab = "criteria"}} label="Criteria ({criteriaCount})"></mwc-tab>
      <mwc-tab on:click={() => {activeTab = "proposals"}}  label="Proposals ({proposalCount})"></mwc-tab>
      <!-- <mwc-tab on:click={() => {activeTab = "activity"}}  label="Activity"></mwc-tab> -->
    </mwc-tab-bar>
    
    <!-- <p>Active tab: {activeTab}</p> -->
  </div>

</div>

{#if activeTab == "criteria"}
  <!--<FaSort/> -->
  <p>What characteristics should a proposal have?</p>
  <select bind:value={selectedSortBy} on:change={sortItems}>
    {#each sortByOptions as option}
    <option value={option.value}>  Sort by: {option.label}</option>
    {/each}
  </select>
  <div class="search-button"><FaSearch/></div>
  <button on:click={() => {criterionFormPopup = true; console.log(criterionFormPopup)}} class="add-button">Add criterion</button>
  <!-- <mwc-button dense outlined>Add criterion</mwc-button> -->
  <!-- {#if criterionForm} -->
  <CreateCriterion deliberationHash={deliberationHash} alternativeTo={null} bind:criterionFormPopup />
  <!-- {/if} -->
  <br><br>
  <AllCriteria deliberationHash={deliberationHash} bind:criteriaCount />
{:else if activeTab == "proposals"}
  <p>What solutions would meet our criteria?</p>
  <select bind:value={selectedSortBy} on:change={sortItems}>
    {#each sortByOptions as option}
    <option value={option.value}>  Sort by: {option.label}</option>
    {/each}
  </select>
  <div class="search-button"><FaSearch/></div>
  <button on:click={() => {proposalFormPopup = true; console.log(proposalFormPopup)}} class="add-button">Add proposal</button>

  <CreateProposal deliberationHash={deliberationHash} bind:proposalFormPopup/>
  <br><br>
  <AllProposals deliberationHash={deliberationHash} bind:proposalCount/>
{/if}
{/if}

