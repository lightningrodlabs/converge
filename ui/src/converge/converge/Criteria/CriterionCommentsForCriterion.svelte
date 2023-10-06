<script lang="ts">
import { onMount, afterUpdate, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import '@material/mwc-checkbox';
import type { Record, EntryHash, ActionHash, AgentPubKey, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../../contexts';
import CriterionCommentDetail from './CriterionCommentDetail.svelte';
import CreateCriterionComment from './CreateCriterionComment.svelte';
import CreateAlternative from './CreateAlternative.svelte';
import type { ConvergeSignal } from '../types';

export let criterionHash: ActionHash;
export let objections;
export let alternatives;
export let deliberationHash;
export let mySupport;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;

let loading = true;
let error: any = undefined;
let commentReference;
let chatWindow;
let filter = "all";
let commentIsAnObjection: boolean = false;

$: hashes, loading, error, chatWindow;

async function scrollToBottom() {
  // if (chatWindow) {
    await new Promise(res => setTimeout(res, 100));
    chatWindow.scrollTop = chatWindow.scrollHeight;
    await new Promise(res => setTimeout(res, 500));
    chatWindow.scrollTop = chatWindow.scrollHeight;
    console.log(chatWindow.scrollTop, chatWindow.scrollHeight, chatWindow)
  // }
}

onMount(async () => {
  if (criterionHash === undefined) {
    throw new Error(`The criterionHash input is required for the CriterionCommentsForCriterion element`);
  }

  try {
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'get_criterion_comments_for_criterion',
      payload: criterionHash,
    });
    hashes = records.map(r => r.signed_action.hashed.hash);
    scrollToBottom();
  } catch (e) {
    error = e;
  }
  loading = false;

  client.on('signal', signal => {
    if (signal.zome_name !== 'converge') return;
    const payload = signal.payload as ConvergeSignal;
    if (payload.type !== 'LinkCreated') return;
    let linkType = Object.keys(payload.link_type)[0]
    console.log(linkType)
    if (linkType !== 'CriterionToCriterionComments') return;
    hashes = [...hashes, payload.action.hashed.content.target_address];
    scrollToBottom();
  });
});

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
// afterUpdate(() => {
//   scrollToBottom()
// });

</script>

<style>
  .criterion-popup-header {
    display: flex;
    flex-direction: row;
    /* justify-content: space-between; */
    align-items: center;
    padding: 8px;
    /* background-color: black; */
    border-top: 1px solid gray;
    border-bottom: 1px solid rgb(213, 213, 213);
    background-color: rgb(247, 247, 247);
  }

  .underline {
    text-decoration: underline;
  }
</style>

{#if loading }
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching criterion comments: {error.data.data}.</span>
<!-- {:else if hashes && hashes.length === 0} -->
<!-- <span>No criterion comments found for this criterion.</span> -->
{:else}
<div class="criterion-popup-header">
  <!-- <select bind:value={filter}>
    <option value='all'>Filter: none</option>
    <option value='objections'>Filter: objections</option>
    <option value='alternatives'>Filter: alternatives</option>
  </select> -->

  <!-- <ul>
    <li value='all'>Filter: none</li>
    <li value='objections'>Filter: objections</li>
    <li value='alternatives'>Filter: alternatives</li>
  </ul> -->

  <!-- filters as tabs -->
  Filter:
  <div style="display: flex; flex-direction: row; margin-left: 8px;">
    <div style="margin-right: 8px; cursor: pointer;" class:underline={filter=='all'} on:click={() => {filter='all'}}>all</div>
    <div style="margin-right: 8px; cursor: pointer;" class:underline={filter=='objections'} on:click={() => {filter='objections'}}>objections</div>
    <div style="margin-right: 8px; cursor: pointer;" class:underline={filter=='alternatives'} on:click={() => {filter='alternatives'}}>alternatives</div>
  </div>

  {#if objections}
    {@const agentStringToCheck = Object.values(client.myPubKey).join(',')}
    {@const agentIsPresent = objections.some(agentObj => Object.values(agentObj.agent).join(',') === agentStringToCheck)}
    {#if agentIsPresent}
      {@const agent = objections.find(agentObj => Object.values(agentObj.agent).join(',') === agentStringToCheck)}
      <div style="margin: 8px; color: red;">You objected to this criterion</div>
      <div style="margin-right: 8px; cursor: pointer; color: gray; text-decoration: underline;" on:click={() => {removeObjection()}}>Remove objection</div>
  
    {/if}
  {/if}
  <!-- <label>Filter: </label> -->

  <!-- <div on:click={()=>{filter='all'}} style="margin-right: 8px; cursor: pointer; color: gray; font-weight: bold; text-decoration: underline;">All</div> -->

</div>
<div bind:this={chatWindow} style="display: flex; flex-direction: column; max-height: 60vh; min-height: 10px; overflow-y: scroll; overflow-x: hidden;">
  {#each hashes as hash}
  <!-- <div style="margin-bottom: 8px;"> -->
    <CriterionCommentDetail {objections} {filter} criterionCommentHash={hash} {mySupport} {criterionHash} bind:commentReference></CriterionCommentDetail>
    <!-- </div> -->
    {/each}
  </div>
  {/if}

  <div style="display: flex; flex-direction: row">

<CreateAlternative {criterionHash} {deliberationHash} {mySupport} {alternatives}></CreateAlternative>

<!-- <div style="flex-align: center; position: relative;
bottom: -7px;
left: 6px;"> -->
<div>
  <!-- <mwc-switch on:click={(e) => {console.log(e.target)}} name="choice"></mwc-switch>  -->
  <mwc-formfield style="padding: 10px;" label="Submit comment as objection to the criterion">
    <input type="checkbox" bind:checked={commentIsAnObjection} />
    </mwc-formfield>
</div>
</div>

<CreateCriterionComment on:criterion-comment-created={commentReference=undefined} {criterionHash} {commentReference} bind:commentIsAnObjection></CreateCriterionComment>
<!-- <div style="margin-bottom: 16px">
  <mwc-textarea style="width: 35vw; height: 100px" outlined label="Comment" on:input={e => { objection = e.target.value; console.log(objection)}} required></mwc-textarea>          
</div> -->
<!-- <mwc-button on:click = {() => {addObjection()}}>Submit</mwc-button> -->