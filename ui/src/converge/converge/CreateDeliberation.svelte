<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Deliberation } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textfield';

import '@material/mwc-textarea';
let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();


let title: string = '';
let description: string = '';
let settings: string = '';

let errorSnackbar: Snackbar;

$: title, description, settings;
$: isDeliberationValid = true && title !== '' && description !== '' && settings !== '';

onMount(() => {
});

async function createDeliberation() {  
  const deliberationEntry: Deliberation = { 
    title: title!,
    description: description!,
    settings: settings!,
  };
  
  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'create_deliberation',
      payload: deliberationEntry,
    });
    dispatch('deliberation-created', { deliberationHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the deliberation: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create Deliberation</span>
  

  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Title" value={ title } on:input={e => { title = e.target.value;} } required></mwc-textarea>          
  </div>
            
  <div style="margin-bottom: 16px">
    <mwc-textfield outlined label="Description" value={ description } on:input={e => { description = e.target.value; } } required></mwc-textfield>          
  </div>
            
  <div style="margin-bottom: 16px">
    <mwc-textfield outlined label="Settings" value={ settings } on:input={e => { settings = e.target.value; } } required></mwc-textfield>          
  </div>
            

  <mwc-button 
    raised
    label="Create Deliberation"
    disabled={!isDeliberationValid}
    on:click={() => createDeliberation()}
  ></mwc-button>
</div>
