<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Criterion, CreateCriterionInput } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textfield';

export let criterionFormPopup; // Prop to control popup visibility
function dismissPopup() {
  criterionFormPopup = false; // Set active to false to hide the popup
}

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let deliberationHash: ActionHash;

let title: string = '';

let errorSnackbar: Snackbar;

$: title, criterionFormPopup;
$: isCriterionValid = true && title !== '';

onMount(() => {
    // // Add an event listener to the backdrop to dismiss the popup when clicked outside
    // function handleOutsideClick(event) {
    //   if (criterionFormPopup && !event.target.closest('.popup-container')) {
    //     dismissPopup();
    //   }
    // }

    // document.addEventListener('click', handleOutsideClick);

    // // Clean up the event listener when the component is destroyed
    // return () => {
    //   document.removeEventListener('click', handleOutsideClick);
    // };
  });

async function createCriterion() {  
  const criterionEntry: CreateCriterionInput = { 
    criterion: {
      title: title!,
    },
    deliberation: deliberationHash,
  };
  
  let criterionHash;

  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'create_criterion',
      payload: criterionEntry,
    });
    criterionHash = record.signed_action.hashed.hash;
    dispatch('criterion-created', { criterionHash: record.signed_action.hashed.hash });
  } catch (e) {
    errorSnackbar.labelText = `Error creating the criterion: ${e.data.data}`;
    errorSnackbar.show();
  }

  // try {
  //   let record = await client.callZome({
  //     cap_secret: null,
  //     role_name: 'converge',
  //     zome_name: 'converge',
  //     fn_name: 'add_criterion_for_supporter',
  //     payload: {
  //       base_supporter: client.myPubKey,
  //       target_criterion_hash: criterionHash,
  //       percentage: "1"
  //     },
  //   });
  //   if (record) {
  //     console.log(record)
  //   }
  // } catch (e) {
  //   console.log(e);
  // }
  dismissPopup()
}

</script>
{#if criterionFormPopup}
  <div class="backdrop">
    <div class="popup-container">

      <mwc-snackbar bind:this={errorSnackbar} leading>
      </mwc-snackbar>
      <div style="display: flex; flex-direction: column">
        <h2>Add a new criterion</h2>
        

        <div style="margin-bottom: 16px">
          <mwc-textarea style="width: 30vw; height: 20vh" outlined label="Title" value={ title } on:input={e => { title = e.target.value; } } required></mwc-textarea>          
        </div>
                  

        <div style="display: flex; flex-direction: row">
          <mwc-button
            outlined
            label="Cancel"
            on:click={() => dismissPopup()}
            style="flex: 1; margin-right: 16px"
          ></mwc-button>

          <mwc-button 
            style="display: inline-block"
            raised
            label="Create Criterion"
            disabled={!isCriterionValid}
            on:click={() => createCriterion()}
          ></mwc-button>
        </div>
        
      </div>
    </div>
  </div>
{/if}