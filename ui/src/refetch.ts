import { encodeHashToBase64 } from "@holochain/client";
import { setAllDeliberations } from "./store";

export async function refetchDeliberations(client) {
  try {
    const deliberations = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'get_all_deliberations_complete',
      payload: null,
    });
    console.log("records", deliberations)
    // let criteria = deliberations.criteria?.map(hash => {encodeHashToBase64(hash)})
    // let deliberators = deliberations.deliberators?.map(hash => {encodeHashToBase64(hash)})
    // let outcomes = deliberations.outcomes?.map(hash => {encodeHashToBase64(hash)})

    setAllDeliberations(deliberations.reverse())
  } catch (e) {
    console.log("error", e)
  }
}