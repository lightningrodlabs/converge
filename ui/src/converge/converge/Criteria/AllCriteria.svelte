<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { EntryHash, Record, AgentPubKey, ActionHash, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../../contexts';
import Criterion from './Criterion.svelte';
import type { ConvergeSignal } from '../types';

export let deliberationHash: ActionHash;
export let criteriaCount = 0;
export let filter;
export let sortedCriteria = [];
export let sort;

const dispatch = createEventDispatcher();

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;
let loading = true;
let error: any = undefined;
let sortableCriteria = {};
let showUnsupportedCriteria = false;
let numberOfUnsupportedCriteria = 0;

async function sortCriteria() {
// setTimeout(() => {
  let sortedCriteriaJoined = Object.values(sortableCriteria).sort((a, b) => {
    if (sort === 'support') {
      return b.support - a.support;
    } else if (sort === 'objections') {
      return b.objections - a.objections;
    } else if (sort === 'comments') {
      return b.comments - a.comments;
    } else if (sort === 'weight') {
      return b.weight - a.weight;
    } else if (sort === 'my support') {
      return b.mySupport - a.mySupport;
    } else if (sort === 'my objections') {
      return b.myObjections - a.myObjections;
    }
  });
  numberOfUnsupportedCriteria = 0;
  sortedCriteriaJoined = sortedCriteriaJoined.filter((c) => {
    if (c.supporters > 0) {
      return true;
    } else {
      numberOfUnsupportedCriteria++;
      if (showUnsupportedCriteria) {
        return true;
      } else {
        return false;
      }
    }
  });
  sortedCriteria = sortedCriteriaJoined.map((c) => c.hash);
  console.log(sort, sortedCriteriaJoined)
  // }, 4000)
}

$: hashes, loading, error, sortedCriteria, sortableCriteria, sort, filter, numberOfUnsupportedCriteria;
$: if (sort && sortableCriteria && hashes && Object.values(sortableCriteria).length == hashes.length) {
  sortCriteria();
}

onMount(async () => {

  await fetchCriteria();
  await sortCriteria()
  client.on('signal', signal => {
    if (signal.zome_name !== 'converge') return;
    const payload = signal.payload as ConvergeSignal;
    if (payload.type !== 'EntryCreated') return;
    if (payload.app_entry.type !== 'Criterion') return;
    // hashes = [...hashes, payload.action.hashed.hash];
    // sortedCriteria = [...sortedCriteria, payload.action.hashed.hash];
    fetchCriteria()
    sortCriteria()
  });
});

async function fetchCriteria() {
  try {
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'get_criteria_for_deliberation',
      payload: deliberationHash,
    });
    criteriaCount = records.length;
    hashes = records.map(r => r.signed_action.hashed.hash);
    sortedCriteria = hashes;
  } catch (e) {
    error = e;
  }
  loading = false;
}

async function joinSignal() {
  dispatch('criterion-rated',{})
}

</script>

<!-- {JSON.stringify(sortableCriteria)} -->
<!-- {JSON.stringify(sortedCriteria)} -->
{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the criteria: {error.data.data}.</span>
{:else if hashes.length === 0}
<span>Add some criteria.</span>
{:else}
<div style="display: flex; flex-direction: column">
  {#if sort == "support"}
    {#each sortedCriteria as hash}
      <Criterion on:criterion-rated={joinSignal} criterionHash={hash} {deliberationHash} {filter} bind:sortableCriteria on:criterion-deleted={() => fetchCriteria()}  on:transfer={(e) => {
        console.log("transfer", e.detail)
        // scroll to e.detail.to
        
      }} />
    {/each}
  {:else if sort == "objections"}
    {#each sortedCriteria as hash}
      <Criterion on:criterion-rated={joinSignal} criterionHash={hash} {deliberationHash} {filter} bind:sortableCriteria on:criterion-deleted={() => fetchCriteria()}></Criterion>
    {/each}
  {:else if sort == "comments"}
    {#each sortedCriteria as hash}
      <Criterion on:criterion-rated={joinSignal} criterionHash={hash} {deliberationHash} {filter} bind:sortableCriteria on:criterion-deleted={() => fetchCriteria()}></Criterion>
    {/each}
  {:else if sort == "weight"}
    {#each sortedCriteria as hash}
      <Criterion on:criterion-rated={joinSignal} criterionHash={hash} {deliberationHash} {filter} bind:sortableCriteria on:criterion-deleted={() => fetchCriteria()}></Criterion>
    {/each}
  {:else if sort == "my support"}
    {#each sortedCriteria as hash}
      <Criterion on:criterion-rated={joinSignal} criterionHash={hash} {deliberationHash} {filter} bind:sortableCriteria on:criterion-deleted={() => fetchCriteria()}></Criterion>
    {/each}
  {:else if sort == "my objections"}
    {#each sortedCriteria as hash}
      <Criterion on:criterion-rated={joinSignal} criterionHash={hash} {deliberationHash} {filter} bind:sortableCriteria on:criterion-deleted={() => fetchCriteria()}></Criterion>
    {/each}
  {/if}
  {#if numberOfUnsupportedCriteria > 0}
    {#if showUnsupportedCriteria}
      <button
        class="show-more-button"
        on:click={() => {
          showUnsupportedCriteria = false;
          sortCriteria();
        }}
      >Hide {numberOfUnsupportedCriteria} unsupported criteria?</button>
    {:else}
      <button
        class="show-more-button"
        on:click={() => {
          showUnsupportedCriteria = true;
          sortCriteria();
        }}
      >Show {numberOfUnsupportedCriteria} unsupported criteria?</button>
    {/if}
  {/if}
</div>
{/if}

<style>
  .show-more-button {
    background: #dee5ff;
    border: 0;
    border-radius: 4px;
    padding: 6px;
    cursor: pointer;
  }
  .show-more-button:hover {
    background: #c4d2ff;
  }
</style>