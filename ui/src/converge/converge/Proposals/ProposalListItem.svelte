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
import RateCriteria from './RateCriteria.svelte';
import '@material/mwc-linear-progress';
import '@material/mwc-icon-button'
import ProposalDetail from './ProposalDetail.svelte';


const dispatch = createEventDispatcher();

export let proposalHash: ActionHash;
export let deliberationHash: ActionHash | undefined;
export let allProposalScores;
export let filter;
export let hashes;
export let sortableProposals;
export let anyProposalPopup;
// let deliberationHash: ActionHash | undefined;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let loading = true;
let error: any = undefined;

let record: Record | undefined;
let proposal: Proposal | undefined;

let convergence;
let maxWeight;
let bestScoreKey;
let proposalPopup = false;
let proposalDetailHash = proposalHash;
let allSupport;

let errorSnackbar: Snackbar;
  
$:  error, loading, record, proposal, proposalPopup, proposalDetailHash, hashes;

$: if (convergence && maxWeight) {
  allProposalScores[proposalHash.join(',')] = convergence / maxWeight;
  bestScoreKey = Object.keys(allProposalScores).reduce((a, b) => allProposalScores[a] > allProposalScores[b] ? a : b);

  if (!proposalPopup) {
    let hashKey = proposalHash.join(',')
    sortableProposals[hashKey] = {
      score: convergence / maxWeight,
      respondants: allSupport,
      hash: proposalHash,
    };
    // console.log(sortableProposals)
  }
}

onMount(async () => {
  if (proposalHash === undefined) {
    throw new Error(`The proposalHash input is required for the ProposalDetail element`);
  }
  await fetchProposal();
  // await fetchDeliberation();

  window.addEventListener("keydown", checkKey);

});

function checkKey(e) {
  console.log(e.key)
  if (e.key === "Escape" && !e.shiftKey) {
    e.preventDefault();
    proposalDetailHash=proposalHash;
    proposalPopup = false;
    anyProposalPopup = false;
  } else if (e.key === "ArrowRight") {
    moveRight()
  } else if (e.key === "ArrowLeft") {
    moveLeft()
  }
}

function moveRight() {
  console.log(proposalDetailHash)
  // find next hash
  if (hashes.length > hashes.indexOf(proposalDetailHash) + 1) {
    let nextHash = hashes[hashes.indexOf(proposalDetailHash) + 1]
    if (nextHash) {
      proposalDetailHash = nextHash
    }
    console.log(proposalDetailHash)
  }
}

function moveLeft() {
  console.log(proposalDetailHash)
  // find next hash
  if (hashes.indexOf(proposalDetailHash) > 0) {
    let nextHash = hashes[hashes.indexOf(proposalDetailHash) - 1]
    if (nextHash) {
      proposalDetailHash = nextHash
    }
    console.log(proposalDetailHash)
  }
}

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
      // console.log(proposal)
    }
  } catch (e) {
    error = e;
  }

  loading = false;
}

async function rateAlert() {
  console.log('proposal-rated-3')
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

// async function fetchDeliberation() {
//   try {
//     let records = await client.callZome({
//       cap_secret: null,
//       role_name: 'converge',
//       zome_name: 'converge',
//       fn_name: 'get_deliberations_for_proposal',
//       payload: proposalHash,
//     });
//     if (records && records.length) {
//       deliberationHash = records[0].signed_action.hashed.hash
//     }
//   } catch (e: any) {
//     error = e
//   }
// }
</script>

<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>

{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the proposal: {error.data.data}</span>
{:else if !filter || proposal.title.includes(filter)}

<!-- {JSON.stringify(sortableProposals[proposalHash.join(',')].score)} -->

<div class="outlined-item list-item-mini criterion-outer" on:click={()=>{
  proposalPopup = true;
  anyProposalPopup = true;
  x = document.getElementsByClassName("popup-container")[0]
  x.style.display = 'none';
  x.style.display = 'block';
  }}>
  <div style="display: flex; flex-direction: column; font-size: .8em">
    <!-- <div class="vertical-progress-bar-container"> -->
  
    {#if convergence && maxWeight}
    <!-- {#each Array.from({ length: 35 * 10 / 16 }) as _, index}
      <div class="progress-line" style="opacity: {10 / 16}; background-color: rgb(254, 18, 18)"></div>
    {/each} -->
    <div class="vertical-progress-bar-container" style="height: 100px">
      {#each Array.from({ length: 35 * convergence / maxWeight }) as _, index}
        <div class="progress-line" style="opacity: {convergence / maxWeight}; background-color: red"></div>
      {/each}
    </div>
    {/if}
    </div>
  <!-- </div> -->
<div class="two-sides">
  <div style="display: flex; flex: 1; flex-direction: column">
    <div style="display: flex; flex-direction: row; margin-bottom: 16px">
      <span style="white-space: pre-line">{ proposal.title }</span>
    </div>

    <!-- <div style="flex: 1; display: flex; flex-direction: row;">
      <mwc-linear-progress progress="0.5" style="flex-grow: 1;"></mwc-linear-progress>
    </div> -->
    
    <div class="overflow-content" style="display: flex; flex-direction: row; margin-bottom: 16px; font-size: 0.8em; position: relative;">
      <span style="white-space: pre-line; max-height: 56px; overflow: hidden;">
        { proposal.description }
      </span>
    </div>

    <div class="overflow-content" style="display: flex; flex-direction: row; margin-bottom: 16px; font-size: 0.8em; position: relative;">
      <span style="white-space: pre-line">score: { Math.round(convergence / maxWeight * 100) }%</span>
    </div>

  </div>
  <div style="display: flex; flex-direction: column">
    
    {#if bestScoreKey == proposalHash.join(',')}
      <div style="display: flex; flex-direction: column; align-items: center; justify-content: center; margin-right: 16px">
        <mwc-icon-button icon="#1" style="color: red;"></mwc-icon-button>
      </div>
    {/if}

    <!-- <div style="display: flex; flex-direction: row; margin-bottom: 16px; font-size: .8em">
      <button on:click={() => console.log("clicked")}>Evaluate</button>
    </div> -->
  </div>
  <!-- <div> -->
    <!-- <mwc-linear-progress progress="0.5"></mwc-linear-progress> -->
  <!-- </div> -->
</div>
</div>

<RateCriteria on:proposal-rated={rateAlert} bind:allSupport bind:convergence bind:maxWeight deliberationHash={deliberationHash} proposalHash={proposalHash} display={false} />

{#if proposalPopup}
<div class="backdrop">
  <div on:click={moveLeft}>
<mwc-icon-button style="top: 8px; background-color: white;
border-radius: 50px; margin-right: 8px;
position: relative;" icon="⇦"></mwc-icon-button>  
</div>
<button class="close-button" on:click={() => {proposalDetailHash=proposalHash; proposalPopup = false; anyProposalPopup = false;}}>esc</button><br>
<div class="popup-container" style="padding: 30px; width: 100%; height: 76%; overflow: scroll;">
<!-- <div class="popup-container" style="padding: 30px 24px 30px 30px;"> -->
  <!-- {#if proposalDetailHash} -->
  <ProposalDetail on:proposal-rated={rateAlert} proposalHash={proposalDetailHash} on:dismiss={() => {proposalPopup = false; anyProposalPopup = false;}} />
    <!-- {/if} -->
  </div>
  <div on:click={moveRight}>
<mwc-icon-button style="top: 8px; background-color: white;
border-radius: 50px; margin: 8px;
position: relative;" icon="⇨"></mwc-icon-button>  
</div>
</div>
{/if}

{/if}