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
import { WeClient, isWeContext, initializeHotReload, type HrlB64WithContext, type Hrl } from '@lightningrodlabs/we-applet';
import type { AppletInfo, AttachmentType } from "@lightningrodlabs/we-applet";
import AttachmentsBind from '../../../AttachmentsBind.svelte';
import { HoloHashMap, type EntryHashMap } from "@holochain-open-dev/utils";
let client: AppAgentClient = (getContext(clientContext) as any).getClient();
import { weClientStored } from '../../../store.js';
import app from '../../../main';
import { hrlB64WithContextToRaw, hrlWithContextToB64, getMyDna } from '../../../util';

const dispatch = createEventDispatcher();

let attachmentsDialog : AttachmentsDialog
let title: string = '';
let description: string = '';
let settings: any = {scoring: 'classic'};
let attachments: Array<HrlB64WithContext> = [];
let errorSnackbar: Snackbar;
// let threadsInfos: HoloHashMap<EntryHash, AppletInfo> = new HoloHashMap
let discussionApps = {};
let selectedDiscussionApp: string = "none";
let attachmentTypes = [];
let dna;

$: title, description, settings, attachments, discussionApps;
$: isDeliberationValid = true && title !== '' && settings !== '';

let weClient;
weClientStored.subscribe(value => {
  weClient = value;
});

onMount(async() => {
    dna = await getMyDna("converge", client)

    attachmentTypes = Array.from(weClient.attachmentTypes.entries())
    console.log(attachmentTypes)
    console.log(discussionApps)
    for (const [appletHash, record] of attachmentTypes) {
      console.log(record)
      let appletInfo = await weClient.appletInfo(appletHash)
      if (!discussionApps[appletInfo.appletName]) {
        console.log(record)
        const recordTypeNames = Object.keys(record).map((key) => JSON.stringify({
          app: appletInfo.appletName,
          type: key
        }))
        recordTypeNames.forEach((recordTypeName) => {
          const recordType = JSON.parse(recordTypeName).type
          console.log(recordTypeName)
          if (appletInfo.appletName.includes("threads")) {
            console.log("appletInfo", appletInfo)
            console.log("record", record)
            discussionApps[recordTypeName] = [appletHash, record[recordType]]
            selectedDiscussionApp = recordTypeName;
          } else {
            discussionApps[recordTypeName] = [appletHash, record[recordType]]
          }
        })
      }
    }
    discussionApps = {...discussionApps}
    console.log("discussionApps", discussionApps)
    console.log(selectedDiscussionApp)
});

async function createDeliberation() {  
  const deliberationEntry: Deliberation = { 
    title: title!,
    description: description!,
    settings: JSON.stringify(settings!),
    attachments: attachments.map(a => {
      return {
        hrl: JSON.stringify(a.hrl),
        context: a.context
      }
    }),
    // discussionApp: selectedDiscussionApp.appletName
  };

  console.log("createDeliberation", deliberationEntry)
  
  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'create_deliberation',
      payload: deliberationEntry,
    });
    dispatch('deliberation-created', { deliberationHash: record.signed_action.hashed.hash });

    // join deliberation
    await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'add_deliberation_for_deliberator',
      payload: {
        base_deliberator: client.myPubKey,
        target_deliberation_hash: record.signed_action.hashed.hash
      },
    });

    if (selectedDiscussionApp != "none") {
      // add discussion
      const appletHash = discussionApps[selectedDiscussionApp][0]
      const appletInfo = await weClient.appletInfo(appletHash)
      console.log("appletInfo2", appletInfo)
      const aType = discussionApps[selectedDiscussionApp][1]
      console.log("aType", aType)
      const threadsCreate = {hrl:[dna, record.signed_action.hashed.hash], attachmentType:"{}"}
      console.log("threadsCreate", threadsCreate)
      const hrlWithContext = await aType.create(threadsCreate)
      console.log("hrl", hrlWithContext)
      console.log("b64", hrlWithContextToB64(hrlWithContext))
      
      const hrlB64 = hrlWithContextToB64(hrlWithContext)
      const deliberationUpdate = {
        original_deliberation_hash: record.signed_action.hashed.hash,
        previous_deliberation_hash: record.signed_action.hashed.hash,
        updated_deliberation: {
          ...deliberationEntry,
          discussion: {
            hrl: JSON.stringify(hrlB64.hrl),
            context: hrlB64.context,
          }
        }
      }
      
      console.log("deliberationUpdate", deliberationUpdate)
      
      // update deliberation to include newly created discussion
      await client.callZome({
        cap_secret: null,
        role_name: 'converge',
        zome_name: 'converge',
        fn_name: 'update_deliberation',
        payload: deliberationUpdate,
      });
    }
    
    navigate("deliberation", record.signed_action.hashed.hash)
  } catch (e) {
    console.log(e)
    // errorSnackbar.labelText = `Error creating the deliberation: ${e}`;
    // errorSnackbar.show();
  }
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

  {#if isWeContext}
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
      <label>Attachments &nbsp;
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
        console.log("add-attachments", e.detail)
        attachments = e.detail.attachments
        // props.attachments = e.detail.attachments
        // bind.refresh()
      }
    }></AttachmentsDialog>
    </div>
  {/if}
  
  <label class="instructions">Warning: you will not be able to edit the above details after creating.</label>

  <mwc-button
    raised
    label="Create Deliberation"
    disabled={!isDeliberationValid}
    on:click={() => createDeliberation()}
  ></mwc-button>
</div>
