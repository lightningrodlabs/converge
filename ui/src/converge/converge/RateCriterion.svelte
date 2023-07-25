<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Criterion, ConvergeSignal } from './types';
import '@material/mwc-circular-progress';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-snackbar';
import '@material/mwc-icon-button';

const dispatch = createEventDispatcher();

export let criterionHash: ActionHash;
export let proposalHash: ActionHash;
export let ratings: any[] | undefined;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let loading = true;
let error: any = undefined;

let record: Record | undefined;
let criterion: Criterion | undefined;
let supporters: Array<string> | undefined;
let sponsored: boolean | undefined;
let support: number | undefined;
let responded: boolean | undefined;
let openEvaluation = true;
let addEvaluationPercentage = 5;

let errorSnackbar: Snackbar;
  
$:  error, loading, record, criterion, supporters, sponsored, ratings, responded;

$: responded = ratings.some(item => item["agentAsString"] === client.myPubKey.join(","));

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
    ratings = ratings
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

async function removeRating() {
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'remove_criterion_for_proposal',
      payload: {
        base_proposal_hash: proposalHash,
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

async function addRating() {
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'add_criterion_for_proposal',
      payload: {
        base_proposal_hash: proposalHash,
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


<div class="criterion">
  <div style="display: flex; flex-direction: column; font-size: .8em">
    <div class="vertical-progress-bar-container">
  
    {#if support}
    {#each Array.from({ length: 35 * support / supporters.length }) as _, index}
      <div class="progress-line" style="opacity: {support / supporters.length}"></div>
    {/each}
    {/if}
    </div>
  </div>
  <div class="two-sides">
    <div style="display: flex; flex-direction: column">
      <!-- <div style="display: flex; flex-direction: row">
        <span style="flex: 1"></span>
        <mwc-icon-button style="margin-left: 8px" icon="delete" on:click={() => deleteCriterion()}></mwc-icon-button>
      </div> -->
  
      <div style="display: flex; flex-direction: row; margin-bottom: 16px">
        <span style="white-space: pre-line">{ criterion.title }</span>
        <!-- {#each supporters as supporter}
          <span style="white-space: pre-line">{ supporter }</span>
        {/each} -->
      </div>
  
      <!-- <span style="white-space: pre-line">{ criterion.objections }</span> -->
      {#if support}
        <!-- <div style="display: flex; flex-direction: row; margin-bottom: 16px; font-size: .8em">
          {supporters.length} supporters
        </div> -->
        <div style="display: flex; flex-direction: row; font-size: .8em">
          <!-- {JSON.stringify(support / supporters.length)} average support -->
          {support} support
        </div>
      {:else}
        <div style="display: flex; flex-direction: row; font-size: .8em">
          0 supporters
        </div>
      {/if}
  
    </div>
  
    <div style="display: flex; flex-direction: column; margin-bottom: 16px; font-size: .8em">
        {#if sponsored}
          <div style="display: flex; flex-direction: row; margin-bottom: 16px; font-size: .8em">
            <mwc-button dense outlined on:click={() => removeRating()}>Remove evaluation</mwc-button>
          </div>
        {:else if openEvaluation}
          <div style="text-align: center; flex-direction: row; font-size: 1em">
            <span style="white-space: pre-line;">How well is this criterion met by the proposal?</span>
          </div>
          <div style="display: flex; flex-direction: row;  font-size: .8em">
          <!-- <input type="number" bind:value={support} /> -->
            <span style="white-space: pre-line; text-align: center;  top: 12px; position: relative;">NOT
            MET</span>
            <mwc-slider
              on:change={e => {
                addEvaluationPercentage = Math.max(e.detail.value, 1)
                addRating()
              }}
              value={addEvaluationPercentage}
              class="star-slider"
              step="1"
              max="10"
              >
            </mwc-slider>
            <span style="white-space: pre-line; text-align: center; top: 12px; position: relative;">FULLY
              MET</span>
          </div>
          <!-- <div style="text-align: center; flex-direction: row; mfont-size: .8em"> -->
            <!-- <button on:click={() => openEvaluation = false}>Cancel</button> -->
            <!-- <mwc-button dense outlined on:click={() => openSupport = false}>Cancel</mwc-button> -->
            
            <!-- <button on:click={() => addRating()}>Save</button> -->
            <!-- <mwc-button class="custom-button" dense raised on:click={() => addSupport()}>Save</mwc-button> -->
          <!-- </div> -->
        {:else}
          <div style="display: flex; flex-direction: row; font-size: .8em">
  
            <!-- <button on:click={() => openEvaluation = true}>Add evaluation</button> -->
            <mwc-button class="custom-button" dense outlined on:click={() => openEvaluation = true}>Add evaluation</mwc-button>
  
          <!-- <button on:click={() => addRating()}>Add evaluation</button> -->
          </div>
        {/if}
    </div>
  </div>
  </div>










{#if false}
<div class="criterion">
<div style="display: flex; flex-direction: column">

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="white-space: pre-line">{ criterion.title }</span>
    {#if support}
    <span style="white-space: pre-line">{support}</span>
    {/if}

    <!-- {ratings.length} -->

    <!-- {#each supporters as supporter}
      <span style="white-space: pre-line">{ supporter }</span>
    {/each} -->
  </div>

</div>
<div style="display: flex; flex-direction: column">
  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    {#if responded}
      <button on:click={() => removeRating()}>Remove Rating</button>
    {:else}
      <button on:click={() => addRating()}>Add Rating</button>
    {/if}
  </div>
</div>
</div>
{/if}
{/if}

