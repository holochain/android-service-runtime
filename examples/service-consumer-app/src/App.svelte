<script lang="ts">
  import { installApp, ensureAppWebsocket, isAppInstalled } from "tauri-plugin-holochain-service-consumer-api";
  import Labelled from './Labelled.svelte';
  import happUrl from "./forum.happ?url";
  import { AppWebsocket } from "@holochain/client";
  import { decode } from "@msgpack/msgpack";
  import { v7 as uuidv7 } from "uuid";
  
  let adminPort;
  let appId = `forum-${uuidv7()}`;
  let networkSeed = "p2p-shipyard-dev-2024-09-30";
  let newPost = {title: "", content: ""};
  let allPosts = [];
  let appWs: AppWebsocket | undefined = undefined;
  let appWsAuth;
  let isInstalled;

  const loadHolochainClient = async () => {
    appWs = await AppWebsocket.connect();
  };

  const setupApp = async () => {
    isInstalled = await isAppInstalled(appId);
    if (!isInstalled) {
      await installForumApp();
      isInstalled = true;
    }
    await createAppWebsocketAuth();
  }

  const installForumApp = async () => {
    await installApp({
      appId,
      appBundleBytes: new Uint8Array(await (await fetch(happUrl)).arrayBuffer()),
      membraneProofs: {},
      agent: null,
      networkSeed,
    });
    console.log("app installed");
  };

  const createAppWebsocketAuth = async () => {
    appWsAuth = await ensureAppWebsocket(appId);
    console.log("app websocket auth", appWsAuth);
  }

  const loadIsAppInstalled = async () => {
    isInstalled = await isAppInstalled(appId);
    console.log("isAppInstalled", isInstalled);
  }

  const callZomeCreatePost = async () => {
    if(!appWs) throw Error("No AppWebsocket connected");

    appWs.callZome({
      role_name: "forum",
      zome_name: "posts",
      fn_name: "create_post",
      payload: newPost,
    });
  };
  
  const callZomeGetAllPosts = async () => {
    if(!appWs) throw Error("No AppWebsocket connected");

   const links = await appWs.callZome({
      cap_secret: null,
      role_name: 'forum',
      zome_name: 'posts',
      fn_name: 'get_all_posts',
      payload: null,
    });
    const hashes = links.map(l => l.target);

    allPosts = [];

    await Promise.all(hashes.map(async (hash) => {
      const record = await appWs.callZome({
        cap_secret: null,
        role_name: 'forum',
        zome_name: 'posts',
        fn_name: 'get_latest_post',
        payload: hash,
      });
      if (record) {
        const post = decode((record.entry as any).Present.entry);
        allPosts = [...allPosts, post];
      }
    }));
  }

  let interval = setInterval(async () => {
    if(!adminPort) {
      adminPort = await loadHolochainClient();
    } else {
      clearInterval(interval);
    }
  }, 500);

  setupApp();
</script>

<main class="container">
  <h1 style="line-height: 2.5rem;">tauri-plugin-holochain-service-consumer demo</h1>

  <div class="my-4 flex-center">
    <h2>Install Forum App</h2>
    <Labelled label="App Id">
      <input bind:value={appId} />
    </Labelled>
    <Labelled label="Network Seed">
      <input bind:value={networkSeed} />
    </Labelled>
    <div>
      <button on:click={installForumApp}>Install App</button>
    </div>
  </div>


  <div class="my-4 flex-center">
    <h2>Create App Websocket Auth</h2>
    {#if appWsAuth}
      <Labelled label="Port">
        {appWsAuth.port}
      </Labelled>
      <Labelled label="Token">
        <textarea disabled cols="40" rows="10">{appWsAuth.token}</textarea>
      </Labelled>
    {/if}
    <div>
      <button on:click={createAppWebsocketAuth}>Create App WS Auth</button>
    </div>
  </div>


  <div class="my-4 flex-center">
    <h2>Is App Installed?</h2>
    {#if isInstalled !== null && isInstalled !== undefined}
      <b>{isInstalled ? "Yes" : "No"}</b><br/>
    {/if}
    <div>
      <button on:click={loadIsAppInstalled}>Check if app is installed</button>
    </div>
  </div>

   <div class="my-4 flex-center">
    <h2>Zome Call</h2>
    <p>Call zome fn "create_post"</p>
    <Labelled label="title">
      <input bind:value={newPost.title} />
    </Labelled>
    <Labelled label="content">
      <input bind:value={newPost.content} />
    </Labelled>
    <div>
      <button on:click={callZomeCreatePost} disabled={newPost.title?.length === 0 || newPost.content?.length === 0}>Create Post</button>
    </div>
   </div>

   <div class="my-4 flex-center">
    <h2>Zome Call</h2>
    <p>Call zome fn "get_all_posts"</p>
    <div>
      <button on:click={callZomeGetAllPosts}>Get All Posts</button>
    </div>
    <ul>
      {#each allPosts as post}
       <li>
        <Labelled label="Title">{post.title}</Labelled>
        <Labelled label="Content">{post.content}</Labelled>
       </li>
      {/each}
    </ul>
   </div>
</main>

<style>
  .my-4 {
    margin-top: 1rem;
    margin-bottom: 1rem;
  }
  .flex-center {
    display: flex;
    flex-direction: column;
    align-items: center;
  }
</style>