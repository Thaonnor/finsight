import { createRouter, createWebHistory } from 'vue-router';

/**
 * Application route definitions.
 * @type {Array<Object>}
 */
const routes = [
    {
        path: '/',
        name: 'Dashboard',
        component: () => import('../views/Dashboard.vue'),
    },
];

/**
 * Creates and configures the Vue Router instance.
 * Uses HTML5 history mode for clean URLs.
 * @returns {Router} Configured Vue router instance
 */
const router = createRouter({
    history: createWebHistory(),
    routes,
});

export default router;
