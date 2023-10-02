<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../../contexts';
import type { CriterionComment } from '../types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';
import '@vaadin/date-time-picker/theme/material/vaadin-date-time-picker.js';
import '@material/mwc-checkbox';
import { encodeHashToBase64 } from "@holochain/client";

import '@material/mwc-textfield';
let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let criterionHash: ActionHash;
export let commentReference: any;
export let commentIsAnObjection: boolean;

// let commentReference!: ActionHash | undefined;
let objectionReference!: ActionHash | undefined;
let alternativeReference!: ActionHash | undefined;
let author: AgentPubKey = client.myPubKey;
let comment: string = '';
let created: number = Date.now();

let errorSnackbar: Snackbar;

$: comment, commentReference, objectionReference, alternativeReference, author, created;
$: isCriterionCommentValid = true && comment !== '';

onMount(() => {
  // if (commentReference === undefined) {
  //   throw new Error(`The commentReference input is required for the CreateCriterionComment element`);
  // }
  // if (objectionReference === undefined) {
  //   throw new Error(`The objectionReference input is required for the CreateCriterionComment element`);
  // }
  // if (alternativeReference === undefined) {
  //   throw new Error(`The alternativeReference input is required for the CreateCriterionComment element`);
  // }
  // if (author === undefined) {
  //   throw new Error(`The author input is required for the CreateCriterionComment element`);
  // }
});

function checkKey(e) {
  if (e.key === "Enter" && !e.shiftKey) {
    e.preventDefault();
    if (commentIsAnObjection) {
      addObjection()
    } else {
      createCriterionComment();
    }
  }
}

async function createCriterionComment() {
  // commentReference = null;
  console.log(commentReference)
  let cr: ActionHash | undefined = undefined;
  if (commentReference && commentReference.hash) {
    cr = commentReference.hash!;
  }
  const criterionComment: CriterionComment = { 
    comment: comment!,
    comment_reference: cr!,
    objection_reference: objectionReference!,
    alternative_reference: alternativeReference!,
    author: author!,
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

    comment = ''
    commentReference = null;
    commentIsAnObjection = false;
    dispatch('criterion-comment-created', { criterionCommentHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the criterion comment: ${e.data.data}`;
    errorSnackbar.show();
  }
}

async function createCriterionCommentCustom(inputComment, comment_reference, objection_reference, alternative_reference, created) {  
    const criterionComment: CriterionComment = { 
      comment: inputComment!,
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

      comment = ''
      commentReference = null;
      commentIsAnObjection = false;
      dispatch('criterion-comment-created', { criterionCommentHash: record.signed_action.hashed.hash });
    } catch (e) {
      errorSnackbar.labelText = `Error creating the criterion comment: ${e.data.data}`;
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
      console.log("error", e)
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
          comment: comment
        },
      });
      console.log(res)
      // let s = decode(res) as
      // let s = decode((record.entry as any).Present.entry) as Criterion;
      // console.log(s)

      createCriterionCommentCustom('', null, res, null, Date.now())

    } catch (e) {
      console.log("error", e)
    }
  }

</script>

<style>
  .chat-input-container {
    display: flex !important;
    justify-content: space-between !important;
    align-items: center;
    border: 1px solid #ccc;
    padding: 8px;
  }

  .chat-input {
    flex: 1;
    padding: 8px;
    margin-right: 8px;
    border: none;
    outline: none;
  }

  .send-button {
    cursor: pointer;
    padding: 10px;
    /* background-color: #007BFF; */
    background-color: blue;
    color: #ffffff;
    height: 40px;
    width: 40px;
    border: none;
    border-radius: 4px;
  }

  .responding-to {
    padding: 10px;
    background-color: #efeeee;
    display: flex;
    flex-direction: row;
    justify-content: space-between;
  }
</style>

<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <!-- <span style="font-size: 18px">Create CriterionComment</span> -->
  
  {#if commentReference}
  <div class="responding-to">
    <div>⬐ Responding to: {JSON.stringify(commentReference.comment)}</div>
    <div style="cursor: pointer; display:flex; flex-direction: column; align-items:end;" on:click={()=>{commentReference = undefined}}>✖</div>
  </div>
  {/if}
  <div class="chat-input-container" style={commentIsAnObjection ? "border: 2px solid red;" : ""}>
    <textarea
      class="chat-input"
      bind:value={comment}
      on:keydown={checkKey}
      placeholder="Type your message..."
    ></textarea>
    <button class="send-button" on:click={() => {
      if (commentIsAnObjection) {
        addObjection()
      } else {
        createCriterionComment();
      }
    }}>
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" class="h-4 w-4 m-1 md:m-0" stroke-width="2"><path d="M.5 1.163A1 1 0 0 1 1.97.28l12.868 6.837a1 1 0 0 1 0 1.766L1.969 15.72A1 1 0 0 1 .5 14.836V10.33a1 1 0 0 1 .816-.983L8.5 8 1.316 6.653A1 1 0 0 1 .5 5.67V1.163Z" fill="currentColor"></path></svg>
    </button>
  </div>

  <!-- <div style="margin-bottom: 16px">
    <mwc-textfield outlined label="Comment" value={ comment } on:input={e => {
      // console.log(e.target.value)
      comment = e.target.value;
      console.log(comment)
       } } required></mwc-textfield>          
  </div> -->
            
  <!-- <div style="margin-bottom: 16px">
    <vaadin-date-time-picker label="Created" value={new Date(created / 1000).toISOString()} on:change={e => { created = new Date(e.target.value).valueOf() * 1000;} } required></vaadin-date-time-picker>          
  </div> -->
            
<!-- 
  <mwc-button 
    style="width: 10px;
    height: 10px;
    margin: 10px"
    raised
    label=">"
    disabled={!isCriterionCommentValid}
    on:click={() => createCriterionComment()}
  ></mwc-button> -->
</div>
