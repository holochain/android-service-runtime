<script lang="ts">
import type { ActionHash, AppClient } from "@holochain/client";
import { AppWebsocket } from "@holochain/client";
import { onMount, setContext } from "svelte";
import AllPosts from "./forum/posts/AllPosts.svelte";
import CreatePost from "./forum/posts/CreatePost.svelte";
import { type ClientContext, clientContext } from "./contexts";

let client: AppClient | undefined;
let loading = false;

const appClientContext = {
  getClient: async () => {
    if (!client) {
      client = await AppWebsocket.connect();
    }
    return client;
  },
};

onMount(async () => {
  loading = true;
  try {
    client = await appClientContext.getClient();
  } catch (e) {
    console.error(e);
  } finally {
    loading = false;
  }
});

setContext<ClientContext>(clientContext, appClientContext);
</script>

<main>
  {#if loading}
    <progress />
  {:else}
    <div>
      <h2>Welcome to the Forum hApp</h2>
      <AllPosts />
      <CreatePost />
    </div>
  {/if}
</main>
