<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, DnaHash, ActionHash } from '@holochain/client';
import { decode } from '@msgpack/msgpack';
import { clientContext } from '../../../contexts';
import type { CriterionComment } from '../types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';
import '@vaadin/date-time-picker/theme/material/vaadin-date-time-picker.js';

import '@material/mwc-textfield';
let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let originalCriterionCommentHash!: ActionHash;

export let currentRecord!: Record;
let currentCriterionComment: CriterionComment = decode((currentRecord.entry as any).Present.entry) as CriterionComment;

let comment: string | undefined = currentCriterionComment.comment;
let created: number | undefined = currentCriterionComment.created;

let errorSnackbar: Snackbar;

$: comment, created;
$: isCriterionCommentValid = true && comment !== '' && true;

onMount(() => {
  if (currentRecord === undefined) {
    throw new Error(`The currentRecord input is required for the EditCriterionComment element`);
  }
  if (originalCriterionCommentHash === undefined) {
    throw new Error(`The originalCriterionCommentHash input is required for the EditCriterionComment element`);
  }
});

async function updateCriterionComment() {

  const criterionComment: CriterionComment = { 
    comment: comment!,
    created: created!,
    comment_reference: currentCriterionComment.comment_reference,
    objection_reference: currentCriterionComment.objection_reference,
    alternative_reference: currentCriterionComment.alternative_reference,
    author: currentCriterionComment.author,
  };

  try {
    const updateRecord: Record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'update_criterion_comment',
      payload: {
        original_criterion_comment_hash: originalCriterionCommentHash,
        previous_criterion_comment_hash: currentRecord.signed_action.hashed.hash,
        updated_criterion_comment: criterionComment
      }
    });
  
    dispatch('criterion-comment-updated', { actionHash: updateRecord.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error updating the criterion comment: ${e.data.data}`;
    errorSnackbar.show();
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Edit CriterionComment</span>
  
  <div style="margin-bottom: 16px">
    <mwc-textfield outlined label="Comment" value={ comment } on:input={e => { comment = e.target.value; } } required></mwc-textfield>    
  </div>

  <div style="margin-bottom: 16px">
    <vaadin-date-time-picker label="Created" value={new Date(created / 1000).toISOString()} on:change={e => { created = new Date(e.target.value).valueOf() * 1000;} } required></vaadin-date-time-picker>    
  </div>


  <div style="display: flex; flex-direction: row">
    <mwc-button
      outlined
      label="Cancel"
      on:click={() => dispatch('edit-canceled')}
      style="flex: 1; margin-right: 16px"
    ></mwc-button>
    <mwc-button 
      raised
      label="Save"
      disabled={!isCriterionCommentValid}
      on:click={() => updateCriterionComment()}
      style="flex: 1;"
    ></mwc-button>
  </div>
</div>
