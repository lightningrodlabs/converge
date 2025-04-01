import { decode } from "@msgpack/msgpack";
import { encodeHashToBase64 } from "@holochain/client";
import { setAllDeliberations, addSomeProposals, addSomeEvaluations } from "./store";

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

    deliberations.forEach(d => {
      d.proposals = d.proposals.reverse()
    })
    setAllDeliberations(deliberations.reverse())
  } catch (e) {
    console.log("error", e)
  }
}

export async function refetchDeliberation(deliberationHash, client) {
  return "Unimplemented"
}


interface Proposal { 
  title: string;
  description: string;
  attachments: string[]
}

export async function refetchProposalsForDeliberation(deliberationHash, client) {
  try {
    const proposals = await client.callZome({
      cap_secret: null,
      role_name: 'converge',
      zome_name: 'converge',
      fn_name: 'get_proposals_for_deliberation',
      payload: deliberationHash,
    });
    addSomeProposals(proposals.map(p => (
      {...decode((p.entry as any).Present.entry) as Proposal,
        action_hash: p.signed_action.hashed.hash,
      }
    )))
  } catch (e) {
    console.log("error", e)
  }
}

export async function refetchEvaluationsForProposals(proposalHashes, client) {
  proposalHashes.forEach(async proposalHash => {
    try {
      console.log("proposalHash", proposalHash)
      const evaluations = await client.callZome({
        cap_secret: null,
        role_name: 'converge',
        zome_name: 'converge',
        fn_name: 'get_evaluators_for_proposal',
        payload: proposalHash,
      });
      console.log("raw evaluations", evaluations)
      let formattedEvaluations = evaluations.map(e => ({
        base_proposal_hash: encodeHashToBase64(e.base),
        target_evaluator: encodeHashToBase64(e.author),
        tag: decode(e.tag) as String,
      }))

      console.log("evaluations", formattedEvaluations)
      addSomeEvaluations(formattedEvaluations)
    } catch (e) {
      console.log("error", e)
    }
  })
}