<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import type { Record, ActionHash, AppClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
import { encodeHashToBase64 } from '@holochain/client';
import { clientContext } from '../../../contexts';
import type { Criterion, ConvergeSignal } from '../types';
import '@material/mwc-circular-progress';
import '@material/mwc-slider';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-snackbar';
import '@material/mwc-icon-button';
import CriterionPopup from './CriterionPopup.svelte';
import { countViewed, addToViewed } from '../../../viewed.js';
import { navigate } from '../../../store.js';
import SvgIcon from "../SvgIcon.svelte";

const dispatch = createEventDispatcher();

export let showSlider: boolean = true;
export let deliberationHash: ActionHash;
export let criterionHash: ActionHash;
export let sortableCriteria;
export let filter;
export let reference;
export let showUnsupportedCriteria = false;
export let unsupportedCriteria = [];

let client: AppClient = (getContext(clientContext) as any).getClient();

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
let unreadCommentsNumber;
let commentHashes = [];
const scoringLevel = 4;

let errorSnackbar: Snackbar;
  
$:  error, loading, record, criterion, supporters, sponsored, criterionPopupBoolean, filter, unsupportedCriteria;

onMount(async () => {
  if (criterionHash === undefined) {
    throw new Error(`The criterionHash input is required for the CriterionDetail element`);
  }
  await fetchCriterion();
  await addToViewed(criterionHash, client);
  await fetchSupport();
  await fetchObjections();
  // console.log(objections)
  let criterionHashKey = criterionHash.join(',')
  if (showSlider) {
    sortableCriteria[criterionHashKey] = {
      objections: objections.length,
      support: support,
      hash: criterionHash,
      comments: commentsNumber,
      weight: support / supporters.length,
      supporters: supporters.length,
      mySupport: mySupport,
      myObjections: objections.filter(item => item.agent.join(",") === client.myPubKey.join(",")).length,
    };
  }
    // console.log(sortableCriteria)

  client.on('signal', async signal => {
    if (signal.App.zome_name !== 'converge') return;
    const payload = signal.App.payload as ConvergeSignal;

    if (payload.message == "criterion-comment-created") {
      console.log("this is a new message", payload)
      if (JSON.stringify(payload.deliberation_hash), JSON.stringify(deliberationHash)) {
        const notificationCriterionHash = JSON.parse(payload.context)?.criterionHash;
        console.log(notificationCriterionHash, criterionHash)
        if(notificationCriterionHash === encodeHashToBase64(criterionHash)) {
          commentsNumber = commentsNumber + 1;
          unreadCommentsNumber = unreadCommentsNumber + 1;
        }
      }
    }

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

      // add to unsupported if no supporters
      if (supporters.length === 0) {
        unsupportedCriteria = Array.from(new Set([...unsupportedCriteria, criterionHash]));
      }

      support = supporters.reduce((sum, item) => {
        let percentage = Number(JSON.parse(item["tag"]).percentage);
        return isNaN(percentage) ? sum : sum + percentage;
      }, 0);
      sponsored = supporters.some(item => item["agent"] === client.myPubKey.join(","));
      if (sponsored) {
        mySupport = JSON.parse(supporters.find(item => item["agent"] === client.myPubKey.join(","))["tag"]).percentage;
        addSupportPercentage = mySupport * scoringLevel;
      } else {
        mySupport = 0;
        addSupportPercentage = 0;
      }

      // get comments
      await getComments()
      loading = false;
    }
    // console.log(records)
  } catch (e) {
    console.log(e)
    error = e;
  }
}

async function getComments() {
  try {
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'get_criterion_comments_for_criterion',
      payload: criterionHash,
    });
    commentsNumber = records.length;
    // console.log(records, "comments")
    commentHashes = records.map(r => r.signed_action.hashed.hash)
    unreadCommentsNumber = Math.max(0, commentsNumber - countViewed(commentHashes));
    console.log("unreadCommentsNumber", unreadCommentsNumber)
  } catch (e) {
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
    // console.log(addSupportPercentage)
    // console.log(scoringLevel)
    let tag = {
      percentage: String(addSupportPercentage / scoringLevel),
      transferedFrom: null
    }

    // console.log(tag)

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
    // if (record) {
    //   console.log("record: ")
    //   console.log(record)
    //   // openSupport = false;
    // }
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
  if (!criterionPopupBoolean) return;
  await new Promise(res => setTimeout(res, 200));
  myDiv.scrollIntoView({ behavior: 'smooth' });
  // console.log(myDiv)
}
</script>

{#if !unsupportedCriteria?.includes(criterionHash) || showUnsupportedCriteria}
<!-- <button on:click={myDiv.scrollIntoView({ behavior: 'smooth' })}>Scroll to Div</button> -->
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center;">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the criterion: {error}</span>
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
    <div class="criterion-line progress-line" style="opacity: {support / supporters.length}; background-color: blue;"></div>
  {/each}
  {/if}
  </div>
</div>
<div class="two-sides" bind:this={myDiv}>
  <div style="display: flex; flex-direction: column; width: 100%;">
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
        <!-- <SvgIcon color="black" icon=faUser size=10/> -->
        Supporters: {supporters.length}
      </div>
      <div style="display: flex; flex-direction: row; font-size: .8em">
        <!-- <SvgIcon color="black" icon=faCog size=10/> -->
        Importance: {Math.round(support / supporters.length * 100)}%
      </div>
      <!-- <div style="display: flex; flex-direction: row; font-size: .8em">
        {JSON.stringify(support / supporters.length)} average support
      </div> -->
    {/if}

    <!-- {#if commentsNumber && commentsNumber > 0}
      <div style="display: flex; flex-direction: row; font-size: .8em">
        Comments: {commentsNumber}
      </div>
    {/if} -->

    {#if objections && objections.length > 0}
    <div style="display: flex; flex-direction: row; margin-bottom: 16px; font-size: .8em; color: red;">
      Objections: {objections.length}
    </div>
    {/if}

  </div>

  {#if showSlider}
    <div style="display: flex; flex-direction: column; margin-bottom: 16px; font-size: .8em"
    on:mouseenter={e => {
      openSupport = true
    }}
    on:mouseleave={e => {
      openSupport = false
    }}>
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
                // console.log(addSupportPercentage, mySupport)
                if (addSupportPercentage == 0) {
                  removeSupport()
                } else {
                  addSupport()
                }
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
  {/if} 
  <!-- SLIDER END -->

  <!-- COMMENTS BUTTON -->
  <div style="flex-direction: column; font-size: .8em; width: 100%; text-align: right;">
    <button style="height: 80%; width: 80px; 
    background-color: transparent;
    border: none;" 
    on:click={async () => {
      criterionPopupBoolean = !criterionPopupBoolean; scrollToDiv()
      await getComments()
    }}
    >
    
    {#if criterionPopupBoolean}
      <mwc-icon-button style="bottom: -8px; position: relative; background-color: #f1f1f1; border-radius: 100%;" icon="↰">
      </mwc-icon-button>
    {:else}
      <mwc-icon-button style="top: 8px;
        position: relative; background-color: #f1f1f1; border-radius: 100%;">
        {#if commentsNumber && commentsNumber > 0}
          <div id="commentsNumber" style="margin-right: {commentsNumber > 9 ? "20px" : "14px"}">
            {commentsNumber}
          </div>
        {/if}
        <SvgIcon icon=faComments size=20/>
      </mwc-icon-button>
      {#if unreadCommentsNumber && unreadCommentsNumber > 0}
        <div id="unreadCommentsNumber">
          {unreadCommentsNumber}
        </div>
      {/if}
    {/if}
    </button>

    {#if !showSlider}
      <button style="height: 80%;
      background-color: transparent;
      border: none;"
      on:click={() => reference(criterion.title)}
      >
      <mwc-icon-button style="top: 8px; position: relative; background-color: #f1f1f1; border-radius: 100%; --mdc-icon-size: 10px;">
          <span style="font-size: 11px; top: -2px; left: -9px; position: relative;">Quote</span>
      </mwc-icon-button>
      </button>
    {/if}
  </div>

</div>
</div>
  <!-- {#if showSlider} -->
  <!-- <div style="display: flex; flex-direction: row;"> -->
    <CriterionPopup on:switched-tab={scrollToDiv} {criterionHash} {objections} {deliberationHash} {showSlider} bind:criterionPopupBoolean {criterion} {supporters} {sponsored} {support} {addSupportPercentage} {mySupport} on:transfer={(e) => {
      dispatch('transfer', e.detail);
    }}
    on:criterion-comment-created={(e) => {
      unreadCommentsNumber = Math.max(0, commentsNumber - countViewed(commentHashes));
      dispatch('criterion-comment-created', e.detail);
    }} />
  <!-- </div> -->
  <!-- {/if} -->
</div>
{/if}
{/if}
{/if}