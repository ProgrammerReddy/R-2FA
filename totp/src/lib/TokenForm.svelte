<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  
  // biome-ignore lint/style/useConst: <cannot use const, because it should be writable.>
  let account_name = "";
  // biome-ignore lint/style/useConst: <cannot use const, because it should be writable.>
  let issuer = "";
  // biome-ignore lint/style/useConst: <cannot use const, because it should be writable.>
  let secret = "";
  
  let account_name_err = "";
  let issuer_err = "";
  let secret_err = "";

  const check: string = "✅";
  const bytes: number = new Blob([JSON.stringify(check)]).size + 1;
  const bits: number = 7;
  const regex: RegExp = /^[a-z0-9$,@#.^%-]+$/i;
  const base32: RegExp = /^[a-z2-7]+$/i;
  const unmatch_regex: string = "Can only contain alphanumeric types or the following characters: [$,@#.^%-]!";
  const unmatch_base32: string = "Can only contain base32 types: [a-z, A-Z, 2-7]!";
  const empty: string = "Cannot be empty!";
  const character_limit: string = "Is to long!";

  $: account_name_err = account_name.length === 0 ? empty : !account_name.match(regex) 
    ? unmatch_regex : account_name.length > 255 
    ? character_limit : check;

  $: issuer_err = issuer.length === 0 ? empty : !issuer.match(regex) 
    ? unmatch_regex : issuer.length > 50
    ? character_limit : check;

  $: secret_err = secret.length === 0 ? empty : secret.length < 26 
    ? "Secret to short!" : !secret.match(regex)
    ? unmatch_regex : secret.length > 64
    ? character_limit : !secret.match(base32) 
    ? unmatch_base32 : check;
  
  $: errors = Array.from([account_name_err, issuer_err, secret_err]);
  $: disabled = (): boolean => new Blob([JSON.stringify(errors)]).size !== bits + bytes * (errors.length - 1);
  $: token = Array.from([account_name, issuer, secret]);

  $: submit_token = async (): Promise<void> => {
    if (new Blob([JSON.stringify(errors)]).size !== bits + bytes * (errors.length - 1)) {
      return;
    }
    
    await invoke("submit_token", { new_token: token });
  };
</script>

<form class="p-3 grid grid-cols-1 gap-5" on:submit={submit_token} action="/" autocomplete="off">
  <h2 class="text-3xl font-semibold">Create new token</h2>

  <div>
    <label for="account" class="token_form_label">Account name:&nbsp <p class="token_form_p">{account_name_err}</p></label>
    <input 
      id="account" 
      name={"account_name"} 
      placeholder="Enter your account name here." 
      class="token_form_input" 
      bind:value={account_name}
    />

    <label for="institution" class="token_form_label">Issuer:&nbsp <p class="token_form_p">{issuer_err}</p></label>
    <input id="institution" name={"issuer"} placeholder="Enter your issuer here." class="token_form_input" bind:value={issuer} />
    <label for="key" class="token_form_label">Secret:&nbsp <p class="token_form_p">{secret_err}</p></label>
    <input id="key" name={"secret"} placeholder="Enter your secret here." class="token_form_input" bind:value={secret} />
  </div>

  <button type="submit" class="token_form_button disabled:bg-gray-400" disabled={disabled()}>Submit token</button>
</form>
