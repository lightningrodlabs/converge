<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Criterion, ConvergeSignal } from './types';
import '@material/mwc-circular-progress';
import '@material/mwc-slider';
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
let supporters: Array<string> | undefined;
let sponsored: boolean | undefined;
let support: number | undefined;

let errorSnackbar: Snackbar;
  
$:  error, loading, record, criterion, supporters, sponsored;

onMount(async () => {
  if (criterionHash === undefined) {
    throw new Error(`The criterionHash input is required for the CriterionDetail element`);
  }
  await fetchCriterion().then(() => fetchSupport());
  client.on('signal', signal => {
    if (signal.zome_name !== 'converge') return;
    const payload = signal.payload as ConvergeSignal;
    if (!['LinkCreated', 'LinkDeleted'].includes(payload.type)) return;
    fetchSupport();
  });
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
    let records = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'get_supporters_for_criterion',
      payload: criterionHash,
    });
    if (records) {
      supporters = Array.from(
        records.reduce((map, item) => {
          const key = item.agent.join(",");
          if (!map.has(key)) {
            map.set(key, { ...item, agent: key });
          }
          return map;
        }, new Map()).values()
      );
      support = supporters.reduce((sum, item) => sum + JSON.parse(item["tag"]), 0);
      sponsored = supporters.some(item => item["agent"] === client.myPubKey.join(","));
    }
  } catch (e) {
    console.log(e)
    error = e;
  }
}

async function removeSupport() {
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'remove_criterion_for_supporter',
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
        percentage: "1"
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

<br>
<mwc-slider
  discrete
  withTickMarks
  step="1"
  max="10"
  value="5">
</mwc-slider>
<br>

<div style="display: flex; flex-direction: column">
  <!-- <div style="display: flex; flex-direction: row">
    <span style="flex: 1"></span>
    <mwc-icon-button style="margin-left: 8px" icon="delete" on:click={() => deleteCriterion()}></mwc-icon-button>
  </div> -->

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Title:</strong></span>
    <span style="white-space: pre-line">{ criterion.title }</span>
    {#if support}
      {support}
    {/if}

    {#if sponsored}
      <button on:click={() => removeSupport()}>Remove Support</button>
    {:else}
      <button on:click={() => addSupport()}>Add Support</button>
    {/if}

    <!-- {#each supporters as supporter}
      <span style="white-space: pre-line">{ supporter }</span>
    {/each} -->
  </div>

</div>
{/if}

