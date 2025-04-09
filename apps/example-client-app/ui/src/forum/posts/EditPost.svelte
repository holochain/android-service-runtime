<script lang="ts">
import type { ActionHash, AgentPubKey, AppClient, DnaHash, EntryHash, HolochainError, Record } from "@holochain/client";
import { decode } from "@msgpack/msgpack";
import { createEventDispatcher, getContext, onMount } from "svelte";
import { type ClientContext, clientContext } from "../../contexts";
import type { Post } from "./types";

let client: AppClient;
const appClientContext = getContext<ClientContext>(clientContext);
const dispatch = createEventDispatcher();

export let currentRecord!: Record;
export let originalPostHash!: ActionHash;

let currentPost: Post = decode((currentRecord.entry as any).Present.entry) as Post;
let title: string | undefined = currentPost.title;
let content: string | undefined = currentPost.content;

$: title, content;
$: isPostValid = true && title !== "" && content !== "";

onMount(async () => {
  if (!currentRecord) {
    throw new Error(`The currentRecord input is required for the EditPost element`);
  }
  if (!originalPostHash) {
    throw new Error(`The originalPostHash input is required for the EditPost element`);
  }
  client = await appClientContext.getClient();
});

async function updatePost() {
  const post: Post = {
    title: title!,
    content: content!,
  };

  try {
    const updateRecord: Record = await client.callZome({
      cap_secret: null,
      role_name: "forum",
      zome_name: "posts",
      fn_name: "update_post",
      payload: {
        original_post_hash: originalPostHash,
        previous_post_hash: currentRecord.signed_action.hashed.hash,
        updated_post: post,
      },
    });

    dispatch("post-updated", { actionHash: updateRecord.signed_action.hashed.hash });
  } catch (e) {
    alert((e as HolochainError).message);
  }
}
</script>

<section>
  <div>
    <label for="Title">Title</label>
    <input name="Title" bind:value={title} required />
  </div>
  <div>
    <label for="Content">Content</label>
    <textarea name="Content" bind:value={content} required />
  </div>

  <div>
    <button on:click={() => dispatch("edit-canceled")}>Cancel</button>
    <button disabled={!isPostValid} on:click={() => updatePost()}>
      Edit Post
    </button>
  </div>
</section>
