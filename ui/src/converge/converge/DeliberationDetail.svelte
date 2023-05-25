<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { decode } from '@msgpack/msgpack';
import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Deliberation } from './types';
import '@material/mwc-circular-progress';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-snackbar';
import '@material/mwc-icon-button';
import EditDeliberation from './EditDeliberation.svelte'; 
import { view, viewHash, navigate } from '../../store.js';
import CreateCriterion from './CreateCriterion.svelte';
import AllCriteria from './AllCriteria.svelte';
import CreateProposal from './CreateProposal.svelte';
import AllProposals from './AllProposals.svelte';

const dispatch = createEventDispatcher();

export let deliberationHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let loading = true;
let error: any = undefined;

let record: Record | undefined;
let deliberation: Deliberation | undefined;

let editing = false;

let errorSnackbar: Snackbar;

$: editing,  error, loading, record, deliberation;

onMount(async () => {
  if (deliberationHash === undefined) {
    throw new Error(`The deliberationHash input is required for the DeliberationDetail element`);
  }
  await fetchDeliberation();
});

async function fetchDeliberation() {
  loading = true;
  error = undefined;
  record = undefined;
  deliberation = undefined;
  
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'get_deliberation',
      payload: deliberationHash,
    });
    if (record) {
      deliberation = decode((record.entry as any).Present.entry) as Deliberation;
    }
  } catch (e) {
    error = e;
  }

  loading = false;
}

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

<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>

{#if loading || !deliberationHash}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the deliberation: {error.data.data}</span>
{:else if editing}
<EditDeliberation
  originalDeliberationHash={ deliberationHash}
  currentRecord={record}
  on:deliberation-updated={async () => {
    editing = false;
    await fetchDeliberation()
  } }
  on:edit-canceled={() => { editing = false; } }
></EditDeliberation>
{:else}

<div style="display: flex; flex-direction: column">
  <div style="display: flex; flex-direction: row">
    <span style="flex: 1"></span>
    <mwc-icon-button style="margin-left: 8px" icon="edit" on:click={() => { editing = true; } }></mwc-icon-button>
    <mwc-icon-button style="margin-left: 8px" icon="delete" on:click={() => deleteDeliberation()}></mwc-icon-button>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Title:</strong></span>
    <span style="white-space: pre-line">{ deliberation.title }</span>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Description:</strong></span>
    <span style="white-space: pre-line">{ deliberation.description }</span>
  </div>

  <div style="display: flex; flex-direction: row; margin-bottom: 16px">
    <span style="margin-right: 4px"><strong>Settings:</strong></span>
    <span style="white-space: pre-line">{ deliberation.settings }</span>
  </div>

</div>

<AllCriteria deliberationHash={deliberationHash} />
<CreateCriterion deliberationHash={deliberationHash} />

<CreateProposal deliberationHash={deliberationHash} />
<AllProposals deliberationHash={deliberationHash} />
{/if}

