import { createApp } from 'vue';
import { createPinia } from 'pinia';
import PrimeVue from 'primevue/config';

import 'normalize.css';
import 'primevue/resources/themes/md-dark-indigo/theme.css';
import 'primevue/resources/primevue.min.css';
import 'primeicons/primeicons.css';

import App from './App.vue';
import './styles/index.less';

const app = createApp(App);

app.use(createPinia());
app.use(PrimeVue);

app.mount('#app');
