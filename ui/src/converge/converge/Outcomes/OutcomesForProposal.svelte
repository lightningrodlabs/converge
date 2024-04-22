<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { Record, EntryHash, ActionHash, AgentPubKey, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../../contexts';
import type { ConvergeSignal } from '../types';
import OutcomeLink from './OutcomeLink.svelte';

export let proposalHash: ActionHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;

let loading = true;
let error: any = undefined;

$: hashes, loading, error;

onMount(async () => {
  if (proposalHash === undefined) {
    throw new Error(`The proposalHash input is required for the OutcomesForProposal element`);
  }

  try {
    const links = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'get_outcomes_for_proposal',
      payload: proposalHash,
    });
    hashes = links.map(l => l.target);
  } catch (e) {
    error = e;
  }
  loading = false;

  client.on('signal', signal => {
    if (signal.zome_name !== 'converge') return;
    const payload = signal.payload as ConvergeSignal;
    if (payload.type !== 'LinkCreated') return;
    if (payload.link_type !== 'ProposalToOutcomes') return;

    hashes = [...hashes, payload.action.hashed.content.target_address];
  });
});

</script>

{#if loading }
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching outcomes: {error.data.data}.</span>
{:else if hashes.length === 0}
<span>No outcomes found for this proposal.</span>
{:else}
  <div style="display:flex; flex-direction:row; align-items:center; margin: 2px;">
    <div style="font-size: 50px; height: 75px;">→</div>
    {#each hashes as hash}
      <OutcomeLink outcomeHash={hash} />
    {/each}
  </div>
{/if}
