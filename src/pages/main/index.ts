import { mount } from 'svelte';
import App from './index.svelte'
import '../../../node_modules/flowbite/dist/flowbite.min.css';
import './index.css';

const app = mount(App, { target: document.getElementById("app") as HTMLElement });

export default app;
