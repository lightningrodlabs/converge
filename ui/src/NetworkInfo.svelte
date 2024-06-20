<script lang="ts">
  import { onMount, setContext } from 'svelte';
  import type { NetworkInfoRequest } from '@holochain/client';
  // import { ReadonlyPeerStatusStore } from '@lightningrodlabs/we-applet'
  import { StoreSubscriber } from '@holochain-open-dev/stores';

  export let weClient;
  let networkInfo;
  $: networkInfo;
  // let peerStatusStore: ReadonlyPeerStatusStore;

  async function refresh() {
    let networkInfoRequest: NetworkInfoRequest = {
      agent_pub_key: weClient?.renderInfo.appletClient.myPubKey,
      dnas: weClient?.renderInfo.appletClient.cachedAppInfo.cell_info.converge.map((cell) => cell.provisioned.cell_id[0])
    }
    let x = await weClient?.renderInfo.appletClient.networkInfoRequester(networkInfoRequest)
    networkInfo = x
  }

  onMount(async () => {
    await refresh()
  });
</script>

<button
  on:click={
    () =>
    {
      refresh()
    }
  }
>refresh network</button>
<!-- --{JSON.stringify(ReadablePeerStatusStore)}-- -->
<br>
--{JSON.stringify(networkInfo)}--