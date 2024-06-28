<script lang="ts">
  import Icon from "@iconify/svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { Token, StructToken } from "./token";
  import RemoveToken from "./RemoveToken.svelte";

  export let size: number;
  
  let totp: Array<string> = Array.from([]);
  let step = 0;
  let struct_token: Array<StructToken> = Array.from([]);
  let tokens: Array<Token> = Array.from([]);

  async function display_tokens(): Promise<void> {
    const period: number = 30;
    const seconds: number = new Date().getSeconds();
    step = seconds < period ? period - seconds : period * 2 - seconds;
    
    try {
      totp = await invoke("generate_token");
    }

    catch (err: unknown) {
      if (err instanceof Object) {
        console.log(err);
        totp = Array.from(["unknown token"]);
      }
    }

    struct_token = await invoke("show_token");
  }

  const delay: number = 1000;
  const token_timer = (): number => setInterval(display_tokens, delay);
  token_timer();

  $: if (struct_token.length > 0) {
    tokens = [];

    // biome-ignore lint/complexity/noForEach: <why making it yourself harder with a for of instead of a forEach?>
    struct_token.forEach((x: StructToken) => {
      tokens
        .sort((a: Token, b: Token) => a.issuer.localeCompare(b.issuer))
        .push({ id: x.id, placeholder: "vscode-icons:file-type-objidconfig", issuer: x.issuer, otp: totp.join(" ") });
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
