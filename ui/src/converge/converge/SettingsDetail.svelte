<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Settings } from './types';
import '@material/mwc-circular-progress';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-snackbar';
import '@material/mwc-icon-button';
import EditSettings from './EditSettings.svelte'; 

const dispatch = createEventDispatcher();

export let settingsHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let loading = true;
let error: any = undefined;

let record: Record | undefined;
let settings: Settings | undefined;

let editing = false;

let errorSnackbar: Snackbar;
  
$: editing,  error, loading, record, settings;

onMount(async () => {
  if (settingsHash === undefined) {
    throw new Error(`The settingsHash input is required for the SettingsDetail element`);
  }
  await fetchSettings();
});

async function fetchSettings() {
  loading = true;
  error = undefined;
  record = undefined;
  settings = undefined;
  
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'get_settings',
      payload: settingsHash,
    });
    if (record) {
      settings = decode((record.entry as any).Present.entry) as Settings;
    }
  } catch (e) {
    error = e;
  }

  loading = false;
}

async function deleteSettings() {
  try {
    await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'delete_settings',
      payload: settingsHash,
    });
    dispatch('settings-deleted', { settingsHash: settingsHash });
  } catch (e: any) {
    errorSnackbar.labelText = `Error deleting the settings: ${e.data.data}`;
    errorSnackbar.show();
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
<span>Error fetching the settings: {error.data.data}</span>
{:else if editing}
<EditSettings
  originalSettingsHash={ settingsHash}
  currentRecord={record}
  on:settings-updated={async () => {
    editing = false;
    await fetchSettings()
  } }
  on:edit-canceled={() => { editing = false; } }
></EditSettings>
{:else}

<div style="display: flex; flex-direction: column">
  <div style="display: flex; flex-direction: row">
    <span style="flex: 1"></span>
    <mwc-icon-button style="margin-left: 8px" icon="edit" on:click={() => { editing = true; } }></mwc-icon-button>
    <mwc-icon-button style="margin-left: 8px" icon="delete" on:click={() => deleteSettings()}></mwc-icon-button>
  </div>

</div>
{/if}

