<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { EntryHash, Record, AgentPubKey, ActionHash, AppClient, NewEntryAction } from '@holochain/client';
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

  client.on('signal', signal => {
    // console.log("signal", signal)
    if (signal.value.zome_name !== 'converge') return;
    const payload = signal.value.payload as ConvergeSignal;
    if (payload.type !== 'EntryCreated') return;
    if (payload.app_entry.type !== 'Criterion') return;
    console.log("signal", signal)

    // hashes = [...hashes, payload.action.hashed.hash];
    // sortedCriteria = [...sortedCriteria, payload.action.hashed.hash];
    // fetchCriteria()
    fetchAndSort()
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
    sortedCriteria.reverse()
    console.log("fetched criteria", sortedCriteria)
  } catch (e) {
    error = e;
  }
  loading = false;
}

async function joinSignal() {
  dispatch('criterion-rated',{})
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