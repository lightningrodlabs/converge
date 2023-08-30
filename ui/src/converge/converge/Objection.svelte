<script lang="ts">
  import { onMount, getContext } from 'svelte';
  import { clientContext } from '../../contexts';
  import type { ActionHash, AppAgentClient } from "@holochain/client";

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

{#if objection}
<div><div class="red-alarm">!</div> <span style="font-weight: bold; color: red;">Objection: </span>{objection.comment}</div>
{:else}
.........
{/if}