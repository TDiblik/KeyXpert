<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import type { Profile, ProfileDetailsInfo } from "src/models";
  import { onMount } from "svelte";

  export let selected_profile: Profile;
  export let active_profile: string;

  let _profile_info: ProfileDetailsInfo = {
    ...selected_profile,
    use_this_profile: selected_profile.id === active_profile
  };;
  
  $: console.log(_profile_info)
</script>

<div class="profile-info-row">
  <div class="profile-name-wrapper">
    <label for="profile-name" id="profile-name-label" class="label"> Name </label>
    <input type="text" id="profile-name" name="profile-name" bind:value={_profile_info.name} />
  </div>
  <div class="is-active-wrapper">
    <input name="is-active-main" id="is-active-main" type="checkbox" bind:checked={_profile_info.use_this_profile} />
    <label for="is-active-main" id="is-active-main-wrapper">Use this profile</label>
  </div>
</div>
  
<div class="rebind-list">
  <div class="subheading">Keys</div>

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
  <div class="subheading">Shortcuts</div>

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
  <div class="save-button-wrapper">
    <button class="btn save">Save</button>
  </div>
</div>

<style>
.profile-info-row {
  margin-top: 35px;
  margin-bottom: 5px;
  display: flex;
  flex-direction: row;
  align-items: center;
}

.profile-name-wrapper {
  width: 70%;
}

#profile-name-label {
  margin-right: 10px;
}

#profile-name {
  width: 60%;
}

.is-active-wrapper {
  margin-left: auto;
}

#is-active-main-wrapper {
  margin-left: 5px;
}

.rebind-list {
  margin-top: 10px;
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
  margin-top: 15px;
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: flex-end;
}

.save-button-wrapper {
  margin-bottom: 10px; /* should show scroll before btn overflows page */
}
</style>