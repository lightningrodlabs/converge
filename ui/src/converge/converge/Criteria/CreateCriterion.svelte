<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../../contexts';
import type { Criterion, CreateCriterionInput, CriterionComment } from '../types';
import '@material/mwc-button';
import { decode } from '@msgpack/msgpack';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textfield';
import { encodeHashToBase64 } from "@holochain/client";
import StarSlider from './StarSlider.svelte';

export let criterionFormPopup; // Prop to control popup visibility
export let alternativeTo: ActionHash;
let alternativeToFull: Criterion;
function dismissPopup() {
  supportPercentage = 0;
  criterionFormPopup = false; // Set active to false to hide the popup
  title = '';
}

let client: AppClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let deliberationHash: ActionHash;

let title: string = '';

let errorSnackbar: Snackbar;
let supportPercentage = 0;
let isCreating = false;

const SOURCE_CHAIN_MOVED_ERROR = 'source chain head has moved';

$: title, criterionFormPopup, supportPercentage, isCreating;
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

function getErrorMessage(error: any): string {
  return String(error?.message ?? error?.data?.data ?? error ?? 'Unknown error');
}

async function callZomeWithRetry(request: Parameters<AppClient['callZome']>[0], maxRetries = 4) {
  for (let attempt = 0; attempt < maxRetries; attempt += 1) {
    try {
      return await client.callZome(request);
    } catch (error) {
      const message = getErrorMessage(error).toLowerCase();
      if (message.includes(SOURCE_CHAIN_MOVED_ERROR) && attempt < maxRetries - 1) {
        await new Promise(resolve => setTimeout(resolve, 150 * (attempt + 1)));
        continue;
      }
      throw error;
    }
  }

  throw new Error('Unable to complete zome call after retries');
}

async function createCriterion() {
  if (isCreating) {
    return;
  }

  isCreating = true;

  // Auto-join the deliberation if not already joined
  try {
    await callZomeWithRetry({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'add_deliberation_for_deliberator',
      payload: {
        base_deliberator: client.myPubKey,
        target_deliberation_hash: deliberationHash,
      },
    });
  } catch (e) {
    // User may already be joined, continue
    console.log("Note: User may already be joined to deliberation", e);
  }

  // console.log(supportPercentage)
  const criterionEntry: CreateCriterionInput = { 
    criterion: {
      title: title!,
    },
    deliberation: deliberationHash,
  };
  
  let criterionHash;

  try {
    const record = await callZomeWithRetry({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'create_criterion',
      payload: criterionEntry,
    }) as Record;

    criterionHash = record.signed_action.hashed.hash;
    // const criterionActionHash: ActionHash = record

    if (alternativeTo) {
      await callZomeWithRetry({
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
      const commentRecord = await callZomeWithRetry({
        cap_secret: null,
        role_name: 'converge',
        zome_name: 'converge',
        fn_name: 'create_criterion_comment',
        payload: criterionCommentEntry,
      }) as Record;

      dispatch('criterion-comment-created', {context: JSON.stringify({criterionCommentHash: encodeHashToBase64(commentRecord.signed_action.hashed.hash), criterionHash: encodeHashToBase64(alternativeTo)})});
    }

    title = '';
  } catch (e) {
    console.log("error", e)
    errorSnackbar.labelText = `Error creating the criterion: ${getErrorMessage(e)}`;
    errorSnackbar.show();
    isCreating = false;
    return;
  }


  if (supportPercentage > 0) {
    try {
      const tag = {
        percentage: supportPercentage / 4,
        transferedFrom: null
      };

      await callZomeWithRetry({
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
    } catch (e) {
      console.log(e);
    }
  }
  dispatch('criterion-created', {  });

  dismissPopup()
  isCreating = false;
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
        <div><span class="green-alert">↯ </span><strong>Alternative to {alternativeToFull.title}</strong></div>
        <br>
        {/if}
        <details class="instructions" style="margin: 0 0 16px 0; width: calc(100vw - 111px); max-width: 388px; margin-bottom: 0;">
          <summary style="font-size: 14px; font-weight: bold; cursor: pointer;">TIP: Does the criterion have these helpful characteristics?</summary>
          <div style="white-space: pre-line; font-size: 14px; margin-top: 8px;">
            • Is noncontroversial<br>
            • Can be evaluated (subjectively)<br>
            • Uses positive language (what is wanted rather than what is not wanted)<br>
            • Is relevant to the context<br>
            • Is operational (can in principle be made to happen)<br>
            • Applies to everyone (not just one person’s need)<br>
            • Doesn’t have a specific outcome
          </div>
        </details>

        <br>

        <div style="margin-bottom: 16px">
          <mwc-textarea style="max-width: 400px; height: 90px; width: calc(100vw - 84px);" outlined label="Description" value={ title } on:input={e => { title = e.target.value; } } required></mwc-textarea>          
        </div>
                  
        <div style="display: flex; flex-direction: column; max-width: 400px; margin-bottom: 16px">
          <div style="text-align: center; flex-direction: row; font-size: 1em; width: calc(100vw - 84px); max-width: 400px;">
            <span style="white-space: pre-line;">How important is this criterion to you?</span>
          </div>
          <div style="display: flex; flex-direction: row;  font-size: .8em; width: calc(100vw - 84px); max-width: 400px;">
          <!-- <input type="number" bind:value={support} /> -->
            <span style="white-space: pre-line; text-align: center;  top: 12px; position: relative;">NOT
            IMPORTANT</span>
            <!-- <StarSlider
              max={5}
              rating={supportPercentage}
              size={32}
              color="blue"
              hoverColor="lightblue"
              on:rate={e => {
                supportPercentage = e.detail.rating;
                // console.log(supportPercentage)
              }}
            ></StarSlider> -->
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
            label={isCreating ? 'Creating...' : 'Create Criterion'}
            disabled={!isCriterionValid || isCreating}
            on:mousedown={() => createCriterion()}
          ></mwc-button>
        </div>
        
      </div>
    </div>
  </div>
{/if}