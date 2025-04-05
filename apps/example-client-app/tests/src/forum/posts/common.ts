import {
  ActionHash,
  AppBundleSource,
  fakeActionHash,
  fakeAgentPubKey,
  fakeDnaHash,
  fakeEntryHash,
  hashFrom32AndType,
  NewEntryAction,
  Record,
} from "@holochain/client";
import { CallableCell } from "@holochain/tryorama";

export async function samplePost(cell: CallableCell, partialPost = {}) {
  return {
    ...{
      title: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
      content: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
    },
    ...partialPost,
  };
}

export async function createPost(cell: CallableCell, post = undefined): Promise<Record> {
  return cell.callZome({
    zome_name: "posts",
    fn_name: "create_post",
    payload: post || await samplePost(cell),
  });
}

export async function sampleComment(cell: CallableCell, partialComment = {}) {
  return {
    ...{
      comment: "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
      post_hash: (await createPost(cell)).signed_action.hashed.hash,
    },
    ...partialComment,
  };
}

export async function createComment(cell: CallableCell, comment = undefined): Promise<Record> {
  return cell.callZome({
    zome_name: "posts",
    fn_name: "create_comment",
    payload: comment || await sampleComment(cell),
  });
}
