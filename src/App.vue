<template>
    <v-app>
        <v-navigation-drawer permanent>
            <AppSidebar />
        </v-navigation-drawer>
        <v-main>
            <router-view />
        </v-main>
    </v-app>
</template>

<script setup>
    import AppSidebar from './components/AppSidebar.vue';
</script>

<style>
    html {
        overflow-y: hidden;
    }
</style>
