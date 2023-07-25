<script lang="ts">
  import Logo from "../../assets/logo.png";
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
  
  // import {
  //   ProfilesStore,
  //   ProfilesClient,
  //   CreateProfile,
  //   ProfilePrompt,
  //   profilesStoreContext,
  //   MyProfile,
  //   ProfilesContext,
  //   ProfileDetail,
  //   ProfileListItemSkeleton,
  //   AgentAvatar,
  // } from '@holochain-open-dev/profiles';

  let client: AppAgentClient = (getContext(clientContext) as any).getClient();
  let currentView;
  
  view.subscribe(value => {
    currentView = value;
  });

  // let client: AppAgentClient;
  // let myAgentPubKey: AgentPubKey = (getContext(clientContext) as any).getClient();
  
  // if (!customElements.get('my-profile')){
  //   customElements.define('my-profile', MyProfile)
  // }

  // if (!customElements.get('profile-detail')){
  //   customElements.define('profile-detail', ProfileDetail)
  // }

  // if (!customElements.get('profile-list-item-skeleton')){
  //   customElements.define('profile-list-item-skeleton', ProfileListItemSkeleton)
  // }

  // if (!customElements.get('agent-avatar')){
  //   customElements.define('agent-avatar', AgentAvatar)
  // }

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
    navigate("all-coordinations", {});
  }
  </script>
  
  <header>
      <nav class="navbar">
        <div class="container-fluid converge-header">
          <div>
            <a id="logo" class="navbar-brand" on:click={() => navigate("instructions")}>
              <img class="logo-image" src={Logo} alt="whos-in logo"/>
            </a>    
          </div>
        <div>
  
        <ul class="nav navbar-nav float-right">
  
        <!-- <li class="bulletin" on:click={goToBulletin}>
          {#if currentView == "all-coordinations"}
          <div class="bulletin-icon" style="color:#1952bb">
            <FaBullhorn />
          </div>
          {:else}
          <div class="bulletin-icon">
            <FaBullhorn />
          </div>
          {/if}
        </li> -->
  
        <li class="dashboard" on:click={goToDashboard}>
          {#if currentView == "dashboard"}
          <div class="dashboard-icon" style="color:#1952bb">
            <FaList />
          </div>
          {:else}
          <div class="dashboard-icon">
            <FaList />
          </div>
          {/if}
        </li>
  
        <li class="notifications-li">
          <div class="notifications" on:click={goToNotifications}>
            {#if currentView == "notifications"}
              <span style="color:#1952bb"><FaBell /></span>
            {:else}
              <span><FaBell /></span>
            {/if}
            <span class="notifications-count">
              <!-- <Notifications client={client}></Notifications> -->
            </span>
          </div>
        </li>
      
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
            <span id="new-action">New deliberation</span>
          </div>
        </li>
  
        </ul>
        </div><!-- /.navbar-collapse -->
        </div><!-- /.container-fluid -->
      </nav>
    </header>
  