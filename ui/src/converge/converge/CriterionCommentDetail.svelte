<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { CriterionComment, Objection } from './types';
import '@material/mwc-circular-progress';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-snackbar';
import '@material/mwc-icon-button';
import EditCriterionComment from './EditCriterionComment.svelte'; 

const dispatch = createEventDispatcher();

export let criterionCommentHash: ActionHash;
export let objections;
export let alternatives;
export let commentReference;;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let loading = true;
let error: any = undefined;

let record: Record | undefined;
let criterionComment: CriterionComment | undefined;
let objectionsLookup: any = undefined;
let alternativesLookup: any = undefined;
let objection;
let alternative;
let respondingTo;

let editing = false;

let errorSnackbar: Snackbar;
  
$: editing,  error, loading, record, criterionComment, objections, alternatives;
// object of objection hashes linked with .join(',') referencing their whole objection
$: if (objections) {
  objectionsLookup = Object.fromEntries(objections.map((o) => [o.objection_hash, o]));
}

$: if (alternatives) {
  alternativesLookup = Object.fromEntries(alternatives.map((o) => [o.alternative_hash, o]));
}

onMount(async () => {
  if (criterionCommentHash === undefined) {
    throw new Error(`The criterionCommentHash input is required for the CriterionCommentDetail element`);
  }
  await fetchCriterionComment();
});

async function fetchObjection(objection_hash) {
  try {
    let record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'get_objection_link',
      payload: objection_hash,
    });
    return record
  } catch (e: any) {
    console.log(e)
  }
}

async function fetchAlternative(objection_hash) {
  try {
    let record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'get_alternative_link',
      payload: objection_hash,
    });
    return record
  } catch (e: any) {
    console.log(e)
  }
}

async function fetchCriterionComment() {
  loading = true;
  error = undefined;
  record = undefined;
  criterionComment = undefined;
  
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'get_criterion_comment',
      payload: criterionCommentHash,
    });
    if (record) {
      criterionComment = decode((record.entry as any).Present.entry) as CriterionComment;
      console.log(criterionComment.objection_reference)

      if (criterionComment.objection_reference) {
        objection = await fetchObjection(criterionComment.objection_reference)
      } else if (criterionComment.alternative_reference) {
        alternative = await fetchAlternative(criterionComment.alternative_reference)
      } else if (criterionComment.comment_reference) {
        let res = await await client.callZome({
          cap_secret: null,
          role_name: 'converge',
          zome_name: 'converge',
          fn_name: 'get_criterion_comment',
          payload: criterionComment.comment_reference,
        });
        respondingTo = decode((res.entry as any).Present.entry) as CriterionComment;
      }

      // console.log(test)
      // let x = decode(test.tag) as string;
    }
  } catch (e) {
    error = e;
  }

  loading = false;
}

async function deleteCriterionComment() {
  try {
    await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'delete_criterion_comment',
      payload: criterionCommentHash,
    });
    dispatch('criterion-comment-deleted', { criterionCommentHash: criterionCommentHash });
  } catch (e: any) {
    errorSnackbar.labelText = `Error deleting the criterion comment: ${e.data.data}`;
    errorSnackbar.show();
  }
}
</script>

<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>

{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the criterion comment: {error.data.data}</span>
{:else if editing}
<EditCriterionComment
  originalCriterionCommentHash={ criterionCommentHash}
  currentRecord={record}
  on:criterion-comment-updated={async () => {
    editing = false;
    await fetchCriterionComment()
  } }
  on:edit-canceled={() => { editing = false; } }
></EditCriterionComment>
{:else}

<div style="display: flex; flex-direction: column;">
  <!-- <div style="display: flex; flex-direction: row">
    <span style="flex: 1"></span>
    <mwc-icon-button style="margin-left: 8px" icon="edit" on:click={() => { editing = true; } }></mwc-icon-button>
    <mwc-icon-button style="margin-left: 8px" icon="delete" on:click={() => deleteCriterionComment()}></mwc-icon-button>
  </div> -->
  
  <!-- {#if criterionComment && criterionComment.objection_reference && objectionsLookup}
    {JSON.stringify(objectionsLookup[criterionComment.objection_reference.join(',')].tag)}
  {/if}

  {#if criterionComment && criterionComment.alternative_reference && alternativesLookup}
    {JSON.stringify(alternativesLookup[criterionComment.alternative_reference.join(',')].tag)}
  {/if} -->

  {#if objection}
    {objection.comment}
  {/if}

  {#if alternative}
    {JSON.stringify(alternative)}
  {/if}

  {#if respondingTo}
    {JSON.stringify(respondingTo.comment)}
  {/if}

  <div style="display: flex; flex-direction: row; margin-bottom: 4px">
    <span style="margin-right: 4px"><strong>Comment:</strong></span>
    <span style="white-space: pre-line">{ criterionComment.comment }</span>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 4px">
    <span style="margin-right: 4px"><strong>Created:</strong></span>
    <span style="white-space: pre-line">{ new Date(criterionComment.created / 1000).toLocaleString() }</span>
  </div>

  <button style="width: fit-content;" on:click={() => {commentReference = {hash: criterionCommentHash, comment: criterionComment.comment}}}>Reply</button>

</div>
{/if}

