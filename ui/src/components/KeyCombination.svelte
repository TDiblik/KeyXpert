<script lang="ts">
    import "./Key.css";

    export let is_new: boolean;
    export let current_key: number;
    
    $: current_key_char = String.fromCharCode(current_key);

    let is_key_changing = false;

    // TODO: Sucks for non-alphabet characters
    function capture_key(e: KeyboardEvent) {
      // e.which === Windows VK definitions
      console.log(e);
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