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
  SignalType,
} from "@holochain/client";
import { SignalType } from "@holochain/client";
import { getContext, onMount } from "svelte";
import { type ClientContext, clientContext } from "../../contexts";
import CommentDetail from "./CommentDetail.svelte";
import type { Comment, PostsSignal } from "./types";

let client: AppClient;
const appClientContext = getContext<ClientContext>(clientContext);

let hashes: Array<ActionHash> | undefined = [];
let loading: boolean;
let error: any = undefined;

export let postHash: ActionHash;

$: hashes, loading, error;

onMount(async () => {
  if (postHash === undefined) {
    throw new Error(`The postHash input is required for the CommentsForPost element`);
  }
  client = await appClientContext.getClient();
  await fetchComments();

  client.on("signal", async signal => {
    if (!(SignalType.App in signal)) return;
    if (signal.App.zome_name !== "posts") return;
    const payload = signal.App.payload as PostsSignal;
    if (!(payload.type === "EntryCreated" && payload.app_entry.type === "Comment")) return;
    await fetchComments();
  });
});

async function fetchComments() {
  loading = true;
  try {
    const links: Array<Link> = await client.callZome({
      cap_secret: null,
      role_name: "forum",
      zome_name: "posts",
      fn_name: "get_comments_for_post",
      payload: postHash,
    });
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
  <div class="alert">Error fetching comments: ${error.message}.</div>
{:else if hashes.length === 0}
  <div class="alert">No comments found for this post.</div>
{:else}
  <div>
    {#each hashes as hash}
      <CommentDetail commentHash={hash} on:comment-deleted={fetchComments} />
    {/each}
  </div>
{/if}
