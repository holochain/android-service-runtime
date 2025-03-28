<script lang="ts">
import type { ActionHash, AgentPubKey, AppClient, DnaHash, EntryHash, HolochainError, Record } from "@holochain/client";
import { decode } from "@msgpack/msgpack";
import { createEventDispatcher, getContext, onMount } from "svelte";
import { type ClientContext, clientContext } from "../../contexts";
import EditPost from "./EditPost.svelte";
import type { Post } from "./types";

let client: AppClient;
const appClientContext = getContext<ClientContext>(clientContext);
const dispatch = createEventDispatcher();

let loading: boolean = false;
let editing = false;
let error: HolochainError | undefined;
let record: Record | undefined;
let post: Post | undefined;

export let postHash: ActionHash;

$: editing, error, loading, record, post;

onMount(async () => {
  if (postHash === undefined) {
    throw new Error(`The postHash input is required for the PostDetail element`);
  }
  client = await appClientContext.getClient();
  await fetchPost();
});

async function fetchPost() {
  loading = true;
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: "forum",
      zome_name: "posts",
      fn_name: "get_latest_post",
      payload: postHash,
    });
    if (record) {
      post = decode((record.entry as any).Present.entry) as Post;
    }
  } catch (e) {
    error = e as HolochainError;
  } finally {
    loading = false;
  }
}

async function deletePost() {
  try {
    await client.callZome({
      cap_secret: null,
      role_name: "forum",
      zome_name: "posts",
      fn_name: "delete_post",
      payload: postHash,
    });
    dispatch("post-deleted", { postHash: postHash });
  } catch (e) {
    alert((e as HolochainError).message);
  }
}
</script>

{#if loading}
  <progress />
{:else if error}
  <div class="alert">Error fetching the post: {error.message}</div>
{:else if editing}
  <EditPost
    originalPostHash={postHash}
    currentRecord={record}
    on:post-updated={async () => {
      editing = false;
      await fetchPost();
    }}
    on:edit-canceled={() => {
      editing = false;
    }}
  />
{:else}
  <section>
    <div>
      <span><strong>Title:</strong></span>
      <span>{post?.title}</span>
    </div>
    <div>
      <span><strong>Content:</strong></span>
      <span>{post?.content}</span>
    </div>

    <div>
      <button
        on:click={() => {
          editing = true;
        }}
      >edit</button>
      <button on:click={() => deletePost()}>delete</button>
    </div>
  </section>
{/if}
