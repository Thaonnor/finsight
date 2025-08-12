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
    background: var(--surface-2);
    color: var(--text);
    padding: 20px;
}

.sidebar h1 {
    color: var(--accent);
}

.sidebar ul {
    list-style: none;
    padding: 0;
}

.sidebar li {
    margin: 10px 0;
}

.sidebar a {
    color: var(--text);
    text-decoration: none;
    padding: 8px 12px;
    border-radius: 4px;
    display: block;
    transition: background-color 0.2s ease;
}

.sidebar a:hover {
    color: var(--accent);
    background: var(--surface-1);
}
</style>
