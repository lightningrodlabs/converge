<script lang="ts">
  import { onMount, setContext } from 'svelte';
  import type { ActionHash, AppAgentClient } from '@holochain/client';
  import { AppWebsocket, AppAgentWebsocket, AdminWebsocket } from '@holochain/client';
  import { view, viewHash, navigate, setWeClient } from './store.js';
  import { clientContext, profilesStoreContext } from './contexts';
  import { ProfilesStore, ProfilesClient } from "@holochain-open-dev/profiles";
  import { encodeHashToBase64, type AgentPubKey } from "@holochain/client";
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
  import { WeClient, isWeContext, initializeHotReload, type HrlWithContext, type Hrl } from '@lightningrodlabs/we-applet';  
  import Holochain from "./assets/holochain.png";
  import { appletServices } from './we';
  // import AttachmentsList from './AttachmentsList.svelte';
  // import AttachmentsBind from './AttachmentsBind.svelte';
  // import AttachmentsDialog from './AttachmentsDialog.svelte';

  const appId = import.meta.env.VITE_APP_ID ? import.meta.env.VITE_APP_ID : 'converge'
  const roleName = 'converge'
  const appPort = import.meta.env.VITE_APP_PORT ? import.meta.env.VITE_APP_PORT : 8888
  const adminPort = import.meta.env.VITE_ADMIN_PORT
  const url = `ws://localhost:${appPort}`;

  let client: AppAgentClient | undefined;
  let loading = true;
  let store = undefined;
  let currentHash: ActionHash | undefined;
  let currentView: string | undefined;
  let profilesStore = undefined;
  let initialized: boolean = false;
  let dna;

  let connected = false

  enum RenderType {
    App,
    Hrl,
    BlockActiveBoards
  }

  let renderType = RenderType.App
  let hrlWithContext: HrlWithContext
  let weClient: WeClient

  $: client, loading, store, profilesStore, initialized, dna;
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


  async function initialize() : Promise<void> {
    console.log(import.meta.env)
    let profilesClient
    if ((import.meta as any).env.DEV) {
      try {
        await initializeHotReload();
      } catch (e) {
        console.warn("Could not initialize applet hot-reloading. This is only expected to work in a We context in dev mode.")
      }
    }
    if (!isWeContext()) {
      console.log("adminPort is", adminPort)
      if (adminPort) {
        const adminWebsocket = await AdminWebsocket.connect(new URL(`ws://localhost:${adminPort}`))
        const x = await adminWebsocket.listApps({})
        console.log("apps", x)
        const cellIds = await adminWebsocket.listCellIds()
        console.log("CELL IDS",cellIds)
        await adminWebsocket.authorizeSigningCredentials(cellIds[0])
      }
      console.log("appPort and Id is", appPort, appId)
      client = await AppAgentWebsocket.connect(new URL(url), appId)
      profilesClient = new ProfilesClient(client, appId);
    
      // client = await AppAgentWebsocket.connect('', 'dcan');
      // profilesStore = new ProfilesStore(new ProfilesClient(client, 'converge'), {
      //   avatarMode: "avatar-optional",
      //   minNicknameLength: 3,
      // });
    }
    else {
      // const weClient = await WeClient.connect();
      weClient = await WeClient.connect(appletServices);
      // store set
      setWeClient(weClient)
      // weClient = await WeClient.connect();

      switch (weClient.renderInfo.type) {
        case "applet-view":
          switch (weClient.renderInfo.view.type) {
            case "main":
              // here comes your rendering logic for the main view
              break;
            case "block":
              switch(weClient.renderInfo.view.block) {
                case "active_boards":
                  renderType = RenderType.BlockActiveBoards
                  break;
                default:
                  throw new Error("Unknown applet-view block type:"+weClient.renderInfo.view.block);
              }
              break;
            case "attachable":
              switch (weClient.renderInfo.view.roleName) {
                case "kando":
                  switch (weClient.renderInfo.view.integrityZomeName) {
                    case "syn_integrity":
                      switch (weClient.renderInfo.view.entryType) {
                        case "document":
                          renderType = RenderType.Hrl
                          hrlWithContext = weClient.renderInfo.view.hrlWithContext
                          break;
                        default:
                          throw new Error("Unknown entry type:"+weClient.renderInfo.view.entryType);
                      }
                      break;
                    default:
                      throw new Error("Unknown integrity zome:"+weClient.renderInfo.view.integrityZomeName);
                  }
                  break;
                default:
                  throw new Error("Unknown role name:"+weClient.renderInfo.view.roleName);
              }
              break;
            default:
              throw new Error("Unsupported applet-view type");
          }
          break;
        case "cross-applet-view":
          switch (this.weClient.renderInfo.view.type) {
            case "main":
              // here comes your rendering logic for the cross-applet main view
              //break;
            case "block":
              //
              //break;
            default:
              throw new Error("Unknown cross-applet-view render type.")
          }
          break;
        default:
          throw new Error("Unknown render view type");

      }
      
      //@ts-ignore
      client = weClient.renderInfo.appletClient;
      //@ts-ignore
      profilesClient = weClient.renderInfo.profilesClient;
    }
    profilesStore = new ProfilesStore(profilesClient);
    connected = true
  }

  onMount(async () => {
    await initialize()
    // // profilesStore = setupProfilesStore();
    // // We pass '' as url because it will dynamically be replaced in launcher environments
    // // client = await AppAgentWebsocket.connect('', 'converge');
    // // profilesStore = new ProfilesStore(new ProfilesClient(client, 'converge'), {
    // //   avatarMode: "avatar-optional",
    // // });

    // if ((import.meta as any).env.DEV) {
    //   try {
    //     await initializeHotReload();
    //   } catch (e) {
    //     console.warn("Could not initialize applet hot-reloading. This is only expected to work in a We context in dev mode.")
    //   }
    // }

    // if (!isWeContext()) {
    //   // We pass '' as url because it will dynamically be replaced in launcher environments
    //   client = await AppAgentWebsocket.connect('', 'dcan');
    //   profilesStore = new ProfilesStore(new ProfilesClient(client, 'converge'), {
    //     avatarMode: "avatar-optional",
    //     minNicknameLength: 3,
    //   });
    // } else {
    //   const weClient = await WeClient.connect();
    //   // console.log(weClient.renderInfo)

    //   // if (
    //   //   !(weClient.renderInfo.type === "applet-view")
    //   //   && !(weClient.renderInfo.view.type === "main")
    //   // ) throw new Error("This Applet only implements the applet main view.");

    //   // client = weClient.renderInfo.appletClient;
    //   // console.log("client... ", client)
    //   // profilesStore = new ProfilesStore(weClient.renderInfo.profilesClient, {
    //   //   avatarMode: "avatar-optional",
    //   //   minNicknameLength: 3,
    //   // })
    // }


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

    try {
      dna = await client
        .callZome({
            cap_secret: null,
            role_name: 'converge',
            zome_name: 'converge',
            fn_name: 'get_dna_hash',
            payload: null,
        });
        console.log("dna")
      console.log(dna)
    } catch (e) {
      console.log("no dna")

      console.log(e)
    }


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

<!-- <AttachmentsList attachments={[]} /> -->
<!-- <AttachmentsBind /> -->
<!-- <AttachmentsDialog /> -->

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

{#if dna && !loading && currentView != "instructions" && currentView != ""}
<footer style="margin: 10px;">
<small>
  <img class="holochain-logo" src={Holochain} alt="holochain logo"/>
  Private Holochain network: {dna}
</small>
</footer>
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
