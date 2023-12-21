import './app.css'
import App from './App.svelte'

import init from "../bevy_sprites/Cargo.toml";

if (!import.meta.env.SSR) {
  init().then((module: any) => {
    console.log("Loaded WASM");
    module.run();
  });
}

const app = new App({
  target: document.getElementById('app'),
})

export default app
