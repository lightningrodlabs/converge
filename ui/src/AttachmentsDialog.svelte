<script lang="ts">
  import { type HrlB64WithContext, isWeContext, type HrlWithContext } from "@lightningrodlabs/we-applet";
  import { cloneDeep } from "lodash";
  // import type { Board, Piece } from "./board";
  import { getContext, onMount } from "svelte";
  // import type { GamezStore } from "./store";
  import { hrlWithContextToB64} from "./util";
  import SvgIcon from "./SvgIcon.svelte";
  import '@shoelace-style/shoelace/dist/components/button/button.js';
  import '@shoelace-style/shoelace/dist/components/dialog/dialog.js';
  import AttachmentsList from "./AttachmentsList.svelte";
  import AttachmentsBind from "./AttachmentsBind.svelte";
  import { weClientStored } from './store.js';
  import { createEventDispatcher } from 'svelte';

  // const { getStore } :any = getContext("gzStore");
  // let store: GamezStore = getStore();
  // let piece: Piece | undefined
  export let attachments: Array<HrlB64WithContext> = []
  const dispatch = createEventDispatcher();
 
  let weClient;
  weClientStored.subscribe(value => {
    weClient = value;
  });

  $:attachments = attachments

  // export let activeBoard: Board
  export const close=()=>{dialog.hide()}
  export const open=()=>{
    bind.refresh()
    dialog.show()
  }
  // export const open=(p: Piece)=>{
  //   piece = p
  //   if (piece) {
  //     attachments = piece.attachments ? cloneDeep(piece.attachments): []
  //   } else {
  //     attachments = activeBoard.state().props.attachments
  //   }
  //   bind.refresh()
  //   dialog.show()
  // }
  let dialog
  $: attachments
  let bind

  function removeAttachment(index: number) {
    attachments.splice(index, 1);
    attachments = attachments
    // handleSave()
  }

  const addAttachment = async () => {
    const hrl = await weClient.userSelectHrl()
    if (hrl) {
      _addAttachment(hrl)
    }
  }

  const _addAttachment = (hrl: HrlWithContext) => {
    attachments.push(hrlWithContextToB64(hrl))
    attachments = attachments
    // handleSave()
  }

  // const handleSave = async () => {
    // dispatch('save-attachments', { attachments })
    // if (piece) {
    //   activeBoard.requestChanges([{
    //     type: 'set-piece-attachments', 
    //     id: piece.id,
    //     attachments
    //   }])
    // } else {
    //   const props = cloneDeep(activeBoard.state().props)
    //   props.attachments = cloneDeep(attachments)
    //   activeBoard.requestChanges([{type: 'set-props', props }])
    // }
  // }

  onMount(async () => {
    bind.refresh()
    dialog.show()
  })
</script>

<!-- <sl-dialog label="Add links" bind:this={dialog}> -->
  {#if isWeContext()}
  
  <!-- <div>
    <h3>Search Linkables:</h3> 
  </div>  -->
  <button style="margin-top:5px;margin-right: 5px; width: fit-content;" on:click={()=>addAttachment()} >
    <SvgIcon icon=link size=16/>
  </button>
  
  <AttachmentsBind
  bind:this = {bind}
  on:add-binding={(e)=>_addAttachment(e.detail)} 
  />
  
  <div style="display:block; width: 100%">
    <AttachmentsList attachments={attachments}
        on:remove-attachment={(e)=>removeAttachment(e.detail)}/>
  </div>

  {/if}
<!-- </sl-dialog> -->

<style>


  sl-dialog::part(panel) {
      background: #FFFFFF;
      border: 2px solid rgb(166 115 55 / 26%);
      border-bottom: 2px solid rgb(84 54 19 / 50%);
      border-top: 2px solid rgb(166 115 55 / 5%);
      box-shadow: 0px 15px 40px rgb(130 107 58 / 35%);
      border-radius: 10px;
  }
</style>