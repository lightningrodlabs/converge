<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { Record, EntryHash, ActionHash, AgentPubKey, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../contexts';
import CriterionDetail from './Criterion.svelte';
import type { ConvergeSignal } from './types';
//  import mwc-checkbox
import '@material/mwc-switch';

// export let objector: AgentPubKey;
export let criterionHash: EntryHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;

let loading = true;
let error: any = undefined;
let objections;

$: hashes, loading, error;

onMount(async () => {
  if (criterionHash === undefined) {
    throw new Error(`The objector input is required for the CriteriaForObjector element`);
  }

  try {
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'get_objectors_for_criterion',
      payload: criterionHash,
    });
    objections = records;
  } catch (e) {
    error = e;
  }
  loading = false;

  client.on('signal', signal => {
    if (signal.zome_name !== 'converge') return;
    const payload = signal.payload as ConvergeSignal;
    if (payload.type !== 'LinkCreated') return;
    console.log(payload.link_type)
    if (payload.link_type != 'CriterionToObjectors') return;
    console.log(payload)

    // hashes = [...hashes, payload.action.hashed.content.target_address];
  });
});

</script>

{#if loading }
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching criteria: {error.data.data}.</span>
{:else if objections.length === 0}
<span>No Objections found for this criteria.</span>
{:else}
<div style="display: flex; flex-direction: column">
  {#each objections as objection}
    <div style="margin-bottom: 8px;">
      {JSON.stringify(objection.tag)}
      <!-- <CriterionDetail criterionHash={hash}></CriterionDetail> -->
    </div>
  {/each}
</div>
{/if}
