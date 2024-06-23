<script lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import type { Token } from "./token";
import Tokens from "./token";

// biome-ignore lint/style/useConst: <cannot use const, because it should be writable.>
let account_name = "";
// biome-ignore lint/style/useConst: <cannot use const, because it should be writable.>
let issuer = "";
// biome-ignore lint/style/useConst: <cannot use const, because it should be writable.>
let secret = "";

$: token = Array.from([account_name, issuer, secret]);
const submit_token = async (): Promise<void> =>
	await invoke("submit_token", { new_token: token });

const tokens: Token[] = Tokens.new([
	{
		name: "Account name",
		placeholder: "Enter your account name here.",
		label_input_name: "account",
	},
	{
		name: "Issuer",
		placeholder: "Enter your issuer here.",
		label_input_name: "institution",
	},
	{
		name: "Secret",
		placeholder: "Enter your secret here.",
		label_input_name: "key",
	},
]);
</script>

<form class="token_form" on:submit={submit_token} action="/">
  <h2 class="token_form_h2">Create new token</h2>

  {#each tokens as token} 
    <article>
      <label for={token.label_input_name} class="token_form_label">{token.name}</label>
      {#if token.name == "Account name"}
        <input 
          id={token.label_input_name} 
          name={token.name?.toLowerCase().split(" ").join("_")} 
          placeholder={token.placeholder} 
          class="token_form_input" 
          bind:value={account_name}
        />
      {:else if token.name == "Issuer"}
        <input 
          id={token.label_input_name} 
          name={token.name?.toLowerCase()} 
          placeholder={token.placeholder} 
          class="token_form_input" 
          bind:value={issuer}
        />
      {:else}
        <input 
          id={token.label_input_name} 
          name={token.name?.toLowerCase()} 
          placeholder={token.placeholder} 
          class="token_form_input" 
          bind:value={secret}
        />
      {/if}
    </article>
  {/each}

  <button type="submit" class="token_form_button">Submit token</button>
</form>
