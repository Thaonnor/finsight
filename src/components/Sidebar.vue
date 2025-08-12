<template>
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
</template>

<script setup></script>

<style scoped>
.sidebar {
    width: 200px;
    background: var(--color-background-secondary);
    color: var(--color-text);
    padding: 20px;
}

.sidebar h1 {
    color: var(--color-accent);
    font-size: var(--font-size-page-title);
}

.sidebar ul {
    list-style: none;
    padding: 0;
}

.sidebar li {
    margin: 10px 0;
}

.sidebar a {
    color: var(--color-text);
    text-decoration: none;
}

.sidebar a:hover {
    color: var(--color-accent);
}
</style>
