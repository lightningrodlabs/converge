<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { WeClient, isWeContext, initializeHotReload, type HrlB64WithContext, type Hrl } from '@lightningrodlabs/we-applet';
import { clientContext } from '../../../contexts';
import type { Proposal, CreateProposalInput } from '../types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textarea';
import '@material/mwc-textfield';
import AttachmentsDialog from "../../../AttachmentsDialog.svelte"
    import Criterion from '../Criteria/Criterion.svelte';

let client: AppAgentClient = (getContext(clientContext) as any).getClient();
let attachmentsDialog : AttachmentsDialog
let attachments: Array<HrlB64WithContext> = [];
  let showCriteria = true;
  const dispatch = createEventDispatcher();
  
export let deliberationHash: ActionHash;
export let sortedCriteria = [];

export let proposalFormPopup; // Prop to control popup visibility
function dismissPopup() {
  proposalFormPopup = false; // Set active to false to hide the popup
  title = '';
  description = '';
}

let title: string = '';
let description: string = '';

let errorSnackbar: Snackbar;

$: title, description, proposalFormPopup, sortedCriteria;
$: isProposalValid = true && title !== '' && description !== '';

function checkKey(e) {
  if (e.key === "Escape" && !e.shiftKey) {
    e.preventDefault();
    dismissPopup();
  }
}

function reference(r) {
  // add r to description
  description += "\n" + r + ":\n";
}

onMount(() => {
  window.addEventListener("keydown", checkKey);
});

async function createProposal() {  
  const proposalEntry: Proposal = { 
    title: title!,
    description: description!,
    attachments: attachments.map(a => {
      return {
        hrl: JSON.stringify(a.hrl),
        context: a.context
      }
    }),
  };

  const createProposalInput: CreateProposalInput = {
    proposal: proposalEntry,
    deliberation: deliberationHash,
  };
  
  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'create_proposal',
      payload: createProposalInput,
    });
    dismissPopup()
    dispatch('proposal-created', { proposalHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the proposal: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
{#if proposalFormPopup}
<div class="backdrop">
  {#if sortedCriteria.length > 0 && showCriteria}
    <div class="popup-container criterion-side-list">
      <button
      style="
        display: inline-block;
        margin-top: 10px;
        margin-bottom: 10px;
        border: none;
        padding: 2px 8px;
        border-radius: 4px;
        cursor: pointer;
        font-size: 16px;
        
        width: fit-content;"
      on:click={() => showCriteria = false}>&times; hide criteria</button>
      {#each sortedCriteria as c}
        <Criterion criterionHash={c} {reference} showSlider={false} />
      {/each}
    </div>
  {/if}
  <div class="popup-container" style="min-width: fit-content;">

      <mwc-snackbar bind:this={errorSnackbar} leading>
      </mwc-snackbar>
      <div style="display: flex; flex-direction: column">
        <mwc-snackbar bind:this={errorSnackbar} leading>
        </mwc-snackbar>
        <div style="display: flex; flex-direction: column">
          <h2 style="font-size: 18px">Create Proposal</h2>

          <div style="margin-bottom: 16px">
            <mwc-textfield style="width: 50vw" outlined label="Title" value={ title } on:input={e => { title = e.target.value;} } required></mwc-textfield>          
          </div>
                    
          <div style="margin-bottom: 16px">
            <mwc-textarea style="width: 50vw; height: 50vh" outlined label="Description" value={ description } on:input={e => { description = e.target.value; } } required></mwc-textarea>          
          </div>

          <AttachmentsDialog bind:this={attachmentsDialog} bind:attachments on:add-attachments={
            (e) => {
              console.log("add-attachments", e.detail)
              attachments = e.detail.attachments
              // props.attachments = e.detail.attachments
              // bind.refresh()
            }
          }></AttachmentsDialog>
                    
          {#if sortedCriteria.length > 0 && !showCriteria}
            <button
              style="
              display: inline-block;
              margin-top: 10px;
              border: none;
              padding: 2px 8px;
              border-radius: 4px;
              cursor: pointer;
              font-size: 16px;
              
              width: fit-content;"
              on:click={() => showCriteria = true}>
              + show criteria
            </button>
          {/if}

          <label class="instructions">Warning: you will not be able to edit the above details after creating.</label>

          <div style="display: flex; flex-direction: row">
            <mwc-button
              outlined
              label="Cancel"
              on:click={() => dismissPopup()}
              on:keydown={(e) => {
                if (e.key === 'Escape') {
                  dismissPopup();
                }
              }}
              style="flex: 1; margin-right: 16px"
            ></mwc-button>
            <mwc-button 
              raised
              label="Create Proposal"
              disabled={!isProposalValid}
              on:click={() => createProposal()}
              on:keydown={(e) => {
                if (e.key === 'Escape') {
                  dismissPopup();
                }
              }}
              style="flex: 1; margin-right: 16px"
            ></mwc-button>
          </div>
        </div>
      </div>
    </div>
  </div>
{/if}