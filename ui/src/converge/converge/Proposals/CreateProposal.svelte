<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../../contexts';
import type { Proposal, CreateProposalInput } from '../types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';

import '@material/mwc-textarea';
import '@material/mwc-textfield';
let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let deliberationHash: ActionHash;

export let proposalFormPopup; // Prop to control popup visibility
function dismissPopup() {
  proposalFormPopup = false; // Set active to false to hide the popup
}

let title: string = '';
let description: string = '';

let errorSnackbar: Snackbar;

$: title, description, proposalFormPopup;
$: isProposalValid = true && title !== '' && description !== '';

function checkKey(e) {
  if (e.key === "Escape" && !e.shiftKey) {
    e.preventDefault();
    dismissPopup();
  }
}

onMount(() => {
  window.addEventListener("keydown", checkKey);
});

async function createProposal() {  
  const proposalEntry: Proposal = { 
    title: title!,
    description: description!,
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
  <div class="popup-container">

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