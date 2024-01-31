<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { Record, EntryHash, ActionHash, AgentPubKey, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../../contexts';
import DeliberationDetail from './DeliberationDetail.svelte';
import type { ConvergeSignal } from '../types';
    import DeliberationListItem from './DeliberationListItem.svelte';
    import { view, viewHash, navigate } from '../../../store.js';

export let deliberator: AgentPubKey;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;

let loading = true;
let error: any = undefined;

$: hashes, loading, error;

onMount(async () => {
  if (deliberator === undefined) {
    throw new Error(`The deliberator input is required for the DeliberationsForDeliberator element`);
  }

  try {
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'get_deliberations_for_deliberator',
      payload: deliberator,
    });
    hashes = records.map(r => r.signed_action.hashed.hash);
  } catch (e) {
    error = e;
  }
  loading = false;

  client.on('signal', signal => {
    if (signal.zome_name !== 'converge') return;
    const payload = signal.payload as ConvergeSignal;
    if (payload.type !== 'LinkCreated') return;
    if (payload.link_type !== 'DeliberatorToDeliberations') return;

    hashes = [...hashes, payload.action.hashed.content.target_address];
  });
});

</script>

<br>
<h2>Deliberations I've joined</h2>

{#if loading }
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching deliberations: {error.data.data}.</span>
{:else if hashes.length === 0}
<span>No deliberations found for this deliberator.</span>
{:else}
<div style="display: flex; flex-direction: column">
  {#each hashes.reverse() as hash}
    <div style="margin-bottom: 8px;">
    <span on:click={() => navigate('deliberation', hash)}>
      <!-- <DeliberationDetail deliberationHash={hash}></DeliberationDetail> -->
      <DeliberationListItem deliberationHash={hash}></DeliberationListItem>
    </span>
    </div>
  {/each}
</div>
{/if}
