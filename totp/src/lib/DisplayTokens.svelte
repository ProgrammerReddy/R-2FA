<script lang="ts">
  import Icon from "@iconify/svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { Token, StructToken } from "./token";
  import Tokens from "./token";
  import { Link } from "svelte-navigator";

  let totp: Array<string> = Array.from([]);
  let step = 0;
  let struct_token: Array<StructToken> = Array.from([]);
  const size: number = 40;

  async function display_tokens(): Promise<void> {
    const period: number = 30;
    const seconds = new Date().getSeconds();
    step = seconds < period ? period - seconds : period * 2 - seconds;

    totp = await invoke("generate_token");
    struct_token = await invoke("show_token");
  }

  function token_timer(): void {
    const delay: number = 1000;
    setInterval(display_tokens, delay);
  }

  token_timer();
  let tokens: Token[] = [];

  $: if (totp) {
    tokens = Tokens.new([
      {
        issuer: struct_token.map((x: StructToken) => x.issuer).join(" "),
        placeholder: "vscode-icons:file-type-objidconfig",
        otp: totp.join(" "),
      },
    ]);
  }

  $: mapped_tokens = tokens.flatMap((x: Token) =>
    Array.from({ length: struct_token.length }).map(() => x),
  );

  $: remove_token = async (): Promise<void> => {
    await invoke("drop_token", { remove_id: struct_token.map((x: StructToken) => x.id) });
  }
</script>

<section class="preffered-bg">
  <article class="preffered-color">
    <article class="items-center to-center">
      <h1 class="flex items-center pr-5 h-24 text-4xl font-bold">Overview of R-2FA TOTP tokens</h1>
    </article>

    <article class="w-full">
      <Link to={"/new_token"}>
        <div class="flex items-center p-2 h-14 border-t-2 shadow-md duration-200 ease-out cursor-pointer hover:bg-red-50 hover:ease-in dark:hover:bg-red-700">
          <div class="w-1/12"><Icon icon="typcn:plus" width={size} height={size} /></div>
          <p class="pl-4 w-11/12 text-xl">Add new token</p>
        </div>
      </Link>

      {#each mapped_tokens as tks}
        <div class="flex items-center p-2 h-14 border-t-2 shadow-md duration-200 ease-out cursor-pointer hover:bg-red-50 hover:ease-in dark:hover:bg-red-700">
          <div class="w-1/12"><Icon icon="{tks.placeholder}" width={size} height={size} /></div>
          <div class="w-1/6"><p class="pl-4 text-xl">{tks.issuer}</p></div>
          <div class="w-1/6"><p class="pl-4 text-xl">{tks.otp}</p></div>
          <div class="w-1/6"><p class="pl-4 text-xl">{step}</p></div>
          <Icon icon="mdi:bin-empty" class="text-red-600 dark:text-white" on:click={remove_token} width={size} height={size} />
        </div>
      {/each}
    </article>
  </article>
</section>
