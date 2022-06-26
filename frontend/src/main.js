import wasm from '../../backend/Cargo.toml'
import App from './App.svelte';
import 'carbon-components-svelte/css/g80.css';

const init = async () => {
	const bindings = await wasm();
	const app = new App({
		target: document.body,
		props: {
			bindings,
		}
	});
};

init().then(() => console.log('App ready'));