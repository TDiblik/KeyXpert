<script lang="ts">
    export let is_new: boolean;
    export let current_key: number;
    
    $: current_key_char = String.fromCharCode(current_key);

    let is_key_changing = false;

    
    // TODO: Sucks for non-alphabet characters
    function capture_key(e: KeyboardEvent) {
      // e.which === Windows VK definitions
      current_key = e.which;
    }
    
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
    <img class="edit-pen" src="/edit-pen.svg" alt="edit button"  />
    {:else}
    <img class="edit-pen" src="/check-lg.svg" alt="edit button"  />
    {/if}
  </button>
</div>

<style>
.keys-container {
  display: flex;
  gap: 10px;
  margin: 0px 15px;
  align-items: center;
}

.key {
  padding: 7.5px 15px;
  font-size: var(--key-size);
  font-weight: 500 !important;
  line-height: var(--key-size);
  border: 2px solid var(--gray);
  border-radius: var(--border-radius);
  white-space: nowrap;
}

.key.new {
  background-color: var(--blue);
}

.edit-pen {
  height: var(--key-size);
  margin-left: 5px;
}
.edit-pen:hover {
  cursor: pointer;
}

</style>