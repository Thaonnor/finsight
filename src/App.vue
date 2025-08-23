<template>
    <v-app>
        <v-navigation-drawer permanent>
            <Sidebar />
        </v-navigation-drawer>
        <v-main>
            <router-view />
        </v-main>
    </v-app>
</template>

<script setup>
    import Sidebar from './components/Sidebar.vue';
</script>

<style>
    html {
        overflow-y: hidden;
    }
</style>
