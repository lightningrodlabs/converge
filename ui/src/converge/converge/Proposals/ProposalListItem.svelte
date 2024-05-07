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
import { countViewed, addToViewed } from '../../../store.js';
import ProposalDetail from './ProposalDetail.svelte';
import OutcomesForProposal from '../Outcomes/OutcomesForProposal.svelte';


const dispatch = createEventDispatcher();

export let proposalHash: ActionHash;
export let deliberationHash: ActionHash | undefined;
export let allProposalScores;
export let filter;
export let hashes;
export let sortableProposals;
export let anyProposalPopup;
export let userRatings;
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
let popupStyle = "block";
let popupElement;
let switchBool = true;

let errorSnackbar: Snackbar;
  
$:  error, loading, record, proposal, proposalPopup, proposalDetailHash, hashes, popupStyle, popupElement, switchBool;

$: if (popupElement && proposalPopup) popupElement.showModal();

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
  addToViewed(proposalHash, client);
  // await fetchDeliberation();

  window.addEventListener("keydown", checkKey);

});

function checkKey(e) {
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
  // find next hash
  if (hashes.length > hashes.indexOf(proposalDetailHash) + 1) {
    let nextHash = hashes[hashes.indexOf(proposalDetailHash) + 1]
    if (nextHash) {
      proposalDetailHash = nextHash
    }
    proposalPopup = false;
    anyProposalPopup = false;
    setTimeout(function () {
      proposalPopup = true;
      anyProposalPopup = true;
    }, 100)
  }
}

function moveLeft() {
  // find next hash
  if (hashes.indexOf(proposalDetailHash) > 0) {
    let nextHash = hashes[hashes.indexOf(proposalDetailHash) - 1]
    if (nextHash) {
      proposalDetailHash = nextHash
    }
    console.log(proposalDetailHash)
    proposalPopup = false;
    anyProposalPopup = false;
    setTimeout(function () {
      proposalPopup = true;
      anyProposalPopup = true;
    }, 100)
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
  dispatch('proposal-rated');
  // For kangaroo
  // proposalPopup = false;
  // anyProposalPopup = false;
  // switchBool = false;
  // setTimeout(function () {
  //   proposalPopup = true;
  //   anyProposalPopup = true;
  //   switchBool = true;
  // }, 10)
  // For kangaroo ends
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
    errorSnackbar.labelText = `Error deleting the proposal: ${e}`;
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
<span>Error fetching the proposal: {error}</span>
{:else if !filter || proposal.title.includes(filter)}

<!-- {JSON.stringify(sortableProposals[proposalHash.join(',')].score)} -->
<div class="outlined-item list-item-mini criterion-outer" on:mousedown={()=>{
  proposalPopup = true;
  anyProposalPopup = true;
  // popupStyle = "none";
  // proposalPopup = false;
  // setTimeout(function () {
    // hideAndShow();
    // popupElement.style.display = 'block';
    // popupStyle = "block";
  //   proposalPopup = true;
  // }, 100)

  // let x = document.getElementsByClassName("popup-container")[0]
  // console.log(x)
  // x.style.display = 'block';
  }}
  >
  <div style="display: flex; flex-direction: column; font-size: .8em">
    <!-- <div class="vertical-progress-bar-container"> -->
  
    {#if switchBool && convergence && maxWeight}
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
    <div style="display: flex; flex-direction: row; margin-bottom: 3px">
      <span style="white-space: pre-line">{ proposal.title }</span>
    </div>

    <!-- <div style="flex: 1; display: flex; flex-direction: row;">
      <mwc-linear-progress progress="0.5" style="flex-grow: 1;"></mwc-linear-progress>
    </div> -->

    <div class="overflow-content" style="display: flex; flex-direction: row;; font-size: 0.8em; position: relative;">
      <span style="white-space: pre-line; max-height: 56px; overflow: hidden;">
        <strong>evaluations:</strong> {userRatings?.length || 0}&nbsp;&nbsp;
      </span>

      <span style="white-space: pre-line">
        <strong>score:</strong> { Math.round(convergence / maxWeight * 100) }%
      </span>
    </div>
    
    <div class="overflow-content" style="display: flex; flex-direction: row; font-size: 0.8em; position: relative; margin-top: 3px;">
      <span style="white-space: pre-line; max-height: 44px; overflow: hidden;">
        { proposal.description }
      </span>
    </div>

  </div>
  <OutcomesForProposal proposalHash={proposalHash} />

  <div style="display: flex; flex-direction: column; flex-direction: row; align-items: center; margin: 2px;">
    
    {#if bestScoreKey == proposalHash.join(',')}
      <div style="display: flex; flex-direction: column; align-items: center; justify-content: center; margin-right: 16px">
        <div style="color: red; font-size: 1.2em">#1</div>
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

<RateCriteria on:proposal-rated={rateAlert} bind:userRatings bind:allSupport bind:convergence bind:maxWeight deliberationHash={deliberationHash} proposalHash={proposalHash} display={false} />

<!-- <div style="display:relative; z-index: 9999">
  <button on:click={()=>{popupElement.style.width = "90%"; console.log('none')}}>shrink</button>
  <button on:click={()=>{popupElement.style.width = "100%"}}>expand</button>
</div> -->
<!-- <div bind:this={popupElement}> -->
{#if proposalPopup}
<!-- <div class="backdrop"> -->
  <dialog bind:this={popupElement} class="popup-container" style="padding: 30px; width: 100%; height: 76%; overflow: scroll; display: {popupStyle}">
  <div id="move-left" on:mousedown={moveLeft}>
    <mwc-icon-button style="top: 8px; background-color: white;
    border-radius: 50px; margin-right: 8px;
    
    position: relative;">⇦</mwc-icon-button>
    </div>
  <button class="close-button" on:click={() => {proposalDetailHash=proposalHash; proposalPopup = false; anyProposalPopup = false;}}>esc</button><br>
<!-- <div class="popup-container" style="padding: 30px 24px 30px 30px;"> -->
  <!-- {#if proposalDetailHash} -->
  <ProposalDetail on:proposal-rated={rateAlert} on:outcome-created={(v)=>{
      dispatch('outcome-created', { outcomeHash: v.detail.outcomeHash });
      proposalPopup=false
    }
  } proposalHash={proposalDetailHash} on:dismiss={() => {proposalPopup = false; anyProposalPopup = false;}} />
    <!-- {/if} -->
  <div id="move-right" on:mousedown={moveRight}>
    <mwc-icon-button style="top: 8px; background-color: white;
    border-radius: 50px; margin: 8px;
    position: relative;">⇨</mwc-icon-button>  
    </div>
</dialog>
<!-- </div> -->
{/if}
<!-- </div> -->
{/if}


<style>
  dialog {
      max-width: 80vw;
      border-radius: 0.2em;
      border: none;
      padding: 0;
  }
  dialog::backdrop {
      background: rgba(0, 0, 0, 0.3);
  }
  dialog > div {
      padding: 1em;
  }
  dialog[open] {
      animation: zoom 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
  }
  @keyframes zoom {
      from {
          transform: scale(0.95);
      }
      to {
          transform: scale(1);
      }
  }
  dialog[open]::backdrop {
      animation: fade 0.2s ease-out;
  }
  #move-left {
      width: fit-content;
      position: fixed;
      left: 0px;
      top: 40vh;
      opacity: 0; /* Initially hidden */
      animation: showMoveLeft 0.1s forwards 0.3s; /* Appears after 0.3s delay (same as dialog's animation duration) */
  }
  @keyframes showMoveLeft {
      to {
          opacity: 1;
      }
  }
  #move-right {
    width: fit-content;
    position: fixed;
    right: 0px;
    top: 40vh;
    margin-right: -10px;
    opacity: 0; /* Initially hidden */
    animation: showMoveLeft 0.1s forwards 0.3s; /* Appears after 0.3s delay (same as dialog's animation duration) */
  }
  .close-button {
    opacity: 0; /* Initially hidden */
    animation: showMoveLeft 0.1s forwards 0.3s; /* Appears after 0.3s delay (same as dialog's animation duration) */
  }
</style>