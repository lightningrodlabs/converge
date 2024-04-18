<script lang="ts">
  import { createEventDispatcher, onMount, getContext } from 'svelte';
  import '@material/mwc-circular-progress';
  import { decode } from '@msgpack/msgpack';
  import type { Record, ActionHash, AppAgentClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
  import { clientContext } from '../../../contexts';
  import type { Outcome } from '../types';
  import '@material/mwc-circular-progress';
  import type { Snackbar } from '@material/mwc-snackbar';
  import '@material/mwc-snackbar';
  import '@material/mwc-icon-button';
  import '@material/mwc-linear-progress';
  import '@material/mwc-icon-button'
  import AttachmentsList from '../../../AttachmentsList.svelte';
  import { weaveUrlToWAL } from "@lightningrodlabs/we-applet";
  import ProposalListItem from '../Proposals/ProposalListItem.svelte';
  import '@lightningrodlabs/we-elements/dist/elements/wal-embed.js';

  export let outcomeHash: ActionHash;
  export let deliberationHash: ActionHash;
  
  const dispatch = createEventDispatcher();

  let client: AppAgentClient = (getContext(clientContext) as any).getClient();

  
  let loading = true;
  let error: any = undefined;

  let record: Record | undefined;
  let outcome: Outcome | undefined;

  let allProposalScores = {};
  let sortableProposals = {};

  let walEmbed;


  async function fetchOutcomes() {
    // loading = true;
    // error = undefined;
    // record = undefined;
    // proposal = undefined;
    
    try {
      record = await client.callZome({
        cap_secret: null,
        role_name: 'converge',
        zome_name: 'converge',
        fn_name: 'get_proposal',
        payload: outcomeHash,
      });
      if (record) {
        outcome = decode((record.entry as any).Present.entry) as Outcome;
        // console.log(proposal)
      }
    } catch (e) {
      error = e;
    }

    loading = false;
  }

  onMount(async () => {
    if (outcomeHash === undefined) {
      throw new Error(`The proposalHash input is required for the ProposalDetail element`);
    }
    await fetchOutcomes();
    // await fetchDeliberation();

    window.addEventListener("keydown", checkKey);

  });


  function checkKey(e) {
    if (e.key === "Escape" && !e.shiftKey) {
      e.preventDefault();
      // outcomeDetailHash=outcomeHash;
      // outcomePopup = false;
      // anyOutcomePopup = false;
    }
  }

</script>

{#if loading}
  <mwc-circular-progress indeterminate></mwc-circular-progress>
{:else}
  {#if error}
    <div style="color:red">Error getting outcome: {error}</div>
  {:else}
    <!-- <div style="display: flex; flex-direction:column;"> -->
      {#if outcome.proposal}
        <!-- <div style="width: 30vw; display: inline-block;"> -->
        <div>
          <!-- export let proposalHash: ActionHash;
export let deliberationHash: ActionHash | undefined;
export let allProposalScores;
export let filter;
export let hashes;
export let sortableProposals;
export let anyProposalPopup; -->
          <ProposalListItem proposalHash={outcome.proposal} {deliberationHash} {allProposalScores} {sortableProposals} />
        </div>
      {/if}
      <button on:click={()=>{
        walEmbed.resizeIFrameToFitContent()
      }}>wal embed</button>
      <wal-embed
        bind:this={walEmbed}
        on:click={(e)=>{
          console.log(e)
          e.stopPropagation()
        }}
        class="embed"
        style="margin-top: 20px; height: 1000px;"
        src={outcome.outcome_attachment}
          >
      </wal-embed>
      <!-- <AttachmentsList attachments={[outcome?.outcome_attachment]} allowDelete={false} /> -->
      <!-- <h3>{outcome?.title}</h3>
      <p>{outcome?.description}</p> -->
    <!-- </div> -->
  {/if}
{/if}
