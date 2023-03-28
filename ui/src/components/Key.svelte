<script lang="ts">
    export let is_new: boolean;
    
    function change_key_state() {
        document.onkeydown = function(event) {
          var char = String.fromCharCode(event.which);
          console.log(char);
        };    
    }
</script>

<div class="keys-container">
  <div class={`key ${is_new ? "new" : ""}`}> A </div>
  <button class="img-btn-wrapper" on:click={change_key_state}>
    <img class="edit-pen" src="/edit-pen.svg" alt="edit button"  />
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