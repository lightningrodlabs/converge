import { encodeHashToBase64 } from '@holochain/client';
import { writable } from 'svelte/store';

export const view = writable("home");
export const viewHash = writable(new Uint8Array([]));
export const notifications = writable([]);
export const weClientStored = writable(null);
export const viewed = writable([]);

export function navigate(location, hash) {
    view.update(v => location);
    viewHash.update(h => hash)
}

export function notifications_update(new_notifications) {
    let ordered = new_notifications.sort((a, b) => parseFloat(b.timestamp) - parseFloat(a.timestamp));
    notifications.update(v => ordered)
}

export function setWeClient(client) {
    weClientStored.update(v => client);
}

export function setViewed(hashes) {
    viewed.update(v => hashes);
}

async function createViewed(viewedHash, client) {  
    console.log("client", client)
    const viewedEntry = { 
        viewed_hash: viewedHash,
        viewed_date: new Date().getSeconds() * 1000,
    };
    
    // console.log("viewedEntry", viewedEntry)
    
    try {
        await client.callZome({
            cap_secret: null,
            role_name: 'converge',
            zome_name: 'converge',
            fn_name: 'create_viewed',
            payload: viewedEntry,
        });
    } catch (e) {
        console.log(e)
    }
}

export function addToViewed(hash, client) {
    let alreadyViewed = checkIfViewed(encodeHashToBase64(hash));
    if (alreadyViewed) {
        return;
    }
    viewed.update(v => {
        v.push(encodeHashToBase64(hash));
        return v;
    });
    createViewed(hash, client);
}

export function checkIfViewed(hash) {
    let isViewed;
    viewed.subscribe(v => {
        isViewed = v.includes(hash);
    })();
    return isViewed;
}

export function countViewed(hashes) {
    let count = 0;
    hashes.forEach(hash => {
        let viewed = checkIfViewed(encodeHashToBase64(hash));
        if (viewed) {
            count++;
        }
    });
    return count;
}