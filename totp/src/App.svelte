<script lang="ts">
  import { Router, Route } from "svelte-navigator";
  import DisplayTokens from "./lib/DisplayTokens.svelte";
  import AddToken from "./lib/AddToken.svelte";
</script>

<main class="totp">
  <Router>
    <Route path="/" component={DisplayTokens} />
    <Route path="/new_token" component={AddToken} />
  </Router>
</main>
