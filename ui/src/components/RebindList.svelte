<script lang="ts">
  import type { KeyRemap } from "src/models";
  import RebindItem from "./RebindItem.svelte";

  export let title: string;
  export let bindings: KeyRemap[];
  
  function on_add() {
    bindings = [...bindings, {
      from: 0x41,
      to: 0x42
    }];
  }
  
  function on_remove_item(index: number) {
    let new_bindings = bindings;
    new_bindings.splice(index, 1)
    bindings = new_bindings;
  }
  
</script>

<div class="rebind-list">
  <div class="subheading">{title}</div>
  
  {#each bindings as binding, i}
    <RebindItem remap={binding} on_delete={() => on_remove_item(i)} />
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
