<template>
    <v-toolbar>
        <v-toolbar-title>finsight</v-toolbar-title>
    </v-toolbar>
    <v-list>
        <v-list-item to="/" title="Dashboard" />
        <v-list-item to="/accounts" title="Accounts" />
        <v-list-item to="/categories" title="Categories" />
        <v-list-item to="/import" title="Import" />
        <v-divider thickness="2" />
        <v-list-subheader title="Accounts" class="text-uppercase" />
        <AccountNavItem
            v-for="account in accountsWithBalances"
            :key="account.id"
            :account-name="account.name"
            :account-balance="account.balance"
            :account-id="account.id"
        />
    </v-list>
</template>

<script setup>
    import { ref, onMounted, computed } from 'vue';
    import { invoke } from '@tauri-apps/api/core';
    import AccountNavItem from './AccountNavItem.vue';
    import { useAccounts } from '../composables/useAccounts.js';

    const accounts = ref([]);
    const { balances, getBalance } = useAccounts();

    onMounted(() => {
        getAccounts();
    });

    const getAccounts = async () => {
        try {
            const accountList = await invoke('get_accounts');

            // Fetch balance for each account
            for (let account of accountList) {
                await getBalance(account.id);
            }

            accounts.value = accountList;
        } catch (error) {
            console.error('Failed to load accounts:', error);
        }
    };

    const accountsWithBalances = computed(() => {
        return accounts.value.map((account) => ({
            ...account,
            balance: balances[account.id] || 0,
        }));
    });
</script>

<style scoped></style>
