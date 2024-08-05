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
import { type WAL, isWeContext } from '@lightningrodlabs/we-applet';
import AttachmentsList from '../../../AttachmentsList.svelte';
import SvgIcon from "../../../SvgIcon.svelte";
import { getMyDna } from '../../../util';
import { weClientStored } from '../../../store.js';
import { countViewed, addToViewed } from '../../../viewed.js';
import CreateOutcome from '../Outcomes/CreateOutcome.svelte';
import OutcomesForProposal from '../Outcomes/OutcomesForProposal.svelte';

const dispatch = createEventDispatcher();

export let proposalHash: ActionHash;
let deliberationHash: ActionHash | undefined;
let attachments = [];

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let loading = true;
let error: any = undefined;

let record: Record | undefined;
let proposal: Proposal | undefined;

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
  
$:  error, loading, record, proposal, allRatings;
$: if (proposalHash) {
  fetchProposal();
  fetchDeliberation();
}

onMount(async () => {
  dnaHash = await getMyDna("converge", client)

  if (proposalHash === undefined) {
    throw new Error(`The proposalHash input is required for the ProposalDetail element`);
  }
  await fetchProposal();
  await fetchDeliberation();
  addToViewed(proposalHash, client);
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
      attachments = proposal.attachments
    }
  } catch (e) {
    error = e;
  }

  loading = false;
}

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
          <!-- <span style="white-space: pre-line">{ proposal.title }</span> -->
          <div style="display: flex; flex-direction: column; font-size: 0.8em; position: relative;">
            <span style="margin-right: 4px"><strong>{ proposal.title }</strong></span>
            <br>
            <span style="white-space: pre-line; max-height: 56px; overflow: hidden;">
              <strong>evaluations:</strong> {["userRatings"]?.length || 0}&nbsp;
            </span>
            <span style="white-space: pre-line">
              <strong>score:</strong> { Math.round(convergence / maxWeight * 100) }%
            </span>
          </div>
          <div>
            <div>
              <!-- button to switch to side-by-side view -->
              <button
              style="height: 30px; cursor: pointer; border: 0; border-radius: 4px; color: #313030;"
              on:click={() => {
                sideBySide = !sideBySide
              }}
            >
            <SvgIcon icon="faEye" size="20px"/>
            View { sideBySide ? "vertical" : "side-by-side"}</button>
            <button
            style="height: 30px; cursor: pointer; border: 0; border-radius: 4px; color: #313030;"
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
          
        </div>
      </div>
    </div>
  </div>
</div>

<div style="display: {sideBySide ? "flex" : "inherit"};">

<div style="display: flex; flex-direction: row; margin-bottom: 16px; width:inherit; padding: 30px 30px 0 0;">
  <!-- <span style="margin-right: 4px"><strong>Description:</strong></span> -->
  {#if isWeContext}
    <div style="display: flex; flex-direction: row; margin-bottom: 5px">
      <AttachmentsList {attachments} allowDelete={false}/>
    </div>
  {/if}
  <span style="white-space: pre-line; width:inherit">{ proposal.description }</span>
</div>

<!-- <div class="deliberation-section" style="display: flex; flex-direction: row; margin-bottom: 16px"> -->
    <!-- <span style="margin-right: 4px"><strong>Evaluate criteria</strong></span> -->
  <!-- </div> -->

  <div style="flex-direction: row; margin-bottom: 16px;">
    <h2 style="margin-bottom: 0; margin-top: 30px;">Evaluation</h2>
    <!-- two-option choice. question is "could you accept this proposal? answers are yes and no" -->
    <div style="display: flex; flex-direction: row; margin-bottom: 16px; width:inherit; padding: 30px 30px 0 0;">
      <span style="margin-right: 4px; margin: 3px 8px 0 0;">Could you accept this proposal? </span>
      <div style="display: flex; flex-direction: row; margin-bottom: 5px">
        <button style="height: 26px; cursor: pointer; border: 0; border-radius: 4px; color: #313030;" on:click={() => {
          dispatch('proposal-rated');
        }}>Yes</button>
        <button style="height: 26px; margin-left: 4px; cursor: pointer; border: 0; border-radius: 4px; color: #313030;" on:click={() => {
          dispatch('proposal-rated');
        }}>No</button>
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
