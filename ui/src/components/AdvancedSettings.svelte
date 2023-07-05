<script lang="ts">
  import type {ServiceConfig} from "src/models";
  import {advanced_settings_props} from "./AdvancedSettingsStore";
  import {handle_tauri_result} from "../utils";
  import {invoke} from "@tauri-apps/api/tauri";
  import {onMount} from "svelte";

  let dialog: HTMLDialogElement;
  let service_config: ServiceConfig = {
    active_profile: "",
    profiles: [],
    start_on_boot: true,
    enable_recursive_remapping: false,
    enable_recursive_shortcuts: false,
  };
  onMount(async () => {
    handle_tauri_result<ServiceConfig>(await invoke("get_service_config", {}), (result) => {
      service_config = result;
    });
  });

  async function on_save() {
    handle_tauri_result<void>(
      await invoke("save_advanced_settings", {
        startOnBoot: service_config.start_on_boot,
        enableRecursiveRemapping: service_config.enable_recursive_remapping,
        enableRecursiveShortcuts: service_config.enable_recursive_shortcuts,
      }),
      () => {
        advanced_settings_props.set(null);
      }
    );
  }

  $: if ($advanced_settings_props != null && dialog && !dialog.open) dialog.showModal();
  $: if ($advanced_settings_props == null && dialog) dialog.close();
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<dialog
  bind:this={dialog}
  on:click|self={() => {
    advanced_settings_props.set(null);
  }}
>
  <div on:click|stopPropagation>
    <div class="modal-head">
      <h2>Advanced settings</h2>
      <img
        class="modal close-button"
        src="/x-lg.svg"
        alt="close modal button"
        on:click={() => advanced_settings_props.set(null)}
      />
    </div>

    <hr />
    <div class="modal-body">
      <div>
        <input type="checkbox" bind:checked={service_config.start_on_boot} /> Start on boot
      </div>
      <div title="TODO">
        <input type="checkbox" bind:checked={service_config.enable_recursive_remapping} /> ENABLE_RECURSIVE_REMAPPING
      </div>
      <div title="TODO">
        <input type="checkbox" bind:checked={service_config.enable_recursive_shortcuts} /> ENABLE_RECURSIVE_SHORTCUTS
      </div>
    </div>

    <!-- svelte-ignore a11y-autofocus -->
    <div class="save-container">
      <button class="btn save bottom" autofocus on:click={() => on_save()}> Save </button>
    </div>
  </div>
</dialog>

<style>
  dialog {
    min-width: 300px;
    max-width: 32em;
    border-color: black;
    border-radius: var(--border-radius);
    padding: 0;
    background: var(--black);
  }
  dialog::backdrop {
    background: rgba(0, 0, 0, 0.3);
  }
  dialog > div {
    padding: 1em;
  }
  dialog[open] {
    animation: zoom 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
  }
  @keyframes zoom {
    from {
      transform: scale(0.95);
    }
    to {
      transform: scale(1);
    }
  }
  dialog[open]::backdrop {
    animation: fade 0.2s ease-out;
  }
  @keyframes fade {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  button {
    display: block;
  }

  .modal-head {
    display: flex;
    flex-direction: row;
  }

  h2 {
    font-size: 18px;
  }

  .save-container {
    display: flex;
    flex-direction: row;
    justify-content: flex-end;
    gap: 20px;
    margin-top: 10px;
  }

  .modal-body * {
    margin-top: 5px;
  }

  input[type="checkbox"] {
    margin-right: 10px;
  }

  .close-button {
    margin-left: auto;
  }
  .close-button:hover {
    cursor: pointer;
  }
  .modal.close-button {
    width: 28px;
  }
</style>
