<script lang="ts">
  import { onMount, setContext } from 'svelte';
  import type { NetworkInfoRequest } from '@holochain/client';

  export let WeaveClient;
  let networkInfo;
  $: networkInfo;

  async function refresh() {
    console.log("1")
    // let x = WeaveClient.renderInfo
    console.log(WeaveClient?.renderInfo.appletClient.cachedAppInfo.cell_info.converge[0].provisioned.cell_id[0])
    console.log(WeaveClient?.renderInfo.appletClient.cachedAppInfo.cell_info.converge.map((cell) => cell.provisioned.cell_id[0]))
    let networkInfoRequest: NetworkInfoRequest = {
      agent_pub_key: WeaveClient?.renderInfo.appletClient.myPubKey,
      dnas: WeaveClient?.renderInfo.appletClient.cachedAppInfo.cell_info.converge.map((cell) => cell.provisioned.cell_id[0])
    }
    console.log(networkInfoRequest)
    let x = await WeaveClient?.renderInfo.appletClient.appWebsocket.networkInfo(networkInfoRequest)
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