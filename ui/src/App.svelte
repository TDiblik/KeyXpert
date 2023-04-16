<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { getVersion } from "@tauri-apps/api/app";
  import { arch } from "@tauri-apps/api/os";
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
  
  // https://docs.github.com/en/rest/releases/releases?apiVersion=2022-11-28#get-the-latest-release
  async function check_for_updates() {
    const req = await fetch("https://api.github.com/repos/TDiblik/KeyXpert/releases/latest"); // Gets latest release metadata (reference above)
    if (!req.ok) {
      const err_status = req.status;
      let err_text = "";
      try { err_text = JSON.stringify(await req.json()); } catch{}

      modal_info.set({
        type: "error",
        title: "Error occured :(",
        description: "Unable to get data about latest release. This error could either mean that Github is down, OR (the more realistic reason), I fucked up releases, so you should probably go onto my github and open issue/check for manual updates.",
        show_error_info: true,
        error: {
          url: req.url,
          status: err_status,
          status_err_text: err_text,
        },
      } as ModalProps);
      return;
    }
  
    const content = await req.json()
    let latest_release_version: string = content.tag_name; // Ensured by respose schema (reference above)
    if (latest_release_version.charAt(0) == 'v') {
      latest_release_version = latest_release_version.substring(1);
    }

    const app_version = await getVersion();
    if (app_version == latest_release_version) {
      modal_info.set({
        type: "info",
        title: "Everything up-to-date",
        description: `Latest released version is ${latest_release_version} and your current version is ${app_version}.`,
      } as ModalProps);
      return;
    }
    
    let system_archtecture: string = await arch();
    if (system_archtecture == "x86_64") {
      system_archtecture = "x64";
    }
    const expected_installer_name = `KeyXpert_${latest_release_version}_${system_archtecture}_en-US.msi`; // TODO: Match extension against platform
    const download_url = content.assets.find(s => s.name == expected_installer_name).browser_download_url // Ensured by respose schema (reference above)
    if (download_url == null) {
      modal_info.set({
        type: "error",
        title: "Unable to find installer for your architecture",
        description: `New version is available (${latest_release_version}), but UI was unable to find installer for your system architecture (${system_archtecture}). Please check whether installer for your system architecture is available, and if so, proceed to install manually (https://github.com/TDiblik/KeyXpert/releases/latest/).`,
      } as ModalProps);
      return;
    }

    modal_info.set({
      type: "question",
      title: "Update available",
      description: `You are currently using version ${app_version}, however newer version ${latest_release_version} is available. Would you like to update?`,
      keep_open_after_yes: true,
      yes_callback: async () => {
        modal_info.set({
          type: "fixed-info",
          title: "Installing new update...",
          description: "It's gonna take some time, please be patient...",
        } as ModalProps);

        const could_update = await invoke("download_and_install_update", { urlPath: download_url, expectedInstallerName: expected_installer_name });
        if (!could_update) {
          modal_info.set({
            type: "error",
            title: "Unable to install new version",
            description: "Something happened while installing new version. Please update manually or try again later.",
          } as ModalProps);
        }
      }
    } as ModalProps);
  }
  
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
        type: "info",
        title: "Successfully saved profile",
        description: "Your profile changes should be have been successfully written into config file.",
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
    <div class="check-for-updates-button-wrapper">
      <button class="btn primary" on:click={check_for_updates}>Check for updates</button>
    </div>
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
    {#if selected_profile_id?.length > 0}
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
  
  <div class="info-box">
  {#if service_config.active_profile == undefined || service_config.active_profile.length < 1}
    <p class="warning">No active profile (unable to start remapping)</p>
  {/if}
  </div>
</main>

<Modal />
{/if}