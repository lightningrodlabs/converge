<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { EntryHash, Record, AgentPubKey, ActionHash, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../contexts';
import Criterion from './Criterion.svelte';
import type { ConvergeSignal } from './types';

export let deliberationHash: ActionHash;
export let criteriaCount = 0;
export let filter;
export let sort;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;
let loading = true;
let error: any = undefined;
let sortableCriteria = {};
let sortedCriteria = [];

$: hashes, loading, error, sortedCriteria, sortableCriteria, sort, filter;
$: if (sort && sortableCriteria && hashes && Object.values(sortableCriteria).length == hashes.length) {
  console.log(sortableCriteria)
  let sortedCriteriaJoined = Object.values(sortableCriteria).sort((a, b) => {
    if (sort === 'support') {
      return b.support - a.support;
    } else if (sort === 'oppose') {
      // return a.support - b.support
      if (a.objections && b.objections) {
        return a.objections.length - b.objections.length;
      } else {
        return 0
      }
    // } else if (sort === 'abstain') {
    //   return b.abstain - a.abstain;
    // } else if (sort === 'created') {
    //   return b.created - a.created;
    }
    //  else {
    //   return hashes
    // }
  });
  sortedCriteria = sortedCriteriaJoined.map((c) => c.hash);
  console.log(sortedCriteria)
}

onMount(async () => {

  await fetchCriteria();
  client.on('signal', signal => {
    if (signal.zome_name !== 'converge') return;
    const payload = signal.payload as ConvergeSignal;
    if (payload.type !== 'EntryCreated') return;
    if (payload.app_entry.type !== 'Criterion') return;
    // hashes = [...hashes, payload.action.hashed.hash];
    fetchCriteria();
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
<span>No criteria found.</span>
{:else}
<div style="display: flex; flex-direction: column">
  {#if sort == "support"}
    {#each sortedCriteria as hash}
      <Criterion criterionHash={hash} {deliberationHash} {filter} bind:sortableCriteria on:criterion-deleted={() => fetchCriteria()}></Criterion>
    {/each}
  {:else if sort == "objections"}
    {#each sortedCriteria as hash}
      <Criterion criterionHash={hash} {deliberationHash} {filter} bind:sortableCriteria on:criterion-deleted={() => fetchCriteria()}></Criterion>
    {/each}
  {/if}
</div>
{/if}

