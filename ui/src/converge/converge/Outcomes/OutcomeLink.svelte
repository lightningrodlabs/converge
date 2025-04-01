<script lang="ts">
  import { createEventDispatcher, onMount, getContext } from 'svelte';
  import '@material/mwc-circular-progress';
  import { decode } from '@msgpack/msgpack';
  import type { Record, ActionHash, AppClient, EntryHash, AgentPubKey, DnaHash } from '@holochain/client';
  import { clientContext } from '../../../contexts';
  import type { Outcome } from '../types';
  import '@material/mwc-circular-progress';
  import type { Snackbar } from '@material/mwc-snackbar';
  import '@material/mwc-snackbar';
  import '@material/mwc-icon-button';
  import '@material/mwc-linear-progress';
  import '@material/mwc-icon-button'
  import AttachmentsList from '../../../AttachmentsList.svelte';
  import { weaveUrlToWAL, weaveUrlFromWal,  } from "@theweave/api";
  import ProposalListItem from '../Proposals/ProposalListItem.svelte';
  // import '@lightningrodlabs/we-elements/dist/elements/wal-embed.js';
  import { weClientStored } from '../../../store.js';
  import SvgIcon from '../SvgIcon.svelte';
  import { getMyDna } from '../../../util';

  export let outcomeHash: ActionHash;
  
  const dispatch = createEventDispatcher();

  let weClient;
  let dnaHash: DnaHash;
  weClientStored.subscribe(value => {
    weClient = value;
  });

  let client: AppClient = (getContext(clientContext) as any).getClient();

  
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
    <AttachmentsList attachments={[outcome?.outcome_attachment]} allowDelete={false} />
  {/if}
{/if}
