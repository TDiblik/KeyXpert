<script lang="ts">
  import type { Profile, ProfileDetailsInfo } from "src/models";
  import RebindList from "./RebindList.svelte";

  export let selected_profile: Profile;
  export let active_profile: string;
  export let on_save: (profile_info: ProfileDetailsInfo) => {};

  let _profile_info: ProfileDetailsInfo = {
    ...selected_profile,
    use_this_profile: selected_profile.id === active_profile
  };
  
  // $: console.log(_profile_info)
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

<RebindList title="Keys" bind:bindings={_profile_info.key_remaps} is_shortcut={false} />
<RebindList title="Shortcuts" bind:bindings={_profile_info.shortcut_remaps} is_shortcut={true}/>

<div class="bottom-row">
  <div class="save-button-wrapper">
    <button class="btn save" on:click={() => on_save(_profile_info)}>Save</button>
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