// Demo UI for the in-process tauri-plugin-holochain. Uses @holochain/client to
// connect to the conductor the plugin injected into this webview, make signed
// zome calls, and receive a signal. With the plugin's default direct mode this
// all flows over Tauri IPC — no app websocket.
import { AppWebsocket } from "@holochain/client";

const report = (step, ok, detail) => {
  try {
    window.__TAURI__?.core?.invoke("report", { step, ok, detail });
  } catch (_) {}
};
const show = (id, text, ok) => {
  const el = document.getElementById(id);
  if (el) {
    el.textContent = text;
    el.className = ok === undefined ? "" : ok ? "ok" : "err";
  }
  if (ok !== undefined) report(id, ok, text);
};

// 1. Prove the plugin injected an env into this webview. Direct mode injects
// __HC_TAURI_HOLOCHAIN__ (no websocket); legacy mode injects __HC_LAUNCHER_ENV__.
const tauriEnv = window.__HC_TAURI_HOLOCHAIN__;
const wsEnv = window.__HC_LAUNCHER_ENV__;
if (tauriEnv) {
  show(
    "env",
    "direct Tauri IPC — INSTALLED_APP_ID=" +
      tauriEnv.INSTALLED_APP_ID +
      ", hasSigner=" +
      !!window.__HC_ZOME_CALL_SIGNER__ +
      ", hasSignalBridge=" +
      !!tauriEnv.subscribeSignals,
    true
  );
} else if (wsEnv && wsEnv.APP_INTERFACE_PORT) {
  show("env", "websocket — APP_INTERFACE_PORT=" + wsEnv.APP_INTERFACE_PORT, true);
} else {
  show("env", "no holochain env injected", false);
}

try {
  const client = await AppWebsocket.connect();
  const info = await client.appInfo();
  show(
    "conn",
    "connected — appInfo.installed_app_id = " + (info?.installed_app_id ?? "(none)"),
    true
  );

  // 5. Listen for signals before triggering one. create_post's post_commit hook
  // emits an EntryCreated/LinkCreated signal, which must reach us here.
  let signalSeen = false;
  client.on("signal", (signal) => {
    if (signalSeen) return;
    signalSeen = true;
    show(
      "signal",
      "received — type=" +
        signal.type +
        " zome=" +
        (signal.value?.zome_name ?? "?"),
      true
    );
  });

  // 3. Read: get_all_posts.
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

  // 4. Write: create_post (commits an entry, which emits a signal).
  try {
    await client.callZome({
      role_name: "forum",
      zome_name: "posts",
      fn_name: "create_post",
      payload: { title: "hello", content: "from direct mode" },
    });
    show("create", "create_post OK — should emit a signal", true);
  } catch (e) {
    show("create", "create_post error: " + (e?.message || e), false);
  }

  // Fail the signal check if nothing arrives shortly after the commit.
  setTimeout(() => {
    if (!signalSeen) show("signal", "no signal received within 10s", false);
  }, 10000);
} catch (e) {
  show("conn", "client error: " + (e?.message || e), false);
}
