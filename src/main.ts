import './app.css'
import App from './App.svelte'

import init from 'bevy_sprites';

const app = new App({
  target: document.getElementById('app'),
})

init().then((res) => {
  console.log('init wasm-pack');
  res.run();
});

export default app
