<script lang="ts">
import type {
  ActionHash,
  AgentPubKey,
  AppClient,
  EntryHash,
  HolochainError,
  Link,
  NewEntryAction,
  Record,
} from "@holochain/client";
import { SignalType } from "@holochain/client";
import { getContext, onMount } from "svelte";
import { type ClientContext, clientContext } from "../../contexts";
import PostDetail from "./PostDetail.svelte";
import type { PostsSignal } from "./types";

let client: AppClient;
const appClientContext = getContext<ClientContext>(clientContext);

let hashes: Array<ActionHash> = [];
let loading = false;
let error: any = undefined;

$: hashes, loading, error;

onMount(async () => {
  client = await appClientContext.getClient();
  await fetchPosts();
  client.on("signal", signal => {
    if (signal.type !== SignalType.App) return;
    if (signal.value.zome_name !== "posts") return;
    const payload = signal.value.payload as PostsSignal;
    if (payload.type !== "EntryCreated") return;
    if (payload.app_entry.type !== "Post") return;
    hashes = [...hashes, payload.action.hashed.hash];
  });
});

async function fetchPosts() {
  loading = true;
  try {
    const links: Array<Link> = await client.callZome({
      cap_secret: null,
      role_name: "forum",
      zome_name: "posts",
      fn_name: "get_all_posts",
      payload: null,
    });
    if (links.length) {
      hashes = links.map(l => l.target);
    }
    hashes = links.map(l => l.target);
  } catch (e) {
    error = e as HolochainError;
  } finally {
    loading = false;
  }
}
</script>

{#if loading}
  <progress />
{:else if error}
  <div class="alert">Error fetching the posts: {error.message}.</div>
{:else if !hashes.length}
  <div class="alert">No posts found.</div>
{:else}
  <div>
    {#each hashes as hash}
      <PostDetail postHash={hash} on:post-deleted={() => fetchPosts()} />
    {/each}
  </div>
{/if}
