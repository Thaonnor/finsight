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
    {
        path: '/accounts',
        name: 'Accounts',
        component: () => import('../views/Accounts.vue'),
    },
    {
        path: '/categories',
        name: 'Categories',
        component: () => import('../views/Categories.vue'),
    },
    {
        path: '/import',
        name: 'Import',
        component: () => import('../views/Import.vue'),
    },
    {
        path: '/accounts/:id',
        name: 'AccountDetails',
        component: () => import('../views/AccountDetails.vue'),
    },
    {
        path: '/reports',
        name: 'Reports',
        // TODO: Implement reports view
        component: () => import('../views/Dashboard.vue'),
    }
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
