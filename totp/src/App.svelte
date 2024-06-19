<script lang="ts">
  import { Router, Route } from "svelte-navigator";
  import Tokens from "./lib/Tokens.svelte";
</script>

<main class="h-screen preffered-bg">
  <Router>
    <Route path="/" component={Tokens} />
  </Router>
</main>
