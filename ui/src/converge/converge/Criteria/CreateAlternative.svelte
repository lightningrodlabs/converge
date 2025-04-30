<script lang="ts">
  import { onMount, getContext, createEventDispatcher } from 'svelte';
  import '@material/mwc-circular-progress';
  import type { Record, EntryHash, ActionHash, AgentPubKey, AppAgentClient, NewEntryAction } from '@holochain/client';
  import { clientContext } from '../../../contexts';
  import { decode } from '@msgpack/msgpack';
  import CreateCriterion from './CreateCriterion.svelte';
  // import CriterionDetail from './CriterionDetail.svelte';
  import type { ConvergeSignal, Criterion, CriterionComment } from '../types';
  import { encodeHashToBase64 } from "@holochain/client";
  
  export let deliberationHash: ActionHash;
  export let criterionHash: ActionHash;
  export let mySupport;
  // let alternativeCriterionHash: ActionHash;
  export let alternatives: Array<any> | undefined;
  
  
  let client: AppAgentClient = (getContext(clientContext) as any).getClient();

  const dispatch = createEventDispatcher();
  
  let hashes: Array<ActionHash> | undefined;
  let allCriteria: Array<any> | undefined;
  let dialog;
  let criterionFormPopup = false;
  let loading = true;
  let error: any = undefined;
  let selectedCriterion;
  
  $: hashes, loading, error, alternatives, criterionFormPopup;
  
  onMount(async () => {
    if (criterionHash === undefined) {
      throw new Error(`The criterionHash input is required for the CriteriaForCriterion element`);
    }

    await fetchCriteria()
    await fetchAlternatives()
    loading = false;
  
    client.on('signal', signal => {
      if (signal.value.zome_name !== 'converge') return;
      const payload = signal.value.payload as ConvergeSignal;
      if (payload.type !== 'LinkCreated') return;
      let linkType = Object.keys(payload.link_type)[0]
      if (linkType !== 'CriterionToCriteria') return;
      // console.log(payload)
      fetchAlternatives()
      // alternatives = [...hashes, payload.action.hashed.content.target_address];
    });
  });

  async function fetchAlternatives() {
    try {
      const records = await client.callZome({
        cap_secret: null,
        role_name: 'converge',
        zome_name: 'converge',
        fn_name: 'get_criteria_for_criterion',
        payload: criterionHash,
      });
      // hashes = records.map(r => r.signed_action.hashed.hash);
      if (records) {
        alternatives = records.map(record => {
          let a = decode((record.entry as any).Present.entry) as Criterion;
          return {title: a.title, hash: record.signed_action.hashed.hash}
        })
      } else {
        return {}
      }
    } catch (e) {
      console.log(e)
      error = e;
    }
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
      // criteriaCount = records.length;
      // allCriteriaHashes = records.map(r => r.signed_action.hashed.hash);
      allCriteria = records.map(record => {
        let a = decode((record.entry as any).Present.entry) as Criterion;
        return {title: a.title, hash: record.signed_action.hashed.hash}
      })
    } catch (e) {
      console.log(e)
      error = e;
    }
    loading = false;
  }

  async function createCriterionComment(comment, comment_reference, objection_reference, alternative_reference, created) {  
    const criterionComment: CriterionComment = { 
      comment: comment!,
      comment_reference: comment_reference,
      objection_reference: objection_reference,
      alternative_reference: alternative_reference,
      author: client.myPubKey,
      created: created!,
    };

    const criterionCommentEntry = {
      criterion_comment: criterionComment,
      criterion_hash: criterionHash
    }
    
    try {
      const record: Record = await client.callZome({
        cap_secret: null,
        role_name: 'converge',
        zome_name: 'converge',
        fn_name: 'create_criterion_comment',
        payload: criterionCommentEntry,
      });

      dispatch('criterion-comment-created', {context: JSON.stringify({criterionCommentHash: encodeHashToBase64(record.signed_action.hashed.hash), criterionHash: encodeHashToBase64(criterionHash)})});
    } catch (e) {
      console.log(e)
    }
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
      let title = allCriteria.filter(c => c.hash.join(',') === alternativeHash.join(','))[0].title
      // console.log(title)
      createCriterionComment('Alternative: ' + title, null, null, alternativeHash, Date.now())
      selectedCriterion = undefined;
      // console.log(res)
    } catch (e) {
      error = e;
    }
  }

  async function acceptAlternative(alternativeHash: ActionHash) {
    try {
      let tag = {
        percentage: mySupport,
        transferedFrom: criterionHash
      }

      let record1 = await client.callZome({
        cap_secret: null,
        role_name: 'converge',
        zome_name: 'converge',
        fn_name: 'remove_criterion_for_supporter',
        payload: {
          base_supporter: client.myPubKey,
          target_criterion_hash: criterionHash,
        },
      });

      let record2 = await client.callZome({
        cap_secret: null,
        role_name: 'converge',
        zome_name: 'converge',
        fn_name: 'add_criterion_for_supporter',
        payload: {
          base_supporter: client.myPubKey,
          target_criterion_hash: alternativeHash,
          tag: String(JSON.stringify(tag)),
        },
      });

    } catch (e) {
      console.log(e)
    }
  }
  
  </script>
  

  {#if loading }
  <div style="display: flex; flex: 1; align-items: center; justify-content: center">
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>
  {:else if error}
  <span>Error fetching criteria: {error}.</span>
  {:else if allCriteria && allCriteria.length === 0}
  <span>No criteria found for this criterion.</span>
  {:else if alternatives && allCriteria}
  {#if true}
  
  <!-- <div style="display: flex; flex-direction: column; height: 160px; overflow: scroll;"> -->
    <!-- {#each alternatives as a}
      <div style="margin-bottom: 8px;">
        {a.title}
        {#if mySupport}
        <button on:click={()=>{
          acceptAlternative(a.hash)
        }}>Transfer</button>
        {/if}
      </div>
    {/each} -->
  <!-- </div> -->
  <CreateCriterion deliberationHash={deliberationHash} alternativeTo = {criterionHash} bind:criterionFormPopup 
  on:criterion-comment-created={(e) => {
    dispatch('criterion-comment-created', e.detail);
  }}
  />
    <!-- {JSON.stringify(alternatives)} -->
    <!-- {JSON.stringify(alternatives.map((a)=>{return a.hash.join(',')}))} -->
    <!-- {#each allCriteria as c}
    {#if c.hash.join(',') != criterionHash.join(',') && !alternatives.map((a)=>{return a.hash.join(',')}).includes(c.hash.join(','))}
      <div style="margin-bottom: 8px;">
        {c.title}
        <button on:click={()=>{
          addAlternative(c.hash)
        }}>Suggest</button>
      </div>
    {/if}
    {/each} -->

    <select bind:value={selectedCriterion} on:change={()=>{dialog.show()}} style="width: fit-content; margin: 6px; max-width: 40%;">
      <option value="" disabled selected>Suggest existing alternative</option>
      
      {#each allCriteria as c (c.hash.join(','))}
        {#if c.hash.join(',') != criterionHash.join(',') && !alternatives.map(a => a.hash.join(',')).includes(c.hash.join(','))}
          <option value={c.hash} on:click={dialog.show()}>
            {c.title}
          </option>
        {/if}
      {/each}
    </select>

    {#if allCriteria && selectedCriterion}
    {@const selectedCriterionName = allCriteria.filter(a => a.hash.join(',') === selectedCriterion.join(','))[0].title}
    <sl-dialog label="Suggest {selectedCriterionName} as an alternative?" bind:this={dialog}>
      <button
        on:click={(e)=>{
          selectedCriterion = undefined
        }}
      >Cancel</button>
      <button on:click={(e)=>{
        addAlternative(selectedCriterion)
      }}>Suggest</button>
    </sl-dialog>
    {/if}
    <!-- <button on:click={() => addAlternative(selectedCriterion)} style="width: fit-content">Suggest</button> -->
    
    <button on:click={(e)=>{criterionFormPopup = true}} style="width: fit-content; margin: 6px;">Create alternative</button>
  {/if}
  {:else}
  No
  {/if}

  <style>
    mwc-tab {
      border: 1px solid #7b1fa2;
    }

    sl-dialog::part(panel) {
      text-align: center;
      background: #FFFFFF;
      border: 2px solid rgb(166 115 55 / 26%);
      border-bottom: 2px solid rgb(84 54 19 / 50%);
      border-top: 2px solid rgb(166 115 55 / 5%);
      box-shadow: 0px 15px 40px rgb(130 107 58 / 35%);
      border-radius: 10px;
  }
  </style>