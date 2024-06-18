<script lang="ts">
  import Icon from "@iconify/svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { Token } from "./token";
  import Tokens from "./token";
  import { Link } from "svelte-navigator";
  
  let totp = "";
  let step = 0;

  async function invoke_token(): Promise<void> {
    const period: number = 30;
    step = (new Date().getSeconds() < period) ? period - new Date().getSeconds() : period * 2 - new Date().getSeconds();
    totp = await invoke("generate_token");
  };

  function token_timer(): void {
    setInterval(invoke_token, 1000);
  }

  token_timer();
  let tokens: Token[] = [];

  $: if (totp) {
    tokens = Tokens.new([
      {
        issuer: "LinkedIn",
        icon: "openmoji:linkedin",
        otp: totp,
      },
      {
        issuer: "Google",
        icon: "devicon:google",
        otp: totp,
      },
      {
        issuer: "Microsoft",
        icon: "logos:microsoft-icon",
        otp: totp,
      },
      {
        issuer: "Discord",
        icon: "logos:discord-icon",
        otp: totp,
      },
      {
        issuer: "Dashlane",
        icon: "logos:dashlane-icon",
        otp: totp,
      },
      {
        issuer: "Proton",
        icon: "simple-icons:proton",
        otp: totp,
      },
      {
        issuer: "Atlassian",
        icon: "logos:atlassian",
        otp: totp,
      },
      {
        issuer: "Figma",
        icon: "devicon:figma",
        otp: totp,
      },
    ]);
  }

  //const sorted_tokens = Object.fromEntries(Object.entries(tokens).sort(([,a], [,b]) => a.issuer.localeCompare(b.issuer)));
  //const new_tokens = Object.values(sorted_tokens);
  const tk: string = "Overview of R-2FA TOTP tokens";
</script>

<section class="preffered-bg">
  <article class="preffered-color">
    <article class="to-center items-center">
      <h1 class="h-24 flex items-center text-4xl font-bold pr-5">{tk}</h1>
      <Link to={"/"}>
        <Icon icon={"ion:home"} width={36} class="invert-0" />
      </Link>
    </article>

    <article class="w-full">
      <div class="border-t-2 shadow-md h-14 flex items-center p-2 duration-200 hover:bg-red-50 hover:ease-in ease-out 
        cursor-pointer dark:hover:bg-red-700">
        <div class="w-1/12"><Icon icon="typcn:plus" width="40" height="40" /></div>
        <div class="w-11/12"><p class="pl-4 text-xl">Add new token</p></div>
      </div>
      {#each tokens as tks}
        <div class="border-t-2 shadow-md h-14 flex items-center p-2 duration-200 hover:bg-red-50 hover:ease-in ease-out 
          cursor-pointer dark:hover:bg-red-700">
          {#if tks.issuer.startsWith("Dashlane")} 
            <div class="w-1/12 invert-0 dark:invert"><Icon icon="{tks.icon}" width="40" height="40" /></div>
            {:else}
              <div class="w-1/12"><Icon icon="{tks.icon}" width="40" height="40" /></div>
          {/if}
          <div class="w-1/6"><p class="pl-4 text-xl">{tks.issuer}</p></div>
          <div class="w-1/6"><p class="pl-4 text-xl">{tks.otp}</p></div>
          <div class="w-1/6"><p class="pl-4 text-xl">{step}</p></div>
        </div>
      {/each}
    </article>
  </article>
</section>
