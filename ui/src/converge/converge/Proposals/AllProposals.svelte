<script lang="ts">
import { createEventDispatcher, onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import type { EntryHash, Record, AgentPubKey, ActionHash, AppAgentClient, NewEntryAction } from '@holochain/client';
import { clientContext } from '../../../contexts';
import ProposalListItem from './ProposalListItem.svelte';
import type { ConvergeSignal } from '../types';
import { view, viewHash, navigate } from '../../../store.js';
import { allProposals } from '../../../store.js';
import { encodeHashToBase64 } from "@holochain/client";

export let deliberationHash: ActionHash;
export let proposalCount = 0;
export let filter;
export let sort = "score";
export let hashes: Array<ActionHash> | undefined;

const dispatch = createEventDispatcher();

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

// let loading = true;
let error: any = undefined;
let allProposalScores = {};
let sortableProposals = {};
let sortedProposals = [];
let anyProposalPopup = false;

allProposals.subscribe(value => {
  hashes.forEach((hash) => {
    let stringHash = encodeHashToBase64(hash);
    if (value[stringHash]) {
      // allProposalScores[hash] = value[hash].score;
      // sortableProposals[hash] = value[hash];
      sortedProposals.push(value[stringHash]);
    }
  });
});

$: hashes, error, allProposalScores, sortableProposals, sortedProposals, filter;
// $: if (sort && sortableProposals && hashes && Object.values(sortableProposals).length == hashes.length && anyProposalPopup == false) {
// $: if ((anyProposalPopup == false) && sort && sortableProposals && hashes && Object.values(sortableProposals).length == hashes.length) {
// $: if (sort == "score" || sort == "respondants") {
  // console.log(sortableProposals)
  // if ((anyProposalPopup == false) && sort && sortableProposals && hashes && Object.values(sortableProposals).length == hashes.length) {
  //   let sortedProposalsJoined = Object.values(sortableProposals).sort((a, b) => {
  //     if (sort === 'score') {
  //       return b.score - a.score;
  //     } else if (sort === 'respondants') {
  //       return a.respondants - b.respondants;
  //     } else {
  //       return 1
  //     }
  //     return 1
  //   });
  //   let x = sortedProposalsJoined.map((proposal) => {
  //     return proposal.hash;
  //   })
  //   // let x = {}
  //   // for (let i = 0; i < sortedProposalsJoined.length; i++) {
  //   //   x[i] = sortedProposalsJoined[i].hash.join('');
  //   // }
  //   console.log(sortedProposals)
  //   console.log(sortedProposalsJoined)
  //   console.log(x)
  // }
  // sortedProposals = [...sortedProposals]
  // }
// }

// async function sortProposals() {
//   // console.log(sortableProposals)
//   // if ((anyProposalPopup == false) && sort && sortableProposals && hashes && Object.values(sortableProposals).length == hashes.length) {
//     let sortedProposalsJoined = Object.values(sortableProposals).sort((a, b) => {
//       if (sort === 'score') {
//         return b.score - a.score;
//       } else if (sort === 'respondants') {
//         return a.respondants - b.respondants;
//       } else {
//         return 1
//       }
//       return 1
//     });
//     let x = sortedProposalsJoined.map((proposal) => {
//       return proposal.hash;
//     })
//     // let x = {}
//     // for (let i = 0; i < sortedProposalsJoined.length; i++) {
//     //   x[i] = sortedProposalsJoined[i].hash.join('');
//     // }
//   // }
//   sortedProposals = [...sortedProposals]
// }


onMount(async () => {
  // await fetchProposals();

  // await sortProposals();

  // client.on('signal', signal => {
  //   if (signal.App.zome_name !== 'converge') return;
  //   const payload = signal.App.payload as ConvergeSignal;
  //   if (payload.type !== 'EntryCreated') return;
  //   if (payload.app_entry.type !== 'Proposal') return;
  //   // hashes = [...hashes, payload.action.hashed.hash];
  //   // fetchProposals();
  // });
});

// async function fetchProposals() {
//   try {
//     const records = await client.callZome({
//       cap_secret: null,
//       role_name: 'converge',
//       zome_name: 'converge',
//       fn_name: 'get_proposals_for_deliberation',
//       payload: deliberationHash,
//     });
//     // proposalCount = 1;
//     hashes = records.map(r => r.signed_action.hashed.hash)
//     proposalCount = hashes.length
//     sortedProposals = hashes
//   } catch (e) {
//     error = e;
//   }
//   // loading = false;
// } 

async function rateAlert() {
  dispatch('proposal-rated');
}

</script>

{#if !hashes}
<div style="display: flex; flex: 1; align-items: center; justify-content: center">
  <mwc-circular-progress indeterminate></mwc-circular-progress>
</div>
{:else if error}
<span>Error fetching the proposals: {error.data.data}.</span>
{:else if hashes.length === 0}
<span style="font-style: italic;">Try adding a proposal. How could we meet our criteria?</span>
{:else}
<div style="display: flex; flex-direction: column">
  {#each sortedProposals as hash (hash)}
    {#if deliberationHash}
      <ProposalListItem on:proposal-rated={rateAlert} on:outcome-created={(v) => {
        dispatch('outcome-created', v);
      }
      } bind:anyProposalPopup bind:sortableProposals bind:allProposalScores proposalHash={hash} {deliberationHash} {hashes} {filter} />
    {/if}
  {/each}
</div>
{/if}