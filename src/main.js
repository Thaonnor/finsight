import { createApp } from 'vue';
import App from './App.vue';
import router from './router/router.js';
import './styles/globals.css';

// Vuetify
import 'vuetify/styles';
import '@mdi/font/css/materialdesignicons.css';
import { createVuetify } from 'vuetify';
import * as components from 'vuetify/components';
import * as directives from 'vuetify/directives';

const vuetify = createVuetify({
    components,
    directives,
    theme: {
        defaultTheme: 'dark',
    }
})

createApp(App).use(router).use(vuetify).mount('#app');
