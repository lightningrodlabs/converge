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
  import { WeClient, isWeContext, initializeHotReload, type WAL, type Hrl } from '@lightningrodlabs/we-applet';  
  import Holochain from "./assets/holochain.png";
  import type { Deliberation, ConvergeSignal } from './converge/converge/types';
  import { appletServices } from './we';
  import DeliberationListItem from './converge/converge/Deliberations/DeliberationListItem.svelte';
  import Instructions from './Instructions.svelte';
  import SvgIcon from './SvgIcon.svelte';
  import AllViewed from './converge/converge/AllViewed.svelte';
  import { fade } from 'svelte/transition';
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

  // let hrlWithContext: HrlWithContext
  let weClient: WeClient

  $: client, loading, store, profilesStore, initialized, dna;

  async function checkIfNew() {
      try {
          const records = await client
          .callZome({
              cap_secret: null,
              role_name: 'converge',
              zome_name: 'converge',
              fn_name: 'get_deliberations_for_deliberator',
              payload: client.myPubKey,
          });

          if (records.length > 0) {
              navigate('dashboard');
          } else {
              navigate('instructions');
          }

          console.log(records);

      } catch (e) {
          console.log(e)
      }
      loading = false;
  }

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
        const url = `ws://localhost:${adminPort}`
        console.log("connecting to admin port at:", url)
        const adminWebsocket = await AdminWebsocket.connect({url: new URL(url)})
        const x = await adminWebsocket.listApps({})
        console.log("apps", x)
        const cellIds = await adminWebsocket.listCellIds()
        console.log("CELL IDS",cellIds)
        await adminWebsocket.authorizeSigningCredentials(cellIds[0])

      }
      console.log("appPort and Id is", appPort, appId)
      client = await AppAgentWebsocket.connect(appId,{url: new URL(url)})
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
                  currentView = "dashboard"
                  break;
                default:
                  throw new Error("Unknown applet-view block type:"+weClient.renderInfo.view.block);
              }
              break;
            case "creatable":
              switch (weClient.renderInfo.view.name) {
                case "Deliberation":
                currentView = "create-deliberation-mini"
              }
              break;  
            case "asset":
              switch (weClient.renderInfo.view.roleName) {
                case "converge":
                  switch (weClient.renderInfo.view.integrityZomeName) {
                    case "converge_integrity":
                      switch (weClient.renderInfo.view.entryType) {
                        case "deliberation":
                          currentView = "deliberation-asset"
                          currentHash = weClient.renderInfo.view.wal.hrl[1]
                          // console.log("weClient.renderInfo.view", weClient.renderInfo.view)
                          // hrlWithContext = weClient.renderInfo.view.hrlWithContext
                          break;
                        case "proposal":
                          currentView = "proposal-asset"
                          currentHash = weClient.renderInfo.view.wal.hrl[1]
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

    if (currentView == "home") {
      await checkIfNew()
    }

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

    client.on('signal', signal => {
      console.log("signalll", signal)
      if (signal.zome_name !== 'converge') return;
      const payload = signal.payload as ConvergeSignal;
      const urgentMessages = ['criterion-created', 'proposal-created', 'deliberation-created']
      const messagesFull = {
        'criterion-created': "A new criterion has been added to the deliberation " + payload.title,
        'proposal-created': "A new proposal has been added to the deliberation " + payload.title,
        'deliberation-created': "A new deliberation has been created: " + payload.title,
      }
      const messagesShort = {
        'criterion-created': "New criterion in " + payload.title,
        'proposal-created': "New proposal in " + payload.title,
        'deliberation-created': "New deliberation"
      }
      if (urgentMessages.includes(payload.message)) {
        console.log("activity received", payload)
        
        weClient.notifyFrame([{
          title: messagesShort[payload.message],
          body: messagesFull[payload.message],
          notification_type: "change",
          icon_src: undefined,
          urgency: "high",
          timestamp: Date.now()
        }])
      }
    });
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
  <profiles-context store="{profilesStore}">
    <profile-prompt>
      {#if currentView == "create-deliberation-mini"}
      <!-- style CreateDeliberation to display half its size in every way -->
        <CreateDeliberation />
      {:else if currentView == "deliberation-asset"}
        <main>
          <div class="attachment-container big">
            <DeliberationDetail deliberationHash={currentHash} />
          </div>
          <div class="attachment-container small" style="padding: 0;">
            <DeliberationListItem deliberationHash={currentHash} />
          </div>
        </main>
      {:else if currentView == "proposal-asset"}
        <main>
          <div style="padding-top: 10px !important;" class="attachment-container">
            <ProposalDetail proposalHash={currentHash} />
          </div>
        </main>
      {:else}
        <main class="converge-container">
          <Header />
          {#if currentView == "instructions"}
            <span in:fade={{duration: 200}} out:fade={{duration: 100}}>
              <Instructions />
            </span>
          {:else}
              {#if loading}
              <div style="display: flex; flex: 1; align-items: center; justify-content: center">
                <mwc-circular-progress indeterminate />
              </div>
              {:else if currentView == "deliberation"}
              <div class="white-container" in:fade={{duration: 300}} out:fade={{duration: 100}}>
                <DeliberationDetail deliberationHash={currentHash} />
              </div>
              {:else if currentView == "proposal"}
              <div class="white-container" in:fade={{duration: 300}} out:fade={{duration: 100}}>
                <ProposalDetail proposalHash={currentHash} />
              </div>
              {:else if currentView == "create-deliberation"}
              <div class="white-container" in:fade={{duration: 300}} out:fade={{duration: 100}}>
                <CreateDeliberation />
              </div>
              {:else if currentView == "dashboard"}
              <div class="whiteless-container" in:fade={{duration: 300}} out:fade={{duration: 100}}>
                <DeliberationsForDeliberator deliberator={client.myPubKey} />
              </div>
              {:else}
              <div class="whiteless-container" in:fade={{duration: 300}} out:fade={{duration: 100}}>
              <div id="content" style="display: flex; flex-direction: column; flex: 1;">
                <AllDeliberations />
                <!-- <button on:click={() => navigate("create-deliberation")}>Create Deliberation</button> -->
              </div>
              </div>
              {/if}
          {/if}

          <AllViewed />
        </main>
      {/if}
  </profile-prompt>
  <!-- <create-profile></create-profile> -->
<!-- {/if} -->
</profiles-context>
{/if}

<footer style="margin: 10px;">
    <!-- feedback button -->
    <SvgIcon icon="faBug" size="24" color="#000000" />
    <a href="https://docs.google.com/forms/d/e/1FAIpQLSchqUdQWqNCnjV8LfdLwuuJoqvdy2hWKotxKZ2L7TazaEusUQ/viewform" target="_blank" class="feedback-button">
      <span>Submit feedback</span>
    </a>
    :)
{#if !isWeContext && dna && !loading && currentView != "instructions" && currentView != "" && (!weClient || weClient.renderInfo.view.type != "attachable")}
<small>
  <img class="holochain-logo" src={Holochain} alt="holochain logo"/>
  Private Holochain network: {dna}
</small>
{/if}
</footer>

<style>
  main {
    /* text-align: center; */
    /* padding: 1em; */
    /* max-width: 240px; */
    /* margin: 0 auto; */
  }

  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }

  .small {
    display: none;
  }
  .big {
    display: block;
  }

  @media (max-width: 300px) {
    .big {
      display: none !important;
    }
    .small {
      display: block !important;
    }
    small {
      display: none;
    }
  }
</style>
