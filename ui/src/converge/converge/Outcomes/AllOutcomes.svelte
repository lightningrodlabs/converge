<script lang="ts">
  import { createEventDispatcher, onMount, getContext } from 'svelte';
  import '@material/mwc-circular-progress';
  import type { EntryHash, Record, AgentPubKey, ActionHash, AppClient, NewEntryAction } from '@holochain/client';
  import { clientContext } from '../../../contexts';
  import type { ConvergeSignal } from '../types';
  import Outcome from './Outcome.svelte';
  import { view, viewHash, navigate } from '../../../store.js';
  
  export let deliberationHash: ActionHash;
  export let outcomeCount = 0;
  // export let filter;
  // export let sort = "score";
  let loading = true;
  let hashes: Array<ActionHash> | undefined;
  // let outcomeCount = 0;
  let error: any = undefined;
  
  const dispatch = createEventDispatcher();
  
  let client: AppClient = (getContext(clientContext) as any).getClient();


  onMount(async () => {
    await fetchOutcomes();

    client.on('signal', signal => {
      if (signal.zome_name !== 'converge') return;
      const payload = signal.payload as ConvergeSignal;
      if (payload.type !== 'EntryCreated') return;
      if (payload.app_entry.type !== 'Outcome') return;
      // hashes = [...hashes, payload.action.hashed.hash];
      fetchOutcomes();
    });
  });

  async function fetchOutcomes() {
    try {
      const records = await client.callZome({
        cap_secret: null,
        role_name: 'converge',
        zome_name: 'converge',
        fn_name: 'get_outcomes_for_deliberation',
        payload: deliberationHash,
      });
      // outcomeCount = 1;
      hashes = records.map(r => r.signed_action.hashed.hash).reverse()
      outcomeCount = hashes.length
      // sortedOutcomes = hashes
    } catch (e) {
      console.log(e)
      error = e;
    }
    loading = false;
  } 

  async function rateAlert() {
    // console.log('rate-alert-5')
    dispatch('outcome-rated');
  }

</script>


{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the proposals: {error}.</span>
{:else if hashes.length === 0}
<span style="font-style: italic;">If you are ready to add an outcome, you can link to an asset from another tool such as a {"Who's In?"} coordination. Assets are components inside tools in The Weave/Moss.</span>
{:else}
<div style="display: flex; flex-direction: column">
  {#each hashes as hash}
    {#if deliberationHash}
      <!-- {JSON.stringify(hash)} -->
    <!-- {JSON.stringify(sortableProposals)} -->
      <!-- {#if sort == "score"} -->
        <!-- {#each sortedProposals as hash} -->
          <Outcome on:proposal-rated={rateAlert} outcomeHash={hash} {deliberationHash} on:proposal-deleted={() => fetchOutcomes()} />
        <!-- {/each} -->
      <!-- {:else if sort == "respondants"} -->
        <!-- {#each sortedProposals as hash} -->
          <!-- <ProposalListItem on:proposal-rated={rateAlert} bind:anyProposalPopup bind:sortableProposals bind:allProposalScores proposalHash={hash} {deliberationHash} {hashes} {filter} on:proposal-deleted={() => fetchProposals()} /> -->
        <!-- {/each} -->
      <!-- {/if} -->
    {/if}
  {/each}
</div>
{/if}