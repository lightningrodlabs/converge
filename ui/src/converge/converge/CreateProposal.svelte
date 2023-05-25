<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Proposal, CreateProposalInput } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';

import '@material/mwc-textarea';
import '@material/mwc-textfield';
let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let deliberationHash: ActionHash;

let title: string = '';
let description: string = '';

let errorSnackbar: Snackbar;

$: title, description;
$: isProposalValid = true && title !== '' && description !== '';

onMount(() => {
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
    dispatch('proposal-created', { proposalHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the proposal: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create Proposal</span>
  

  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Title" value={ title } on:input={e => { title = e.target.value;} } required></mwc-textarea>          
  </div>
            
  <div style="margin-bottom: 16px">
    <mwc-textfield outlined label="Description" value={ description } on:input={e => { description = e.target.value; } } required></mwc-textfield>          
  </div>
            

  <mwc-button 
    raised
    label="Create Proposal"
    disabled={!isProposalValid}
    on:click={() => createProposal()}
  ></mwc-button>
</div>
