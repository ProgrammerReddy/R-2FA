<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  
  // biome-ignore lint/style/useConst: <cannot use const, because it should be writable.>
  let account_name = "";
  // biome-ignore lint/style/useConst: <cannot use const, because it should be writable.>
  let issuer = "";
  // biome-ignore lint/style/useConst: <cannot use const, because it should be writable.>
  let secret = "";

  $: token = Array.from([account_name, issuer, secret]);
  const submit_token = async (): Promise<void> => await invoke("submit_token", { new_token: token });
</script>

<form class="p-3 grid grid-cols-1 gap-5" on:submit={submit_token} action="/">
  <h2 class="text-3xl font-semibold">Create new token</h2>

  <div>
    <label for="account" class="token_form_label">Account name</label>
    <input 
      id="account" 
      name={"Account name".toLowerCase().split(" ").join("_")} 
      placeholder="Enter your account name here." 
      class="token_form_input" 
      bind:value={account_name}
    />

    <label for="institution" class="token_form_label">Issuer</label>
    <input 
      id="institution" 
      name={"Issuer".toLowerCase()} 
      placeholder="Enter your issuer here." 
      class="token_form_input" 
      bind:value={issuer}
    />

    <label for="key" class="token_form_label">Secret</label>
    <input 
      id="key" 
      name={"Secret".toLowerCase()} 
      placeholder="Enter your secret here." 
      class="token_form_input" 
      bind:value={secret}
    />
  </div>

  <button type="submit" class="token_form_button">Submit token</button>
</form>
