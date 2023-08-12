<script lang="ts">
  import { createEventDispatcher, onMount, getContext } from 'svelte';
  import '@material/mwc-circular-progress';
  import { decode } from '@msgpack/msgpack';
  import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
  import { clientContext } from '../../contexts';
  import type { Criterion, ConvergeSignal } from './types';
  import '@material/mwc-circular-progress';
  import '@material/mwc-slider';
  import type { Snackbar } from '@material/mwc-snackbar';
  import '@material/mwc-snackbar';
  import '@material/mwc-icon-button';
  import { navigate } from '../../store';
    import ObjectionsForCriteria from './ObjectionsForCriteria.svelte';
  
  const dispatch = createEventDispatcher();
  
  export let criterionHash: ActionHash;
  export let criterionPopupBoolean: boolean;
  
  let client: AppAgentClient = (getContext(clientContext) as any).getClient();
  
  let loading = true;
  let error: any = undefined;
  
  let record: Record | undefined;
  let criterion: Criterion | undefined;
  let supporters: Array<string> | undefined;
  let sponsored: boolean | undefined;
  let support: number | undefined;
  let openSupport = false;
  let addSupportPercentage = 0;
  let mySupport;
  let activeTab;
  let objection;
  const scoringLevel = 4;
  
  let errorSnackbar: Snackbar;
    
  $:  error, loading, record, criterion, supporters, sponsored, criterionPopupBoolean, objection;
  
  onMount(async () => {
    if (criterionHash === undefined) {
      throw new Error(`The criterionHash input is required for the CriterionDetail element`);
    }
    await fetchCriterion().then(() => fetchSupport());
    client.on('signal', signal => {
      if (signal.zome_name !== 'converge') return;
      const payload = signal.payload as ConvergeSignal;
      if (!['LinkCreated', 'LinkDeleted'].includes(payload.type)) return;
      fetchSupport();
    });
  });
  
  async function fetchCriterion() {
    loading = true;
    error = undefined;
    record = undefined;
    criterion = undefined;
    
    try {
      record = await client.callZome({
        cap_secret: null,
        role_name: 'converge',
        zome_name: 'converge',
        fn_name: 'get_criterion',
        payload: criterionHash,
      });
      if (record) {
        criterion = decode((record.entry as any).Present.entry) as Criterion;
      }
    } catch (e) {
      error = e;
    }
  
    loading = false;
  }
  
  async function fetchSupport() {
    try {
      let records = await client.callZome({
        cap_secret: null,
        role_name: 'converge',
        zome_name: 'converge',
        fn_name: 'get_supporters_for_criterion',
        payload: criterionHash,
      });
      if (records) {
        supporters = Array.from(
          records.reduce((map, item) => {
            const key = item.agent.join(",");
            if (!map.has(key)) {
              map.set(key, { ...item, agent: key });
            }
            return map;
          }, new Map()).values()
        );
        support = supporters.reduce((sum, item) => sum + JSON.parse(item["tag"]), 0);
        // average support
        // support = support / supporters.length;
        sponsored = supporters.some(item => item["agent"] === client.myPubKey.join(","));
        if (sponsored) {
          mySupport = supporters.find(item => item["agent"] === client.myPubKey.join(","))["tag"];
          addSupportPercentage = mySupport * scoringLevel;
        }
      }
    } catch (e) {
      console.log(e)
      error = e;
    }
  }
  
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

  async function addObjection() {
    console.log(objection)
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
    } catch (e) {
      error = e;
    }
    loading = false;
  }
  </script>
  
  {#if criterionPopupBoolean}
  <div class="backdrop">
    <div class="popup-container">

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

<!-- ACTIVITY STARTS -->
<div class="deliberation-section">
  <mwc-tab-bar style="--mdc-theme-primary: blue; margin-bottom: 10px;">
    <mwc-tab on:click={() => {activeTab = "all"}} label="All responses"></mwc-tab>
    <mwc-tab on:click={() => {activeTab = "objections"}}  label="Objections"></mwc-tab>
    <mwc-tab on:click={() => {activeTab = "alternatives"}}  label="Alternatives"></mwc-tab>
  </mwc-tab-bar>
</div>


<!-- COMMENTS STARTS -->
<ObjectionsForCriteria {criterionHash}></ObjectionsForCriteria>

<div style="margin-bottom: 16px">
  <mwc-textarea style="width: 35vw; height: 20vh" outlined label="Comment" on:input={e => { objection = e.target.value; console.log(objection)}} required></mwc-textarea>          
</div>
<div style="margin-bottom: 16px">
  <!-- check box is this an objection -->
  <!-- <mwc-formfield label="Is your comment an objection to the criterion?">
    <mwc-checkbox></mwc-checkbox>
  </mwc-formfield> -->
  <label>
    <mwc-switch name="choice"></mwc-switch> Is your comment an objection to the criterion?
  </label>

  <mwc-button on:click = {() => {addObjection()}}>Submit</mwc-button>
</div>

</div>
</div>
{/if}
