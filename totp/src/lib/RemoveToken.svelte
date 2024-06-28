<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  export let id: number | null;
  export let close_dialog: boolean;
  
  function close_modal(): void {
    close_dialog = !close_dialog;
    const dialog = document.getElementById("dialog-state") as HTMLDialogElement;
    dialog != null ? dialog.close("dialog-state") : null;
  }

  async function remove_token(id: number | null): Promise<void> {
    await invoke("drop_token", { remove_id: id });
    close_modal();
  }

  function escape_modal(event: KeyboardEvent): void {
    if (event.key === "Escape" && close_dialog) {
      close_dialog = !close_dialog;
    }
  }
</script>

{#if close_dialog}
  <dialog id="dialog-state" class="preffered-bg h-full inset-0 to-center items-center z-10">
    <article class="w-11/12 rounded-xl text-center bg-gray-300 dark:bg-gray-950 p-8">
      <h2 class="text-4xl font-semibold text-black dark:text-white">Confirmation deletion token with id: {id}</h2>
      <p class="text-xl my-16 text-black dark:text-white">
        Are you sure you want to remove this token? After submitting, there is no way to recover! 
        Proceed with caution if you know what you're doing. 
      </p>
      
      <article class="grid grid-cols-2 gap-6">
        <button class="bg-green-600 hover:bg-green-500 remove_token_button" on:click={() => close_modal()}>Cancel</button>
        <button class="bg-red-600 hover:bg-red-500 remove_token_button" on:click={() => remove_token(id)}>
          Delete token
        </button>
      </article>
    </article>
  </dialog>
{/if}

<svelte:window on:keydown={escape_modal}></svelte:window>
