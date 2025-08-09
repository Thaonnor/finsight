<template>
    <div id="app">
        <div class="layout">
            <nav class="sidebar">
                <h1>finsight</h1>
                <ul>
                    <li><router-link to="/">Dashboard</router-link></li>
                    <li><router-link to="/accounts">Accounts</router-link></li>
                    <li>
                        <router-link to="/categories">Categories</router-link>
                    </li>
                    <li><router-link to="/import">Import</router-link></li>
                </ul>
            </nav>
            <main class="content">
                <router-view />
            </main>
        </div>
    </div>
</template>

<script>
    export default {
        name: 'App',
    };
</script>

<style>
    body {
        margin: 0;
        font-family: system-ui, -apple-system, sans-serif;
    }

    h1 {
        color: #2c3e50;
    }

    /* Everything below this is placeholder */
    .layout {
        display: flex;
        height: 100vh;
    }

    .sidebar {
        width: 200px;
        background: #f5f5f5;
        padding: 20px;
    }

    .sidebar ul {
        list-style: none;
        padding: 0;
    }

    .sidebar li {
        margin: 10px 0;
    }

    .content {
        flex: 1;
        padding: 20px;
    }
</style>
