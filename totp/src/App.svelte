<script lang="ts">
  import { Router, Route } from "svelte-navigator";
  import Tokens from "./lib/Tokens.svelte";
  import AddToken from "./lib/AddToken.svelte";
</script>

<main class="h-screen preffered-bg">
  <Router>
    <Route path="/" component={Tokens} />
    <Route path="/new_token" component={AddToken} />
  </Router>
</main>
