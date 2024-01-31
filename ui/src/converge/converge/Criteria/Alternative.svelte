<script lang="ts">
  import { onMount, getContext, createEventDispatcher } from 'svelte';
  import { clientContext } from '../../../contexts';
  import type { ActionHash, AppAgentClient } from "@holochain/client";

  export let alternative;
  export let mySupport;
  export let criterionHash;
  export let showPopupModal = false;
  export let criterion;

  let client: AppAgentClient = (getContext(clientContext) as any).getClient();
  const dispatch = createEventDispatcher();

  async function acceptAlternative(alternativeHash: ActionHash) {
    try {
      const tag = {
        percentage: mySupport,
        transferedFrom: criterionHash
      }

      const record1 = await client.callZome({
        cap_secret: null,
        role_name: 'converge',
        zome_name: 'converge',
        fn_name: 'remove_criterion_for_supporter',
        payload: {
          base_supporter: client.myPubKey,
          target_criterion_hash: criterionHash,
        },
      });

      // console.log(record1)
      // console.log(tag)
      // console.log(alternative)
      // console.log(alternativeHash)

      const record2 = await client.callZome({
        cap_secret: null,
        role_name: 'converge',
        zome_name: 'converge',
        fn_name: 'add_criterion_for_supporter',
        payload: {
          base_supporter: client.myPubKey,
          target_criterion_hash: alternativeHash,
          tag: String(JSON.stringify(tag)),
        },
      });
      // console.log(record2)
      dispatch('transfer', { from: criterionHash, to: alternativeHash});
      // popup to confirm
      showPopupModal = false;
    } catch (e) {
      console.log(e)
    }
  }
</script>

<div><div class="green-alert">↯</div> <span style="font-weight: bold; color: green;">Alternative: </span>{alternative.title}
{#if mySupport}
<!-- <button on:click={() => acceptAlternative(alternative.hash)}>Transfer support</button> -->
<button on:click={() => showPopupModal=true}>Transfer support</button>
{/if}
</div>

{#if showPopupModal}
  <div class="popup-modal">
    <div class="popup-modal-content">
      <!-- <div class="popup-modal-header">
        <h2>Transfer support</h2>
        <button on:click={() => showPopupModal=false}>X</button>
      </div> -->
      <!-- <div class="popup-modal-body"> -->
        <p>Transfer support from the criterion to the alternative?</p>
        <div style="text-align: center">
        <p><span style="font-weight: bold;">Original: </span>{criterion.title}</p>
        <!-- down arrow -->
          <div style="border: solid black; border-width: 0 3px 3px 0; display: inline-block; padding: 3px; transform: rotate(45deg); -webkit-transform: rotate(45deg);"></div>
          <p><span style="font-weight: bold;">Alternative: </span>{alternative.title}</p>
        </div>
        <div class="popup-modal-buttons">
          <mwc-button outlined on:click={() => showPopupModal=false}>Cancel</mwc-button>&nbsp;&nbsp;
          <mwc-button raised on:click={() => acceptAlternative(alternative.hash)}>Transfer support</mwc-button>
        </div>
      <!-- </div> -->
    </div>
  </div>
{/if}

<style lang="scss">
  .green-alert {
    font-size: 20px;
    font-weight: bold;
    color: green;
    margin-right: 10px;
  }
  .popup-modal {
    position: fixed;
    z-index: 1;
    left: 0;
    top: 0;
    width: 100%;
    height: 100%;
    overflow: auto;
    background-color: rgb(0,0,0);
    background-color: rgba(0,0,0,0.4);
    text-align: center;
  }
  .popup-modal-content {
    background-color: #fefefe;
    margin: 15% auto;
    padding: 20px;
    border: 1px solid #888;
    width: fit-content;
  }
  .popup-modal-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }
  .popup-modal-header h2 {
    margin: 0;
  }
  .popup-modal-header button {
    background: none;
    border: none;
    font-size: 20px;
    font-weight: bold;
    cursor: pointer;
  }
  .popup-modal-body {
    margin-top: 20px;
  }
  .popup-modal-buttons {
    display: flex;
    justify-content: center;
    margin-top: 20px;
  }
  .popup-modal-buttons button {
    background: none;
    border: none;
    font-size: 16px;
    font-weight: bold;
    cursor: pointer;
    margin-left: 10px;
  }
</style>