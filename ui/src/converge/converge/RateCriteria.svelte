<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { EntryHash, Record, AgentPubKey, ActionHash, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../contexts';
import RateCriterion from './RateCriterion.svelte';
import type { ConvergeSignal } from './types';

export let deliberationHash: ActionHash;
export let proposalHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;
let loading = true;
let error: any = undefined;
let allRatings = undefined;

$: hashes, loading, error, allRatings;
$: if (allRatings) {
    console.log('allRatings updated:', allRatings);
    hashes.forEach(hash => {
      console.log(`allRatings[${hash.join(',')}]:`, allRatings[hash.join(',')]);
    });
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

async function fetchRatings() {
  try {
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'get_ratings_for_proposal',
      payload: proposalHash,
    });
    console.log(records)
    let newRatings = records.reduce((acc, item) => {
      // Convert Uint8Arrays to strings for comparison and use as keys
      let agentString = item.agent.join(",");
      let criterionString = item.criterion.join(",");

      // If the criterion doesn't exist in the result yet, add an empty array
      if (!acc[criterionString]) {
        acc[criterionString] = [];
      }

      console.log('/')
      console.log(criterionString)

      // If the agent doesn't exist in the criterion array yet, add it
      if (!acc[criterionString].some((el) => el.agentAsString === agentString)) {
        acc[criterionString].push({ agentAsString: agentString, tag: item.tag });
      }

      return acc;
    }, {});
    allRatings = newRatings
    // console.log(allRatings)
  } catch (e) {
    console.log(e)
    error = e;
  }
  loading = false;
}

</script>

{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the criteria: {error.data.data}.</span>
{:else if hashes.length === 0}
<span>No criteria found.</span>
{:else}
<div style="display: flex; flex: 1; flex-direction: column">
  {#each hashes as hash}
    <!-- <div style="margin-bottom: 8px;"> -->
      {#if allRatings && allRatings[hash.join(',')]}
        <RateCriterion criterionHash={hash} proposalHash={proposalHash} ratings={allRatings[hash.join(',')]} />
      {:else}
        <RateCriterion criterionHash={hash} proposalHash={proposalHash} ratings={[]} />
      {/if}
    <!-- </div> -->
  {/each}
</div>
{/if}

