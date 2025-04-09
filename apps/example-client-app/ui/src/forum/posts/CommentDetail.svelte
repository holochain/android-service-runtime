<script lang="ts">
import type { ActionHash, AgentPubKey, AppClient, DnaHash, EntryHash, HolochainError, Record } from "@holochain/client";
import { decode } from "@msgpack/msgpack";
import { createEventDispatcher, getContext, onMount } from "svelte";
import { type ClientContext, clientContext } from "../../contexts";
import type { Comment } from "./types";

let client: AppClient;
const appClientContext = getContext<ClientContext>(clientContext);
const dispatch = createEventDispatcher();

let loading: boolean = false;
let error: HolochainError | undefined;
let record: Record | undefined;
let comment: Comment | undefined;

export let commentHash: ActionHash;

$: error, loading, record, comment;

onMount(async () => {
  if (commentHash === undefined) {
    throw new Error(`The commentHash input is required for the CommentDetail element`);
  }
  client = await appClientContext.getClient();
  await fetchComment();
});

async function fetchComment() {
  loading = true;
  try {
    record = await client.callZome({
      cap_secret: null,
      role_name: "forum",
      zome_name: "posts",
      fn_name: "get_comment",
      payload: commentHash,
    });
    if (record) {
      comment = decode((record.entry as any).Present.entry) as Comment;
    }
  } catch (e) {
    error = e as HolochainError;
  } finally {
    loading = false;
  }
}

async function deleteComment() {
  try {
    await client.callZome({
      cap_secret: null,
      role_name: "forum",
      zome_name: "posts",
      fn_name: "delete_comment",
      payload: commentHash,
    });
    dispatch("comment-deleted", { commentHash: commentHash });
  } catch (e) {
    alert((e as HolochainError).message);
  }
}
</script>

{#if loading}
  <progress />
{:else if error}
  <div class="alert">Error fetching the comment: {error.message}</div>
{:else}
  <section>
    <div>
      <span><strong>Comment:</strong></span>
      <span>{comment?.comment}</span>
    </div>

    <div>
      <button on:click={() => deleteComment()}>delete</button>
    </div>
  </section>
{/if}
