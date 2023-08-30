<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
import { clientContext, profilesStoreContext } from '../../contexts';
import type { CriterionComment, Objection, Criterion } from './types';
import type { ProfilesStore } from "@holochain-open-dev/profiles";
import '@material/mwc-circular-progress';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-snackbar';
import '@material/mwc-icon-button';
import EditCriterionComment from './EditCriterionComment.svelte'; 
import ObjectionMini from './Objection.svelte'
import AlternativeMini from './Alternative.svelte'
import { encodeHashToBase64 } from "@holochain/client";
import type { AsyncStatus } from "@holochain-open-dev/stores";
import type { Profile } from "@holochain-open-dev/profiles";

const dispatch = createEventDispatcher();

export let criterionCommentHash: ActionHash;
export let filter;
export let mySupport;
export let criterionHash;
// export let objections;
// export let alternatives;
export let commentReference;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();
let profilesStore: ProfilesStore = (getContext(profilesStoreContext) as any).getProfileStore();

let loading = true;
let error: any = undefined;

let record: Record | undefined;
let criterionComment: CriterionComment | undefined;
// let objectionsLookup: any = undefined;
// let alternativesLookup: any = undefined;
let objection;
let alternative;
let respondingTo;
let nickName;

let editing = false;
let errorSnackbar: Snackbar;

  
$: editing,  error, loading, record, criterionComment, alternative, nickName;

// $: s = store.profilesStore.profiles.get(agentPubKey)
// $: profile = $s.status == "complete" ? $s.value : undefined
// object of objection hashes linked with .join(',') referencing their whole objection
// $: if (objections) {
//   objectionsLookup = Object.fromEntries(objections.map((o) => [o.objection_hash, o]));
// }

// $: if (alternatives) {
//   alternativesLookup = Object.fromEntries(alternatives.map((o) => [o.alternative_hash, o]));
// }

// const getNickName = (asyncProfile:AsyncStatus<Profile>) : string => {
//   if (asyncProfile.status != "complete") return  "..."
//   return asyncProfile.value ? asyncProfile.value.nickname : "?"
// }

onMount(async () => {
  if (criterionCommentHash === undefined) {
    throw new Error(`The criterionCommentHash input is required for the CriterionCommentDetail element`);
  }
  await fetchCriterionComment();
  loading = false;

  await profilesStore.profiles.get(criterionComment.author).subscribe((profile) => {
    if (profile.status == "complete") {
      nickName = profile.value.nickname
    }
  })
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

async function fetchAlternative(alternative_hash) {
  try {
    let record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'get_criterion',
      payload: alternative_hash,
    });
    let c = decode((record.entry as any).Present.entry) as Criterion;
    let output:any = {...c};
    output.hash = alternative_hash;
    return output
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
        console.log('fetching objection')
        objection = await fetchObjection(criterionComment.objection_reference)
      }
      else if (criterionComment.alternative_reference) {
        console.log('fetching alternative')
        alternative = await fetchAlternative(criterionComment.alternative_reference)
      }
      else if (criterionComment.comment_reference) {
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

{#if filter == "all" || filter == "objections" && objection || filter == "alternatives" && alternative}

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
{:else if false}

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
====================<br>
  {#if objection}
    {objection.comment}
  {/if}

  {#if alternative}
    {JSON.stringify(alternative.title)}
    hi
  {/if}

  {#if respondingTo}
    {JSON.stringify(respondingTo.comment)}
  {/if}

  <!-- {JSON.stringify(criterionComment.alternative_reference)} -->
  <!-- <button on:click={()=>{fetchAlternative(criterionComment.alternative_reference)}}>fetch alternative</button> -->

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
{:else}
<!-- Assuming you have necessary MWC components and styles imported -->

<div class="chat-container">
  
  <!-- Example of a Comment Card -->
  <agent-avatar disable-tooltip={true} disable-copy={true} size={40} agent-pub-key="{encodeHashToBase64(criterionComment.author)}"></agent-avatar>

  <div style="width: 100%; margin-right: 20px;">
    <div>{nickName}</div>
    <div class="comment-card" style={criterionComment.objection_reference ? "border: 1px solid red;" : ""}>
      <!-- {JSON.stringify()} -->
      <!-- import profile-user.png -->
      <!-- <img src="profile-user.png" alt="Profile Picture" width="50" height="50"> -->
      <!-- {JSON.stringify(criterionComment.alternative_reference)} -->
      <!-- Comment content -->
      {#if objection}
        <div>
          <ObjectionMini objectionHash={criterionComment.objection_reference}></ObjectionMini>
        </div>
      {:else if alternative}
        <AlternativeMini {alternative} {mySupport} {criterionHash}></AlternativeMini>
        <!-- <div>{alternative.title}</div> -->
      {:else if respondingTo}
        <div class="comment-bubble">
          {#if respondingTo.objection_reference}
            <ObjectionMini objectionHash={respondingTo.objection_reference}></ObjectionMini>
          {:else}
            {respondingTo.comment}
          <!-- {:else if respondingTo.alternative_reference} -->
            <!-- <AlternativeMini alternative={} {mySupport} {criterionHash}></AlternativeMini> -->
          {/if}
        </div>
      {/if}

      <!-- Comment details -->
      <div class="comment-details">
        <span class="comment-text">{criterionComment.comment}</span>
        <span class="timestamp">{new Date(criterionComment.created / 1000).toLocaleString()}</span>
        <span class="timestamp">
        <button class="reply" on:click={() => {commentReference = {hash: criterionCommentHash, comment: criterionComment.comment};}}>Reply</button>
        </span>
      </div>

      <!-- Action Buttons -->
      <!-- <div class="action-buttons"> -->
        <!-- <mwc-icon-button icon="edit" on:click={() => { editing = true; }}></mwc-icon-button> -->
        <!-- <mwc-icon-button icon="delete" on:click={() => deleteCriterionComment()}></mwc-icon-button> -->
        <!-- <button class="reply" on:click={() => {commentReference = {hash: criterionCommentHash, comment: criterionComment.comment}}}>Reply</button> -->
      <!-- </div> -->
    </div>
  </div>

</div>

{/if}

<style>
  .reply {
    width: fit-content;
    border: none;
    background: lightgray;
    border-radius: 4px;
    margin-left: 10px;
    color: gray;
    cursor: pointer;
  }

  .reply:hover {
    background: gray;
    color: white;
  }

  .chat-container {
    display: flex;
    flex-direction: row;
    gap: 12px;
    padding: 10px;
  }

  .comment-card {
    background-color: #f3f3f3;
    border-radius: 10px;
    padding: 10px;
    position: relative;
    margin: 6px;
    width: 100%;
    margin-left: 0;
  }

  .comment-bubble {
    background-color: #e9e9e9;
    border-radius: 10px;
    padding: 10px;
    margin-bottom: 8px;
  }

  .comment-details {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: end;
  }

  .comment-text {
    white-space: pre-line;
    flex: 1;
  }

  .timestamp {
    font-size: 0.8em;
    color: #7f7f7f;
  }

  .action-buttons {
    position: absolute;
    right: 5px;
    top: 5px;
    display: flex;
    gap: 5px;
  }
</style>

{/if}

