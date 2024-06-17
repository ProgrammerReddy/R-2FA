<script lang="ts">
  import { Link } from "svelte-navigator";
  import Footer from "./Footer.svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { message } from "@tauri-apps/api/dialog";

  let totp: string = "";

  async function generate_token(): Promise<void> {
    //window.location.href = "/tokens";
    await message("TOTP token has been added!");
    totp = await invoke("generate_token");
  }
</script>

<section class="preffered-bg to-center px-3">
  <article class="my-14 w-full">
    <h1 class="text-5xl font-medium preffered-color">Add token</h1>

    <form class="to-center flex-col" on:submit|preventDefault={generate_token}>
      <label for="token_key" id="totp_key" class="mb-3 text-lg mt-9 preffered-color">Enter your totp key here: </label>
      <input type="text" name="token_key" placeholder="Your TOTP key" class="add-token mb-9" />
      
      <article class="grid grid-cols-3 gap-5 w-auto font-medium text-xl">
        <Link to={"/tokens"}>
          <button class="button-uncta cta w-full">Go back</button>
        </Link>
        
        <button class="button-uncta cta bg-gray-100" disabled>Coming soon</button>
        <button class="bg-red-500 cta text-white hover:bg-red-600 shadow-md shadow-red-500" type="submit">
          Submit token
        </button>
      </article>
      <p class="preffered-color text-5xl">a: {totp}</p>
    </form>
 
    <Footer />
  </article>
</section>
