<script lang="ts">
  import { onMount, getContext } from 'svelte';
  import { clientContext } from '../../../contexts';
  import type { ActionHash, AppAgentClient } from "@holochain/client";

  export let alternative;
  export let mySupport;
  export let criterionHash;

  let client: AppAgentClient = (getContext(clientContext) as any).getClient();

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

      console.log(record1)
      console.log(tag)
      console.log(alternative)
      console.log(alternativeHash)

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
      console.log(record2)

    } catch (e) {
      console.log(e)
    }
  }
</script>

<div><div class="green-alert">↯</div> <span style="font-weight: bold; color: green;">Alternative: </span>{alternative.title}
{#if mySupport}
<button on:click={() => acceptAlternative(alternative.hash)}>Transfer support</button>
{/if}
</div>