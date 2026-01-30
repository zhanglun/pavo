import { mount } from 'svelte';
import App from './index.svelte'
import './index.css'

const app = mount(App, { target: document.getElementById("app") as HTMLElement });

export default app;
