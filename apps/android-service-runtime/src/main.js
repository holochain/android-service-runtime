import "./style.css";
import App from "./App.svelte";
import { initTranslations } from './translations';
import { waitLocale } from "svelte-i18n";

initTranslations();
await waitLocale();

const app = new App({
  target: document.getElementById("app"),
});

export default app;
