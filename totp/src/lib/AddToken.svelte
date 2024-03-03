<script lang="ts">
  import { Link } from "svelte-navigator";
  import Footer from "./Footer.svelte";
</script>

<section class="preffered-bg to-center px-3">
  <article class="my-14 w-full">
    <h1 class="text-5xl font-medium preffered-color">Add token</h1>

    <form class="to-center flex-col my-9 preffered-color">
      <label for="token_key" id="totp_key" class="mb-3 text-lg">Enter your totp key here: </label>
      <input type="text" name="token_key" placeholder="Your TOTP key" class="p-3 w-full rounded-3xl placeholder:text-gray-500 preffered-color bg-gray-100 dark:text-gray-500" />
    </form>

    <article class="grid grid-cols-3 gap-5 w-auto font-medium text-xl">
      <Link to={"/tokens"}>
        <button class="button-uncta cta w-full">Go back</button>
      </Link>
      <button class="button-uncta cta">Coming soon</button>
      <button class="bg-red-500 cta text-white hover:bg-red-600 shadow-md shadow-red-500">Submit token</button>
    </article>

    <Footer />
  </article>
</section>
