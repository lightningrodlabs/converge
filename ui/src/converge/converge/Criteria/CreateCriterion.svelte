<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../../contexts';
import type { Criterion, CreateCriterionInput, CriterionComment } from '../types';
import '@material/mwc-button';
import { decode } from '@msgpack/msgpack';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textfield';

export let criterionFormPopup; // Prop to control popup visibility
export let alternativeTo: ActionHash;
let alternativeToFull: Criterion;
function dismissPopup() {
  supportPercentage = 0;
  criterionFormPopup = false; // Set active to false to hide the popup
  title = '';
}

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let deliberationHash: ActionHash;

let title: string = '';

let errorSnackbar: Snackbar;
let supportPercentage = 0;

$: title, criterionFormPopup, supportPercentage;
$: isCriterionValid = true && title !== '';

function checkKey(e) {
  if (e.key === "Escape" && !e.shiftKey) {
    e.preventDefault();
    dismissPopup();
  }
}

onMount(() => {
  window.addEventListener("keydown", checkKey);

    // // Add an event listener to the backdrop to dismiss the popup when clicked outside
    // function handleOutsideClick(event) {
    //   if (criterionFormPopup && !event.target.closest('.popup-container')) {
    //     dismissPopup();
    //   }
    // }

    // document.addEventListener('click', handleOutsideClick);

    // // Clean up the event listener when the component is destroyed
    // return () => {
    //   document.removeEventListener('click', handleOutsideClick);
    // };
    if (alternativeTo) {
      fetchAlternative();
    }
  });

async function fetchAlternative() {
  let record
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'get_criterion',
      payload: alternativeTo,
    });
    if (record) {
      alternativeToFull = decode((record.entry as any).Present.entry) as Criterion;
    }
  } catch (e) {
    console.log(e);
  }
}

async function createCriterion() {
  // console.log(supportPercentage)
  const criterionEntry: CreateCriterionInput = { 
    criterion: {
      title: title!,
    },
    deliberation: deliberationHash,
  };
  
  let criterionHash;

  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'create_criterion',
      payload: criterionEntry,
    });

    criterionHash = record.signed_action.hashed.hash;
    // const criterionActionHash: ActionHash = record

    if (alternativeTo) {
      const res = await client.callZome({
        cap_secret: null,
        role_name: 'converge',
        zome_name: 'converge',
        fn_name: 'add_criterion_for_criterion',
        payload: {
          base_criterion_hash: alternativeTo,
          target_criterion_hash: criterionHash,
        },
      });
      const criterionComment: CriterionComment = { 
        comment: title,
        comment_reference: null,
        objection_reference: null,
        alternative_reference: criterionHash,
        author: client.myPubKey,
        created: Date.now(),
      };
      const criterionCommentEntry = {
        criterion_comment: criterionComment,
        criterion_hash: alternativeTo
      }
      const record: Record = await client.callZome({
        cap_secret: null,
        role_name: 'converge',
        zome_name: 'converge',
        fn_name: 'create_criterion_comment',
        payload: criterionCommentEntry,
      });
    }

    title = '';
  } catch (e) {
    console.log(e)
    errorSnackbar.labelText = `Error creating the criterion: ${e.data.data}`;
    errorSnackbar.show();
  }


  if (supportPercentage > 0) {
    try {
      let tag = {
        percentage: supportPercentage / 4,
        transferedFrom: null
      }

      let record = await client.callZome({
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
      // if (record) {
        // console.log(record)
      // }
      dispatch('criterion-created', {  });
    } catch (e) {
      console.log(e);
    }
  } else {
    dispatch('criterion-created', {  });
  }

  dismissPopup()
}

</script>
{#if criterionFormPopup}
  <div class="backdrop">
    <div class="popup-container">

      <mwc-snackbar bind:this={errorSnackbar} leading>
      </mwc-snackbar>
      <div style="display: flex; flex-direction: column">
        <h2>Add a new criterion</h2>
        {#if alternativeToFull}
        <div><span class="green-alert">↯ </span><strong>Alternative to {alternativeToFull.title}</strong></div><br>
        {/if}
        

        <div style="margin-bottom: 16px">
          <mwc-textarea style="width: 35vw; height: 20vh" outlined label="Title" value={ title } on:input={e => { title = e.target.value; } } required></mwc-textarea>          
        </div>
                  
        <div style="display: flex; flex-direction: column; width: 35vw; margin-bottom: 16px">
          <div style="text-align: center; flex-direction: row; font-size: 1em">
            <span style="white-space: pre-line;">How important is this criterion to you?</span>
          </div>
          <div style="display: flex; flex-direction: row;  font-size: .8em">
          <!-- <input type="number" bind:value={support} /> -->
            <span style="white-space: pre-line; text-align: center;  top: 12px; position: relative;">NOT
            IMPORTANT</span>
            <mwc-slider
              on:change={e => {
                supportPercentage = e.detail.value;
                // console.log(supportPercentage)
              }}
              value={0}
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
        </div>

        <div style="display: flex; flex-direction: row">
          <mwc-button
            outlined
            label="Cancel"
            on:mousedown={() => dismissPopup()}
            style="flex: 1; margin-right: 16px"
          ></mwc-button>

          <mwc-button 
            style="display: inline-block"
            raised
            label="Create Criterion"
            disabled={!isCriterionValid}
            on:mousedown={() => createCriterion()}
          ></mwc-button>
        </div>
        
      </div>
    </div>
  </div>
{/if}