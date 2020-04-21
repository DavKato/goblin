import App from './App.svelte';
import { initConfig } from './stores';

window.onTauriInit = () => {
  window.tauri.listen('initConfig', (data) => {
    initConfig(data.payload);
  });
};

const app = new App({
  target: document.body,
});

export default app;
