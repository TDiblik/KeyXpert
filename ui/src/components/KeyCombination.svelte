<script lang="ts">
  import "./Key.css";
  import {vk_to_string, cover_special_vk_cases, if_keycode_pressed, prevent_event_bubbling} from "../utils";
  import {onDestroy} from "svelte";

  export let is_new: boolean;
  export let holding_keys: number[];
  export let execution_key: number;

  $: execution_key_char = vk_to_string(execution_key);
  $: holding_keys_to_chars = holding_keys.map((s) => (s != 0 ? vk_to_string(s) : ""));

  let is_key_changing = false;

  let all_currently_pressed_keys = [];
  function capture_down(e: KeyboardEvent) {
    all_currently_pressed_keys[e.code] = true;

    {
      let new_holding_keys = [];
      if_keycode_pressed(all_currently_pressed_keys, "Meta", 0x5b, 0xbc, new_holding_keys);
      if_keycode_pressed(all_currently_pressed_keys, "Control", 0xa2, 0xa3, new_holding_keys);
      if_keycode_pressed(all_currently_pressed_keys, "Alt", 0xa4, 0xa5, new_holding_keys);
      // Check for ALTGr
      if (
        new_holding_keys.length >= 2 &&
        new_holding_keys[new_holding_keys.length - 2] == 0xa2 &&
        new_holding_keys[new_holding_keys.length - 1] == 0xa5
      ) {
        new_holding_keys.pop();
        new_holding_keys.pop();
        new_holding_keys.push(0xa5);
      }
      if_keycode_pressed(all_currently_pressed_keys, "Shift", 0xa0, 0xa1, new_holding_keys);
      if (!(holding_keys.length == 1 && (holding_keys[0] == 0x5b || holding_keys[0] == 0xbc))) {
        holding_keys = new_holding_keys;
      }
    }

    // e.which || e.keyCode === Windows VK
    let pressed_key = cover_special_vk_cases(e.which || e.keyCode, e.code);

    // if not in holding keys range --- 0xA to 0xA5 => range for alt, ctrl, shift, 0x5B == L_WIN, 0x5C == R_WIN
    if ((pressed_key < 0xa0 || pressed_key > 0xa5) && pressed_key != 0x5b && pressed_key != 0x5c) {
      execution_key = pressed_key;
    } else {
      execution_key = 0x0;
    }

    if (execution_key != 0x0 && holding_keys?.length > 0) {
      change_key_state();
    }

    return prevent_event_bubbling(e);
  }

  function capture_up(e: KeyboardEvent) {
    all_currently_pressed_keys[e.code] = false;

    return prevent_event_bubbling(e);
  }

  function cleanup_events() {
    all_currently_pressed_keys = [];
    window.removeEventListener("keydown", capture_down, true);
    window.removeEventListener("keyup", capture_up, true);
  }

  function change_key_state() {
    if (!is_key_changing) {
      window.addEventListener("keydown", capture_down, true);
      window.addEventListener("keyup", capture_up, true);
    } else {
      cleanup_events();
    }
    is_key_changing = !is_key_changing;
  }

  onDestroy(() => cleanup_events());
</script>

<div class="keys-container">
  {#each holding_keys_to_chars as holding_key}
    {#if holding_key != ""}
      <div class={`key ${is_new ? "new" : ""}`}>{holding_key}</div>
    {/if}
  {/each}
  <div class={`key ${is_new ? "new" : ""}`}>{execution_key_char}</div>
  <button class="img-btn-wrapper" on:click={change_key_state}>
    {#if !is_key_changing}
      <img class="edit-icon" src="/edit-pen.svg" alt="edit button" title="Click to start editing" />
    {:else}
      <img class="edit-icon" src="/check-lg.svg" alt="edit button" title="Click to finish editing" />
    {/if}
  </button>
</div>

<style>
  .key {
    font-size: 15px !important;
  }
</style>
