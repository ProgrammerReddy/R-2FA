<script lang="ts">
  import { Link } from "svelte-navigator";
  import { service_token } from "./service_token";
  import Icon from "@iconify/svelte";
  import Footer from "./Footer.svelte";
</script>

<section class="to-center preffered-bg px-3">
  <article class="w-auto my-14 text-center">
    <h1 class="preffered-color text-5xl font-medium">Overview of tokens</h1>

    <article class="grid grid-cols-3 gap-6 my-9 w-full">
      {#each service_token as token}
        <div class="token">
          <h2 class="preffered-color text-3xl">{token.name}</h2>
          
          <div class="to-center my-6">
            <Icon icon={token.icon} width={50} />
          </div>

          <!--<progress max={100} value={50}></progress>-->
          <p class="preffered-color mt-0 text-xl">{token.token}</p>
        </div>
      {/each}

      <Link to={"/tokens/add"} class="token preffered-color to-center items-center cursor-pointer flex-col">
        <h2 class="preffered-color text-3xl mb-6">Add token</h2>
        <Icon icon={"uiw:plus"} width={100} />
      </Link>
    </article>

    <Footer />
  </article>
</section>
