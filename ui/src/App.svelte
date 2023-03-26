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
      <option>Ahoje</option>
      <option>Ahoje</option>
      <option>Ahoje</option>
      <option>Ahoje</option>
      <option>Ahoje</option>
    </select>

    <div class="profile-actions">
      <button class="btn primary">Add profile</button>
      <button class="btn delete">Delete profile</button>
    </div>
  </div>
  
  <div class="rebind-list">
    <div class="sub-heading">Keys</div>

    <div class="rebind-item">
      <div class="keys-container">
        <div class="key"> A </div>
        <!-- <div class="key"> A </div> -->
        <div class="edit-pen"> 
          <img src="/public/edit-pen.svg" />
        </div>
      </div>
      <div class="keys-container">
        <div class="key new"> B </div>
      </div>
      <div class="remove-container">
        <button class="btn delete">Delete</button>
      </div>
    </div>

    <div class="rebind-item">
      <div class="keys-container">
        <div class="key"> A </div>
      </div>
      <div class="keys-container">
        <div class="key new"> B </div>
      </div>
    </div>

  </div>

  <div class="row">
    <Greet />
  </div>


</main>

<style>
</style>