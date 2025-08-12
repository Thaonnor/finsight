<template>
    <div id="app">
        <div class="layout">
            <Sidebar />
            <main class="content">
                <router-view />
            </main>
        </div>
    </div>
</template>

<script setup>
    import Sidebar from './components/Sidebar.vue';
</script>

<style>
    body {
        margin: 0;
        font-family: system-ui, -apple-system, sans-serif;
    }

    .layout {
        display: flex;
        height: 100vh;
    }

    .content {
        flex: 1;
        padding: 20px;
    }
</style>
