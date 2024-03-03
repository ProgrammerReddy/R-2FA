<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { message } from "@tauri-apps/api/dialog";
  import { links } from "./links";
  import { Link } from "svelte-navigator";

  async function abort(): Promise<void> {
    await message("Reddy2FA - TOTP is succesfully been closed.");
    await invoke("abort");
  }
</script>

<section class="to-center px-3 preffered-bg">
  <article class="my-14 w-auto text-center">
    <h1 class="text-5xl font-medium preffered-color">Welcome to the Reddy2FA TOTP!</h1>
    <p class="mt-6 mb-12 text-2xl font-normal preffered-color">You can click on the button below to get started.</p>

    <div class="to-center">
      <a href="https://github.com/RedIsGaming/Reddy2FA" target="_blank">
        <figure><img src="/tauri.svg" alt="Tauri Logo" width="100" /></figure>
      </a>
    </div>

    <div class="mt-12 mb-6 to-center gap-5 font-medium text-xl">
      <button class="bg-gray-100 cta hover:bg-gray-300 shadow-md shadow-gray-50" on:click={abort}>Quit application</button>
      <Link to="/tokens">
        <button class="bg-red-500 cta text-white hover:bg-red-600 shadow-md shadow-orange-500">Get started here</button>
      </Link>
    </div>

    <div class="to-center gap-5 mb-5">
      {#each links as link}
        <a href={link.url} target={link.target}>
          <figure><img src={link.source} alt={link.alt} width={link.width} /></figure>
        </a>
      {/each}
    </div>

    <div class="to-center gap-1 preffered-color">
      <strong>Copyright © 2024 All Rights Reserved -</strong>
      <p>Powered by RedIsGaming</p>
    </div>
  </article>
</section>
