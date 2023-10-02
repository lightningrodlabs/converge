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
import CriterionPopup from './CriterionPopup.svelte';
    import { navigate } from '../../store';

const dispatch = createEventDispatcher();

export let deliberationHash: ActionHash;
export let criterionHash: ActionHash;
export let sortableCriteria;
export let filter;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let loading = true;
let error: any = undefined;

let record: Record | undefined;
let criterion: Criterion | undefined;
let objections;
let supporters: Array<string> | undefined;
let sponsored: boolean | undefined;
let support: number | undefined;
let openSupport = false;
let addSupportPercentage = 0;
let mySupport;
let criterionPopupBoolean = false;
let myDiv;
let commentsNumber;
const scoringLevel = 4;

let errorSnackbar: Snackbar;
  
$:  error, loading, record, criterion, supporters, sponsored, criterionPopupBoolean, filter;

onMount(async () => {
  if (criterionHash === undefined) {
    throw new Error(`The criterionHash input is required for the CriterionDetail element`);
  }
  await fetchCriterion();
  await fetchSupport();
  await fetchObjections();
  console.log(objections)
  let criterionHashKey = criterionHash.join(',')
  sortableCriteria[criterionHashKey] = {
    objections: objections.length,
    support: support,
    hash: criterionHash,
    comments: commentsNumber,
    weight: support / supporters.length,
    mySupport: mySupport,
    myObjections: objections.filter(item => item.agent.join(",") === client.myPubKey.join(",")).length,
  };
  console.log(sortableCriteria)

  client.on('signal', signal => {
    if (signal.zome_name !== 'converge') return;
    const payload = signal.payload as ConvergeSignal;
    if (!['LinkCreated', 'LinkDeleted'].includes(payload.type)) return;
    fetchSupport();
    fetchObjections();
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

async function fetchObjections() {
  try {
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'get_objectors_for_criterion',
      payload: criterionHash,
    });
    objections = records;
  } catch (e) {
    error = e;
  }
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
      // console.log(JSON.parse(supporters[0].tag))
      support = supporters.reduce((sum, item) => {
        let percentage = Number(JSON.parse(item["tag"]).percentage);
        return isNaN(percentage) ? sum : sum + percentage;
      }, 0);
      // average support
      // support = support / supporters.length;
      sponsored = supporters.some(item => item["agent"] === client.myPubKey.join(","));
      // console.log(sponsored, criterionHash, support)
      if (sponsored) {
        // console.log(supporters)
        mySupport = JSON.parse(supporters.find(item => item["agent"] === client.myPubKey.join(","))["tag"]).percentage;
        addSupportPercentage = mySupport * scoringLevel;
      } else {
        mySupport = 0;
        addSupportPercentage = 0;
      }

      // get comments
      try {
        const records = await client.callZome({
          cap_secret: null,
          role_name: 'converge',
          zome_name: 'converge',
          fn_name: 'get_criterion_comments_for_criterion',
          payload: criterionHash,
        });
        commentsNumber = records.length;
      } catch (e) {
        error = e;
      }
      loading = false;
    }
    // console.log(records)
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
    console.log(addSupportPercentage)
    console.log(scoringLevel)
    let tag = {
      percentage: String(addSupportPercentage / scoringLevel),
      transferedFrom: null
    }

    console.log(tag)

    record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'add_criterion_for_supporter',
      payload: {
        base_supporter: client.myPubKey,
        target_criterion_hash: criterionHash,
        tag: String(JSON.stringify(tag)),
      },
    });
    dispatch('criterion-rated', { criterionHash: criterionHash });
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

async function scrollToDiv() {
  await new Promise(res => setTimeout(res, 100));
  myDiv.scrollIntoView({ behavior: 'smooth' });
}
</script>

<!-- <button on:click={myDiv.scrollIntoView({ behavior: 'smooth' })}>Scroll to Div</button> -->
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the criterion: {error.data.data}</span>
{:else}
<!-- {criterionHash}
{criterion.title} -->
<!-- {objections} -->
{#if !filter || (filter && criterion.title.includes(filter))}
<div class="criterion-outer" style="margin-bottom: 8px;">
<div class="criterion">
<div style="display: flex; flex-direction: column; font-size: .8em">
  <div class="vertical-progress-bar-container">

  {#if support}
  {#each Array.from({ length: 35 * support / supporters.length }) as _, index}
    <div class="progress-line" style="opacity: {support / supporters.length}; background-color: blue;"></div>
  {/each}
  {/if}
  </div>
</div>
<div class="two-sides" bind:this={myDiv}>
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
      <div style="display: flex; flex-direction: row; font-size: .8em">
        support: {support / supporters.length}
      </div>
      <!-- <div style="display: flex; flex-direction: row; font-size: .8em">
        {JSON.stringify(support / supporters.length)} average support
      </div> -->
    {/if}

    {#if commentsNumber && commentsNumber > 0}
      <div style="display: flex; flex-direction: row; font-size: .8em">
        comments: {commentsNumber}
      </div>
    {/if}

    {#if objections && objections.length > 0}
    <div style="display: flex; flex-direction: row; margin-bottom: 16px; font-size: .8em; color: red;">
      objections: {objections.length}
    </div>
    {/if}

  </div>

  <div style="display: flex; flex-direction: column; margin-bottom: 16px; font-size: .8em">
      {#if openSupport}
        <div style="text-align: center; flex-direction: row; font-size: 1em">
          <span style="white-space: pre-line;">How important is this criterion to you?</span>
        </div>
        <div style="display: flex; flex-direction: row;  font-size: .8em">
        <!-- <input type="number" bind:value={support} /> -->
          <span style="white-space: pre-line; text-align: center;  top: 12px; position: relative;">NOT
          IMPORTANT</span>
          <mwc-slider
            style="--mdc-theme-primary: blue;"
            on:change={e => {
              addSupportPercentage = e.detail.value
              mySupport = addSupportPercentage / scoringLevel;
              console.log(addSupportPercentage, mySupport)
              if (addSupportPercentage == 0) {
                removeSupport()
              } else {
                addSupport()
              }
            }}
            on:mouseleave={e => {
              openSupport = false
            }}
            value={addSupportPercentage}
            withTickMarks
            discrete
            class="star-slider"
            step="1"
            max="4"
            >
          </mwc-slider>
          <span style="white-space: pre-line; text-align: center; top: 12px; position: relative;">VERY
            IMPORTANT</span>
        </div>
        <!-- <div style="text-align: center; flex-direction: row; mfont-size: .8em"> -->
          <!-- <button on:click={() => openSupport = false}>Cancel</button> -->
          <!-- <mwc-button dense outlined on:click={() => openSupport = false}>Cancel</mwc-button> -->
          
          <!-- <button on:click={() => addSupport()}>Save</button> -->
          <!-- <mwc-button class="custom-button" dense raised on:click={() => addSupport()}>Save</mwc-button> -->
        <!-- </div> -->
      {:else if sponsored}
        <!-- <div style="display: flex; flex-direction: row; margin-bottom: 16px; font-size: .8em">
          <mwc-button dense outlined on:click={() => removeSupport()}>Remove Support</mwc-button>
        </div> -->
        <div style="text-align: center; flex-direction: row; font-size: 1em;">
          <span style="white-space: pre-line; opacity: 0;">Importance to you:</span>
        </div>
        <div style="display: flex; flex-direction: row; font-size: .8em">
          <span style="white-space: pre-line; text-align: center;  top: 12px; position: relative; opacity: 0">NOT
            IMPORTANT</span>
          <mwc-slider
          style="--mdc-theme-primary: blue;"
          on:mouseover={e => {
            openSupport = true
          }}
          value={mySupport * scoringLevel}
          class="star-slider"
          step="1"
          max="4"
          >
        </mwc-slider>
        <span style="white-space: pre-line; text-align: center;  top: 12px; position: relative; opacity: 0;">VERY
          IMPORTANT</span>
        </div>
      {:else}
        <div style="text-align: center; flex-direction: row; font-size: 1em; opacity: 0;">
          <span style="white-space: pre-line;">How important is this criterion to you?</span>
        </div>
        <div style="display: flex; flex-direction: row; font-size: .8em">
          <span style="white-space: pre-line; text-align: center;  top: 12px; position: relative; opacity: 0">NOT
            IMPORTANT</span>
          <mwc-slider
          style="--mdc-theme-primary: blue;"
          on:mouseover={e => {
            openSupport = true
          }}
          disabled=true
          value={addSupportPercentage}
          class="star-slider"
          step="1"
          max="4"
          >
        </mwc-slider>
        <span style="white-space: pre-line; text-align: center;  top: 12px; position: relative; opacity: 0;">VERY
          IMPORTANT</span>
          <!-- <button on:click={() => openSupport = true}>Add Support</button> -->
          <!-- <mwc-button class="custom-button" dense outlined on:click={() => openSupport = true}>Add support</mwc-button> -->

        <!-- <button on:click={() => addSupport()}>Add Support</button> -->
        </div>
      {/if}
  </div>

  <!-- OBJECT BUTTON -->
  <div style="display: flex; flex-direction: column; font-size: .8em">
    <button style="height: 80%;; width: 80px; 
    background-color: transparent;
    border: none;
    " on:click={() => {criterionPopupBoolean = !criterionPopupBoolean; console.log(criterionPopupBoolean); scrollToDiv()}}>
      {#if criterionPopupBoolean}
      <mwc-icon-button style="bottom: 8px;
      position: relative;" icon="↰"></mwc-icon-button>
      {:else}
      <mwc-icon-button style="top: 8px;
      position: relative;" icon="↲"></mwc-icon-button>
      <!-- <svg style="width: 20px; hegith: 20px;" viewBox="0 0 156 156"><rect fill="none" height="8px" width="8px"/><polyline fill="none" points="208 96 128 176 48 96" stroke="#000" stroke-linecap="round" stroke-linejoin="round" stroke-width="16"/></svg> -->
      {/if}
    </button>
  </div>

</div>
</div>
<!-- <div style="display: flex; flex-direction: row;"> -->
  <!-- hi -->
  <CriterionPopup on:switched-tab={scrollToDiv} {criterionHash} {objections} {deliberationHash} bind:criterionPopupBoolean {criterion} {supporters} {sponsored} {support} {addSupportPercentage} {mySupport}/>
<!-- </div> -->
</div>
{/if}
{/if}