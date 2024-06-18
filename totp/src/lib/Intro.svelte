<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { message } from "@tauri-apps/api/dialog";
  import { links } from "./links";
  import { Link } from "svelte-navigator";
  import RedIsGaming from "../../public/RedIsGaming.webp";

  async function abort(): Promise<void> {
    await message("Reddy2FA - TOTP is succesfully been closed.");
    await invoke("abort");
  }
</script>

<section class="px-3 to-center preffered-bg">
  <article class="my-14 w-auto text-center">
    <h1 class="text-5xl font-medium preffered-color">Welcome to the R-2FA TOTP!</h1>
    <p class="mt-6 mb-12 text-2xl font-normal preffered-color">You can click on the button below to get started.</p>

    <div class="to-center">
      <a href="https://github.com/RedIsGaming/" target="_blank">
        <figure><img src="{RedIsGaming}" alt="RedIsGaming profile" width="100" /></figure>
      </a>
    </div>

    <div class="gap-5 mt-12 mb-6 text-xl font-medium to-center">
      <button class="button-uncta cta" on:click={abort}>Quit application</button>
      <Link to={"/tokens"}>
        <button class="text-white bg-red-500 shadow-md hover:bg-red-600 cta shadow-red-500">Get started here</button>
      </Link>
    </div>

    <div class="gap-5 mb-5 to-center">
      {#each links as link}
        <a href={link.url} target={link.target}>
          <figure><img src={link.source} alt={link.alt} width={link.width} /></figure>
        </a>
      {/each}
    </div>

    <div class="gap-1 to-center preffered-color">
      <strong>Copyright © 2024 All Rights Reserved -</strong>
      <p>Powered by RedIsGaming</p>
    </div>
  </article>
</section>
