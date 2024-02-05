<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Settings } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let discussionApp!: string;



let errorSnackbar: Snackbar;

$: discussionApp;
$: isSettingsValid = true;

onMount(() => {
  if (discussionApp === undefined) {
    throw new Error(`The discussionApp input is required for the CreateSettings element`);
  }
});

async function createSettings() {  
  const settingsEntry: Settings = { 
    discussion_app: discussionApp!,
  };
  
  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'create_settings',
      payload: settingsEntry,
    });
    dispatch('settings-created', { settingsHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the settings: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create Settings</span>
  


  <mwc-button 
    raised
    label="Create Settings"
    disabled={!isSettingsValid}
    on:click={() => createSettings()}
  ></mwc-button>
</div>
