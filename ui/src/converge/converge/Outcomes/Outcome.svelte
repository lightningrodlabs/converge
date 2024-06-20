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
  import { weaveUrlToWAL, weaveUrlFromWal,  } from "@lightningrodlabs/we-applet";
  import ProposalListItem from '../Proposals/ProposalListItem.svelte';
  import '@lightningrodlabs/we-elements/dist/elements/wal-embed.js';
  import { weClientStored } from '../../../store.js';
  import { countViewed, addToViewed } from '../../../viewed.js';
  import SvgIcon from '../SvgIcon.svelte';
  import { getMyDna } from '../../../util';

  export let outcomeHash: ActionHash;
  export let deliberationHash: ActionHash;
  
  const dispatch = createEventDispatcher();

  let weClient;
  let dnaHash: DnaHash;
  weClientStored.subscribe(value => {
    weClient = value;
  });

  let client: AppAgentClient = (getContext(clientContext) as any).getClient();

  
  let loading = true;
  let error: any = undefined;

  let record: Record | undefined;
  let outcome: Outcome | undefined;

  let allProposalScores = {};
  let sortableProposals = {};

  let walEmbed;
  let showEmbed = false;
  let weaveUrl;
  $: showEmbed, weaveUrl;



  async function fetchOutcome() {
    // loading = true;
    // error = undefined;
    // record = undefined;
    // proposal = undefined;
    
    try {
      record = await client.callZome({
        cap_secret: null,
        role_name: 'converge',
        zome_name: 'converge',
        fn_name: 'get_outcome',
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
    await fetchOutcome();
    addToViewed(outcomeHash, client);
    // await fetchDeliberation();

    dnaHash = await getMyDna("converge", client)

    weaveUrl = weaveUrlFromWal({
      hrl: [dnaHash, outcome.proposal],
      context: {}
    })

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
    <div class="outcome-outer" style="display:flex; padding: 6px 28px; flex-direction: column">
      <div style="display:flex; align-items:center; flex-direction: row;">
      <!-- <div > -->
      <!-- {#if outcome?.title} -->
        <h1
          style="font-size: 24px; margin-right: auto;"
        >
          {outcome?.title}
        </h1>
      <!-- {/if} -->
      {#if weaveUrl}
        <AttachmentsList attachments={[
          // outcome?.proposal as WAL attachment
          weaveUrl
        ]} allowDelete={false} />
        <div style="font-size: 50px; height: 75px;">→</div>
      {/if}
      <AttachmentsList attachments={[outcome?.outcome_attachment]} allowDelete={false} />
      <div
        style="display: flex; align-items: center; margin-left: auto;"
      >
      {#if showEmbed}
        <button
          class="expand-button"
          on:click={() => {
            showEmbed = false;
            console.log(showEmbed)
          }}
          >
          Collapse
        </button>
      {:else}
        <button
          class="expand-button"
          on:click={() => {
            showEmbed = true;
            console.log(showEmbed)
          }}
          >
          Expand
        </button>
        {/if}
      </div>
    </div>
    {#if showEmbed}
      <div class="expanded-outcome">
        {#if outcome.proposal}
          <div>
            <ProposalListItem proposalHash={outcome.proposal} {deliberationHash} {allProposalScores} {sortableProposals} />
          </div>
        {/if}
        <wal-embed
          bind:this={walEmbed}
          on:click={(e)=>{
            console.log(e)
            e.stopPropagation()
          }}
          class="embed"
          style="margin-top: 20px;"
          src={outcome.outcome_attachment}
            >
        </wal-embed>
      </div>
    {/if}
  </div>

  {/if}
{/if}

<style>
  .outcome-outer {
    display:flex; 
    /* align-items:center;  */
    padding: 6px 28px; 
    margin-top: 6px; 
    box-shadow: 0px 0px 6px 4px #00000019;
    border-radius: 6px;
  }

  .outcome-outer:hover {
    box-shadow: 0px 0px 6px 4px #00000034;
  }

  .expanded-outcome {
    /* margin-top: -6px; border-radius: 6px; display: flex;  box-shadow: 0px 4px 6px #00000034; flex-direction:column; border-top: 0; padding: 6px; background: white; */
  }

  .expand-button {
    background: rgb(245, 245, 245);
    border: none;
    padding: 6px;
    cursor: pointer;
    border-radius: 4px;
    background-color: #86658e;
    color: white;
  }

  .expand-button:hover {
    background-color: #5c455e;
  }
</style>
