<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
    import { cover_special_vk_cases, vk_to_string } from "../utils";
    import "./Key.css";

    export let is_new: boolean;
    export let current_key: number;
    
    $: current_key_char = vk_to_string(current_key);
    async function capture_key(e: KeyboardEvent) {
      // e.which || e.keyCode === Windows VK
      current_key = cover_special_vk_cases(
        e.which || e.keyCode,
        e.code
      );

      e.cancelBubble = true;
      e.preventDefault();
    }
    
    let is_key_changing = false;
    function change_key_state() {
      if (!is_key_changing) {
        window.addEventListener("keydown", capture_key, true);
      } else {
        window.removeEventListener("keydown", capture_key, true);
      }
      is_key_changing = !is_key_changing;
    }
</script>

<div class="keys-container">
  <div class={`key ${is_new ? "new" : ""}`}> {current_key_char} </div>
  <button class="img-btn-wrapper" on:click={change_key_state}>
    {#if !is_key_changing}
    <img class="edit-icon" src="/edit-pen.svg" alt="edit button"  />
    {:else}
    <img class="edit-icon" src="/check-lg.svg" alt="edit button"  />
    {/if}
  </button>
</div>