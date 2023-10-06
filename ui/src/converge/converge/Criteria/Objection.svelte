<script lang="ts">
  import { onMount, getContext } from 'svelte';
  import { clientContext } from '../../../contexts';
  import type { ActionHash, AppAgentClient } from "@holochain/client";

  export let objections;
  export let objectionHash;
  let objection;

  let client: AppAgentClient = (getContext(clientContext) as any).getClient();

  onMount(async () => {
    objection = await fetchObjection(objectionHash)
  });

  async function fetchObjection(objection_hash) {
    try {
      let record = await client.callZome({
        cap_secret: null,
        role_name: 'converge',
        zome_name: 'converge',
        fn_name: 'get_objection_link',
        payload: objection_hash,
      });
      return record
    } catch (e: any) {
      console.log(e)
    }
  }

  $: objection;
</script>

<style>
  /* .red-alarm {
    background-color: black;
    color: red;
    display: inline-block;
    padding: 4px 10px;
    border-radius: 50%;
    font-weight: bolder;
  }
   */
</style>



{#if objections}
{#if objection}
{@const objectionStringToCheck = Object.values(objectionHash).join(',')}
{@const objectionIsPresent = objections.some(agentObj => Object.values(agentObj.objection_hash).join(',') === objectionStringToCheck)}
{#if objectionIsPresent}
  {@const agent = objections.find(agentObj => Object.values(agentObj.objection_hash).join(',') === objectionStringToCheck)}
  <!-- <div style="margin-right: 8px; cursor: pointer; color: gray; text-decoration: underline;" on:click={() => {removeObjection()}}>Remove objection</div> -->

  
  <div><div class="red-alarm">!</div> <span style="font-weight: bold; color: red;">Objection: </span>{objection.comment}</div>
  {:else}
  <div><span style="font-weight: bold; color: orange;">Objection (removed): </span>{objection.comment}</div>
  {/if}
{/if}
{/if}