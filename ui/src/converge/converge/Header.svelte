<script lang="ts">
  import Logo from "../../assets/logo-svg-6.svg";
  // import Logo from "../../assets/logo-svg.svg";
  import FaPlusCircle from 'svelte-icons/fa/FaPlusCircle.svelte';
  import FaBell from 'svelte-icons/fa/FaBell.svelte';
  import FaBullhorn from 'svelte-icons/fa/FaBullhorn.svelte';
  import FaList from 'svelte-icons/fa/FaList.svelte';
  import FaHome from 'svelte-icons/fa/FaHome.svelte';
  import { navigate, view } from '../../store.js';
  import { clientContext } from '../../contexts';
  import type { EntryHash, Record, AgentPubKey, ActionHash, AppAgentClient, NewEntryAction } from '@holochain/client';
  import { onMount, setContext, getContext } from 'svelte';
  import { decode } from '@msgpack/msgpack';
  import "@holochain-open-dev/profiles/dist/elements/agent-avatar.js";
  import "@holochain-open-dev/profiles/dist/elements/my-profile.js";
  import "@holochain-open-dev/profiles/dist/elements/profiles-context.js";
  import type { Profile } from "@holochain-open-dev/profiles";
  import { encodeHashToBase64 } from "@holochain/client";
  import Avatar from "./Avatar.svelte";
  import { WeaveClient, isWeContext } from '@lightningrodlabs/we-applet';
  import SvgIcon from "./SvgIcon.svelte";

  // export let initialized: boolean = false;

  let client: AppAgentClient = (getContext(clientContext) as any).getClient();
  let currentView;
  
  view.subscribe(value => {
    currentView = value;
  });

  async function goToCreate() {
    navigate("create-coordination", {});
  }
  
  async function goToNotifications() {
    navigate("notifications", {});
  }
  
  async function goToDashboard() {
    navigate("dashboard", {});
  }
  
  async function goToBulletin() {
    navigate("all-deliberations", {});
  }

  onMount(async () => {
    console.log("onMount")
    // bind.refresh()
  })

  </script>

  <style>
    .converge-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
    }
    #converge-title {
      font-size: 24px;
      font-weight: 600;
      color: #000000;
      margin: 0;
      font-family: 'Montserrat', sans-serif;
      letter-spacing: 3.15px;
      padding: 4px 10px;
      border-radius: 100px;
      /* border: 2px solid black; */
    }

    #subtitle {
      font-size: 12px;
      font-weight: 600;
      color: #3fadab;
      margin: 0;
      letter-spacing: 1.15px;
    }
    
    /* show new action on tablet and mobile */
    @media (max-width: 991px) {
      #new-action, #subtitle {
        display: none !important;
      }
    }

    /* show only on mobile */
    @media (max-width: 767px) {
      #minilogo {
        display: none !important;
      }
    }
  </style>

  
  <header >
      <nav class="navbar">
        <div class="container-fluid converge-header">
          <div>
            {#if false && !isWeContext()}
            <a id="logo" class="navbar-brand" on:click={() => navigate("instructions")}>
              <img class="logo-image" src={Logo} alt="whos-in logo"/>
            </a>  
            {:else}
            <a id="logo" class="navbar-brand" on:click={() => navigate("instructions")}>

            <h1 id="converge-title" style="display: flex;
              align-items: center;
            ">
              <img id="minilogo" class="logo-image small" src={Logo} alt="whos-in logo"/>
              <!-- <span>Converge</span> -->
              <!-- <small id="subtitle">&nbsp;for Moss</small> -->
            </h1>
            </a>
            {/if}
          </div>
        <div>
  
        <!-- {#if initialized} -->

        <ul class="nav navbar-nav float-right">
  
        <li class="bulletin" on:click={goToBulletin}>
          {#if currentView == "all-deliberations"}
          <div class="bulletin-icon" style="color:#d92ed9">
            <FaBullhorn />
          </div>
          {:else}
          <div class="bulletin-icon">
            <FaBullhorn />
          </div>
          {/if}
        </li>
  
        <li class="dashboard" on:click={goToDashboard}>
          {#if currentView == "dashboard"}
          <div class="dashboard-icon" style="color:#d92ed9">
            <FaList />
          </div>
          {:else}
          <div class="dashboard-icon">
            <FaList />
          </div>
          {/if}
        </li>
  
        <!-- <li class="notifications-li">
          <div class="notifications" on:click={goToNotifications}>
            {#if currentView == "notifications"}
              <span style="color:#1952bb"><FaBell /></span>
            {:else}
              <span><FaBell /></span>
            {/if}
            <span class="notifications-count">
            </span>
          </div>
        </li> -->
        
        <svg xmlns="http://www.w3.org/2000/svg" style="margin: 0 10" width="1" height="30" viewBox="0 0 1 30"><defs><style>.a{fill:none;stroke:rgba(0,0,0,0.15);}</style></defs><line class="a" y2="30" transform="translate(0.5)"/></svg>
        <li class="middle-of-header-right">
          <div class="new-action-button"  on:click={() => {
            navigate("create-deliberation", {})
          }}>
            <div class="icon">
              <FaPlusCircle />
            </div>
            <!-- <i class="fas fa-plus white-circle-plus"></i> -->
            <!-- <img class="nav-image" src="/assets/add_circle_black_24dp-b42cee553b2665d6f62bd5d9ffc02837cf3c5a3084fc6a5674f5edf83776f565.svg" alt="Add circle black 24dp" border="0"> -->
            <!-- show if not mobile or tablet -->
            <span id="new-action">New deliberation</span>
          </div>
        </li>

        <svg xmlns="http://www.w3.org/2000/svg" style="margin: 0 10" width="1" height="30" viewBox="0 0 1 30"><defs><style>.a{fill:none;stroke:rgba(0,0,0,0.15);}</style></defs><line class="a" y2="30" transform="translate(0.5)"/></svg>
        <li class="notifications-li" style="margin-left: 10px;">
          <Avatar showNickname={true} agentPubKey={client.myPubKey}  size={24} namePosition="row"></Avatar>
        </li>
        
      </ul>
      <!-- {/if} -->
        </div><!-- /.navbar-collapse -->
        </div><!-- /.container-fluid -->
      </nav>
    </header>