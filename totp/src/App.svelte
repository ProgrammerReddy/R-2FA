<script lang="ts">
  import Intro from "./lib/Intro.svelte";
  import { Router, Route } from "svelte-navigator";
  import Tokens from "./lib/Tokens.svelte";
</script>

<main class="h-screen preffered-bg">
  <Router>
    <Route path="/" component={Intro} />
    <Route path="/tokens" component={Tokens} />
  </Router>
</main>
