import "./style.css";
import App from "./App.svelte";
import { initTranslations } from './translations';

initTranslations();

const app = new App({
  target: document.getElementById("app"),
});

export default app;
