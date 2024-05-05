<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Viewed } from './types';
import '@material/mwc-circular-progress';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-snackbar';
import '@material/mwc-icon-button';

const dispatch = createEventDispatcher();

export let viewedHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let loading: boolean;
let error: any = undefined;

let record: Record | undefined;
let viewed: Viewed | undefined;


let errorSnackbar: Snackbar;
  
$:  error, loading, record, viewed;

onMount(async () => {
  if (viewedHash === undefined) {
    throw new Error(`The viewedHash input is required for the ViewedDetail element`);
  }
  await fetchViewed();
});

async function fetchViewed() {
  loading = true;
  
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'get_viewed',
      payload: viewedHash,
    });
    if (record) {
      viewed = decode((record.entry as any).Present.entry) as Viewed;
    }
  } catch (e) {
    error = e;
  }

  loading = false;
}

async function deleteViewed() {
  try {
    await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'delete_viewed',
      payload: viewedHash,
    });
    dispatch('viewed-deleted', { viewedHash: viewedHash });
  } catch (e: any) {
    errorSnackbar.labelText = `Error deleting the viewed: ${e.data.data}`;
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
<span>Error fetching the viewed: {error.data.data}</span>
{:else}

<div style="display: flex; flex-direction: column">
  <div style="display: flex; flex-direction: row">
    <span style="flex: 1"></span>
    <mwc-icon-button style="margin-left: 8px" icon="delete" on:click={() => deleteViewed()}></mwc-icon-button>
  </div>

</div>
{/if}

