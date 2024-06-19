<script lang="ts">
  import Icon from "@iconify/svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { Token } from "./token";
  import Tokens from "./token";
  import { Link } from "svelte-navigator";
  
  let totp = "";
  let step = 0;
  let token_issuer = "";

  async function invoke_token(): Promise<void> {
    const period: number = 30;
    step = (new Date().getSeconds() < period) ? period - new Date().getSeconds() : period * 2 - new Date().getSeconds();
    totp = await invoke("generate_token");
    token_issuer = await invoke("show_tokens");
  };

  function token_timer(): void {
    setInterval(invoke_token, 1000);
  }

  token_timer();
  let tokens: Token[] = [];

  $: if (totp) {
    tokens = Tokens.new([
      {
        issuer: token_issuer,
        icon: "vscode-icons:file-type-objidconfig",
        otp: totp,
      },
    ]);
  }

  const tk: string = "Overview of R-2FA TOTP tokens";
  const new_token: string = "Add new token";
  const size: number = 40;
</script>

<section class="preffered-bg">
  <article class="preffered-color">
    <article class="to-center items-center">
      <h1 class="h-24 flex items-center text-4xl font-bold pr-5">{tk}</h1>
    </article>

    <article class="w-full">
      <Link to={"/new_token"}>
        <div class="border-t-2 shadow-md h-14 flex items-center p-2 duration-200 hover:bg-red-50 hover:ease-in ease-out 
          cursor-pointer dark:hover:bg-red-700">
          <div class="w-1/12"><Icon icon="typcn:plus" width={size} height={size} /></div>
          <p class="pl-4 text-xl w-11/12">{new_token}</p>
        </div>
      </Link>
      {#each tokens as tks}
        <div class="border-t-2 shadow-md h-14 flex items-center p-2 duration-200 hover:bg-red-50 hover:ease-in ease-out 
          cursor-pointer dark:hover:bg-red-700">
          <div class="w-1/12"><Icon icon="{tks.icon}" width={size} height={size} /></div>
          <div class="w-1/6"><p class="pl-4 text-xl">{tks.issuer}</p></div>
          <div class="w-1/6"><p class="pl-4 text-xl">{tks.otp}</p></div>
          <div class="w-1/6"><p class="pl-4 text-xl">{step}</p></div>
        </div>
      {/each}
    </article>
  </article>
</section>
