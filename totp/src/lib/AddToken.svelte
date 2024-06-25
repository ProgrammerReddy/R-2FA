<script lang="ts">
  import { Link } from "svelte-navigator";
  import Icon from "@iconify/svelte";
  import TokenForm from "./TokenForm.svelte";

  const size: number = 40;
</script>

<section class="preffered-bg">
  <article class="preffered-color">
    <article class="items-center to-center">
      <h1 class="display_add_tokens_h1">Add new R-2FA TOTP token</h1>
    </article>

    <article class="w-full">
      <Link to={"/"}>
        <div class="display_handle_add_tokens">
          <div class="w-1/12"><Icon icon="mingcute:enter-door-line" width={size} height={size} /></div>
          <p class="display_add_tokens_p">back</p>
        </div>
      </Link>

      <TokenForm />
    </article>
  </article>
</section>
