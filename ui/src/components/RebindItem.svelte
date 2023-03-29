<script lang="ts">
  import type { KeyRemap } from "src/models";
  import Key from "./Key.svelte";
  
  export let remap: KeyRemap;
  export let on_delete: () => void;

</script>

<div class="rebind-item">
  <Key is_new={false} bind:current_key={remap.from} />
  <span class="from-to-label">to</span>
  <Key is_new={true} bind:current_key={remap.to} />
  <div class="remove-container">
    <button class="btn delete" on:click={on_delete}>Delete</button>
  </div>
</div>

<!-- <div class="rebind-item">
  <div class="keys-container">
    <div class="key"> Ctrl (left) </div>
    <div class="key"> Alt (right) </div>
    <div class="key"> ` </div>
    <img class="edit-pen" src="/edit-pen.svg" alt="edit button" />
  </div>
  <span class="from-to-label">to</span>
  <div class="keys-container">
    <div class="key new"> Ctrl (left) </div>
    <div class="key new"> Alt (right) </div>
    <div class="key new"> Ý </div>
    <img class="edit-pen" src="/edit-pen.svg" alt="edit button" />
  </div>
  <div class="remove-container">
    <button class="btn delete">Delete</button>
  </div>
</div> -->

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
}

.remove-container {
  margin-left: auto;
  margin-right: 10px;
}

.from-to-label {
  margin: 0 20px;
}
</style>