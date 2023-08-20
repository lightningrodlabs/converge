<script lang="ts">
import { onMount, afterUpdate, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { Record, EntryHash, ActionHash, AgentPubKey, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../contexts';
import CriterionCommentDetail from './CriterionCommentDetail.svelte';
import CreateCriterionComment from './CreateCriterionComment.svelte';
import type { ConvergeSignal } from './types';

export let criterionHash: ActionHash;
export let objections;
export let alternatives;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;

let loading = true;
let error: any = undefined;
let commentReference;
let chatWindow;

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

// afterUpdate(() => {
//   scrollToBottom()
// });

</script>

{#if loading }
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching criterion comments: {error.data.data}.</span>
{:else if hashes.length === 0}
<span>No criterion comments found for this criterion.</span>
{:else}
<div bind:this={chatWindow} style="display: flex; flex-direction: column; max-height: 50vh; overflow-y: scroll; overflow-x: hidden;">
  {#each hashes as hash}
    <!-- <div style="margin-bottom: 8px;"> -->
      <CriterionCommentDetail criterionCommentHash={hash} {objections} {alternatives} bind:commentReference></CriterionCommentDetail>
    <!-- </div> -->
  {/each}
</div>
{/if}
<CreateCriterionComment {criterionHash} {commentReference}></CreateCriterionComment>

