import { encodeHashToBase64 } from '@holochain/client';
import { writable } from 'svelte/store';

export const view = writable("home");
export const viewHash = writable(new Uint8Array([]));
export const weClientStored = writable(null);
export const allDeliberations = writable([])
export const allCriteria = writable([])
export const allProposals = writable([])
export const allOutcomes = writable([])

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

export function setAllCriteria(criteria) {
    allCriteria.update(v => criteria);
}

export function setAllProposals(proposals) {
    allProposals.update(v => proposals);
}

export function setAllOutcomes(outcomes) {
    allOutcomes.update(v => outcomes);
}