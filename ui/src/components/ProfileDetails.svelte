<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  export let profile_id: string = "";

  let greetMsg = ""
  async function greet(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    greetMsg = await invoke("greet", { name })
  }
</script>
  
<div class="rebind-list">
  <div class="sub-heading">Keys</div>

  <div class="rebind-item">
    <div class="keys-container">
      <div class="key"> A </div>
      <img class="edit-pen" src="/edit-pen.svg" alt="edit button" />
    </div>
    <span class="from-to-label">to</span>
    <div class="keys-container">
      <div class="key new"> B </div>
      <img class="edit-pen" src="/edit-pen.svg" alt="edit button" />
    </div>
    <div class="remove-container">
      <button class="btn delete">Delete</button>
    </div>
  </div>

  <div class="rebind-item">
    <div class="keys-container">
      <div class="key"> A </div>
      <img class="edit-pen" src="/edit-pen.svg" alt="edit button" />
    </div>
    <span class="from-to-label">to</span>
    <div class="keys-container">
      <div class="key new"> B </div>
      <img class="edit-pen" src="/edit-pen.svg" alt="edit button" />
    </div>
    <div class="remove-container">
      <button class="btn delete">Delete</button>
    </div>
  </div>
  
  <div class="add-item-container">
    <img class="add-item" src="/plus-circle-fill.svg" alt="add button" />
  </div>
</div>

<div class="rebind-list">
  <div class="sub-heading">Shortcuts</div>

  <div class="rebind-item">
    <div class="keys-container">
      <div class="key"> A </div>
      <img class="edit-pen" src="/edit-pen.svg" alt="edit button" />
    </div>
    <span class="from-to-label">to</span>
    <div class="keys-container">
      <div class="key new"> B </div>
      <img class="edit-pen" src="/edit-pen.svg" alt="edit button" />
    </div>
    <div class="remove-container">
      <button class="btn delete">Delete</button>
    </div>
  </div>

  <div class="rebind-item">
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
  </div>
  
  <div class="add-item-container">
    <img class="add-item" src="/plus-circle-fill.svg" alt="add button" />
  </div>
</div>

<div class="bottom-row">
  <div class="is-active-wrapper">
    <input name="is_active_main" id="is_active_main" type="checkbox" />
    <label for="is_active_main">Use this profile</label>
  </div>
  <div class="save-button-wrapper">
    <button class="btn save">Save</button>
  </div>
</div>

<style>
.rebind-list {
  margin-top: 25px;
}

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

.remove-container {
  margin-left: auto;
  margin-right: 10px;
}

.edit-pen {
  height: var(--key-size);
  margin-left: 5px;
}
.edit-pen:hover {
  cursor: pointer;
}

.from-to-label {
  margin: 0 20px;
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

.bottom-row {
  margin-top: 25px;
  display: flex;
  flex-direction: row;
  align-items: center;
}

.is-active-wrapper > input:hover {
  cursor: pointer;
}

.save-button-wrapper {
  margin-left: auto;
}
</style>