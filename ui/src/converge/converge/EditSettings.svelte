<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, DnaHash, ActionHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';
import { clientContext } from '../../contexts';
import type { Settings } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let originalSettingsHash!: ActionHash;

export let currentRecord!: Record;
let currentSettings: Settings = decode((currentRecord.entry as any).Present.entry) as Settings;


let errorSnackbar: Snackbar;

$: ;
$: isSettingsValid = true;

onMount(() => {
  if (currentRecord === undefined) {
    throw new Error(`The currentRecord input is required for the EditSettings element`);
  }
  if (originalSettingsHash === undefined) {
    throw new Error(`The originalSettingsHash input is required for the EditSettings element`);
  }
});

async function updateSettings() {

  const settings: Settings = { 
    discussion_app: currentSettings.discussion_app,
  };

  try {
    const updateRecord: Record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'update_settings',
      payload: {
        original_settings_hash: originalSettingsHash,
        previous_settings_hash: currentRecord.signed_action.hashed.hash,
        updated_settings: settings
      }
    });
  
    dispatch('settings-updated', { actionHash: updateRecord.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error updating the settings: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Edit Settings</span>
  

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
      disabled={!isSettingsValid}
      on:click={() => updateSettings()}
      style="flex: 1;"
    ></mwc-button>
  </div>
</div>
