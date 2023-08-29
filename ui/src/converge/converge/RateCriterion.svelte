<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Criterion, ConvergeSignal } from './types';
import '@material/mwc-circular-progress';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-snackbar';
import '@material/mwc-icon-button';

const dispatch = createEventDispatcher();

export let criterionHash: ActionHash;
export let proposalHash: ActionHash;
export let ratings: any[] | undefined;
export let allSupport;
export let allCombinedRatings;
export let allWeight;
export let display: boolean = true;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let loading = true;
let error: any = undefined;

let record: Record | undefined;
let criterion: Criterion | undefined;
let objections;
let supporters: Array<string> | undefined;
let sponsored: boolean | undefined;
let support: number | undefined;
let averageSupport: number | undefined = 1;
let responded: boolean | undefined;
let deliberated: boolean | undefined;
let openEvaluation = false;
let addEvaluationPercentage = 0;
let myEvaluation: any | undefined;
let combinedRating = 0;

let errorSnackbar: Snackbar;
const scoringLevel = 4
  
$:  error, loading, record, criterion, objections, supporters, sponsored, responded, myEvaluation, deliberated, ratings, addEvaluationPercentage, averageSupport;

$: responded = ratings.some(item => item["agentAsString"] === client.myPubKey.join(","));
$: if (responded) {
  // console.log("ratings", ratings)
  myEvaluation = ratings.find(item => item["agentAsString"] === client.myPubKey.join(","))["tag"];
  // find average rating as combinedRating
  combinedRating = ratings.reduce((sum, item) => sum + JSON.parse(item["tag"]), 0) / ratings.length;
  let hashTag = criterionHash.join(',');
  allCombinedRatings[hashTag] = combinedRating
  allWeight[hashTag] = combinedRating * averageSupport
} else if (ratings && ratings.length > 0) {
  // console.log('hishihihihi')
  combinedRating = ratings.reduce((sum, item) => sum + JSON.parse(item["tag"]), 0) / ratings.length;
  let hashTag = criterionHash.join(',');
  allCombinedRatings[hashTag] = combinedRating
  allWeight[hashTag] = combinedRating * averageSupport
}

// async function calculateRatings() {
//   // find average rating as combinedRating
//   combinedRating = ratings.reduce((sum, item) => sum + JSON.parse(item["tag"]), 0) / ratings.length;
//   console.log(combinedRating)
//   console.log(ratings)
//   let hashTag = criterionHash.join(',');
//   allCombinedRatings[hashTag] = combinedRating
//   allWeight[hashTag] = combinedRating * averageSupport
//   // addEvaluationPercentage = myEvaluation * 10;
// }

onMount(async () => {
  if (criterionHash === undefined) {
    throw new Error(`The criterionHash input is required for the CriterionDetail element`);
  }
  if (responded) {
    myEvaluation = ratings.find(item => item["agentAsString"] === client.myPubKey.join(","))["tag"];
    addEvaluationPercentage = myEvaluation * scoringLevel;
  }
  await fetchCriterion().then(() => {fetchSupport();});
  client.on('signal', signal => {
    if (signal.zome_name !== 'converge') return;
    const payload = signal.payload as ConvergeSignal;
    if (!['LinkCreated', 'LinkDeleted'].includes(payload.type)) return;
    fetchSupport();
    // fetchRating();
    // ratings = [...ratings]
    // responded = ratings.some(item => item["agentAsString"] === client.myPubKey.join(","));
    // if (responded) {
    //   myEvaluation = ratings.find(item => item["agentAsString"] === client.myPubKey.join(","))["tag"];
    //   addEvaluationPercentage = myEvaluation * 10;
    // }
  });
});

async function fetchCriterion() {
  loading = true;
  error = undefined;
  record = undefined;
  criterion = undefined;
  // let record;
  // let criterion;
  
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
    console.log(e)
    error = e;
  }

  loading = false;
}

async function fetchSupport() {
  await fetchObjections()
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
      support = supporters.reduce((sum, item) => sum + Number(JSON.parse(item["tag"]).percentage), 0);
      let objectionCount;
      if (objections) {
        objectionCount = objections.length;
      } else {
        objectionCount = 0;
      }
      let adjustedSupport = Math.max(0, support - objectionCount)
      console.log(objections, support, objectionCount, adjustedSupport)
      averageSupport = adjustedSupport / supporters.length
      if (supporters) {
        sponsored = supporters.some(item => item["agent"] === client.myPubKey.join(","));
      }
      allSupport[criterionHash.join(',')] = averageSupport;
    }
  } catch (e) {
    console.log(e)
    error = e;
  }
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

// if (sponsored) {
//   myEvaluation = Number(supporters.find(item => item["agent"] === client.myPubKey.join(","))["tag"]);
//   console.log(myEvaluation)
// }

// async function fetchRating() {
//   try {
//     let deliberations = await client.callZome({
//       cap_secret: null,
//       role_name: 'converge',
//       zome_name: 'converge',
//       fn_name: 'get_deliberations_for_criterion',
//       payload: criterionHash,
//     });
//     if (deliberations) {
//       console.log(deliberations)
//       deliberated = deliberations.some(item => item["agent"] === client.myPubKey.join(","));
//       if (deliberated) {
//         myEvaluation = deliberations.find(item => item["agent"] === client.myPubKey.join(","))["tag"];
//         console.log(myEvaluation)
//         addEvaluationPercentage = myEvaluation * 10;
//       }
//     }
//   } catch (e) {
//     error = e;
//   }
// }

async function removeRating() {
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'remove_criterion_for_proposal',
      payload: {
        base_proposal_hash: proposalHash,
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

async function addRating() {
  await removeRating();
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'add_criterion_for_proposal',
      payload: {
        base_proposal_hash: proposalHash,
        target_criterion_hash: criterionHash,
        percentage: String(addEvaluationPercentage / scoringLevel),
      },
    });
    if (record) {
      console.log(record)
    }
  } catch (e) {
    error = e;
  }
}

</script>

{#if display}

<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
  

{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the criterion: {error}</span>
{:else if support}


<div class="outlined-item criterion-outer" style="border-radius: 0;">
  <!-- <div style="display: flex; flex-direction: column; font-size: .8em">
    <div class="vertical-progress-bar-container">
  
    {#if support}
    {#each Array.from({ length: 35 * support / supporters.length }) as _, index}
      <div class="progress-line" style="opacity: {support / supporters.length}"></div>
    {/each}
    {/if}
    </div>
  </div>
  <div style="display: flex; flex-direction: column; font-size: .8em">
    <div class="vertical-progress-bar-container">
    {#if support}
    {#each Array.from({ length: 35 * support / supporters.length }) as _, index}
      <div class="progress-line" style="opacity: {support / supporters.length}; background-color: rgb(254, 18, 18);"></div>
    {/each}
    {/if}
    </div>
  </div> -->
  <div class="two-sides">
    <div style="display: flex; flex-direction: column; margin-right: 20px">
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
        <!-- <div style="display: flex; flex-direction: row; margin-bottom: 16px; font-size: .8em">
          {supporters.length} supporters
        </div> -->
        <!-- <div style="display: flex; flex-direction: row; font-size: .8em">
          weight: {averageSupport * 100}%
        </div> -->
        <div style="display: flex; flex-direction: row; font-size: .8em">
          score: {combinedRating * 100}%
        </div>
        <!-- <div style="display: flex; flex-direction: row; font-size: .8em">
          {support * combinedRating} weight
        </div> -->
        
      {:else}
        <div style="display: flex; flex-direction: row; font-size: .8em">
          <!-- <div style="display: flex; flex-direction: row; font-size: .8em">
            {combinedRating} average rating
          </div>
          <div style="display: flex; flex-direction: row; font-size: .8em">
            {support * combinedRating} weight
          </div> -->
          0 supporters
        </div>
      {/if}
  
    </div>
  
    <div style="display: flex; flex-direction: column; margin-bottom: 16px; font-size: .8em">
        {#if openEvaluation}
          <div style="text-align: center; flex-direction: row; font-size: 1em">
            <span style="white-space: pre-line;">How well is this criterion met by the proposal?</span>
          </div>
          <div style="display: flex; flex-direction: row;  font-size: .8em">
          <!-- <input type="number" bind:value={support} /> -->
            <span style="white-space: pre-line; text-align: center;  top: 12px; position: relative;">NOT
            MET</span>
            <mwc-slider
              style="--mdc-theme-primary: red;"
              on:pointerup={e => {
                addEvaluationPercentage = e.target.value
                myEvaluation = addEvaluationPercentage / scoringLevel;
                addRating();
              }}
              on:mouseleave={e => {
                openEvaluation = false
              }}
              value={addEvaluationPercentage}
              class="star-slider"
              withTickMarks
              discrete
              step="1"
              max="4"
              >
            </mwc-slider>
            <span style="white-space: pre-line; text-align: center; top: 12px; position: relative;">FULLY
              MET</span>
          </div>
          <!-- <div style="text-align: center; flex-direction: row; mfont-size: .8em"> -->
            <!-- <button on:click={() => openEvaluation = false}>Cancel</button> -->
            <!-- <mwc-button dense outlined on:click={() => openSupport = false}>Cancel</mwc-button> -->
            
            <!-- <button on:click={() => addRating()}>Save</button> -->
            <!-- <mwc-button class="custom-button" dense raised on:click={() => addSupport()}>Save</mwc-button> -->
          <!-- </div> -->
        {:else if responded}
          <div style="text-align: center; flex-direction: row; font-size: 1em;">
            <span style="white-space: pre-line; opacity: 0;">Importance to you:</span>
          </div>
          <div style="display: flex; flex-direction: row; font-size: .8em">
            <span style="white-space: pre-line; text-align: center;  top: 12px; position: relative; opacity: 0">NOT
              MET</span>
            <mwc-slider
            style="--mdc-theme-primary: red;"
            on:mouseover={e => {
              openEvaluation = true
            }}
            value={myEvaluation * scoringLevel}
            class="star-slider"
            step="1"
            max="4"
            >
          </mwc-slider>
          <span style="white-space: pre-line; text-align: center;  top: 12px; position: relative; opacity: 0;">FULLY
            MET</span>
          </div>
        {:else}
          <div style="text-align: center; flex-direction: row; font-size: 1em;">
            <span style="white-space: pre-line; opacity: 0;">Importance to you:</span>
          </div>
          <div style="display: flex; flex-direction: row; font-size: .8em">
            <span style="white-space: pre-line; text-align: center;  top: 12px; position: relative; opacity: 0">NOT
              MET</span>
            <mwc-slider
            style="--mdc-theme-primary: red;"
            on:mouseover={e => {
              openEvaluation = true
            }}
            disabled=true
            value={0}
            class="star-slider"
            step="1"
            max="4"
            >
          </mwc-slider>
          <span style="white-space: pre-line; text-align: center;  top: 12px; position: relative; opacity: 0;">FULLY
            MET</span>
          </div>
        {/if}
    </div>
  </div>
  </div>










{#if false}
<div class="criterion">
<div style="display: flex; flex-direction: column">

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="white-space: pre-line">{ criterion.title }</span>
    {#if support}
    <span style="white-space: pre-line">{support}</span>
    {/if}

    <!-- {ratings.length} -->

    <!-- {#each supporters as supporter}
      <span style="white-space: pre-line">{ supporter }</span>
    {/each} -->
  </div>

</div>
<div style="display: flex; flex-direction: column">
  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    {#if responded}
      <button on:click={() => removeRating()}>Remove Rating</button>
    {:else}
      <button on:click={() => addRating()}>Add Rating</button>
    {/if}
  </div>
</div>
</div>
{/if}
{/if}
{/if}