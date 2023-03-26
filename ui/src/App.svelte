<script lang="ts">
  import Greet from './lib/Greet.svelte'
</script>

<main class="container">
  <div class="header-row">
    <h1 class="header">KeyXpert</h1>
    <div class="advanced-settings-button-wrapper">
      <button class="btn primary">Advanced settings</button>
    </div>
  </div>
  
  <div class="profile-selection-row">
    <select id="profile-selector">
      <option>example</option>
      <option>example</option>
      <option>example</option>
      <option>example</option>
      <option>example</option>
    </select>

    <div class="profile-actions">
      <button class="btn primary">Add profile</button>
      <button class="btn delete">Delete profile</button>
    </div>
  </div>
  
  <div class="overall-settings">
  </div>
  
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

</main>

<style>
</style>