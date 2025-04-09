<script lang="ts">
import type { ActionHash, AgentPubKey, AppClient, DnaHash, EntryHash, HolochainError, Record } from "@holochain/client";
import { createEventDispatcher, getContext, onMount } from "svelte";
import { type ClientContext, clientContext } from "../../contexts";
import type { Comment } from "./types";

const dispatch = createEventDispatcher();
let client: AppClient;
const appClientContext = getContext<ClientContext>(clientContext);

let comment: string = "";

export let postHash!: ActionHash;

$: comment, postHash;
$: isCommentValid = true && comment !== "";

onMount(async () => {
  if (postHash === undefined) {
    throw new Error(`The postHash input is required for the CreateComment element`);
  }
  client = await appClientContext.getClient();
});

async function createComment() {
  const commentEntry: Comment = {
    comment: comment!,
    post_hash: postHash!,
  };

  try {
    const record: Record = await client.callZome({
      cap_secret: null,
      role_name: "forum",
      zome_name: "posts",
      fn_name: "create_comment",
      payload: commentEntry,
    });
    dispatch("comment-created", { commentHash: record.signed_action.hashed.hash });
  } catch (e) {
    alert((e as HolochainError).message);
  }
}
</script>

<div>
  <h3>Create Comment</h3>

  <div>
    <label for="Comment">Comment</label>
    <textarea name="Comment" bind:value={comment} required />
  </div>

  <button disabled={!isCommentValid} on:click={() => createComment()}>
    Create Comment
  </button>
</div>
