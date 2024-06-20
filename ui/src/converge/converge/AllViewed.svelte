<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { type EntryHash, type Record, type AgentPubKey, type ActionHash, type AppAgentClient, type NewEntryAction, type decodeHashFromBase64, encodeHashToBase64 } from '@holochain/client';
import { clientContext } from '../../contexts';
import ViewedDetail from './ViewedDetail.svelte';
import type { ConvergeSignal } from './types';
import { viewed, setViewed, addToViewed } from '../../viewed.js';

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

let hashes: Array<ActionHash> | undefined;
let loading = true;
let error: any = undefined;

$: hashes, loading, error;

onMount(async () => {

  await fetchVieweds();
  // client.on('signal', signal => {
  //   if (signal.zome_name !== 'converge') return;
  //   const payload = signal.payload as ConvergeSignal;
  //   if (payload.type !== 'EntryCreated') return;
  //   if (payload.app_entry.type !== 'Viewed') return;
  //   hashes = [...hashes, payload.action.hashed.hash];
  // });
});

async function fetchVieweds() {
  try {
    const records = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'get_all_viewed',
      payload: null,
    });
    // console.log("all viewed records: ", records)
    setViewed(records.map(r => 
      encodeHashToBase64(r.viewed_hash)
    ));
    // hashes = links.map(l => l.target);
  } catch (e) {
    error = e;
  }
  loading = false;
}

</script>

<!-- {#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the vieweds: {error.data.data}.</span>
{:else if hashes.length === 0}
<span>No vieweds found.</span>
{:else}
<div style="display: flex; flex-direction: column">
  {#each hashes as hash}
    <div style="margin-bottom: 8px;">
      <ViewedDetail viewedHash={hash}  on:viewed-deleted={() => fetchVieweds()}></ViewedDetail>
    </div>
  {/each}
</div>
{/if} -->

