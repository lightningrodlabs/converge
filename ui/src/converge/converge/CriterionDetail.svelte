<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Criterion } from './types';
import '@material/mwc-circular-progress';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-snackbar';
import '@material/mwc-icon-button';

const dispatch = createEventDispatcher();

export let criterionHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let loading = true;
let error: any = undefined;

let record: Record | undefined;
let criterion: Criterion | undefined;
let supporters: Array<AgentPubKey> | undefined;


let errorSnackbar: Snackbar;
  
$:  error, loading, record, criterion;

onMount(async () => {
  if (criterionHash === undefined) {
    throw new Error(`The criterionHash input is required for the CriterionDetail element`);
  }
  await fetchCriterion().then(() => fetchSupport());
});

async function fetchCriterion() {
  loading = true;
  error = undefined;
  record = undefined;
  criterion = undefined;
  
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'get_criterion',
      payload: criterionHash,
    });
    if (record) {
      criterion = decode((record.entry as any).Present.entry) as Criterion;
    }
  } catch (e) {
    error = e;
  }

  loading = false;
}

async function fetchSupport() {
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'get_supporters_for_criterion',
      payload: criterionHash,
    });
    if (record) {
      // supporters = decode((record.entry as any).Present.entry) as Array<AgentPubKey>;
      console.log(supporters)
    }
  } catch (e) {
    console.log(e)
    error = e;
  }
}

async function addSupport() {
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'add_criterion_for_supporter',
      payload: {
        base_supporter: client.myPubKey,
        target_criterion_hash: criterionHash,
      },
    });
    if (record) {
      console.log(record)
    }
  } catch (e) {
    error = e;
  }
}

async function deleteCriterion() {
  try {
    await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'delete_criterion',
      payload: criterionHash,
    });
    dispatch('criterion-deleted', { criterionHash: criterionHash });
  } catch (e: any) {
    errorSnackbar.labelText = `Error deleting the criterion: ${e.data.data}`;
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
<span>Error fetching the criterion: {error.data.data}</span>
{:else}

<div style="display: flex; flex-direction: column">
  <div style="display: flex; flex-direction: row">
    <span style="flex: 1"></span>
    <mwc-icon-button style="margin-left: 8px" icon="delete" on:click={() => deleteCriterion()}></mwc-icon-button>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Title:</strong></span>
    <span style="white-space: pre-line">{ criterion.title }</span>
    <button on:click={() => addSupport()}>Add Support</button>
    {supporters}
    <!-- {#each supporters as supporter}
      <span style="white-space: pre-line">{ supporter }</span>
    {/each} -->
  </div>

</div>
{/if}

