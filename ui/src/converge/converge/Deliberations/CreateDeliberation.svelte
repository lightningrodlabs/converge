<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../../contexts';
import type { Deliberation } from '../types';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import '@material/mwc-textfield';
import '@material/mwc-radio';
import '@material/mwc-formfield';
import '@material/mwc-textarea';
import { view, viewHash, navigate } from '../../../store.js';
import AttachmentsDialog from "../../../AttachmentsDialog.svelte"
import SvgIcon from "../../../SvgIcon.svelte";
import { WeaveClient, isWeContext, initializeHotReload, type WAL, type Hrl, weaveUrlFromWal } from '@lightningrodlabs/we-applet';
import AttachmentsBind from '../../../AttachmentsBind.svelte';
import { HoloHashMap, type EntryHashMap } from "@holochain-open-dev/utils";
let client: AppAgentClient = (getContext(clientContext) as any).getClient();
import { weClientStored } from '../../../store.js';
import app from '../../../main';
import type { WALUrl } from '../../../util';
import { createDeliberation } from '../../../publish';

const dispatch = createEventDispatcher();

let attachmentsDialog : AttachmentsDialog
let title: string = '';
let description: string = '';
let settings: any = {scoring: 'classic'};
let attachments: Array<WALUrl> = [];
let discussionAttachments: Array<WALUrl> = [];
let errorSnackbar: Snackbar;
// let threadsInfos: HoloHashMap<EntryHash, AppletInfo> = new HoloHashMap
let discussionApps = {};
let selectedDiscussionApp: string = "none";
let attachmentTypes = [];
let discussionWALUrl: WALUrl = null;

$: title, description, settings, attachments, discussionApps;
$: isDeliberationValid = true && title !== '' && settings !== '';

let weClient;
weClientStored.subscribe(value => {
  weClient = value;
});

onMount(async() => {

    // attachmentTypes = Array.from(weClient.attachmentTypes.entries())
    // console.log(attachmentTypes)
    // console.log(discussionApps)
    // for (const [appletHash, record] of attachmentTypes) {
    //   console.log(record)
    //   let appletInfo = await weClient.appletInfo(appletHash)
    //   if (!discussionApps[appletInfo.appletName]) {
    //     console.log(record)
    //     const recordTypeNames = Object.keys(record).map((key) => JSON.stringify({
    //       app: appletInfo.appletName,
    //       type: key
    //     }))
    //     recordTypeNames.forEach((recordTypeName) => {
    //       const recordType = JSON.parse(recordTypeName).type
    //       console.log(recordTypeName)
    //       if (appletInfo.appletName.includes("threads")) {
    //         console.log("appletInfo", appletInfo)
    //         console.log("record", record)
    //         discussionApps[recordTypeName] = [appletHash, record[recordType]]
    //         selectedDiscussionApp = recordTypeName;
    //       } else {
    //         discussionApps[recordTypeName] = [appletHash, record[recordType]]
    //       }
    //     })
    //   }
    // }
    discussionApps = {...discussionApps}
    // console.log("discussionApps", discussionApps)
    // console.log(selectedDiscussionApp)
});

async function submitDeliberation() {
  const deliberationEntry: Deliberation = { 
    title: title!,
    description: description!,
    settings: JSON.stringify(settings!),
    attachments: attachments,
    discussion: discussionAttachments[0] ? discussionAttachments[0] : null,
  };
  let newDeliberationHash = await createDeliberation(deliberationEntry, client)
  navigate("deliberation", newDeliberationHash)
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <h1>New Deliberation</h1>
  
  <!-- <h2>Deliberation Details</h2> -->
  

  <div style="margin-bottom: 16px; text-align: left;">
    <mwc-textfield style="width: 100%" outlined label="Title" value={ title } on:input={e => { title = e.target.value;} } required></mwc-textfield>          
  </div>
            
  <div style="margin-bottom: 16px; text-align: left;">
    <mwc-textarea style="width: 100%; height: 30vh" outlined label="Description" value={ description } on:input={e => { description = e.target.value; } }></mwc-textarea>          
  </div>
            
  <!-- <div style="margin-bottom: 16px; text-align: left">
    <mwc-formfield label="Binary support">
      <mwc-radio class="scoring-radio" name="group" value="weighted" on:change={(e) => {console.log(e.target.value)}}></mwc-radio>
    </mwc-formfield>
    <mwc-formfield label="Slider support">
      <mwc-radio name="group" value="classic" on:change={(e) => {console.log(e.target.value)}}></mwc-radio>
    </mwc-formfield>
  </div> -->

  {#if isWeContext()}
    {#if false}
    <div style="display:flex; flex-wrap:wrap; align-items: center; margin-bottom:10px;">
      <!-- {JSON.stringify(discussionApps[0])} -->
      <!-- dropdown selector for discussion apps -->
      <label>Discussion type:&nbsp;</label>
      <select value={selectedDiscussionApp} on:change={(e)=>{
        selectedDiscussionApp = e.target.value
        console.log("selectedDiscussionApp", selectedDiscussionApp)
      }} style="margin-right:10px;">
        {#each Object.keys(discussionApps) as appletName}
          {@const [aType, appletInfo] = discussionApps[appletName]}
          {@const formattedName = JSON.parse(appletName).app + ": " + JSON.parse(appletName).type}
          <!-- <option>hi</option> -->
          <option value={appletName}>{formattedName}</option>
        {/each}
        <option value="none">No discussion</option>
      </select>
    </div>
    {/if}
    <div style="display:flex; flex-wrap:wrap; align-items: center; margin-bottom:10px;">
      <label style="margin-top:5px">Link a discussion (e.g. a Vines thread)&nbsp;</label>
      <AttachmentsDialog bind:this={attachmentsDialog} attachmentsLimit={1} bind:attachments={discussionAttachments} 
      on:add-attachments={
        (e) => {
          // console.log("add-attachments", e.detail)
          discussionWALUrl = weaveUrlFromWal(e.detail.attachments[0])
          // console.log(discussionWALUrl)
          // props.attachments = e.detail.attachments
          // bind.refresh()
        }
      }></AttachmentsDialog>

      <label style="margin-top:5px">Attachments &nbsp;
      </label>
      <!-- <div style="margin-left:10px; margin-right:10px;">
        <button class="attachment-button" on:click={()=>attachmentsDialog.open()} >          
          <SvgIcon icon="link" size="16px"/>
        </button>
      </div> -->
      <!-- {#if props.attachments}
        <AttachmentsList attachments={props.attachments}
          on:remove-attachment={(e)=>removeAttachment(e.detail)}/>
      {/if} -->
    <!-- <AttachmentsBind on:add-binding={
      (e) => {
        console.log("add-attachments", e.detail)
      }
    }></AttachmentsBind> -->
    
    <AttachmentsDialog bind:this={attachmentsDialog} bind:attachments on:add-attachments={
      (e) => {
        // console.log("add-attachments", e.detail)
        attachments = e.detail.attachments
        // props.attachments = e.detail.attachments
        // bind.refresh()
      }
    }></AttachmentsDialog>
    </div>
  {/if}
  
  <label class="instructions">Warning: After creating a deliberation, it belongs to everyone and cannot be edited or deleted.</label>

  <mwc-button
    raised
    label="Create Deliberation"
    disabled={!isDeliberationValid}
    on:click={() => {
      submitDeliberation()
    }}
  ></mwc-button>
</div>
