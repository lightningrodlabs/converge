<script lang="ts">
  import { createEventDispatcher, onMount, getContext } from 'svelte';
  import '@material/mwc-circular-progress';
  import { decode } from '@msgpack/msgpack';
  import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
  import { clientContext } from '../../contexts';
  import type { Criterion, ConvergeSignal, CriterionComment } from './types';
  import '@material/mwc-circular-progress';
  import '@material/mwc-slider';
  import type { Snackbar } from '@material/mwc-snackbar';
  import '@material/mwc-snackbar';
  import '@material/mwc-icon-button';
  import { navigate } from '../../store';
    import ObjectionsForCriteria from './ObjectionsForCriteria.svelte';
    import CreateAlternative from './CreateAlternative.svelte';
    import CreateCriterion from './CreateCriterion.svelte';
    import CriterionCommentsForCriterion from './CriterionCommentsForCriterion.svelte';
  
  const dispatch = createEventDispatcher();
  
  export let criterionHash: ActionHash;
  export let criterionPopupBoolean: boolean;
  export let deliberationHash: ActionHash;
  
  let client: AppAgentClient = (getContext(clientContext) as any).getClient();
  
  let loading = true;
  let error: any = undefined;
  
  let record: Record | undefined;
  export let criterion: Criterion | undefined;
  export let supporters: Array<string> | undefined;
  export let sponsored: boolean | undefined;
  export let support: number | undefined;
  let openSupport = false;
  export let addSupportPercentage = 0;
  export let mySupport;
  let activeTab = "all";
  let objection;
  const scoringLevel = 4;
  export let objections;
  let alternatives;
  
  let errorSnackbar: Snackbar;
    
  $:  error, loading, record, criterion, supporters, sponsored, criterionPopupBoolean, objection, support, activeTab, objections;
  
  onMount(async () => {
    if (criterionHash === undefined) {
      throw new Error(`The criterionHash input is required for the CriterionDetail element`);
    }
  });
  
  async function removeSupport() {
    try {
      record = await client.callZome({
        cap_secret: null,
        role_name: 'converge',
        zome_name: 'converge',
        fn_name: 'remove_criterion_for_supporter',
        payload: {
          base_supporter: client.myPubKey,
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
  
  async function addSupport() {
    await removeSupport()
    try {
      record = await client.callZome({
        cap_secret: null,
        role_name: 'converge',
        zome_name: 'converge',
        fn_name: 'add_criterion_for_supporter',
        payload: {
          base_supporter: client.myPubKey,
          target_criterion_hash: criterionHash,
          percentage: String(addSupportPercentage / scoringLevel),
        },
      });
      // openSupport = false;
      if (record) {
        console.log("record: ")
        console.log(record)
        // openSupport = false;
      }
    } catch (e) {
      error = e;
    }
  }
  
  async function deleteCriterion() {
    try {
      await client.callZome({
        cap_secret: null,
        role_name: 'converge',
        zome_name: 'converge',
        fn_name: 'delete_criterion',
        payload: criterionHash,
      });
      dispatch('criterion-deleted', { criterionHash: criterionHash });
    } catch (e: any) {
      errorSnackbar.labelText = `Error deleting the criterion: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  async function removeObjection() {
    try {
      const res = await client.callZome({
        cap_secret: null,
        role_name: 'converge',
        zome_name: 'converge',
        fn_name: 'remove_criterion_for_objector',
        payload: {
          base_objector: client.myPubKey,
          target_criterion_hash: criterionHash,
        },
      });
    } catch (e) {
      error = e;
    }
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
      dispatch('criterion-comment-created', { criterionCommentHash: record.signed_action.hashed.hash });
    } catch (e) {
      errorSnackbar.labelText = `Error creating the criterion comment: ${e.data.data}`;
      errorSnackbar.show();
    }
  }

  async function addObjection() {
    await removeObjection()
    try {
      const res = await client.callZome({
        cap_secret: null,
        role_name: 'converge',
        zome_name: 'converge',
        fn_name: 'add_criterion_for_objector',
        payload: {
          base_objector: client.myPubKey,
          target_criterion_hash: criterionHash,
          comment: objection
        },
      });
      console.log(res)
      // let s = decode(res) as
      // let s = decode((record.entry as any).Present.entry) as Criterion;
      // console.log(s)

      createCriterionComment('', null, res, null, Date.now())

    } catch (e) {
      error = e;
    }
    loading = false;
  }
  </script>
  
  {#if false}
  <!-- <div class="backdrop">
    <div class="popup-container"> -->

      <button on:click={() => {criterionPopupBoolean = false;}}>
        <mwc-icon-button icon="x"></mwc-icon-button>
      </button>
      
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
      <!-- <div style="display: flex; flex-direction: column; font-size: .8em">
        <div class="vertical-progress-bar-container">
      
        {#if support}
        {#each Array.from({ length: 35 * support / supporters.length }) as _, index}
          <div class="progress-line" style="opacity: {support / supporters.length}; background-color: blue;"></div>
        {/each}
        {/if}
        </div>
      </div> -->
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
            <div style="display: flex; flex-direction: row; margin-bottom: 16px; font-size: .8em">
              {supporters.length} supporters
            </div>
            <div style="display: flex; flex-direction: row; font-size: .8em">
              {JSON.stringify(support / supporters.length)} average support
              <!-- {support} support -->
            </div>
          {:else}
            <div style="display: flex; flex-direction: row; font-size: .8em">
              0 supporters
            </div>
          {/if}
      
        </div>

        </div>
      </div>
      {/if}

{/if}

{#if criterionPopupBoolean}

<!-- ACTIVITY STARTS -->
<div>
  <!-- <mwc-button on:click={()=>{}}>Add Alternative</mwc-button> -->
  <!-- <mwc-tab-bar style="--mdc-theme-primary: blue; margin-bottom: 10px;">
    <mwc-tab on:click={() => {activeTab = "all"; dispatch('switched-tab')}} label="All responses"></mwc-tab>
    <mwc-tab on:click={() => {activeTab = "objections"; dispatch('switched-tab')}}  label="Objections"></mwc-tab>
    <mwc-tab on:click={() => {activeTab = "alternatives"; dispatch('switched-tab')}}  label="Alternatives"></mwc-tab>
  </mwc-tab-bar> -->

  <!-- <div class="criterion-popup-header">
    <select bind:value={activeTab}>
      <option value='all'>Filter: none</option>
      <option value='objections'>Filter: objections</option>
      <option value='alternatives'>Filter: alternatives</option>
    </select>
  </div> -->
  
  
  <!-- COMMENTS STARTS -->
  <CriterionCommentsForCriterion {criterionHash} {objections} {alternatives} {deliberationHash} {mySupport}></CriterionCommentsForCriterion>



  <!-- COMMENTS STARTS -->
  <!-- old comments -->
  {#if false}
  {#if activeTab == "all"}
    <CriterionCommentsForCriterion {criterionHash} {objections} {alternatives}></CriterionCommentsForCriterion>
  {:else if activeTab == "objections"}
  <ObjectionsForCriteria {criterionHash} bind:objections></ObjectionsForCriteria>
  <div style="margin-bottom: 16px">
    <mwc-textarea style="width: 35vw; height: 100px" outlined label="Comment" on:input={e => { objection = e.target.value; console.log(objection)}} required></mwc-textarea>          
  </div>
  <div style="margin-bottom: 16px">
    <!-- check box is this an objection -->
    <!-- <mwc-formfield label="Is your comment an objection to the criterion?">
      <mwc-checkbox></mwc-checkbox>
    </mwc-formfield> -->
    <!-- <label>
      <mwc-switch name="choice"></mwc-switch> Is your comment an objection to the criterion?
    </label>
     -->
    <mwc-button on:click = {() => {addObjection()}}>Submit</mwc-button>
  </div>
  {:else if activeTab == "alternatives"}
  <CreateAlternative {criterionHash} {deliberationHash} {mySupport} bind:alternatives></CreateAlternative>
  {/if}
  
  <!-- </div>
  </div> -->
  {/if}
</div>
{/if}
  