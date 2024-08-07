<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import { type Record, type ActionHash, type AppAgentClient, type EntryHash, type AgentPubKey, type DnaHash, encodeHashToBase64 } from '@holochain/client';
import { clientContext } from '../../../contexts';
import type { Deliberation, ConvergeSignal, Proposal } from '../types';
import FaSort from 'svelte-icons/fa/FaSort.svelte';
import FaSearch from 'svelte-icons/fa/FaSearch.svelte';
import '@material/mwc-circular-progress';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-snackbar';
import '@material/mwc-icon-button';
import '@material/mwc-tab';
import '@material/mwc-tab-bar';
import '@material/mwc-select';
import EditDeliberation from './EditDeliberation.svelte';
import { view, viewHash, navigate } from '../../../store.js';
import CreateCriterion from '../Criteria/CreateCriterion.svelte';
import AllCriteria from '../Criteria/AllCriteria.svelte';
import CreateProposal from '../Proposals/CreateProposal.svelte';
// import AllProposals from '../Proposals/AllProposals.svelte';
import AttachmentsList from '../../../AttachmentsList.svelte';
import { isWeContext, weaveUrlToWAL, type WAL } from '@lightningrodlabs/we-applet';
import Avatar from '../Avatar.svelte';
import SvgIcon from "../../../SvgIcon.svelte";
import { weClientStored } from '../../../store.js';
import { getMyDna } from '../../../util';
import type { WALUrl } from '../../../util';
import AllOutcomes from '../Outcomes/AllOutcomes.svelte';
import CreateOutcome from '../Outcomes/CreateOutcome.svelte';
import { allDeliberations, allProposals } from '../../../store.js';
import { refetchDeliberations, refetchProposalsForDeliberation, refetchEvaluationsForProposals } from '../../../refetch';
import ProposalListItem from '../Proposals/ProposalListItem.svelte';
import { decodeHashFromBase64 } from '@holochain/client';

const dispatch = createEventDispatcher();

let sortCriteria;

export let deliberationHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let outdated = false;
let loading = true;
let error: any = undefined;

let record: Record | undefined;
let deliberation: Deliberation | undefined;
let deliberators: String[] | undefined;
let completedDeliberators: String[] | undefined;
let deliberatorsRaw: AgentPubKey[] | undefined;

let criteria: any[] = [];
let proposalHashes: any[] = [];
let proposalsComplete: any = {};
let outcomes: any[] = [];

let editing = false;
let criterionFormPopup = false;
let proposalFormPopup = false;
let outcomeFormPopup = false;

let errorSnackbar: Snackbar;
let activeTab = "criteria";
let criteriaCount = 0;
let criteriaFilter;
let proposalFilter;
let proposalCount = 0;
let sortedCriteria;
let lastMessage;
let detectSort;
let outcomesTab;
let outcomeCount;
let reloadKey = 0;

allProposals.subscribe(value => {
  proposalsComplete = value;
  console.log("proposalsComplete", proposalsComplete)
  console.log("proposal hashes", proposalHashes)
});

let sortByOptions = [
  "support",
  "objections",
  "comments",
  "weight",
  "my support",
  "my objections"
];
let criteriaSort = "support";
let proposalSort;
let attachments: WALUrl[] = [];
let weClient;
let dnaHash;
weClientStored.subscribe(value => {
  weClient = value;
});

$: editing, error, loading, record, deliberation, activeTab, criterionFormPopup, proposalFormPopup, criteriaCount, proposalCount, criteriaFilter, proposalFilter, criteriaSort, proposalSort, sortedCriteria, outcomeCount, deliberators, completedDeliberators, deliberatorsRaw, attachments, weClient, dnaHash;

onMount(async () => {
  dnaHash = await getMyDna("converge", client)

  allDeliberations.subscribe(value => {
  if (value) {
    const deliberationComplete = value.find(d => encodeHashToBase64(d.action_hash) == encodeHashToBase64(deliberationHash));
    if (deliberationComplete) {
      deliberation = deliberationComplete.deliberation;
      criteria = deliberationComplete.criteria;
      criteriaCount = deliberationComplete.criteria.length;
       proposalHashes = deliberationComplete.proposals.map(p => encodeHashToBase64(p));
      proposalCount = deliberationComplete.proposals.length;
      outcomes = deliberationComplete.outcomes;
      outcomeCount = deliberationComplete.outcomes.length;
      completedDeliberators = deliberationComplete.deliberators.filter(d => d.completed).map(d => d.deliberator.join(','));
      deliberators = deliberationComplete.deliberators.filter(d => !d.completed).map(d => d.deliberator.join(','));
      deliberatorsRaw = deliberationComplete.deliberators.map(d => d.deliberator);
      attachments = deliberation.attachments;
      loading = false;
    }
  }
});

  // if (deliberationHash === undefined) {
  //   throw new Error(`The deliberationHash input is required for the DeliberationDetail element`);
  // }
  // await fetchDeliberation();
  await refetchDeliberations(client);
  await refetchProposalsForDeliberation(deliberationHash, client);
  await refetchEvaluationsForProposals(proposalHashes.map(p => p), client);

  client.on('signal', signal => {
    if (deliberation) {

      // console.log("signal", signal)
      if (signal.zome_name !== 'converge') return;
      const payload = signal.payload as ConvergeSignal;
      const updateMessages = ['new-join', 'criterion-created', 'proposal-rated', 'proposal-created', 'criterion-rated', 'outcome-created']
      const urgentMessages = ['criterion-created', 'proposal-created']
      const messagesFull = {
        'new-join': "A new participant has joined the deliberation",
        'criterion-created': "A new criterion has been added to the deliberation " + deliberation?.title,
        'proposal-rated': "A proposal has been rated",
        'proposal-created': "A new proposal has been added to the deliberation " + deliberation?.title,
        'criterion-rated': "A criterion has been rated",
        'outcome-created': "A new outcome has been added to the deliberation " + deliberation?.title,
      }
      if (updateMessages.includes(payload.message) && (payload.deliberation_hash.join(',') == deliberationHash.join(','))) {
        // console.log("activity received", payload)
        outdated = true;
        
        weClient.notifyFrame([{
          title: `New activity in ${deliberation.title}`,
          body: messagesFull[payload.message],
          notification_type: "change",
          icon_src: undefined,
          urgency: "low",
          timestamp: Date.now()
        }])
        
        lastMessage = messagesFull[payload.message];
      } else if (payload.message == "criterion-comment-created") {
        // console.log("this is a new message", payload)
      }
    }
    // console.log(payload)
  });
});

const copyWalToPocket = () => {
  const attachment: WAL = { hrl: [dnaHash, deliberationHash], context: "" }
  weClient?.walToPocket(attachment)
}

async function newActivity(event, context = "") {
  // console.log("new activity", event)
  joinDeliberation()
  sendActivityNotice(event, context)
}

async function sendActivityNotice(event, context = "") {
  // console.log("send activity notice", event)
  try {
    await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'new_activity_sender',
      payload: {
        deliberation_hash: deliberationHash,
        message: event,
        title: deliberation.title,
        context: context
      },
    });
  } catch (e: any) {
    console.log(e)
  }
}

async function joinDeliberation() {
  if (deliberators.includes(client.myPubKey.join(','))) return;
  try {
    await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'add_deliberation_for_deliberator',
      payload: {
        base_deliberator: client.myPubKey,
        target_deliberation_hash: deliberationHash
      },
    });
    deliberators = [...deliberators, client.myPubKey.join(',')]
    deliberatorsRaw = [...deliberatorsRaw, client.myPubKey]
    // dispatch('deliberation-joined', { deliberationHash: deliberationHash });
  } catch (e: any) {
    errorSnackbar.labelText = `Error joining the deliberation: ${e.data.data}`;
    errorSnackbar.show();
  }
}

async function leaveDeliberation() {
  try {
    await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'remove_deliberation_for_deliberator',
      payload: {
        base_deliberator: client.myPubKey,
        target_deliberation_hash: deliberationHash
      },
    });
    deliberators = deliberators.filter(item => item !== client.myPubKey.join(','));
    // remove me from deliberatorsRaw
    deliberatorsRaw = deliberatorsRaw.filter(item => item.join(',') !== client.myPubKey.join(','));
    // dispatch('deliberation-left', { deliberationHash: deliberationHash });
    // navigate('')
  } catch (e: any) {
    console.log("error", e)
    errorSnackbar.labelText = `Error leaving the deliberation: ${e.data.data}`;
    errorSnackbar.show();
  }
}

async function completeDeliberation() {
  await leaveDeliberation();
  try {
    await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'add_completed_tag',
      payload: deliberationHash,
    });
    deliberators = deliberators.filter(item => item !== client.myPubKey.join(','));
    completedDeliberators = [...completedDeliberators, client.myPubKey.join(',')]
    deliberatorsRaw = [...deliberatorsRaw, client.myPubKey]
  } catch (e: any) {
    errorSnackbar.labelText = `Error completing the deliberation: ${e.data.data}`;
    errorSnackbar.show();
  }
}

async function rejoinDeliberation() {
  await new Promise(r => setTimeout(r, 200));
  try {
    await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'remove_completed_tag',
      payload: deliberationHash,
    });
    completedDeliberators = completedDeliberators.filter(item => item !== client.myPubKey.join(','));
    deliberators = deliberators.filter(item => item !== client.myPubKey.join(','));
    deliberatorsRaw = deliberatorsRaw.filter(item => item.join(',') !== client.myPubKey.join(','));
    await new Promise(r => setTimeout(r, 200));
    await joinDeliberation();
  } catch (e: any) {
    console.log("error", e)
    errorSnackbar.labelText = `Error rejoining the deliberation: ${e}`;
    errorSnackbar.show();
  }
}

let isExpanded = false;
let isExpanded2 = false;
function expandSearch() {
  if (isExpanded) {
    criteriaFilter = "";
  }
  isExpanded = !isExpanded;
}
function expandSearch2() {
  if (isExpanded2) {
    proposalFilter = "";
  }
  isExpanded2 = !isExpanded2;
}

</script>

<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>

{#if loading || !deliberationHash}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the deliberation: {JSON.stringify(error)}</span>
{:else}

{#if outdated && lastMessage}
<div on:click={async () => {refetchDeliberations(client);
  await refetchProposalsForDeliberation(deliberationHash, client);
  await refetchEvaluationsForProposals(proposalHashes.map(p => p), client);
  reloadKey += 1;
   outdated = false}} class="reload-page">
  <span style="margin-right: 8px">This page is outdated because {lastMessage.toLowerCase()}. Click to refresh. ↺</span>
</div>
{:else}
<div style="cursor: pointer; display: flex; flex-direction: row; align-items: center; justify-content: center; background: transparent; padding: 4px;">
  <span style="margin-right: 8px">&nbsp;</span>
</div>
{/if}

<div style="display: flex; flex-direction: column;">
  <!-- <div style="display: flex; flex-direction: row">
    <span style="flex: 1"></span>
    <mwc-icon-button style="margin-left: 8px" icon="edit" on:click={() => { editing = true; } }></mwc-icon-button>
    <mwc-icon-button style="margin-left: 8px" icon="delete" on:click={() => deleteDeliberation()}></mwc-icon-button>
  </div> -->
  <div style="display: flex; flex-direction: row; justify-content: space-between;">
    <div style="display: flex; flex-direction: column">
      <div style="display: flex; flex-direction: row; margin-bottom: 0px">
        <h1 style="margin-top: 4px; margin-bottom:4px">{ deliberation.title }</h1>
      </div>
      
      <div style="display: flex; flex-direction: row; margin-bottom: 6px">
        <span style="white-space: pre-line">{ deliberation.description }</span>
      </div>

      {#if isWeContext()}
        <div style="display: flex; flex-direction: row; margin-bottom: 5px">
          <AttachmentsList {attachments} allowDelete={false}/>
        </div>
      {/if}
      
      <div style="display: flex; flex-direction: row; width: fit-content; margin-bottom: 6px;">
        
        {#each deliberatorsRaw.filter(d => !completedDeliberators.includes(d.join(','))) as deliberator}
          <div class="avatar-overlap">
            <Avatar showNickname={false} agentPubKey={deliberator}  size={24} namePosition="row"></Avatar>
          </div>
        {/each}

        {#if !completedDeliberators.includes(client.myPubKey.join(','))}
          {#if deliberators.includes(client.myPubKey.join(','))}
            <mwc-button class="join-leave" on:click={leaveDeliberation}>Leave</mwc-button>
          {:else}
            <mwc-button class="join-leave" on:click={() => {newActivity("new-join")}}>Join</mwc-button>
          {/if}
        {/if}
        <!-- &nbsp;|&nbsp;&nbsp; -->
        
        <!-- {deliberators.length} 
        {#if deliberators.length == 1} participant{:else} participants{/if} -->

      </div>
    </div>
    
    <div style="display:flex; flex-direction:column; justify-content:space-between;">
      <div style="display: flex; flex-direction: row; float: right; width: min-content; flex-shrink:0; align-self:end">
        {#if deliberators.length == 1 && deliberators[0] == client.myPubKey}
          <button style="border: 0; border-radius: 10px; padding: 4px 6px; color: #313131; cursor: pointer;" title="Since no one else has interacted with this deliberation, you can still hide it from the main page." on:click={leaveDeliberation}> 
            <!-- <SvgIcon icon="faEye" size="20px"/> -->
            Hide</button>
        {/if}
        {#if isWeContext()}
          <button title="Add Board to Pocket" class="attachment-button" style="margin: 0 10px; cursor: pointer; border: 0; border-radius: 10px; padding: 4px;" on:click={()=>copyWalToPocket()} >          
            <SvgIcon icon="addToPocket" size="20px"/>
          </button>
        {/if}
        <!-- {JSON.stringify(deliberation.discussion)} -->
        {#if isWeContext() && deliberation.discussion}
          {@const conversation = weaveUrlToWAL(deliberation.discussion)}
          {#await weClient.assetInfo(conversation)}
            <sl-button size="small" loading></sl-button>
          {:then { attachableInfo }}
          <button class="discussion-button"
            on:click={(e)=>{
              e.stopPropagation()
              // activeTab = "discussion"
              weClient.openWal(conversation)
              // weClient.openAppletBlock(hrlWithContext.hrl[0], "active_boards", hrlWithContext.context)
            }} >
              <SvgIcon icon="faComments" size="22px"/>
              <!-- Discussion -->
            </button>
          {:catch error}
            Oops. something's wrong.
          {/await}
        {/if}
      </div>
      <!-- {JSON.stringify(completedDeliberators)}
      {deliberatorsRaw.filter(d => completedDeliberators.includes(d.join(','))).length > 0}
      {!completedDeliberators.includes(client.myPubKey.join(',')) && deliberators.includes(client.myPubKey.join(','))}
      {completedDeliberators.includes(client.myPubKey.join(','))} -->

      <div style="display: flex; flex-direction: row; width: fit-content; margin-bottom: 6px; height: 35px;">
        {#if !completedDeliberators.includes(client.myPubKey.join(',')) && deliberators.includes(client.myPubKey.join(','))}
          <mwc-button class="join-leave" on:click={completeDeliberation}>complete</mwc-button>
        {/if}
        {#if completedDeliberators.length > 0}
          <!-- {#if !completedDeliberators.includes(client.myPubKey.join(',')) && deliberators.includes(client.myPubKey.join(','))} -->
          {#if completedDeliberators.includes(client.myPubKey.join(','))}
            <!-- <div class="leave" style="padding: 14px;
              font-family: monospace;
              font-size: 12px;">
              COMPLETED
            </div> -->
            <div class="leave" on:click={leaveDeliberation}>
              <mwc-button class="join-leave" on:click={rejoinDeliberation}>Rejoin</mwc-button>
            </div>
          {/if}
          {#each deliberatorsRaw.filter(d => completedDeliberators.includes(d.join(','))) as deliberator}
            <div class="avatar-overlap">
              <Avatar showNickname={false} agentPubKey={deliberator}  size={24} namePosition="row"></Avatar>
            </div>
          {/each}

        {/if}
      </div>
    </div>
  </div>

  <div class="details-mini">
    Criteria: {criteriaCount}<br>
    Proposals: {proposalCount}<br>
    Outcomes: {outcomeCount}<br>
  </div>

  <div class="deliberation-section">
    <mwc-tab-bar>
      <mwc-tab on:click={() => {activeTab = "criteria"}} label="Criteria ({criteriaCount})"></mwc-tab>
      <mwc-tab on:click={() => {activeTab = "proposals"}}  label="Proposals ({proposalCount})"></mwc-tab>
      {#if isWeContext()}
        <mwc-tab bind:this={outcomesTab} on:click={() => {activeTab = "outcomes"}}  label="Outcomes ({outcomeCount})"></mwc-tab>
      {/if}
      <!-- <mwc-tab on:click={() => {activeTab = "activity"}}  label="Activity"></mwc-tab> -->
    </mwc-tab-bar>
    
  </div>

</div>

{#if activeTab == "criteria"}
<br>
  <!--<FaSort/> -->
    {#if criteriaCount > 1}
      <!-- <select class="sort-dropdown" bind:value={criteriaSort}
        on:change={() => {
          sortCriteria()
        }}
      >
        {#each sortByOptions as option}
        <option value={option}>  Sort by: {option}</option>
        {/each}
      </select> -->
      
      <div class="search-container">
        <div class="search-button" on:click={expandSearch}><FaSearch/></div>
        <input bind:value={criteriaFilter} type="text" class="search-input {isExpanded ? 'expanded' : ''}" placeholder="Search criteria...">
      </div>
    {/if}

  <!-- <div class="search-container"><FaSearch/></div> -->


  <!-- <div class="search-button"><FaSearch/></div> -->
  <div on:click={() => {criterionFormPopup = true;}} title="What would be necessary for you to accept a proposal?" class="add-button">{window.innerWidth < 768 ? "+" : "Add a criterion"}</div>
  <!-- What should to be true about any solution addressing this issue? -->
  
  <!-- <mwc-button dense outlined>Add criterion</mwc-button> -->
  <!-- {#if criterionForm} -->
  <CreateCriterion on:criterion-created={() => {newActivity("criterion-created")}} deliberationHash={deliberationHash} alternativeTo={null} bind:criterionFormPopup />
  <!-- {/if} -->
  <br><br>
  <!-- {#if criteriaSort == "support"} -->
  
  {#key reloadKey}
  <AllCriteria on:criterion-rated={() => {newActivity("criterion-rated")}} deliberationHash={deliberationHash} 
    on:criterion-comment-created={(e) => {newActivity("criterion-comment-created", e.detail.context)}}
    filter={criteriaFilter} sort={criteriaSort} bind:sortedCriteria bind:criteriaCount bind:sortCriteria  />
  {/key}
  <!-- {:else if criteriaSort == "objections"}
  <AllCriteria deliberationHash={deliberationHash} filter={criteriaFilter} sort="objections" bind:criteriaCount />
  {/if} -->
{:else if activeTab == "proposals"}
  <br>
  <!-- <select bind:value={proposalSort}>
    {#each ["score", "respondants"] as option}
    <option value={option}>  Sort by: {option}</option>
    {/each}
  </select> -->

  {#if proposalCount > 1}
    <div class="search-container">
      <div class="search-button" on:click={expandSearch2}><FaSearch/></div>
      <input bind:value={proposalFilter} type="text" class="search-input {isExpanded2 ? 'expanded' : ''}" placeholder="Search proposals...">
    </div>
  {/if}
  <!-- <div class="search-button"><FaSearch/></div> -->
  <div title="How could we meet our criteria?" on:click={() => {proposalFormPopup = true; console.log(proposalFormPopup)}} class="add-button">{window.innerWidth < 768 ? "+" : "Add a proposal"}</div>

  <CreateProposal on:proposal-created={() => {newActivity("proposal-created")}} {deliberationHash} {sortedCriteria} bind:proposalFormPopup/>
  <br><br>
  
  {#if proposalHashes.length === 0}
    <span style="font-style: italic;">Try adding a proposal. How could we meet our criteria?</span>
  {:else if proposalCount > 0}
    {#each proposalHashes as proposalHash}
      {@const fullProposal = proposalsComplete[proposalHash]}
      <ProposalListItem on:proposal-rated={() => {newActivity("proposal-rated")}} on:outcome-created={(v) => {dispatch('outcome-created', v)}} proposal={fullProposal} proposalHash={decodeHashFromBase64(proposalHash)} {deliberationHash} hashes={proposalHashes.map(h => decodeHashFromBase64(h))}  />
      <!-- {JSON.stringify(fullProposal)} -->
    {/each}
  {/if}
  <!-- <AllProposals hashes={proposals} on:outcome-created={(v)=>{
    dispatch('outcome-created', v);
    // activeTab = "outcomes";
    outcomesTab.click();
  }} on:proposal-rated={() => {newActivity("proposal-rated")}} sort={proposalSort} deliberationHash={deliberationHash} filter={proposalFilter} bind:proposalCount/> -->
{:else if activeTab == "outcomes"}
  <!-- <p class="instructions"></p> -->
  <div title="Link to an asset from another tool such as a {"Who's In?"} coordination" on:click={() => {outcomeFormPopup = true; console.log(proposalFormPopup)}} class="add-button">{window.innerWidth < 768 ? "+" : "Add an outcome"}</div>

  <CreateOutcome on:outcome-created={() => {newActivity("outcome-created"); dispatch('outcome-created'); reloadKey+=1}} {deliberationHash} bind:outcomeFormPopup/>
  <br><br>
  {#key reloadKey}
    <AllOutcomes deliberationHash={deliberationHash} bind:outcomeCount/>
  {/key}
{/if}

{/if}

<style lang="css">
  /* if width is less than 100px, display mini view */
  @media (max-width: 300px) {
    .add-button {
      display: none;
    }
    .deliberation-section {
      display: none;
    }
    .instructions {
      display: none;
    }
    .join-leave {
      display: none;
    }
    .details-mini {
      display: inline-block !important;
    }
    :global(footer) {
      display: none;
    }
    /* hide add to pocket */
    .attachment-button {
      display: none;
    }
    
  }

  .details-mini {
    display: none;
  }

  .discussion-button {
    display: flex;
    flex-direction: row;
    float: right;
    width: min-content;
    flex-shrink: 0;
    align-self: start;
    padding: 14px 18px;
    cursor: pointer;
    border: 0px;
    border-radius: 10px;
    /* background-color: #925ace; */
  }

  .leave {
    cursor: pointer; width: fit-content; display:flex; flex-direction: column;
  }
</style>