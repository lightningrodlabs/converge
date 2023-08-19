<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { CriterionComment } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';
import '@vaadin/date-time-picker/theme/material/vaadin-date-time-picker.js';

import '@material/mwc-textfield';
let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let criterionHash: ActionHash;
export let commentReference: any;

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

async function createCriterionComment() {  
  const criterionComment: CriterionComment = { 
    comment: comment!,
    comment_reference: commentReference.hash!,
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
    dispatch('criterion-comment-created', { criterionCommentHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the criterion comment: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create CriterionComment</span>
  

  <div style="margin-bottom: 16px">
    {#if commentReference}
      {JSON.stringify(commentReference.comment)}<br>
    {/if}
    <mwc-textfield outlined label="Comment" value={ comment } on:input={e => {
      // console.log(e.target.value)
      comment = e.target.value;
      console.log(comment)
       } } required></mwc-textfield>          
  </div>
            
  <!-- <div style="margin-bottom: 16px">
    <vaadin-date-time-picker label="Created" value={new Date(created / 1000).toISOString()} on:change={e => { created = new Date(e.target.value).valueOf() * 1000;} } required></vaadin-date-time-picker>          
  </div> -->
            

  <mwc-button 
    raised
    label="Create CriterionComment"
    disabled={!isCriterionCommentValid}
    on:click={() => createCriterionComment()}
  ></mwc-button>
</div>
