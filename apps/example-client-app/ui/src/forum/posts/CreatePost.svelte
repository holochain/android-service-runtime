<script lang="ts">
import type { ActionHash, AgentPubKey, AppClient, DnaHash, EntryHash, HolochainError, Record } from "@holochain/client";
import { createEventDispatcher, getContext, onMount } from "svelte";
import { type ClientContext, clientContext } from "../../contexts";
import type { Post } from "./types";

const dispatch = createEventDispatcher();
let client: AppClient;
const appClientContext = getContext<ClientContext>(clientContext);

let title: string = "";
let content: string = "";

$: title, content;
$: isPostValid = true && title !== "" && content !== "";

onMount(async () => {
  client = await appClientContext.getClient();
});

async function createPost() {
  const postEntry: Post = {
    title: title!,
    content: content!,
  };

  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: "forum",
      zome_name: "posts",
      fn_name: "create_post",
      payload: postEntry,
    });
    dispatch("post-created", { postHash: record.signed_action.hashed.hash });
  } catch (e) {
    alert((e as HolochainError).message);
  }
}
</script>

<div>
  <h3>Create Post</h3>

  <div>
    <label for="Title">Title</label>
    <input name="Title" bind:value={title} required />
  </div>
  <div>
    <label for="Content">Content</label>
    <textarea name="Content" bind:value={content} required />
  </div>

  <button disabled={!isPostValid} on:click={() => createPost()}>
    Create Post
  </button>
</div>
