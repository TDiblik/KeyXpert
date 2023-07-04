<script lang="ts">
  // @ts-nocheck

  import type {KeyRemap, ShortcutRemap} from "src/models";
  import Key from "./Key.svelte";
  import KeyCombination from "./KeyCombination.svelte";

  export let remap: KeyRemap | ShortcutRemap;
  export let on_delete: () => void;
  export let is_shortcut: boolean;
</script>

<div class="rebind-item">
  {#if !is_shortcut}
    <Key is_new={false} bind:current_key={remap.from} />
  {:else}
    <KeyCombination
      is_new={false}
      bind:holding_keys={remap.from_shortcut_holding_keys}
      bind:execution_key={remap.from_shortcut_execution_key}
    />
  {/if}

  <span class="from-to-label">to</span>

  {#if !is_shortcut}
    <Key is_new={true} bind:current_key={remap.to} />
  {:else}
    <KeyCombination
      is_new={true}
      bind:holding_keys={remap.to_shortcut_holding_keys}
      bind:execution_key={remap.to_shortcut_execution_key}
    />
  {/if}

  <div class="remove-container">
    <button class="btn delete" on:click={on_delete}>Delete</button>
  </div>
</div>

<style>
  .rebind-item {
    display: flex;
    flex-direction: row;
    align-items: center;
    margin-top: 10px;
    background-color: var(--light-black);
    border-radius: var(--border-radius);
    padding: 7.5px 0px;
    overflow-x: auto;
    gap: 20px;
  }

  .remove-container {
    margin-left: auto;
    margin-right: 10px;
  }
</style>
