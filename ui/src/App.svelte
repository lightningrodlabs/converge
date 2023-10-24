<script lang="ts">
  import { onMount, setContext } from 'svelte';
  import type { ActionHash, AppAgentClient } from '@holochain/client';
  import { AppWebsocket, AppAgentWebsocket } from '@holochain/client';
  import { view, viewHash, navigate } from './store.js';
  import { clientContext, profilesStoreContext } from './contexts';
  import { ProfilesStore, ProfilesClient } from "@holochain-open-dev/profiles";
  import { encodeHashToBase64, type AgentPubKey } from "@holochain/client";
  import { WeClient, isWeContext } from '@lightningrodlabs/we-applet';

  import '@shoelace-style/shoelace/dist/themes/light.css';
  import "@holochain-open-dev/profiles/dist/elements/profiles-context.js";
  import "@holochain-open-dev/profiles/dist/elements/profile-prompt.js";
  import "@holochain-open-dev/profiles/dist/elements/my-profile.js";
  import "@holochain-open-dev/profiles/dist/elements/list-profiles.js";
  import "@holochain-open-dev/profiles/dist/elements/profile-list-item-skeleton.js";
  import "@holochain-open-dev/profiles/dist/elements/profile-detail.js";
  import '@material/mwc-circular-progress';
  import "@holochain-open-dev/profiles/dist/elements/profiles-context.js";
  import CreateDeliberation from "./converge/converge/Deliberations/CreateDeliberation.svelte"
  import AllDeliberations from "./converge/converge/Deliberations/AllDeliberations.svelte"
  import CreateCriterion from './converge/converge/Criteria/CreateCriterion.svelte';
  import DeliberationDetail from './converge/converge/Deliberations/DeliberationDetail.svelte';
  import ProposalDetail from './converge/converge/Proposals/ProposalDetail.svelte';
  import Header from './converge/converge/Header.svelte';
  import DeliberationsForDeliberator from './converge/converge/Deliberations/DeliberationsForDeliberator.svelte';
    import { MyProfile } from '@holochain-open-dev/profiles/dist/elements/my-profile.js';

  let client: AppAgentClient | undefined;
  let loading = true;
  let store = undefined;
  let currentHash: ActionHash | undefined;
  let currentView: string | undefined;
  let profilesStore = undefined;
  let initialized: boolean = false;

  $: client, loading, store, profilesStore, initialized;
  // $: prof = profilesStore ? profilesStore.myProfile : undefined
  // $: prof = profilesStore ? profilesStore : undefined

  // let textValue = 'sadfassdaf';
  
  // function logSelectionDetails() {
  //   const textarea = document.getElementById('myTextarea');
  //   const selectionStart = textarea.selectionStart;
  //   const selectionEnd = textarea.selectionEnd;
  //   const selectionLength = selectionEnd - selectionStart;

  //   console.log({
  //     offset: selectionStart,
  //     length: selectionLength
  //   });
  // }

  onMount(async () => {
    // profilesStore = setupProfilesStore();
    // We pass '' as url because it will dynamically be replaced in launcher environments
    // client = await AppAgentWebsocket.connect('', 'converge');
    // profilesStore = new ProfilesStore(new ProfilesClient(client, 'converge'), {
    //   avatarMode: "avatar-optional",
    // });

    if (isWeContext()) {
      const weClient = await WeClient.connect();
      console.log(weClient.renderInfo)

      if (
        !(weClient.renderInfo.type === "applet-view")
        && !(weClient.renderInfo.view.type === "main")
      ) throw new Error("This Applet only implements the applet main view.");

      client = weClient.renderInfo.appletClient;
      console.log("client... ", client)
      profilesStore = new ProfilesStore(weClient.renderInfo.profilesClient, {
        avatarMode: "avatar-optional",
        minNicknameLength: 3,
      })
    } else {
      // We pass '' as url because it will dynamically be replaced in launcher environments
      client = await AppAgentWebsocket.connect('', 'dcan');
      profilesStore = new ProfilesStore(new ProfilesClient(client, 'converge'), {
        avatarMode: "avatar-optional",
        minNicknameLength: 3,
      });
    }


    // await profilesStore;

    // if (profilesStore.profiles && profilesStore.profiles.get(client.myPubKey)) {
    //   await profilesStore.profiles.get(client.myPubKey).subscribe((profile) => {
    //     if (profile.status == "complete") {
    //       console.log(profile.value.nickname)
    //       initialized = true;
    //     } else {
    //       console.log("not complete")
    //     }
    //   })
    // }

    // console.log(profilesStore)
    loading = false;
  });

  setContext(clientContext, {
    getClient: () => client,
  });

  setContext(profilesStoreContext, {
    getProfileStore: () => profilesStore,
  });

  view.subscribe(value => {
    currentView = value;
  });

  viewHash.subscribe(value => {
    currentHash = value;
  });
</script>

{#if profilesStore}
<!-- {JSON.stringify(prof)} -->
<!-- {profile}... -->
<!-- <profiles-context store={profilesStore}> -->
  <profiles-context store="{profilesStore}">
    <!-- {#if !initialized} -->
    <!-- <create-profile></create-profile> -->
    <!-- {:else} -->
    <!-- <list-profiles on:agent-selected={e => alert(e.detail.agentPubKey)}></list-profiles> -->
    <!-- <search-agent include-myself></search-agent> -->
    
    <profile-prompt>
    <main class="converge-container">
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
  </main>
  </profile-prompt>
  <!-- <create-profile></create-profile> -->
<!-- {/if} -->
</profiles-context>
{/if}

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
