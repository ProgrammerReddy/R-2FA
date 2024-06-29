<script lang="ts">
  import Icon from "@iconify/svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { Token, StructToken } from "./token";
  import RemoveToken from "./RemoveToken.svelte";
  import { option, some, none } from "./option";
  import type { Some } from "./option";

  export let size: number;
  
  let totp: Some<string[]> = some([]);
  let step = 0;
  let struct_token: Some<StructToken[]> = some([]);
  let tokens: Array<Token> = Array.from([]);

  async function display_tokens(): Promise<void> {
    const period: number = 30;
    const seconds: number = new Date().getSeconds();
    step = seconds < period ? period - seconds : period * 2 - seconds;
    
    //Awaiting the invoke on "generate_token" will panic the token retrieve, custom error handling is used instead here.
    totp = await option(Promise.resolve(invoke("generate_token"))) as Some<string[]>;
    struct_token = await option(Promise.resolve(await invoke("show_token"))) as Some<StructToken[]>;
  }

  const delay: number = 1000;
  const token_timer = (): number => setInterval(display_tokens, delay);
  token_timer();

  $: if (struct_token.value.length > 0) {
    tokens = [];
    const handle_totp: string = Array.isArray(totp.value) ? totp.value.join() : none;

    // biome-ignore lint/complexity/noForEach: <why making it yourself harder with a for of instead of a forEach?>
    struct_token.value.forEach((x: StructToken) => {
      tokens
        .sort((a: Token, b: Token) => a.issuer.localeCompare(b.issuer))
        .push({ id: x.id, placeholder: "vscode-icons:file-type-objidconfig", issuer: x.issuer, otp: handle_totp });
    });
  }

  let show_dialog = false;
  let token_id: number | null;

  function confirm_remove(id: number | null): void {
    token_id = id;
    
    show_dialog = !show_dialog;
    const dialog: HTMLDialogElement = document.getElementById("dialog-state") as HTMLDialogElement;
    dialog != null ? dialog.showModal() : null;
  };
</script>

{#each tokens as tks}
  <div class="display_handle_add_tokens">
    <div class="w-[8.33%]"><Icon icon="{tks.placeholder}" width={size} height={size} /></div>
    <div class="w-1/4"><p class="display_add_tokens_p w-auto">{tks.issuer}</p></div>
    <div class="w-1/4"><p class="display_add_tokens_p w-auto">{tks.otp}</p></div>
    <div class="w-1/4"><p class="display_add_tokens_p w-auto">{step}</p></div>

    <button on:click={() => confirm_remove(tks.id)}>
      <Icon icon="mdi:bin-empty" class="handle_tokens_icon" width={size} height={size} />
    </button>
  </div>
{/each}

{#if show_dialog}
  <RemoveToken id={token_id} close_dialog={show_dialog} />
{/if}
