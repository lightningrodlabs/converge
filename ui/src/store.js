import { encodeHashToBase64 } from '@holochain/client';
import { writable } from 'svelte/store';
import { decode } from '@msgpack/msgpack';

export const view = writable("home");
export const viewHash = writable(new Uint8Array([]));
export const weClientStored = writable(null);
export const allDeliberations = writable([])
export const allCriteria = writable([])
export const allProposals = writable({})
export const allOutcomes = writable([])
export const allEvaluations = writable({})

export function navigate(location, hash) {
    view.update(v => location);
    viewHash.update(h => hash)
}

export function setWeaveClient(client) {
    weClientStored.update(v => client);
}

export function setAllDeliberations(deliberations) {
    console.log("Setting all deliberations", deliberations);
    allDeliberations.update(v => deliberations);
}

export function updateDeliberation(updatedDeliberation) {
    console.log("updateDeliberation called with:", updatedDeliberation);
    allDeliberations.update(v => {
        let found = false;
        let newDeliberations = v.map(d => {
            if (encodeHashToBase64(d.action_hash) === encodeHashToBase64(updatedDeliberation.record.signed_action.hashed.hash)) {
                found = true;
                console.log("Found matching deliberation, updating proposals from", d.proposals.length, "to", updatedDeliberation.proposals.length);
                return {
                    ...d,
                    proposals: updatedDeliberation.proposals.map(link => link.target),
                    criteria: updatedDeliberation.criteria.map(link => link.target),
                    outcomes: updatedDeliberation.outcomes.map(link => link.target),
                };
            }
            return d;
        });
        if (!found) {
            newDeliberations.push({
                action_hash: updatedDeliberation.record.signed_action.hashed.hash,
                deliberation: decode(updatedDeliberation.record.entry.Present.entry),
                proposals: updatedDeliberation.proposals.map(link => link.target),
                criteria: updatedDeliberation.criteria.map(link => link.target),
                outcomes: updatedDeliberation.outcomes.map(link => link.target),
                deliberators: [],
            });
        }
        return newDeliberations;
    });
}

export function addDeliberation(deliberation) {
    allDeliberations.update(v => [...v, deliberation]);
}

export function setAllCriteria(criteria) {
    allCriteria.update(v => criteria);
}

// export function setAllProposals(proposals) {
//     allProposals.update(v => proposals);
// }

export function addSomeProposals(proposals) {
    allProposals.update(v => {
        let newProposals = {...v};
        for (let proposal of proposals) {
            newProposals[encodeHashToBase64(proposal.action_hash)] = proposal;
        }
        return newProposals;
    });
}

export function addProposalHashesToDeliberation(deliberationHash, proposalHashes) {
    console.log("Adding proposal hashes to deliberation 0", deliberationHash, proposalHashes);
    allDeliberations.update(v => {
        let newDeliberations = v.map(d => {
            if (encodeHashToBase64(d.action_hash) === encodeHashToBase64(deliberationHash)) {
                return {
                    ...d,
                    proposals: [...proposalHashes, ...d.proposals]
                };
            }
            return d;
        });
        console.log("New deliberations", newDeliberations);
        return newDeliberations;
    });
}

export function addSomeEvaluations(evaluations) {
    console.log("Adding some evaluations", evaluations  );
    allEvaluations.update(v => {
        let newEvaluations = {...v};
        for (let evaluation of evaluations) {
            let encodedProposalHash = evaluation.base_proposal_hash
            if (newEvaluations[encodedProposalHash]) {
                let existingEvaluation = newEvaluations[encodedProposalHash].find(e => (e.target_evaluator === evaluation.target_evaluator));
                if (existingEvaluation) {
                    // remove existing evaluation
                    newEvaluations[encodedProposalHash] = newEvaluations[encodedProposalHash].filter(e => e.target_evaluator !== evaluation.target_evaluator);
                    newEvaluations[encodedProposalHash].push(evaluation);
                } else {
                    newEvaluations[encodedProposalHash].push(evaluation);
                }
            } else {
                newEvaluations[encodedProposalHash] = [evaluation];
            }
        }
        console.log("New evaluations", newEvaluations);
        return newEvaluations;
    });
}

export function setAllOutcomes(outcomes) {
    allOutcomes.update(v => outcomes);
}