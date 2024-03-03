<script lang="ts">
  import Intro from "./lib/Intro.svelte";
  import { Router, Route } from "svelte-navigator";
  import Tokens from "./lib/Tokens.svelte";
  import AddToken from "./lib/AddToken.svelte";
</script>

<main class="preffered-bg h-screen">
  <Router>
    <Route path="/" component={Intro} />
    <Route path="/tokens" component={Tokens} />
    <Route path="/tokens/add" component={AddToken} />
  </Router>
</main>
