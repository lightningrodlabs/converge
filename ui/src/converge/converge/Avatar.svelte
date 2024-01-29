<script lang="ts">
import { encodeHashToBase64, type AgentPubKey } from "@holochain/client";
import "@holochain-open-dev/profiles/dist/elements/agent-avatar.js";
import { getContext } from "svelte";
import type { ProfilesStore } from "@holochain-open-dev/profiles";
import SvgIcon from "./SvgIcon.svelte";
import { profilesStoreContext } from '../../contexts';

let profilesStore: ProfilesStore = (getContext(profilesStoreContext) as any).getProfileStore();

export let agentPubKey: AgentPubKey
export let size = 32
export let namePosition = "row"
export let nameColor = "rgba(86, 94, 109, 1.0)"
export let showAvatar = true
export let showNickname = true
export let placeholder = false

$: agentPubKey
$: agentPubKeyB64 = encodeHashToBase64(agentPubKey)
$: profile = profilesStore.profiles.get(agentPubKey)
$: nickname = $profile.status=="complete" && $profile.value ? $profile.value.entry.nickname : agentPubKeyB64.slice(5,9)+"..."

</script>

<div class="avatar-{namePosition}"
    >
    {#if $profile.status == "pending"}
    ( ? )
    {:else if $profile.status == "complete"}
        <!-- {JSON.stringify($profile.value.entry.fields.avatar)} -->

        {#if showAvatar}
            {#if placeholder && !$profile.value.entry.fields.avatar}
                <SvgIcon color="#fff" icon=faUser size="" style="margin-left:5px;margin-right:5px"></SvgIcon>
            {:else}
            <!-- <div title={nickname}> -->
                <!-- <SvgIcon color="black" icon=faUser size="" style="margin-left:5px;margin-right:5px"></SvgIcon> -->
                <agent-avatar title={nickname} disable-tooltip={true} disable-copy={true} size={size} agent-pub-key="{agentPubKeyB64}"></agent-avatar>
            <!-- </div> -->
            {/if}
        {/if}
        {#if showNickname}
            <div class="nickname" style="color: {nameColor}">{ nickname }</div>
        {/if}
    {/if}
</div>

<style>
    .avatar-column {
        align-items: center;
        display:flex;
        flex-direction: column;
    }
    .avatar-row {
        display:inline-flex;
        flex-direction: row;
        justify-content:center;
        position: relative;
        height: 100%;
        align-items: center;
    }
    .avatar-row .nickname{
        margin-left: 0.5em;
    }
</style>