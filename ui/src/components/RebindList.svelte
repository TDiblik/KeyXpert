<script lang="ts">
  import RebindItem from "./RebindItem.svelte";


    export let title: string;
    export let on_add: () => void;
</script>

<div class="rebind-list">
  <div class="subheading">{title}</div>
  
  <RebindItem/>
  
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
