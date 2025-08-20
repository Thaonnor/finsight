<template>
    <nav class="sidebar">
        <h1>finsight</h1>
        <ul>
            <li><router-link to="/">Dashboard</router-link></li>
            <li><router-link to="/reports">Reports</router-link></li>
            <li><router-link to="/accounts">Accounts</router-link></li>
            <li>
                <router-link to="/categories">Categories</router-link>
            </li>
            <li><router-link to="/import">Import</router-link></li>
        </ul>

        <hr />

        <div class="accounts-section">
            <AccountNavItem
                v-for="account in accounts"
                :key="account.id"
                :accountName="account.name"
                :accountBalance="account.balance"
                :accountId="account.id"
            />
        </div>
    </nav>
</template>

<script setup>
    import { ref, onMounted } from 'vue';
    import { invoke } from '@tauri-apps/api/core';
    import AccountNavItem from './AccountNavItem.vue';

    const accounts = ref([]);

    const getAccounts = async () => {
        try {
            const accountList = await invoke('get_accounts');

            // Fetch balance for each account
            for (let account of accountList) {
                account.balance = await invoke('get_balance', {
                    accountId: account.id,
                });
            }

            accounts.value = accountList;
        } catch (error) {
            console.error('Failed to load accounts:', error);
        }
    };

    onMounted(() => {
        getAccounts();
    });
</script>

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
