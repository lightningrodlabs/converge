<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Viewed } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let viewedHash!: ActionHash;

export let viewedDate!: number;



let errorSnackbar: Snackbar;

$: viewedHash, viewedDate;
$: isViewedValid = true;

onMount(() => {
  if (viewedHash === undefined) {
    throw new Error(`The viewedHash input is required for the CreateViewed element`);
  }
  if (viewedDate === undefined) {
    throw new Error(`The viewedDate input is required for the CreateViewed element`);
  }
});

async function createViewed() {  
  const viewedEntry: Viewed = { 
    viewed_hash: viewedHash!,
    viewed_date: viewedDate!,
  };
  
  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'create_viewed',
      payload: viewedEntry,
    });
    dispatch('viewed-created', { viewedHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the viewed: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create Viewed</span>
  


  <mwc-button 
    raised
    label="Create Viewed"
    disabled={!isViewedValid}
    on:click={() => createViewed()}
  ></mwc-button>
</div>
