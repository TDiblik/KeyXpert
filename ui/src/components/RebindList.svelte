<script lang="ts">
  import type {KeyRemap, ShortcutRemap} from "src/models";
  import RebindItem from "./RebindItem.svelte";

  export let title: string;
  export let bindings: KeyRemap[] | ShortcutRemap[];
  export let is_shortcut: boolean;

  function on_add() {
    if (!is_shortcut) {
      bindings = [
        ...bindings,
        {
          from: 0x41,
          to: 0x42,
        },
      ] as KeyRemap[];
    } else {
      bindings = [
        ...bindings,
        {
          from_shortcut_holding_keys: [0xa2],
          from_shortcut_execution_key: 0x41,
          to_shortcut_holding_keys: [0xa2],
          to_shortcut_execution_key: 0x42,
        },
      ] as ShortcutRemap[];
    }
  }

  function on_remove_item(index: number) {
    bindings.splice(index, 1);
    bindings = bindings;
  }
</script>

<div class="rebind-list">
  <div class="subheading">{title}</div>

  {#each bindings as binding, i}
    <RebindItem remap={binding} on_delete={() => on_remove_item(i)} {is_shortcut} />
  {/each}

  <div class="add-item-container">
    <button class="img-btn-wrapper" on:click={on_add}>
      <img class="add-item" src="/plus-circle-fill.svg" alt="add button" />
    </button>
  </div>
</div>

<style>
  .rebind-list {
    margin-top: 10px;
  }

  .add-item-container {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    margin-top: 10px;
  }
  .add-item {
    height: 42px;
  }
  .add-item:hover {
    cursor: pointer;
  }
</style>
