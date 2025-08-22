import { createApp } from 'vue';
import App from './App.vue';
import router from './router/router.js';
import './styles/globals.css';
import { createVuetify } from 'vuetify';
import 'vuetify/styles';

const vuetify = createVuetify({
    theme: {
        defaultTheme: 'dark',
    },
});

createApp(App).use(router).use(vuetify).mount('#app');
