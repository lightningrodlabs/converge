<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import { type AppAgentClient, type Record, type EntryHash, type AgentPubKey, type ActionHash, type DnaHash, type Action, encodeHashToBase64 } from '@holochain/client';
import { clientContext } from '../../../contexts';
import type { Outcome, CreateOutcomeInput } from '../types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textarea';
import '@material/mwc-textfield';
import AttachmentsDialog from "../../../AttachmentsDialog.svelte"
import Criterion from '../Criteria/Criterion.svelte';
import type { WALUrl } from '../../../util';
import AttachmentsList from "../../../AttachmentsList.svelte";
import { countViewed, addToViewed } from '../../../viewed.js';
import { weaveUrlToWAL } from "@theweave/api";
import { allProposals, allDeliberations } from '../../../store';

let client: AppAgentClient = (getContext(clientContext) as any).getClient();
let attachmentsDialog : AttachmentsDialog
let attachments: Array<WALUrl> = [];
let outcome_attachment: WALUrl = null;
let showCriteria = true;
const dispatch = createEventDispatcher();
  
export let deliberationHash: ActionHash;
export let sortedCriteria = [];
export let proposalHash: ActionHash;

export let outcomeFormPopup; // Prop to control popup visibility
function dismissPopup() {
  outcomeFormPopup = false; // Set active to false to hide the popup
  title = '';
  description = '';
  outcome_attachment = null;
  attachments = [];
}

let title: string = '';
let description: string = '';
let deliberation;
let proposals;

let errorSnackbar: Snackbar;

$: title, description, outcomeFormPopup, sortedCriteria;
$: isOutcomeValid = true && outcome_attachment;
$: deliberation, proposals;

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
  allDeliberations.subscribe((d) => {
    console.log("deliberations", d, deliberationHash);
    deliberation = d.find((d) => encodeHashToBase64(d.action_hash) === encodeHashToBase64(deliberationHash));
    console.log("deliberation", deliberation);
  });
  
  allProposals.subscribe((p) => {
    proposals = p;
  });
  window.addEventListener("keydown", checkKey);
});

async function createOutcome() {  
  const outcomeEntry: Outcome = { 
    title: title!,
    description: description!,
    outcome_attachment: outcome_attachment!,
    proposal: proposalHash,
  };

  const createOutcomeInput: CreateOutcomeInput = {
    outcome: outcomeEntry,
    deliberation: deliberationHash,
  };

  console.log("createOutcomeInput", createOutcomeInput);
  
  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'create_outcome',
      payload: createOutcomeInput,
    });

    if (record && proposalHash) {
      await addToViewed(record.signed_action.hashed.hash, client);

      await client.callZome({
        cap_secret: null,
        role_name: 'converge',
        zome_name: 'converge',
        fn_name: 'add_outcome_for_proposal',
        payload: {
          base_proposal_hash: proposalHash,
          target_outcome_hash: record.signed_action.hashed.hash,
        },
      });
    }

    dispatch('outcome-created', { outcomeHash: record.signed_action.hashed.hash });
  } catch (e) {
    console.error("no proposal to add to", e);
    // errorSnackbar.labelText = `Error creating the outcome: ${e}`;
    // errorSnackbar.show();
  }
  dismissPopup()
}

</script>
{#if outcomeFormPopup}
<div class="backdrop">
  <div class="popup-container" style="min-width: fit-content;">

      <mwc-snackbar bind:this={errorSnackbar} leading>
      </mwc-snackbar>
      <div style="display: flex; flex-direction: column">
        <mwc-snackbar bind:this={errorSnackbar} leading>
        </mwc-snackbar>
        <div style="display: flex; flex-direction: column">
          <h2 style="font-size: 18px">Create Next Step</h2>

          <div style="margin-bottom: 16px; margin-right: 10px;">
            <mwc-textfield style="width: 100%;" outlined label="Title (optional)" value={ title } on:input={e => { title = e.target.value;} }></mwc-textfield>          
          </div>

          <!-- dropdown option of sorted proposals -->
          <!-- <select style="margin-bottom: 16px; margin-right: 10px;" on:change={e => { proposalHash = e.target.value; }}>
            {#each sortedProposals as proposal}
              <option value={proposal}>{proposal.title}</option>
            {/each}
          </select> -->
                    
          <!-- <div style="margin-bottom: 16px; margin-right: 10px;">
            <mwc-textarea style="width: 100%; height: 20vh" outlined label="Description" value={ description } on:input={e => { description = e.target.value; } } required></mwc-textarea>          
          </div> -->

          <!-- dropdown option of sorted proposals -->
          <label style="margin-bottom: 16px; margin-right: 10px;">Is this outcome related to a proposal?</label>
          <select style="margin-bottom: 16px; margin-right: 10px;"
            bind:value={proposalHash}>
            {#if deliberation}
            <option value={null}>No proposal selected</option>
            {#each deliberation.proposals as proposalHash}
              <option value={encodeHashToBase64(proposalHash)}>
                {proposals[encodeHashToBase64(proposalHash)].title}
              </option>
            {/each}
            {/if}
          </select>

          <div style="display:inline-block; width:fit-content">Attach the next step: </div>
          <AttachmentsDialog bind:this={attachmentsDialog} attachmentsLimit={1} bind:attachments on:add-attachment={
            (e) => {
              console.log(e)
              console.log("add-attachment", e.detail)
              outcome_attachment = e.detail
              console.log(outcome_attachment)
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

          <label class="instructions">Warning: you will not be able to edit OR DELETE this after creating</label>

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
              label="Create Next Step"
              disabled={!isOutcomeValid}
              on:click={() => createOutcome()}
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