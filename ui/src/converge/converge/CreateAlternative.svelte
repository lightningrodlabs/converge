<script lang="ts">
  import { onMount, getContext } from 'svelte';
  import '@material/mwc-circular-progress';
  import type { Record, EntryHash, ActionHash, AgentPubKey, AppAgentClient, NewEntryAction } from '@holochain/client';
  import { clientContext } from '../../contexts';
  // import CriterionDetail from './CriterionDetail.svelte';
  import type { ConvergeSignal } from './types';
  
  export let deliberationHash: ActionHash;
  export let criterionHash: ActionHash;
  // let alternativeCriterionHash: ActionHash;
  
  let client: AppAgentClient = (getContext(clientContext) as any).getClient();
  
  let hashes: Array<ActionHash> | undefined;
  let allCriteria: Array<ActionHash> | undefined;
  
  let loading = true;
  let error: any = undefined;
  
  $: hashes, loading, error;
  
  onMount(async () => {
    if (criterionHash === undefined) {
      throw new Error(`The criterionHash input is required for the CriteriaForCriterion element`);
    }
  
    try {
      const records = await client.callZome({
        cap_secret: null,
        role_name: 'converge',
        zome_name: 'converge',
        fn_name: 'get_criteria_for_criterion',
        payload: criterionHash,
      });
      hashes = records.map(r => r.signed_action.hashed.hash);
    } catch (e) {
      error = e;
    }

    await fetchCriteria()
    loading = false;
  
    client.on('signal', signal => {
      if (signal.zome_name !== 'converge') return;
      const payload = signal.payload as ConvergeSignal;
      if (payload.type !== 'LinkCreated') return;
      if (payload.link_type !== 'CriterionToCriteria') return;
  
      hashes = [...hashes, payload.action.hashed.content.target_address];
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
      // criteriaCount = records.length;
      allCriteria = records.map(r => r.signed_action.hashed.hash);
    } catch (e) {
      error = e;
    }
    loading = false;
  }

  async function addAlternative(alternativeHash: ActionHash) {
    try {
      const res = await client.callZome({
        cap_secret: null,
        role_name: 'converge',
        zome_name: 'converge',
        fn_name: 'add_criterion_for_criterion',
        payload: {
          base_criterion_hash: criterionHash,
          target_criterion_hash: alternativeHash,
        },
      });
      console.log(res)
    } catch (e) {
      error = e;
    }
  }
  
  </script>
  
  {#if loading }
  <div style="display: flex; flex: 1; align-items: center; justify-content: center">
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>
  {:else if error}
  <span>Error fetching criteria: {error.data.data}.</span>
  {:else if allCriteria.length === 0}
  <span>No criteria found for this criterion.</span>
  {:else}
  <div style="display: flex; flex-direction: column">
    {#each allCriteria as hash}
    {#if hash.join(',') != criterionHash.join(',')}
      <div style="margin-bottom: 8px;">
        {hash}
        <button on:click={()=>{
          addAlternative(hash)
        }}>Suggest</button>
        <!-- <CriterionDetail criterionHash={hash}></CriterionDetail> -->
      </div>
    {/if}
    {/each}
  </div>
  <div style="display: flex; flex-direction: column">
    {#each hashes as hash}
      <div style="margin-bottom: 8px;">
        {hash}
        <!-- <button on:click={()=>{
          addAlternative(hash)
        }}>Suggest</button> -->
        <!-- <CriterionDetail criterionHash={hash}></CriterionDetail> -->
      </div>
    {/each}
  </div>
  {/if}
  