<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Deliberation } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textfield';
import '@material/mwc-radio';
import '@material/mwc-formfield';
import { view, viewHash, navigate } from '../../store.js';

import '@material/mwc-textarea';
let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();


let title: string = '';
let description: string = '';
let settings: any = {scoring: 'classic'};

let errorSnackbar: Snackbar;

$: title, description, settings;
$: isDeliberationValid = true && title !== '' && description !== '' && settings !== '';

onMount(() => {
});

async function createDeliberation() {  
  const deliberationEntry: Deliberation = { 
    title: title!,
    description: description!,
    settings: JSON.stringify(settings!),
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

    // join deliberation
    await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'add_deliberation_for_deliberator',
      payload: {
        base_deliberator: client.myPubKey,
        target_deliberation_hash: record.signed_action.hashed.hash
      },
    });
    
    navigate("deliberation", record.signed_action.hashed.hash)
  } catch (e) {
    errorSnackbar.labelText = `Error creating the deliberation: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <h1>New Deliberation</h1>
  <h2>Deliberation Details</h2>
  

  <div style="margin-bottom: 16px; text-align: left;">
    <mwc-textfield style="width: 100%" outlined label="Title" value={ title } on:input={e => { title = e.target.value;} } required></mwc-textfield>          
  </div>
            
  <div style="margin-bottom: 16px; text-align: left;">
    <mwc-textarea style="width: 100%; height: 30vh" outlined label="Description" value={ description } on:input={e => { description = e.target.value; } } required></mwc-textarea>          
  </div>
            
  <!-- <div style="margin-bottom: 16px; text-align: left">
    <mwc-formfield label="Weighted scoring">
      <mwc-radio class="scoring-radio" name="group" value="weighted" on:change={(e) => {console.log(e.target.value)}}></mwc-radio>
    </mwc-formfield>
    <mwc-formfield label="Classic scoring">
      <mwc-radio name="group" value="classic" on:change={(e) => {console.log(e.target.value)}}></mwc-radio>
    </mwc-formfield>
  </div>
        
  <div style="margin-bottom: 16px; text-align: left">
    <mwc-formfield label="Evaluation method">
      <mwc-radio class="scoring-radio" name="group" value="binary" on:change={(e) => {console.log(e.target.value)}}></mwc-radio>
    </mwc-formfield>
    <mwc-formfield label="Classic scoring">
      <mwc-radio name="group" value="sliding" on:change={(e) => {console.log(e.target.value)}}></mwc-radio>
    </mwc-formfield>
  </div> -->

  <mwc-button 
    raised
    label="Create Deliberation"
    disabled={!isDeliberationValid}
    on:click={() => createDeliberation()}
  ></mwc-button>
</div>
