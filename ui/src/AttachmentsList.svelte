<script lang="ts">
  import "@shoelace-style/shoelace/dist/components/skeleton/skeleton.js";
  import { createEventDispatcher, getContext } from "svelte";
  import { hrlB64WithContextToRaw } from "./util";
  import type { HrlB64WithContext } from "@lightningrodlabs/we-applet";
  import { clientContext } from './contexts';
  import { weClientStored } from './store.js';
  import type { EntryHash, Record, AgentPubKey, ActionHash, AppAgentClient, NewEntryAction } from '@holochain/client';
  import { WeClient, isWeContext, initializeHotReload, type HrlWithContext, type Hrl } from '@lightningrodlabs/we-applet';  
  import SvgIcon from "./SvgIcon.svelte";

  const dispatch = createEventDispatcher()

  export let attachments: Array<HrlB64WithContext>
  export let allowDelete = true

  let weClient;
  weClientStored.subscribe(value => {
    weClient = value;
  });

</script>
<div class="attachments-list">
  {#each attachments as attachment, index}
    <div 
      class:attachment-item-with-delete={allowDelete}
      class:attachment-item={!allowDelete}
    >
      {#await weClient.attachableInfo(hrlB64WithContextToRaw(attachment))}
        <sl-button size="small" loading></sl-button>
      {:then { attachableInfo }}
        <sl-button  size="small"
          on:click={(e)=>{
              e.stopPropagation()
              const hrlWithContext = hrlB64WithContextToRaw(attachment)
              weClient.openHrl(hrlWithContext)
            }}
          style="display:flex;flex-direction:row;margin-right:5px"><sl-icon src={attachableInfo.icon_src} slot="prefix"></sl-icon>
          {attachableInfo.name}
        </sl-button> 
        {#if allowDelete}
          <sl-button size="small"
            on:click={()=>{
              dispatch("remove-attachment",index)
            }}
          >
            <SvgIcon icon=faTrash size=12 />
          </sl-button>
        {/if}
      {:catch error}
        Oops. something's wrong.
      {/await}
    </div>
  {/each}
</div>
<style>
  .attachments-list {
    display:flex;
    flex-direction:row;
    flex-wrap: wrap;
  }
  .attachment-item {
  }
  .attachment-item-with-delete {
    border:1px solid #aaa; 
    background-color:rgba(0,255,0,.1); 
    padding:4px;
    display:flex;
    margin-right:4px;
    border-radius:4px;
  }
</style>