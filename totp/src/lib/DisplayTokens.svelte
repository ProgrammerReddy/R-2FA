<script lang="ts">
  import Icon from "@iconify/svelte";
  import { Link } from "svelte-navigator";
  import HandleTokens from "./HandleTokens.svelte";

  const size: number = 40;
</script>

<section class="preffered-bg">
  <article class="preffered-color">
    <article class="items-center to-center">
      <h1 class="display_add_tokens_h1 pr-5">Overview of R-2FA TOTP tokens</h1>
    </article>

    <article class="w-full">
      <Link to={"/new_token"}>
        <div class="display_handle_add_tokens">
          <div class="w-1/12"><Icon icon="typcn:plus" width={size} height={size} /></div>
          <p class="display_add_tokens_p">Add new token</p>
        </div>
      </Link>

      <HandleTokens size={size} />
    </article>
  </article>
</section>
