<script lang="ts">
  import { onMount, setContext } from 'svelte';
  import type { ActionHash, AppAgentClient } from '@holochain/client';
  import { AppWebsocket, AppAgentWebsocket } from '@holochain/client';
  import { view, viewHash, navigate } from './store.js';
  import { clientContext } from './contexts';
  import { ProfilesStore, ProfilesClient } from "@holochain-open-dev/profiles";
  
  import '@material/mwc-circular-progress';
  import "@holochain-open-dev/profiles/dist/elements/profiles-context.js";
  import CreateDeliberation from "./converge/converge/CreateDeliberation.svelte"
  import AllDeliberations from "./converge/converge/AllDeliberations.svelte"
  import CreateCriterion from './converge/converge/CreateCriterion.svelte';
  import DeliberationDetail from './converge/converge/DeliberationDetail.svelte';
  import ProposalDetail from './converge/converge/ProposalDetail.svelte';
  import Header from './converge/converge/Header.svelte';
  import DeliberationsForDeliberator from './converge/converge/DeliberationsForDeliberator.svelte';

  let client: AppAgentClient | undefined;
  let loading = true;
  let store = undefined;
  let currentHash: ActionHash | undefined;
  let currentView: string | undefined;
  let profilesStore = undefined;

  $: client, loading, store, profilesStore;

  onMount(async () => {
    // profilesStore = setupProfilesStore();
    // We pass '' as url because it will dynamically be replaced in launcher environments
    client = await AppAgentWebsocket.connect('', 'converge');
    profilesStore = new ProfilesStore(new ProfilesClient(client, 'converge'), {
      avatarMode: "avatar-optional",
    });
    console.log(profilesStore)
    loading = false;
  });

  setContext(clientContext, {
    getClient: () => client,
  });

  view.subscribe(value => {
    currentView = value;
  });

  viewHash.subscribe(value => {
    currentHash = value;
  });
</script>

<main class="converge-container">
{#if profilesStore}

<profiles-context store={profilesStore}>
  <profile-prompt>
    <!-- <list-profiles on:agent-selected={e => alert(e.detail.agentPubKey)}></list-profiles> -->
    <!-- <search-agent include-myself></search-agent> -->

  <Header />
  <div class="white-container">
  {#if loading}
    <div style="display: flex; flex: 1; align-items: center; justify-content: center">
      <mwc-circular-progress indeterminate />
    </div>
  {:else if currentView == "deliberation"}
      <DeliberationDetail deliberationHash={currentHash} />
  {:else if currentView == "proposal"}
      <ProposalDetail proposalHash={currentHash} />
  {:else if currentView == "create-deliberation"}
    <CreateDeliberation />
  {:else if currentView == "dashboard"}
    <DeliberationsForDeliberator deliberator={client.myPubKey} />
  {:else}
    <div id="content" style="display: flex; flex-direction: column; flex: 1;">
      <AllDeliberations />
      <!-- <button on:click={() => navigate("create-deliberation")}>Create Deliberation</button> -->
    </div>
  {/if}
  </div>
</profile-prompt>
<create-profile></create-profile>
</profiles-context>
{/if}
</main>

<style>
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
