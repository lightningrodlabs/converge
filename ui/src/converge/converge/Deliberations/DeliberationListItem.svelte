<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import type { Record, ActionHash, AppClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
import { clientContext } from '../../../contexts';
import type { Deliberation } from '../types';
import '@material/mwc-circular-progress';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-snackbar';
import '@material/mwc-icon-button';
import EditDeliberation from './EditDeliberation.svelte'; 
import { countViewed, addToViewed } from '../../../viewed.js';
import CreateCriterion from '../Criteria/CreateCriterion.svelte';
import AllCriteria from '../Criteria/AllCriteria.svelte';
import CreateProposal from '../Proposals/CreateProposal.svelte';
import AllProposals from '../Proposals/AllProposals.svelte';
import type { ConvergeSignal } from '../types';
import SvgIcon from '../SvgIcon.svelte';

const dispatch = createEventDispatcher();

export let deliberation: Deliberation | undefined;

export let deliberationHash: ActionHash;

let client: AppClient = (getContext(clientContext) as any).getClient();

let loading = true;
let error: any = undefined;

let record: Record | undefined;

let editing = false;

let errorSnackbar: Snackbar;

let proposals;
let criteria;
let outcomes;
let newCriteriaCount;
let newProposalCount;
let newOutcomeCount;
let deliberators: String[] | undefined;
let completedDeliberators: String[] | undefined;

$: editing,  error, loading, record, deliberation, proposals, criteria, outcomes;

onMount(async () => {
  // if (deliberationHash === undefined) {
  //   throw new Error(`The deliberationHash input is required for the DeliberationDetail element`);
  // }
  // await fetchDeliberation();

  console.log(deliberation)

  newCriteriaCount = Math.max(0, deliberation.criteria?.length - countViewed(deliberation.criteria))
  newProposalCount = Math.max(0, deliberation.proposals?.length - countViewed(deliberation.proposals))
  newOutcomeCount = Math.max(0, deliberation.outcomes?.length - countViewed(deliberation.outcomes))

  loading = false;

  // client.on('signal', signal => {
  //   if (signal.App.zome_name !== 'converge') return;
  //   const payload = signal.App.payload as ConvergeSignal;
  //   if (payload.type !== 'LinkCreated') return;
  //   if (payload.link_type !== 'DeliberationToProposals') return;

  //   proposals = [...proposals, payload.action.hashed.content.target_address];
  // });
});

// async function fetchDeliberation() {
//   loading = true;
//   error = undefined;
//   record = undefined;
//   deliberation = undefined;
  
//   try {
//     record = await client.callZome({
//       cap_secret: null,
//       role_name: 'converge',
//       zome_name: 'converge',
//       fn_name: 'get_deliberation',
//       payload: deliberationHash,
//     });
//     if (record) {
//       deliberation = decode((record.record.entry as any).Present.entry) as Deliberation;
//       proposals = record.proposals.map(l => l.target);
//       criteria = record.criteria.map(l => l.target);
//       outcomes = record.outcomes.map(l => l.target);
//     }
//   } catch (e) {
//     error = e;
//   }

//   try {
//     // console.log("trying to get deliberators")
//     const records = await client.callZome({
//       cap_secret: null,
//       role_name: 'converge',
//       zome_name: 'converge',
//       fn_name: 'get_deliberators_for_deliberation',
//       payload: deliberationHash,
//     });
//     deliberators = records.map((record) => record.deliberator.join(','));
//     completedDeliberators = records
//       .filter((record) => record.completed)
//       .map((record) => record.deliberator.join(','));
//   } catch (e) {
//     error = e;
//   }

//   loading = false;
// }

async function deleteDeliberation() {
  try {
    await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'delete_deliberation',
      payload: deliberationHash,
    });
    dispatch('deliberation-deleted', { deliberationHash: deliberationHash });
  } catch (e: any) {
    errorSnackbar.labelText = `Error deleting the deliberation: ${e.data.data}`;
    errorSnackbar.show();
  }
}
</script>

<!-- <mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar> -->

{#if loading || !deliberation}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the deliberation: {error.data.data}</span>
<!-- {:else if editing}
<EditDeliberation
  originalDeliberationHash={ deliberationHash}
  currentRecord={record}
  on:deliberation-updated={async () => {
    editing = false;
    await fetchDeliberation()
  } }
  on:edit-canceled={() => { editing = false; } }
></EditDeliberation> -->
{:else if deliberation.deliberators.length > 0}

<div class="dashboard-section">

  <div class="dashboard-item">
    <div>{ deliberation.deliberation.title }</div>
    <!-- <div class="dashboard-item-details">  
      Created | Last updated | Closes
    </div> -->

    <div class="issue-info">
      <!-- <div id="open-collaborators">
        Collaborators<br>
        <div><i class="fas fa-users"></i>
        <strong>1</strong></div>
      </div> -->
      <div>
        Outcomes
        <div>
          <SvgIcon color="#757575" icon="faArrow"></SvgIcon>
          <strong>{deliberation.outcomes.length}</strong>
          {#if newOutcomeCount > 0}
            <span style="
            color: #be59e9;
            font-size: 0.8em;
            font-weight: 400;
            ">
              ({newOutcomeCount} new)
            </span>
          {/if}
        </div>
        <!-- <SvgIcon color="#757575" icon="faArrow"></SvgIcon>
        <strong>{outcomes.length}</strong> -->
      </div>
      <div>
        Proposals
        <div>
          <SvgIcon color="#757575" icon="faBars"></SvgIcon>
          <strong>{deliberation.proposals.length}</strong>
          {#if newProposalCount > 0}
            <span style="
            color: #be59e9;
            font-size: 0.8em;
            font-weight: 400;
            ">
              ({newProposalCount} new)
            </span>
          {/if}
        </div>
        <!-- <div><i class="fas fa-file"></i> -->
        <!-- <SvgIcon color="#757575" icon="faBars"></SvgIcon>
        <strong>{proposals.length}</strong> -->
      </div>
      <div style="
        width: fit-content;
      ">
        Criteria
        <div>
          <SvgIcon color="#757575" icon="faCheck"></SvgIcon>
          <strong>{deliberation.criteria.length}</strong> 
          {#if newCriteriaCount > 0}
            <span style="
            color: #be59e9;
            font-size: 0.8em;
            font-weight: 400;
            ">
              ({newCriteriaCount} new)
            </span>
          {/if}
      </div>
      <!-- joined -->
    </div>
    <div>
      Joined
      <div>
        <SvgIcon color="#757575" icon="faUserGroup"></SvgIcon>
        <strong>{deliberation.deliberators.length}</strong>
      </div>
    </div>

      <!-- <div class="post-signature">
        <i class="fas fa-clock"></i>
        3 months ago
        by
        <em>admin</em>
      </div> -->

    </div>

    <!-- <div class="dashboard-item-details">   -->
    <!-- {#if participants}
      {#if participants.length == 1}
        <span>{participants.length} criterion</span>
      {:else}
        <span>{participants.length} criteria</span>
      {/if}
    {/if}
    {#if proposals}
      {#if proposals.length == 1}
        <span>| {proposals.length} proposal</span>
      {:else}
        <span>| {proposals.length} proposals</span>
      {/if}
    {/if} -->
    <!-- </div> -->
  </div>

</div>
{/if}

<style>
  strong {
    font-weight: 600;
  }

  @media (max-width: 300px) {
    .dashboard-item {
      box-shadow: none;
      border: none;
      flex-direction: column;
    }

    .dashboard-item > div {
      flex-direction: row;
      margin: 0;
      margin-bottom: 1em;
    }
  }
</style>