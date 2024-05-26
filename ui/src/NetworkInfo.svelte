<script lang="ts">
  import { onMount, setContext } from 'svelte';
  import type { NetworkInfoRequest } from '@holochain/client';

  export let weClient;
  let networkInfo;
  $: networkInfo;

  async function refresh() {
    console.log("1")
    // let x = weClient.renderInfo
    console.log(weClient?.renderInfo.appletClient.cachedAppInfo.cell_info.converge[0].provisioned.cell_id[0])
    console.log(weClient?.renderInfo.appletClient.cachedAppInfo.cell_info.converge.map((cell) => cell.provisioned.cell_id[0]))
    let networkInfoRequest: NetworkInfoRequest = {
      agent_pub_key: weClient?.renderInfo.appletClient.myPubKey,
      dnas: weClient?.renderInfo.appletClient.cachedAppInfo.cell_info.converge.map((cell) => cell.provisioned.cell_id[0])
    }
    console.log(networkInfoRequest)
    let x = await weClient?.renderInfo.appletClient.appWebsocket.networkInfo(networkInfoRequest)
    console.log(x)

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
{JSON.stringify(networkInfo)}