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
import { countViewed, addToViewed } from '../../../viewed.js';
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
  // if (e.key === "Enter" && !e.shiftKey) {
  //   e.preventDefault();
  //   if (commentIsAnObjection) {
  //     addObjection()
  //   } else {
  //     createCriterionComment();
  //   }
  // }
}

async function createCriterionComment() {
  // commentReference = null;
  // console.log(commentReference)
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

    addToViewed(record.signed_action.hashed.hash, client);

    comment = ''
    commentReference = null;
    // commentIsAnObjection = false;
    // console.log('criterion-comment-created', {context: JSON.stringify({criterionCommentHash: encodeHashToBase64(record.signed_action.hashed.hash), criterionHash: encodeHashToBase64(criterionHash)})});
    dispatch('criterion-comment-created', {context: JSON.stringify({criterionCommentHash: encodeHashToBase64(record.signed_action.hashed.hash), criterionHash: encodeHashToBase64(criterionHash)})});
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

      addToViewed(record.signed_action.hashed.hash, client);

      comment = ''
      commentReference = null;
      // commentIsAnObjection = false;
      dispatch('criterion-comment-created', {context: JSON.stringify({criterionCommentHash: encodeHashToBase64(record.signed_action.hashed.hash), criterionHash: encodeHashToBase64(criterionHash)})});
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
      // console.log(res)
      // let s = decode(res) as
      // let s = decode((record.entry as any).Present.entry) as Criterion;
      // console.log(s)

      createCriterionCommentCustom(comment, null, res, null, Date.now())

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
    margin-bottom: 8px;
    border-radius: 4px;
  }

  .chat-input {
    flex: 1;
    padding: 8px;
    margin-right: 8px;
    border: none;
    outline: none;
    height: 40px;
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
  {#if commentIsAnObjection}
    <span class="instructions">
      This comment will appear as an objection, and the criterion score will be reduced until your objection is removed.
    </span>
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
        // check if the user supports the criterion
        addObjection()
      } else {
        createCriterionComment();
      }
    }}>
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" fill="white"><!--!Font Awesome Free 6.7.2 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free Copyright 2025 Fonticons, Inc.--><path d="M498.1 5.6c10.1 7 15.4 19.1 13.5 31.2l-64 416c-1.5 9.7-7.4 18.2-16 23s-18.9 5.4-28 1.6L284 427.7l-68.5 74.1c-8.9 9.7-22.9 12.9-35.2 8.1S160 493.2 160 480l0-83.6c0-4 1.5-7.8 4.2-10.8L331.8 202.8c5.8-6.3 5.6-16-.4-22s-15.7-6.4-22-.7L106 360.8 17.7 316.6C7.1 311.3 .3 300.7 0 288.9s5.9-22.8 16.1-28.7l448-256c10.7-6.1 23.9-5.5 34 1.4z"/></svg>
      <!-- <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16" fill="none" class="h-4 w-4 m-1 md:m-0" stroke-width="2"><path d="M.5 1.163A1 1 0 0 1 1.97.28l12.868 6.837a1 1 0 0 1 0 1.766L1.969 15.72A1 1 0 0 1 .5 14.836V10.33a1 1 0 0 1 .816-.983L8.5 8 1.316 6.653A1 1 0 0 1 .5 5.67V1.163Z" fill="currentColor"></path></svg> -->
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
