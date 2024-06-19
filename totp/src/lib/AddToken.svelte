<script lang="ts">
  import { Link } from "svelte-navigator";
  import Icon from "@iconify/svelte";

  const tk: string = "Add new R-2FA TOTP token";
  const home: string = "back";
  const size: number = 40;
</script>

<section class="preffered-bg">
  <article class="preffered-color">
    <article class="to-center items-center">
      <h1 class="h-24 flex items-center text-4xl font-bold pr-5">{tk}</h1>
    </article>

    <article class="w-full">
      <Link to={"/"}>
        <div class="border-t-2 shadow-md h-14 flex items-center p-2 duration-200 hover:bg-red-50 hover:ease-in ease-out 
          cursor-pointer dark:hover:bg-red-700">
          <div class="w-1/12"><Icon icon="mingcute:enter-door-line" width={size} height={size} /></div>
          <p class="pl-4 text-xl w-11/12">{home}</p>
        </div>
      </Link>
    </article>
  </article>
</section>
