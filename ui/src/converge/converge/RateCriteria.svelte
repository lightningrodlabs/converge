<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { EntryHash, Record, AgentPubKey, ActionHash, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../contexts';
import RateCriterion from './RateCriterion.svelte';
import type { ConvergeSignal } from './types';

export let deliberationHash: ActionHash;
export let proposalHash: ActionHash;
export let convergence = 0;
export let maxWeight = 0;
export let display: boolean = true;
export let allSupport = {};

const dispatch = createEventDispatcher();

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;
let loading = true;
let error: any = undefined;
let allCombinedRatings = {};
let allWeight = {};
let allRatings = undefined;

$: hashes, loading, error, allRatings, allWeight, allSupport, convergence, proposalHash;
$: if (allRatings) {
    // console.log('allRatings updated:', allRatings);
    // hashes.forEach(hash => {
    //   console.log(`allRatings[${hash.join(',')}]:`, allRatings[hash.join(',')]);
    // });

    // convergence is combined rating multiplied by support
    // allSupport.reduce((acc, num) => acc + num, 0);
    if (Object.keys(allSupport).length > 0) {
      maxWeight = calculateAverage(allSupport)
    }
    // if (Object.keys(allWeight).length > 0) {
      convergence = calculateAverage(allWeight)
      // console.log("convergence", convergence)
    // }
  }

onMount(async () => {
  await fetchCriteria();
  await fetchRatings();
  client.on('signal', signal => {
    if (signal.zome_name !== 'converge') return;
    const payload = signal.payload as ConvergeSignal;
    if (payload.type !== 'EntryCreated') return;
    if (payload.app_entry.type !== 'Criterion') return;
    // hashes = [...hashes, payload.action.hashed.hash];
    fetchCriteria();
  });
  client.on('signal', signal => {
    if (signal.zome_name !== 'converge') return;
    const payload = signal.payload as ConvergeSignal;
    if (!['LinkCreated', 'LinkDeleted'].includes(payload.type)) return;
    fetchRatings();
  });
});

  // Calculate the average of the numeric values in the data object
  function calculateAverage(data) {
    // console.log('hihihop7987i7yiu7t7utgg')
    const values = Object.values(data);
    // console.log(data)

    // Filter out non-numeric values
    const numericValues = values.filter((value) => typeof value === 'number' && !isNaN(value));
    // console.log(numericValues)

    if (numericValues.length === 0) return 0;

    const sum: any = numericValues.reduce((acc: any, num) => acc + num, 0);
    // console.log(sum, numericValues)
    return sum / numericValues.length;
  }

async function fetchCriteria() {
  try {
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'get_criteria_for_deliberation',
      payload: deliberationHash,
    });
    hashes = records.map(r => r.signed_action.hashed.hash);
  } catch (e) {
    error = e;
  }
  loading = false;
}

async function rateAlert() {
  console.log('proposal-rated-2')
  dispatch('proposal-rated')
  console.log('lkjkl')
}

async function fetchRatings() {
  try {
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'get_ratings_for_proposal',
      payload: proposalHash,
    });
    // console.log(records)
    if (records) {
      let newRatings = records.reduce((acc, item) => {
        // Convert Uint8Arrays to strings for comparison and use as keys
        let agentString = item.agent.join(",");
        let criterionString = item.criterion.join(",");
  
        // If the criterion doesn't exist in the result yet, add an empty array
        if (!acc[criterionString]) {
          acc[criterionString] = [];
        }
  
        // console.log('/')  
        // If the agent doesn't exist in the criterion array yet, add it
        if (!acc[criterionString].some((el) => el.agentAsString === agentString)) {
          acc[criterionString].push({ agentAsString: agentString, tag: item.tag });
        }
  
        return acc;
      }, {});
      allRatings = newRatings
    }
    // console.log(allRatings)
  } catch (e) {
    console.log(e)
    error = e;
  }
  loading = false;
}

</script>

{#if display}

{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the criteria: {error.data.data}.</span>
{:else if hashes.length === 0}
<span>No criteria found.</span>
{:else}
<!-- {JSON.stringify(allRatings)} -->
<!-- {JSON.stringify(allSupport)} -->
<!-- average weighted rating {JSON.stringify(convergence)}<br>
average support {JSON.stringify(maxWeight)}<br>
score {JSON.stringify(convergence / maxWeight)} -->

<div style="display: flex; flex: 1; flex-direction: column">
  {#each hashes as hash}
    <!-- <RateCriterion criterionHash={hash} proposalHash={proposalHash} /> -->
    <!-- <div style="margin-bottom: 8px;"> -->
      {#if allRatings && allRatings[hash.join(',')]}
        <RateCriterion on:proposal-rated={rateAlert} bind:allWeight bind:allSupport bind:allCombinedRatings criterionHash={hash} proposalHash={proposalHash} display={true} ratings={allRatings[hash.join(',')]} />
      {:else}
        <RateCriterion on:proposal-rated={rateAlert} bind:allWeight bind:allSupport bind:allCombinedRatings criterionHash={hash} proposalHash={proposalHash} display={true} ratings={[]} />
      {/if}
    <!-- </div> -->
  {/each}
</div>
{/if}

{:else if deliberationHash}
  {#if hashes && hashes.length > 0}
    {#each hashes as hash}
      {#if allRatings && allRatings[hash.join(',')]}
        <RateCriterion on:proposal-rated={rateAlert} bind:allWeight bind:allSupport bind:allCombinedRatings criterionHash={hash} proposalHash={proposalHash} {display} ratings={allRatings[hash.join(',')]} />
      {:else}
        <RateCriterion on:proposal-rated={rateAlert} bind:allWeight bind:allSupport bind:allCombinedRatings criterionHash={hash} proposalHash={proposalHash} {display} ratings={[]} />
      {/if}
    {/each}
  {/if}
{/if}