<script lang="ts">
  import Greet from "./lib/Greet.svelte";
</script>

<main class="container">
  <h1>Welcome to Reddy2FA!</h1>

  <div class="row">
    <a href="https://github.com/RedIsGaming/Reddy2FA" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
  </div>
  <p>Click on the logo to see more.</p>
  <div class="row">
    <Greet />
  </div>
</main>
