import { createApp } from 'vue';
import { createPinia } from 'pinia';
import 'normalize.css';

import App from './App.vue';
import './styles/index.less';

const app = createApp(App);

app.use(createPinia());

app.mount('#app');
