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
let criteriaFilter;
let proposalFilter;
let proposalCount = 0;
let detectSort;

let sortByOptions = [
  "support",
  "objections",
  "comments",
  "weight",
  "my support",
  "my objections"
];
let criteriaSort;
let proposalSort;

$: editing, error, loading, record, deliberation, activeTab, criterionFormPopup, proposalFormPopup, criteriaCount, proposalCount, criteriaFilter, proposalFilter, criteriaSort, proposalSort;

onMount(async () => {
  if (deliberationHash === undefined) {
    throw new Error(`The deliberationHash input is required for the DeliberationDetail element`);
  }
  await fetchDeliberation();

  // client.on('signal', signal => {
    // if (signal.zome_name !== 'converge') return;
    // const payload = signal.payload as ConvergeSignal;  
    // if (['LinkCreated'].includes(payload.type)) {
      // if (['CriterionToSupporters', 'DeliberationToCriteria', 'DeliberationToProposals', 'ProposalToCriteria'].includes(Object.keys(signal.payload['link_type'])[0])) {
        // console.log("CREATED", Object.keys(signal.payload['link_type'])[0]);
        // wait a second
        // setTimeout(() => {
          // fetchDeliberation();
          // joinDeliberation();
        // }, 2000);
      // }
    // }
    // if (!['LinkCreated', 'LinkDeleted'].includes(payload.type)) return;
    // if (['DeliberatorToDeliberations', 'DeliberationToDeliberators'].includes(Object.keys(signal.payload['link_type'])[0])) {
    // } else {
      // fetchDeliberation();
    // }
    // console.log("----", payload.type, Object.keys(signal.payload['link_type'])[0]);
    // console.log(!['DeliberatorToDeliberations'].includes(Object.keys(signal.payload['link_type'])[0]))
    // joinDeliberation();
  // });
});

async function fetchDeliberation() {
  loading = true;
  error = undefined;
  record = undefined;
  // deliberation = undefined;
  
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

async function newActivity(event) {
  console.log("new activity")
  console.log(event)
  joinDeliberation()
}

async function joinDeliberation() {
  if (deliberators.includes(client.myPubKey.join(','))) return;
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
    deliberators = [...deliberators, client.myPubKey.join(',')]
    // dispatch('deliberation-joined', { deliberationHash: deliberationHash });
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
    deliberators = deliberators.filter(item => item !== client.myPubKey.join(','));
    // dispatch('deliberation-left', { deliberationHash: deliberationHash });
    // navigate('')
  } catch (e: any) {
    errorSnackbar.labelText = `Error leaving the deliberation: ${e.data.data}`;
    errorSnackbar.show();
  }
}

  let isExpanded = false;
  function expandSearch() {
    if (isExpanded) {
      criteriaFilter = "";
      proposalFilter = "";
    }
    isExpanded = !isExpanded;
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
    <div style="display: flex; flex-direction: row; width: fit-content; margin-bottom: 6px;">
      
      {deliberators.length} 
      {#if deliberators.length == 1} participant{:else} participants{/if}
      
      &nbsp;|&nbsp;&nbsp;

      {#if deliberators.includes(client.myPubKey.join(','))}
        <div style="text-decoration: underline; cursor: pointer; width: fit-content; display:flex; flex-direction: column;" on:click={leaveDeliberation}>Leave</div>
      {:else}
        <div on:click={newActivity}>Join</div>
      {/if}

    </div>

    
  <!-- </div> -->

  <div class="deliberation-section">
    <mwc-tab-bar>
      <mwc-tab on:click={() => {activeTab = "criteria"}} label="Criteria ({criteriaCount})"></mwc-tab>
      <mwc-tab on:click={() => {activeTab = "proposals"}}  label="Proposals ({proposalCount})"></mwc-tab>
      <!-- <mwc-tab on:click={() => {activeTab = "activity"}}  label="Activity"></mwc-tab> -->
    </mwc-tab-bar>
    
  </div>

</div>

{#if activeTab == "criteria"}
  <!--<FaSort/> -->
  <p>What characteristics should a proposal have?</p>
  <select bind:value={criteriaSort}>
    {#each sortByOptions as option}
    <option value={option}>  Sort by: {option}</option>
    {/each}
  </select>
  
  <div class="search-container">
    <div class="search-button" on:click={expandSearch}><FaSearch/></div>
    <input bind:value={criteriaFilter} type="text" class="search-input {isExpanded ? 'expanded' : ''}" placeholder="Search criteria...">
  </div>

  <!-- <div class="search-container"><FaSearch/></div> -->


  <!-- <div class="search-button"><FaSearch/></div> -->
  <button on:click={() => {criterionFormPopup = true; console.log(criterionFormPopup)}} class="add-button">Add criterion</button>
  <!-- <mwc-button dense outlined>Add criterion</mwc-button> -->
  <!-- {#if criterionForm} -->
  <CreateCriterion on:criterion-created={newActivity} deliberationHash={deliberationHash} alternativeTo={null} bind:criterionFormPopup />
  <!-- {/if} -->
  <br><br>
  <!-- {#if criteriaSort == "support"} -->
  
  <AllCriteria on:criterion-rated={newActivity} deliberationHash={deliberationHash} filter={criteriaFilter} sort={criteriaSort} bind:criteriaCount />
  <!-- {:else if criteriaSort == "objections"}
  <AllCriteria deliberationHash={deliberationHash} filter={criteriaFilter} sort="objections" bind:criteriaCount />
  {/if} -->
{:else if activeTab == "proposals"}
  <p>What solutions would meet our criteria?</p>
  <!-- <select bind:value={proposalSort}>
    {#each ["score", "respondants"] as option}
    <option value={option}>  Sort by: {option}</option>
    {/each}
  </select> -->

  <div class="search-container">
    <div class="search-button" on:click={expandSearch}><FaSearch/></div>
    <input bind:value={proposalFilter} type="text" class="search-input {isExpanded ? 'expanded' : ''}" placeholder="Search proposals...">
  </div>

  <!-- <div class="search-button"><FaSearch/></div> -->
  <button on:click={() => {proposalFormPopup = true; console.log(proposalFormPopup)}} class="add-button">Add proposal</button>

  <CreateProposal on:proposal-created={newActivity} deliberationHash={deliberationHash} bind:proposalFormPopup/>
  <br><br>
  
  <AllProposals on:proposal-rated={newActivity} sort={proposalSort} deliberationHash={deliberationHash} filter={proposalFilter} bind:proposalCount/>
{/if}
{/if}

<style>
.search-container {
  width: fit-content;
  display: inline-block;
  border: 1px solid rgb(188, 187, 187);
  /* padding: 2px; */
  /* padding: 5px 5px 0px 5px; */
  position: relative;
  /* top: 3px; */
  border-radius: 4px;
}

  .search-button {
    width: 1em;
    padding: 2px 0px 4px 4px;
    display: inline-block;
    position: relative;
    top: 2px;
  }

  .search-input {
      border: none;
      padding: 8px;
      /* border-radius: 5px; */
      padding: 0px;
      width: 0;
      outline: none;
      transition: width 0.2s;
  }

  .search-input.expanded {
      width: 130px;
  }
</style>