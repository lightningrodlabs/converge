<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, DnaHash, ActionHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';
import { clientContext } from '../../../contexts';
import type { Deliberation } from '../types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textfield';

import '@material/mwc-textarea';
let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let originalDeliberationHash!: ActionHash;

export let currentRecord!: Record;
let currentDeliberation: Deliberation = decode((currentRecord.entry as any).Present.entry) as Deliberation;

let title: string | undefined = currentDeliberation.title;
let description: string | undefined = currentDeliberation.description;
let settings: string | undefined = currentDeliberation.settings;

let errorSnackbar: Snackbar;

$: title, description, settings;
$: isDeliberationValid = true && title !== '' && description !== '' && settings !== '';

onMount(() => {
  if (currentRecord === undefined) {
    throw new Error(`The currentRecord input is required for the EditDeliberation element`);
  }
  if (originalDeliberationHash === undefined) {
    throw new Error(`The originalDeliberationHash input is required for the EditDeliberation element`);
  }
});

async function updateDeliberation() {

  const deliberation: Deliberation = { 
    title: title!,
    description: description!,
    settings: settings!,
  };

  try {
    const updateRecord: Record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'update_deliberation',
      payload: {
        original_deliberation_hash: originalDeliberationHash,
        previous_deliberation_hash: currentRecord.signed_action.hashed.hash,
        updated_deliberation: deliberation
      }
    });
  
    dispatch('deliberation-updated', { actionHash: updateRecord.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error updating the deliberation: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Edit Deliberation</span>
  
  <div style="margin-bottom: 16px">
    <mwc-textarea outlined label="Title" value={ title } on:input={e => { title = e.target.value;} } required></mwc-textarea>    
  </div>

  <div style="margin-bottom: 16px">
    <mwc-textfield outlined label="Description" value={ description } on:input={e => { description = e.target.value; } } required></mwc-textfield>    
  </div>

  <div style="margin-bottom: 16px">
    <mwc-textfield outlined label="Settings" value={ settings } on:input={e => { settings = e.target.value; } } required></mwc-textfield>    
  </div>


  <div style="display: flex; flex-direction: row">
    <mwc-button
      outlined
      label="Cancel"
      on:click={() => dispatch('edit-canceled')}
      style="flex: 1; margin-right: 16px"
    ></mwc-button>
    <mwc-button 
      raised
      label="Save"
      disabled={!isDeliberationValid}
      on:click={() => updateDeliberation()}
      style="flex: 1;"
    ></mwc-button>
  </div>
</div>
