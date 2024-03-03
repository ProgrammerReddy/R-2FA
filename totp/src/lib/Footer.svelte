<script lang="ts">
  import { Link } from "svelte-navigator";
  import Icon from "@iconify/svelte";
</script>

<footer class="w-full bottom-0 left-0 fixed to-center h-12 items-center bg-gray-200 dark:bg-gray-900">
  <Link to={"/"}>
    <Icon icon={"ion:home"} width={36} class="invert-0 dark:invert" />
  </Link>
</footer>
