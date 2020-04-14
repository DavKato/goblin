import App from './App.svelte';
import { initConfig } from './stores.js';

window.onTauriInit = () => {
  window.tauri.listen('initConfig', function (data) {
    initConfig(data.payload);
  });
};

const app = new App({
  target: document.body
});

export default app;
