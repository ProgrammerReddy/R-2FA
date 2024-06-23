<script lang="ts">
import { Link } from "svelte-navigator";
import Icon from "@iconify/svelte";
import TokenForm from "./TokenForm.svelte";

const size: number = 40;
</script>

<section class="preffered-bg">
  <article class="preffered-color">
    <article class="items-center to-center">
      <h1 class="flex items-center h-24 text-4xl font-bold">Add new R-2FA TOTP token</h1>
    </article>

    <article class="w-full">
      <Link to={"/"}>
        <div class="flex items-center p-2 h-14 border-t-2 shadow-md duration-200 ease-out cursor-pointer hover:bg-red-50 hover:ease-in dark:hover:bg-red-700">
          <div class="w-1/12"><Icon icon="mingcute:enter-door-line" width={size} height={size} /></div>
          <p class="pl-4 w-11/12 text-xl">back</p>
        </div>
      </Link>

      <TokenForm />
    </article>
  </article>
</section>
