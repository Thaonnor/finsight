import { createApp } from 'vue';
import App from './App.vue';
import router from './router/router.js';
import './styles/globals.css';
import PrimeVue from 'primevue/config';
import Aura from '@primeuix/themes/aura';

createApp(App)
    .use(router)
    .use(PrimeVue, {
        unstyled: true,
    })
    .mount('#app');
