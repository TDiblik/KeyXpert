<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { onMount } from "svelte";

  import ProfileDetails from './components/ProfileDetails.svelte';
  import type { ServiceConfig } from "./models";
  
  let initial_load = true;
  let service_config: ServiceConfig | undefined = undefined;
  onMount(async () => {
    service_config = await invoke("get_service_config", {});
    initial_load = false;
    console.log(service_config)
  });

  let selected_profile: string | undefined = undefined;
  async function create_profile() {
    selected_profile = await invoke("create_profile", {});
    service_config = await invoke("get_service_config", {});
    
    console.log(service_config);
    console.log(selected_profile);
  }
  
  $: console.log(selected_profile)
  
  function chage_profile() {
    console.log("lasje");
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
    <select id="profile-selector" bind:value={selected_profile}>
      {#each service_config?.profiles as profile}
        <option value={profile.id}>{profile.name}</option>
      {/each}
    </select>

    <div class="profile-actions">
      <button class="btn primary" on:click={create_profile}>Add profile</button>
      <button class="btn delete">Delete profile</button>
    </div>
  </div>
  
  {#if selected_profile != undefined}
    <ProfileDetails profile_id={selected_profile} />
  {/if}
</main>
{/if}
