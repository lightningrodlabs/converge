<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { Record, EntryHash, ActionHash, AgentPubKey, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../../contexts';
import CriterionDetail from './Criterion.svelte';
import type { ConvergeSignal } from '../types';
//  import mwc-checkbox
import ObjectionMini from './Objection.svelte'
import '@material/mwc-switch';

// export let objector: AgentPubKey;
export let criterionHash: EntryHash;

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;

let loading = true;
let error: any = undefined;
export let objections;

$: hashes, loading, error;

onMount(async () => {
  if (criterionHash === undefined) {
    throw new Error(`The objector input is required for the CriteriaForObjector element`);
  }

  await fetchObjections()
  loading = false;

  client.on('signal', signal => {
    if (signal.App.zome_name !== 'converge') return;
    const payload = signal.App.payload as ConvergeSignal;
    if (payload.type !== 'LinkCreated') return;
    let linkType = Object.keys(payload.link_type)[0]
    if (linkType != 'CriterionToObjectors') return;
    fetchObjections()
  });
});

async function fetchObjections() {
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
}

</script>

<div style="display: flex; flex-direction: column; height: 160px; overflow: scroll;">
{#if loading }
<!-- <div style="display: flex; flex: 1; align-items: center; justify-content: center"> -->
  <mwc-circular-progress indeterminate></mwc-circular-progress>
<!-- </div> -->
{:else if error}
<span>Error fetching criteria: {error.data.data}.</span>
{:else if objections.length === 0}
<span>No Objections found for this criteria.</span>
{:else}
  {#each objections as objection}
    <ObjectionMini objection={objection.tag}></ObjectionMini>
    <!-- <div style="margin-bottom: 8px;"> -->
      <!-- {JSON.stringify(objection.tag)} -->
      <!-- <CriterionDetail criterionHash={hash}></CriterionDetail> -->
    <!-- </div> -->
  {/each}
  {/if}
</div>
