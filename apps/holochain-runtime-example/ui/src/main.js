// Demo UI for the in-process tauri-plugin-holochain. Uses @holochain/client
// (from npm, bundled by vite) to connect to the conductor the plugin injected
// into this webview, and makes a signed zome call.
import { AppWebsocket } from "@holochain/client";

const $ = (id) => document.getElementById(id);
const report = (step, ok, detail) => {
  try {
    window.__TAURI__?.core?.invoke("report", { step, ok, detail });
  } catch (_) {}
};
const show = (id, text, ok) => {
  const el = $(id);
  el.textContent = text;
  el.className = ok === undefined ? "" : ok ? "ok" : "err";
  if (ok !== undefined) report(id, ok, text);
};

// 1. Prove the plugin injected the launcher env into this webview.
const env = window.__HC_LAUNCHER_ENV__;
if (env && env.APP_INTERFACE_PORT) {
  show(
    "env",
    JSON.stringify(
      {
        INSTALLED_APP_ID: env.INSTALLED_APP_ID,
        APP_INTERFACE_PORT: env.APP_INTERFACE_PORT,
        APP_INTERFACE_TOKEN_len: env.APP_INTERFACE_TOKEN?.length,
        hasZomeCallSigner: !!window.__HC_ZOME_CALL_SIGNER__,
      },
      null,
      2
    ),
    true
  );
} else {
  show("env", "__HC_LAUNCHER_ENV__ NOT injected", false);
}

// 2 + 3. Connect with @holochain/client and make a real signed zome call.
try {
  const client = await AppWebsocket.connect();
  const info = await client.appInfo();
  show("conn", "connected — appInfo.installed_app_id = " + (info?.installed_app_id ?? "(none)"), true);

  try {
    const posts = await client.callZome({
      role_name: "forum",
      zome_name: "posts",
      fn_name: "get_all_posts",
      payload: null,
    });
    show(
      "zome",
      "get_all_posts returned " +
        (Array.isArray(posts) ? posts.length : JSON.stringify(posts)) +
        " record(s) — zome call + signing OK",
      true
    );
  } catch (e) {
    show("zome", "zome call error: " + (e?.message || e), false);
  }
} catch (e) {
  show("conn", "client error: " + (e?.message || e), false);
}
