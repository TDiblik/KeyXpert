<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { onMount } from "svelte";

  import ProfileDetails from './components/ProfileDetails.svelte';
  import type { ServiceConfig } from "./models";
  
  let initial_load = true;
  let service_config: ServiceConfig | undefined = undefined;
  onMount(async () => {
    service_config = await invoke("get_service_config", {});
    // TODO: actively used profile should get selected when app is opened ?? 
    initial_load = false;
    console.log(service_config)
  });

  let selected_profile_id: string | undefined = undefined;
  async function create_profile() {
    selected_profile_id = await invoke("create_profile", {});
    service_config = await invoke("get_service_config", {});
  }
</script>

{#if initial_load}
<h1>Loading...</h1>
{:else}
<main class="container">
  <div class="header-row">
    <h1 class="header">KeyXpert</h1>
    <div class="advanced-settings-button-wrapper">
      <button class="btn primary">Advanced settings</button>
    </div>
  </div>
  
  <div class="profile-selection-row">
    <select id="profile-selector" bind:value={selected_profile_id}>
      <option></option>
      {#each service_config?.profiles as profile}
        <option value={profile.id}>{profile.name}</option>
      {/each}
    </select>

    <div class="profile-actions">
      <button class="btn primary" on:click={create_profile}>Add profile</button>
      <button class="btn delete">Delete profile</button>
    </div>
  </div>
  
  {#if selected_profile_id != undefined && selected_profile_id.length > 0}
    {#key selected_profile_id}
      <ProfileDetails 
        active_profile={service_config.active_profile}
        selected_profile={
          service_config.profiles.find(s => s.id === selected_profile_id)
        } 
      />
    {/key}
  {/if}
</main>
{/if}
