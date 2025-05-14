import "./style.css";
import { initTranslations } from './translations';
import App from "./App.svelte";

const app = new App({
  target: document.getElementById("app"),
});

initTranslations();

export default app;
