<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { EntryHash, Record, AgentPubKey, ActionHash, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../../contexts';
import DeliberationDetail from './DeliberationDetail.svelte';
import type { ConvergeSignal } from '../types';
import { view, viewHash, navigate } from '../../../store.js';
import DeliberationListItem from './DeliberationListItem.svelte';


let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;
let loading = true;
let error: any = undefined;

$: hashes, loading, error;

onMount(async () => {

  await fetchDeliberations();
  client.on('signal', signal => {
    if (signal.zome_name !== 'converge') return;
    const payload = signal.payload as ConvergeSignal;
    if (payload.type !== 'EntryCreated') return;
    if (payload.app_entry.type !== 'Deliberation') return;
    hashes = [...hashes, payload.action.hashed.hash];
  });
});

async function fetchDeliberations() {
  try {
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'get_all_deliberations',
      payload: null,
    });
    hashes = records.map(r => r.signed_action.hashed.hash);
  } catch (e) {
    error = e;
  }
  loading = false;
}

</script>

<h2>All Deliberations</h2>

{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the deliberations: {error.data.data}.</span>
{:else if hashes.length === 0}
<span>No deliberations found.</span>
{:else}
<!-- <div style="display: flex; flex-direction: column"> -->
  {#each hashes.reverse() as hash}
    <span on:click={() => navigate('deliberation', hash)}>
      <DeliberationListItem deliberationHash={hash}  on:deliberation-deleted={() => fetchDeliberations()}></DeliberationListItem>
    </span>
  {/each}
<!-- </div> -->
{/if}