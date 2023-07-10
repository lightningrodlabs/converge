<script lang="ts">
import { createEventDispatcher, getContext, onMount } from 'svelte';
import type { AppAgentClient, Record, EntryHash, AgentPubKey, ActionHash, DnaHash } from '@holochain/client';
import { clientContext } from '../../contexts';
import type { Criterion, CreateCriterionInput } from './types';
import '@material/mwc-button';
import '@material/mwc-snackbar';
import type { Snackbar } from '@material/mwc-snackbar';
import '@material/mwc-textfield';

let client: AppAgentClient = (getContext(clientContext) as any).getClient();

const dispatch = createEventDispatcher();

export let deliberationHash: ActionHash;

let title: string = '';

let errorSnackbar: Snackbar;

$: title;
$: isCriterionValid = true && title !== '';

onMount(() => {
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

  try {
    let record = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'add_criterion_for_supporter',
      payload: {
        base_supporter: client.myPubKey,
        target_criterion_hash: criterionHash,
        percentage: "1"
      },
    });
    if (record) {
      console.log(record)
    }
  } catch (e) {
    console.log(e);
  }
}

</script>
<mwc-snackbar bind:this={errorSnackbar} leading>
</mwc-snackbar>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create Criterion</span>
  

  <div style="margin-bottom: 16px">
    <mwc-textfield outlined label="Title" value={ title } on:input={e => { title = e.target.value; } } required></mwc-textfield>          
  </div>
            

  <mwc-button 
    raised
    label="Create Criterion"
    disabled={!isCriterionValid}
    on:click={() => createCriterion()}
  ></mwc-button>
</div>
