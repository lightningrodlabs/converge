<script lang="ts">
import { createEventDispatcher, onMount, onDestroy, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { EntryHash, Record, AgentPubKey, ActionHash, AppClient } from '@holochain/client';
import { clientContext } from '../../../contexts';
import Criterion from './Criterion.svelte';
import type { ConvergeSignal } from '../types';

export let deliberationHash: ActionHash;
export let criteriaCount = 0;
export let filter;
export let sortedCriteria = [];
export let sort;

const dispatch = createEventDispatcher();

let client: AppClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;
let loading = true;
let error: any = undefined;
let sortableCriteria = {};
let showUnsupportedCriteria = false;
// let numberOfUnsupportedCriteria = 0;
let unsupportedCriteria = [];
let refreshInterval: ReturnType<typeof setInterval> | undefined;
let unsub: () => void;

export const sortCriteria = async () => {
// setTimeout(() => {
  // sortedCriteria = []
  // console.log("sorting criteria", sortedCriteria)
  // let sortedCriteriaJoined = Object.values(sortableCriteria).sort((a, b) => {
  //   if (sort === 'support') {
  //     return b.support - a.support;
  //   } else if (sort === 'objections') {
  //     return b.objections - a.objections;
  //   } else if (sort === 'comments') {
  //     return b.comments - a.comments;
  //   } else if (sort === 'weight') {
  //     return b.weight - a.weight;
  //   } else if (sort === 'my support') {
  //     return b.mySupport - a.mySupport;
  //   } else if (sort === 'my objections') {
  //     return b.myObjections - a.myObjections;
  //   }
  // });
  // console.log("sortedCriteriaJoined 1", sortedCriteriaJoined)
  // numberOfUnsupportedCriteria = 0;
  // sortedCriteriaJoined = sortedCriteriaJoined.filter((c) => {
  //   if (c.supporters > 0) {
  //     return true;
  //   } else {
  //     numberOfUnsupportedCriteria++;
  //     if (showUnsupportedCriteria) {
  //       return true;
  //     } else {
  //       return false;
  //     }
  //   }
  // });
  // console.log("sortedCriteriaJoined 2", sortedCriteriaJoined)
  // sortedCriteria = sortedCriteriaJoined.map((c) => c.hash);
  // console.log(sort, sortedCriteriaJoined)
  // }, 4000)
}

$: hashes, loading, error, sortedCriteria, sortableCriteria, sort, filter, unsupportedCriteria;
// $: if (sort && sortableCriteria && hashes && Object.values(sortableCriteria).length == hashes.length) {
//   sortCriteria();
// }

async function fetchAndSort() {
  await fetchCriteria()
  // wait a second
  // await new Promise(r => setTimeout(r, 5000));
  // await sortCriteria()
  // if (sortableCriteria.length > 0) {
  // }
  // setTimeout(() => {
  //   let trueSort = sort
  //   sort = ""
  //   setTimeout(() => {
  //     sort = trueSort
  //   }, 100)
  // }, 100)
}

onMount(async () => {

  await fetchAndSort()

  unsub = client.on('signal', async signal => {
    // console.log("Received signal in AllCriteria", signal)
    if (signal.value.zome_name !== 'converge') return;
    const payload = signal.value.payload as any;

    if (payload.deliberation_hash) {
      const signalDelibHash = Object.values(payload.deliberation_hash).join(',');
      const currentDelibHash = Array.isArray(deliberationHash) ? deliberationHash.join(',') : deliberationHash.toString();
      if (signalDelibHash !== currentDelibHash) {
        return; // Signal is for a different deliberation
      }
    }
    
    // Handle custom activity notification signals
    if (payload.message === 'criterion-created') {
        // console.log("Criterion created signal for this deliberation - refreshing criteria");
        // console.log("Current criteria count:", hashes?.length || 0);
        // Just refresh the criteria list (our fetchCriteria handles deduplication)
        await new Promise(r => setTimeout(r, 5000)); // slight delay to ensure data consistency
        await fetchCriteria();
        // console.log("After refresh, criteria count:", hashes?.length || 0);
      return;
    } else if (payload.message === 'criterion-comment-created') {
      // console.log("Criterion comment created signal received", payload)
      const context = JSON.parse(payload.context);
      if (context.criterion_hash) {
        // Refresh ratings for that criterion
      }
      return;
    }
    
    // Handle standard ConvergeSignal format (if used elsewhere)
    if (payload.type === 'EntryCreated' && payload.app_entry?.type === 'Criterion') {
      // console.log("Criterion signal received", payload)
      const newCriterionHash = payload.action.hashed.hash;
      const hashToString = (hash) => Array.isArray(hash) ? hash.join(',') : hash.toString();
      
      if (!hashes) {
        hashes = [];
      }
      
      const existingHashStrings = new Set(hashes.map(hashToString));
      
      if (!existingHashStrings.has(hashToString(newCriterionHash))) {
        hashes = [...hashes, newCriterionHash];
        sortedCriteria = [newCriterionHash, ...sortedCriteria];
        criteriaCount = hashes.length;
        // console.log("Added new criterion from signal", newCriterionHash);
      }
    }
  });

  // Set up interval to refresh criteria every 60 seconds
  // Note: This only checks for NEW criteria and appends them
  // Existing criteria components are not recreated, preserving their state
  refreshInterval = setInterval(async () => {
    await fetchCriteria();
  }, 60000);
});

onDestroy(() => {
  // Clean up the interval when component is destroyed
  if (refreshInterval) {
    clearInterval(refreshInterval);
  }
  if (unsub) {
    unsub();
  }
});

async function fetchCriteria() {
  try {
    // console.log("fetchCriteria called, current hashes:", hashes?.length || 0);
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'get_criteria_for_deliberation',
      payload: deliberationHash,
    });
    
    const newHashes = records.map(r => r.signed_action.hashed.hash);
    const newCount = records.length;
    // console.log("Received from backend:", newCount, "criteria");
    
    // Convert hashes to strings for comparison
    const hashToString = (hash) => Array.isArray(hash) ? hash.join(',') : hash.toString();
    
    if (!hashes || hashes.length === 0) {
      // First load - set everything
      criteriaCount = newCount;
      hashes = newHashes;
      sortedCriteria = [...newHashes];
      sortedCriteria.reverse();
      // console.log("fetched criteria (initial load)", sortedCriteria.length, "criteria");
    } else {
      // Check for new criteria only
      const existingHashStrings = new Set(hashes.map(hashToString));
      const newCriteriaHashes = newHashes.filter(hash => !existingHashStrings.has(hashToString(hash)));
      
      // console.log("Checking for new criteria. Existing:", hashes.length, "New found:", newCriteriaHashes.length);
      
      if (newCriteriaHashes.length > 0) {
        // Add new criteria to the beginning of the list (since we reverse)
        hashes = [...hashes, ...newCriteriaHashes];
        sortedCriteria = [...newCriteriaHashes.reverse(), ...sortedCriteria];
        criteriaCount = hashes.length;
        // console.log("fetched criteria (added new)", newCriteriaHashes.length, "new criteria. Total now:", hashes.length);
      } else {
        // console.log("fetched criteria (no new criteria found)");
      }
      
      // Update count in case some were deleted (we just won't remove them from UI to preserve state)
      criteriaCount = newCount;
    }
  } catch (e) {
    console.error("Error fetching criteria:", e);
    error = e;
  }
  loading = false;
}

async function joinSignal(event) {
  dispatch('criterion-rated', event.detail)
}

</script>

{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the criteria: {error.data.data}.</span>
{:else if hashes.length === 0}
<span style="font-style: italic;">To get started, add some criteria. What would be necessary for you to accept a proposal?</span>
{:else}
<div style="display: flex; flex-direction: column" class="criterion-outer-all">
  {#each sortedCriteria as criterionHash (criterionHash)}
  <!-- {JSON.stringify(criterionHash)} -->
    <Criterion on:criterion-rated={joinSignal} {showUnsupportedCriteria} bind:unsupportedCriteria criterionHash={criterionHash} {deliberationHash} {filter} bind:sortableCriteria on:criterion-deleted={() => fetchCriteria()}  on:transfer={(e) => {
      
    }}
    on:criterion-comment-created={(e) => {
      dispatch('criterion-comment-created', e.detail);
    }} />
  {/each}

  <!-- {#if sort == "support"}
  {#each sortedCriteria as criterion (criterion.hash)}
    <Criterion on:criterion-rated={joinSignal} criterionHash={criterion.hash} {deliberationHash} {filter} bind:sortableCriteria on:criterion-deleted={() => fetchCriteria()}  on:transfer={(e) => {
       
      }}
      on:criterion-comment-created={(e) => {
        dispatch('criterion-comment-created', e.detail);
      }} />
    {/each}
  {:else if sort == "objections"}
  {#each sortedCriteria as criterion (criterion.hash)}
      <Criterion on:criterion-rated={joinSignal} criterionHash={criterion.hash} {deliberationHash} {filter} bind:sortableCriteria on:criterion-deleted={() => fetchCriteria()}></Criterion>
    {/each}
  {:else if sort == "comments"}
  {#each sortedCriteria as criterion (criterion.hash)}
      <Criterion on:criterion-rated={joinSignal} criterionHash={criterion.hash} {deliberationHash} {filter} bind:sortableCriteria on:criterion-deleted={() => fetchCriteria()}></Criterion>
    {/each}
  {:else if sort == "weight"}
  {#each sortedCriteria as criterion (criterion.hash)}
      <Criterion on:criterion-rated={joinSignal} criterionHash={criterion.hash} {deliberationHash} {filter} bind:sortableCriteria on:criterion-deleted={() => fetchCriteria()}></Criterion>
    {/each}
  {:else if sort == "my support"}
  {#each sortedCriteria as criterion (criterion.hash)}
      <Criterion on:criterion-rated={joinSignal} criterionHash={criterion.hash} {deliberationHash} {filter} bind:sortableCriteria on:criterion-deleted={() => fetchCriteria()}></Criterion>
    {/each}
  {:else if sort == "my objections"}
  {#each sortedCriteria as criterion (criterion.hash)}
      <Criterion on:criterion-rated={joinSignal} criterionHash={criterion.hash} {deliberationHash} {filter} bind:sortableCriteria on:criterion-deleted={() => fetchCriteria()}></Criterion>
    {/each}
  {/if} -->
  {#if unsupportedCriteria?.length > 0}
    {#if showUnsupportedCriteria}
      <button
        class="show-more-button"
        on:click={() => {
          showUnsupportedCriteria = false;
          sortCriteria();
        }}
      >Hide {unsupportedCriteria?.length} unsupported criteria?</button>
    {:else}
      <button
        class="show-more-button"
        on:click={() => {
          showUnsupportedCriteria = true;
          sortCriteria();
        }}
      >Show {unsupportedCriteria?.length} unsupported criteria?</button>
    {/if}
  {/if}
</div>
{/if}

<style lang="css">
  /* if width is less than 100px, display mini view */
  @media (max-width: 300px) {
    .criterion-outer-all {
      display: none !important;
    }
  }

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