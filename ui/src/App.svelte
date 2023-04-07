<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { onMount } from "svelte";
  import type { ProfileDetailsInfo, ServiceConfig } from "./models";
  import { add_padding_to_keycode_array, handle_tauri_result } from "./utils";
  import ProfileDetails from './components/ProfileDetails.svelte';
  import Modal from "./components/Modal/Modal.svelte";
  import type { ModalProps } from "./components/Modal/ModalStore";
  import { modal_info } from "./components/Modal/ModalStore";
  
  let initial_load = true;
  let service_config: ServiceConfig | undefined = undefined;
  async function update_service_config() {
    handle_tauri_result<ServiceConfig>(await invoke("get_service_config", {}), (result) => {
      service_config = result;
    });
  }

  let selected_profile_id: string | undefined = undefined;
  onMount(async () => {
    await update_service_config();
    selected_profile_id = service_config.active_profile;
    initial_load = false;
  });

  async function create_profile() {
    handle_tauri_result<string>(await invoke("create_profile", {}), (result) => {
      selected_profile_id = result;
    });
    await update_service_config();
  }
  
  async function delete_profile() {
    handle_tauri_result<void>(await invoke("delete_profile", { idToDelete: selected_profile_id }));
    await update_service_config();
    selected_profile_id = undefined;
  }
  
  async function save_profile(profile: ProfileDetailsInfo) {
    // Shortcut remap from/to requires array of 4 elements.
    for (const shortcut_remap of profile.shortcut_remaps) {
      add_padding_to_keycode_array(shortcut_remap.from_shortcut_holding_keys);
      add_padding_to_keycode_array(shortcut_remap.to_shortcut_holding_keys);
    }

    if (handle_tauri_result<void>(await invoke("save_profile", { profile: profile }))) {
      modal_info.set({
        title: "Successfully saved profile",
        description: "Your profile changes should be have been successfully written into config file.",
        type: "info"
      } as ModalProps);
    }
    await update_service_config();
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
        {@const is_active_profile = service_config?.active_profile === profile.id }
        <option class={`${is_active_profile ? "active-profile" : ""}`} value={profile.id}>{profile.name}</option>
      {/each}
    </select>

    <div class="profile-actions">
      <button class="btn primary" on:click={create_profile}>Add profile</button>
      <button class="btn delete" on:click={delete_profile}>Delete profile</button>
    </div>
  </div>
  
  {#key selected_profile_id}
    {#if selected_profile_id != undefined && selected_profile_id.length > 0}
        {@const selected_profile = service_config?.profiles.find(s => s.id === selected_profile_id)}
        {#if selected_profile != null}
          <ProfileDetails 
            active_profile={service_config.active_profile}
            selected_profile={selected_profile} 
            on_save={save_profile}
          />
      {/if}
    {/if}
  {/key}
</main>

<Modal />
{/if}
