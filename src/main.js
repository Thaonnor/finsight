import { createApp } from 'vue';
import App from './App.vue';
import router from './router/router.js';
import './styles/globals.css';
import { Quasar } from 'quasar';
import quasarLang from 'quasar/lang/en-US';
import 'quasar/dist/quasar.css';

createApp(App)
    .use(router)
    .use(Quasar, {
        plugins: {},
        lang: quasarLang,
    })
    .mount('#app');
