<script lang="ts">
import { createEventDispatcher, onMount, onDestroy, getContext } from 'svelte';
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
import { type WAL, isWeContext } from '@lightningrodlabs/we-applet';
import AttachmentsList from '../../../AttachmentsList.svelte';
import SvgIcon from "../../../SvgIcon.svelte";
import { getMyDna } from '../../../util';
import { weClientStored } from '../../../store.js';
import { countViewed, addToViewed } from '../../../viewed.js';
import CreateOutcome from '../Outcomes/CreateOutcome.svelte';
import OutcomesForProposal from '../Outcomes/OutcomesForProposal.svelte';
import { addSimpleEvaluation } from '../../../publish';
import { refetchProposalsForDeliberation } from '../../../refetch';
import { allProposals, allEvaluations } from '../../../store.js';
import { encodeHashToBase64 } from "@holochain/client";

const dispatch = createEventDispatcher();

export let proposalHash: ActionHash;
let deliberationHash: ActionHash | undefined;
let attachments = [];

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let loading = true;
let error: any = undefined;

let record: Record | undefined;
let proposal: Proposal | undefined;
let evaluations;
let myEvaluation;
let yesEvaluationCount = 0;
let noEvaluationCount = 0;

let allRatings;

let convergence;
let maxWeight;

let outcomeFormPopup;

let sideBySide = false;

let weClient;
let dnaHash: DnaHash;
weClientStored.subscribe(value => {
  weClient = value;
});

let errorSnackbar: Snackbar;

allProposals.subscribe(value => {
  proposal = value[encodeHashToBase64(proposalHash)];
  attachments = proposal?.attachments;
});

allEvaluations.subscribe(value => {
  evaluations = value;
  let encodedProposalHash = encodeHashToBase64(proposalHash);
  if ( value[encodedProposalHash]) {
    myEvaluation = evaluations[encodedProposalHash]?.find(e => e.target_evaluator === encodeHashToBase64(client.myPubKey));
    yesEvaluationCount = evaluations[encodedProposalHash]?.filter(e => (e.tag === "1" || e.tag == "49")).length;
    noEvaluationCount = evaluations[encodedProposalHash]?.filter(e => (e.tag === "0" || e.tag == "48")).length;
  }
});
  
$:  error, loading, record, proposal, allRatings;
$: if (proposalHash) {
  // fetchProposal();
  fetchDeliberation();
}

onMount(async () => {
  dnaHash = await getMyDna("converge", client)

  if (proposalHash === undefined) {
    throw new Error(`The proposalHash input is required for the ProposalDetail element`);
  }
  if (proposal && proposal.title) { loading = false }
  // await fetchProposal();
  await fetchDeliberation();
  console.log("fetching deliberation");
  await refetchProposalsForDeliberation(deliberationHash, client);
  loading = false;
  addToViewed(proposalHash, client);

  window.addEventListener("keydown", checkKey);
  
});

onDestroy(() => {
  // stop event listening
  window.removeEventListener("keydown", checkKey);
});

async function checkKey(e) {
  if (e.key === "Escape" && !e.shiftKey) {
    dispatch('escape');
  } else if (e.key === "ArrowRight") {
    e.preventDefault();
    dispatch('moveRight');
  } else if (e.key === "ArrowLeft") {
    e.preventDefault();
    dispatch('moveLeft');
  }
}

// async function fetchProposal() {
//   loading = true;
//   error = undefined;
//   record = undefined;
//   proposal = undefined;
  
//   try {
//     record = await client.callZome({
//       cap_secret: null,
//       role_name: 'converge',
//       zome_name: 'converge',
//       fn_name: 'get_proposal',
//       payload: proposalHash,
//     });
//     if (record) {
//       proposal = decode((record.entry as any).Present.entry) as Proposal;
//       attachments = proposal.attachments
//     }
//   } catch (e) {
//     error = e;
//   }

//   loading = false;
// }

const copyWalToPocket = () => {
  const attachment: WAL = { hrl: [dnaHash, proposalHash], context: "" }
  weClient?.walToPocket(attachment)
}

async function rateAlert() {
  dispatch('proposal-rated');
  // loading = true;
  // setTimeout(() => {
  //   loading = false;
  // }, 10);
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
<span>Error fetching the proposal: {JSON.stringify(error)}</span>
{:else}

<!-- <button on:click={()=>{navigate('deliberation', deliberationHash)}}>back</button> -->

<!-- <div style="display: flex; 
flex: 1;"> -->
<!-- {JSON.stringify(proposalHash.join(''))} -->

<!-- <div style="display: flex;
flex-direction: row;
justify-content: center;">
<div>
  <button
  style="height: 24px; cursor: pointer; border: 0; border-radius: 4px; color: #313030; background-color: #e9e9e9;"
  on:click={() => {
    sideBySide = !sideBySide
  }}
>
  <SvgIcon icon="faEye" size="20px"/>
  View { sideBySide ? "vertical" : "side-by-side"}</button>
  <button
  style="height: 24px; cursor: pointer; border: 0; border-radius: 4px; color: #313030; background-color: #e9e9e9;"
  on:click={()=>{
    outcomeFormPopup = !outcomeFormPopup
  }}>+ Create outcome</button>
  <CreateOutcome on:outcome-created={(v) =>{
    dispatch('outcome-created', {
      outcomeHash: v.detail.outcomeHash
    })
  }
} proposalHash={proposalHash} {deliberationHash} {outcomeFormPopup} />
  {#if isWeContext()}
    <button title="Add Board to Pocket" class="attachment-button" style="height: 30px; top: -1px; position: relative; cursor: pointer; border: 0; border-radius: 4px; padding: 4px;" on:click={()=>copyWalToPocket()} >          
      <SvgIcon icon="addToPocket" size="20px"/>
    </button>
  {/if}
</div>

<OutcomesForProposal proposalHash={proposalHash} />

</div> -->

  <div class="outlined-item list-item criterion-outer donthover" style="width:96%">
    {#if convergence > 0 && maxWeight > 0}
    <div style="display: flex; flex-direction: column; font-size: .8em; max-height: 100px;">
      <div class="vertical-progress-bar-container">
      <!-- <div class="vertical-progress-bar-container" style="height: 100px; border: 1px dotted black;"> -->
        {#each Array.from({ length: 35 * convergence / maxWeight }) as _, index}
          <div class="progress-line" style="opacity: {convergence / maxWeight}; background-color: rgb(254, 18, 18)"></div>
        {/each}
      </div>
      </div>
      {/if}
      <div class="two-sides two-sides-proposal">

      <div style="display: flex; flex: 1; flex-direction: column; width:inherit">
        <div style="display: flex; flex-direction: row; margin-bottom: 16px; width:inherit; justify-content: space-between;  ">
          <div style="display: flex; flex-direction: column; font-size: 0.8em; position: relative;">
            <span style="margin-right: 4px; margin-bottom: 10px;"><strong style="font-size: 16px;">{ proposal.title }</strong></span>
            <!-- <br> -->
            <span style="white-space: pre-line; max-height: 56px; overflow: hidden;">
              <strong>Evaluations:</strong> {["userRatings"]?.length || 0}&nbsp;
            </span>
            <span style="white-space: pre-line">
              <strong>Score:</strong> { convergence > 0 && maxWeight > 0 ? Math.round(convergence / maxWeight * 100) : "_" }%
            </span>
            <span style="white-space: pre-line">
              <strong>Accept:</strong> ✔{yesEvaluationCount} ✗{noEvaluationCount}
            </span>
            <br><span style="white-space: pre-line">
              <OutcomesForProposal showArrow={false} proposalHash={proposalHash} />
            </span>
            {#if proposal.description}
              <span style="white-space: pre-line; width:inherit">
                { proposal.description }
              </span>
            {/if}
          </div>
        </div>
      </div>
      <div style="display: flex; flex: 1; flex-direction: row-reverse; width:inherit">
        {#if isWeContext()}
        <button title="Add Board to Pocket" class="attachment-button" style="height: 30px; top: -1px; position: relative; cursor: pointer; border: 0; border-radius: 4px; padding: 4px;" on:click={()=>copyWalToPocket()} >          
          <SvgIcon icon="addToPocket" size="20px"/>
        </button>
      {/if}
      </div>
  </div>
</div>

<div style="display: {sideBySide ? "flex" : "inherit"};">
  {#if isWeContext()}
    <div style="display: flex; flex-direction: row; margin-bottom: 16px; width:inherit; padding: 30px 30px 0 0;">
    <!-- <span style="margin-right: 4px"><strong>Description:</strong></span> -->
      <div style="display: flex; flex-direction: row; margin-bottom: 5px">
        <AttachmentsList {attachments} allowDelete={false}/>
      </div>
      <!-- <span style="white-space: pre-line; width:inherit">{ proposal.description }</span> -->
    </div>
  {/if}

<!-- <div class="deliberation-section" style="display: flex; flex-direction: row; margin-bottom: 16px"> -->
    <!-- <span style="margin-right: 4px"><strong>Evaluate criteria</strong></span> -->
  <!-- </div> -->

  <div style="flex-direction: row; margin-bottom: 16px;">
    <h2 style="margin-bottom: 0; margin-top: 30px;">Evaluation</h2>
    <div style="display: flex; flex-direction: row; margin-bottom: 16px; width:inherit; padding: 30px 30px 0 0;">
      <span style="margin-right: 4px; margin: 3px 8px 0 0;">Could you accept this proposal? </span>
      <div style="display: flex; flex-direction: row; margin-bottom: 5px">
        <button class="evaluation-yes-button" on:click={() => {
          dispatch('proposal-rated');
          addSimpleEvaluation(
          {
            "base_proposal_hash": proposalHash,
            "target_evaluator": client.myPubKey,
            "tag": "1"
          }  
          , client);
        }}>
        {(myEvaluation?.tag === "1" || myEvaluation?.tag == "49") ? "✔" : ""}
        Yes</button>
        <button class="evaluation-no-button" on:click={() => {
          addSimpleEvaluation({
            "base_proposal_hash": proposalHash,
            "target_evaluator": client.myPubKey,
            "tag": "0"
          }, client);
          dispatch('proposal-rated');
        }}>
        {(myEvaluation?.tag === "0" || myEvaluation?.tag == "48") ? "✗" : ""}
        No</button>
      </div>
    </div>
    <span style="white-space: pre-line">
      <!-- {deliberationHash} -->
      {#if deliberationHash}
      {#key deliberationHash}
        <!-- {JSON.stringify(convergence)} -->
        <RateCriteria on:proposal-rated={rateAlert} bind:convergence bind:maxWeight deliberationHash={deliberationHash} proposalHash={proposalHash} />
      {/key}
      {/if}
    </span>
  </div>
</div>

{/if}

<style>
  .evaluation-yes-button, .evaluation-no-button {
    height: 26px; 
    cursor: pointer; 
    border: 0;
    margin-left: 4px;
    border-radius: 4px;
    padding: 4px 8px;
    font-weight: 600;
    color: #fff;
  }

  .evaluation-no-button {
    background-color: #bd291f;
  }

  .evaluation-no-button:hover {
    background-color: #ef1a1a;
  }

  .evaluation-yes-button {
    background-color: #328b58d9;;
  }

  .evaluation-yes-button:hover {
    background-color: #15cb64d9;
  }
</style>