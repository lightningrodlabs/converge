<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { EntryHash, Record, AgentPubKey, ActionHash, AppClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../../contexts';
import DeliberationDetail from './DeliberationDetail.svelte';
import type { ConvergeSignal } from '../types';
import { refetchDeliberations } from '../../../refetch';
import { view, viewHash, navigate, weClientStored } from '../../../store.js';
import DeliberationListItem from './DeliberationListItem.svelte';
import { allDeliberations } from '../../../store.js';
import type { FrameNotification, WAL } from '@theweave/api';
import { getMyDna } from '../../../util';

let myDNA;
let deliberations = [];
allDeliberations.subscribe(value => {
  console.log('allDeliberations', value);
  deliberations = value;

  let frameNotifications: FrameNotification[] = []
  deliberations.forEach(d => {
    console.log('deliberation', d['deliberators']);
    let agentJoinedNotifications = d['deliberators'].map((deliberator) => {
      const deliberationWAL: WAL = { hrl: [myDNA, d.action_hash], context: "" }
      const notification: FrameNotification = {
          title: "Joined Deliberation",
          body: "Someone joined the deliberation",
          notification_type: "change",
          icon_src: undefined,
          urgency: "low",
          timestamp: deliberator.dateJoined ? Math.round(deliberator.dateJoined / 1000) : 0,
          aboutWal: deliberationWAL,
          fromAgent: deliberator.deliberator,
      }
      return notification;
    });
    frameNotifications = [...frameNotifications, ...agentJoinedNotifications];
  });
  console.log('frameNotifications', frameNotifications[0], new Date(Date.now()).getTime());
  $weClientStored.notifyFrame(frameNotifications);
  
});

let client: AppClient = (getContext(clientContext) as any).getClient();

// let hashes: Array<ActionHash> | undefined;
let loading = true;
let error: any = undefined;

$: deliberations, loading, error;

onMount(async () => {
  myDNA = await getMyDna("converge", client);
  await refetchDeliberations(client);
  loading = false;
  setInterval(() => {
    refetchDeliberations(client);
  }, 60000);
  // await fetchDeliberations();
  // client.on('signal', signal => {
  //   if (signal.value.zome_name !== 'converge') return;
  //   const payload = signal.value.payload as ConvergeSignal;
  //   if (payload.type !== 'EntryCreated') return;
  //   if (payload.app_entry.type !== 'Deliberation') return;
  //   hashes = [...hashes, payload.action.hashed.hash];
  // });
});

// async function fetchDeliberations() {
//   try {
//     const records = await client.callZome({
//       cap_secret: null,
//       role_name: 'converge',
//       zome_name: 'converge',
//       fn_name: 'get_all_deliberations',
//       payload: null,
//     });
//     hashes = records.map(r => r.signed_action.hashed.hash);
//   } catch (e) {
//     error = e;
//   }
//   loading = false;
// }

</script>

<h2>All Deliberations</h2>

{#if loading}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the deliberations: {error.data.data}.</span>
{:else if deliberations.length === 0}
<span>No deliberations found.</span>
{:else if deliberations.length > 0}
  {#each deliberations as deliberation}
    <span on:click={() => navigate('deliberation', deliberation.action_hash)}>
      <DeliberationListItem {deliberation}  on:deliberation-deleted={() => fetchDeliberations()}></DeliberationListItem>
    </span>
  {/each}
  <!-- {#each hashes.reverse() as hash}
    <span on:click={() => navigate('deliberation', hash)}>
      <DeliberationListItem deliberationHash={hash}  on:deliberation-deleted={() => fetchDeliberations()}></DeliberationListItem>
    </span>
  {/each} -->
{/if}