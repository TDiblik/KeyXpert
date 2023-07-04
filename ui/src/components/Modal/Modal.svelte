<script lang="ts">
  import {modal_info} from "./ModalStore";

  let dialog: HTMLDialogElement;

  function capitalizeFirstLetter(string: string) {
    return string.charAt(0).toUpperCase() + string.slice(1);
  }

  $: if ($modal_info != null && dialog && !dialog.open) dialog.showModal();
  $: if ($modal_info == null && dialog) dialog.close();
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<dialog
  bind:this={dialog}
  on:click|self={() => {
    if ($modal_info?.type == "fixed-info") {
      return;
    }
    modal_info.set(null);
  }}
  class={`${$modal_info?.type}`}
>
  <div on:click|stopPropagation>
    <div class="modal-head">
      <h2>{capitalizeFirstLetter($modal_info?.type ?? "  ")} - {$modal_info?.title}</h2>
      {#if $modal_info?.type == "fixed-info"}
        <div />
      {:else}
        <img
          class="modal close-button"
          src="/x-lg.svg"
          alt="close modal button"
          on:click={() => modal_info.set(null)}
        />
      {/if}
    </div>

    <hr />

    <p>{$modal_info?.description}</p>
    {#if $modal_info?.show_error_info}
      <p>Error info:</p>
      <p>{JSON.stringify($modal_info?.error, Object.getOwnPropertyNames($modal_info?.error))}</p>
    {/if}

    <!-- svelte-ignore a11y-autofocus -->
    {#if $modal_info?.type == "question"}
      <div class="yes-no-container">
        <button class="btn delete bottom" autofocus on:click={() => modal_info.set(null)}>No</button>
        <button
          class="btn save bottom"
          autofocus
          on:click={() => {
            if (!$modal_info?.keep_open_after_yes) {
              modal_info.set(null);
            }
            $modal_info?.yes_callback();
          }}
        >
          Yes
        </button>
      </div>
    {:else if $modal_info?.type == "fixed-info"}
      <div />
    {:else}
      <button class="btn primary close-button bottom" autofocus on:click={() => modal_info.set(null)}>Ok</button>
    {/if}
  </div>
</dialog>

<style>
  dialog {
    min-width: 300px;
    max-width: 32em;
    border-color: black;
    border-radius: var(--border-radius);
    padding: 0;
    background: var(--black);
  }
  dialog::backdrop {
    background: rgba(0, 0, 0, 0.3);
  }
  dialog > div {
    padding: 1em;
  }
  dialog[open] {
    animation: zoom 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
  }
  @keyframes zoom {
    from {
      transform: scale(0.95);
    }
    to {
      transform: scale(1);
    }
  }
  dialog[open]::backdrop {
    animation: fade 0.2s ease-out;
  }
  @keyframes fade {
    from {
      opacity: 0;
    }
    to {
      opacity: 1;
    }
  }

  dialog.error {
    background: var(--red-soft);
  }

  button {
    display: block;
  }

  .modal-head {
    display: flex;
    flex-direction: row;
  }

  .yes-no-container {
    display: flex;
    flex-direction: row;
    justify-content: flex-end;
    gap: 20px;
  }

  .close-button {
    margin-left: auto;
  }
  .close-button:hover {
    cursor: pointer;
  }
  .modal.close-button {
    width: 28px;
  }
  .close-button.bottom {
    margin-top: 10px;
  }

  h2 {
    font-size: 18px;
  }
  p {
    line-height: 26px;
    line-break: strict;
    word-wrap: break-word;
  }
</style>
