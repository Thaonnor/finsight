<template>
    <div class="account-details">
        <div class="account-header">
            <h1>Account Name Here</h1>
            <div class="account-balance">
                <span>Account Balance: {{ balance }}</span>
            </div>
        </div>

        <table class="transactions">
            <thead>
                <tr>
                    <th>Date</th>
                    <th>Description</th>
                    <th>Category</th>
                    <th>Type</th>
                    <th>Amount</th>
                    <th>Options</th>
                </tr>
            </thead>
            <tbody>
                <tr v-for="transaction in transactions" :key="transaction.id">
                    <td>{{ transaction.transaction_date }}</td>
                    <td>{{ transaction.description }}</td>
                    <td>{{ transaction.category_id }}</td>
                    <td>{{ transaction.transaction_type }}</td>
                    <td>{{ formatCurrency(transaction.amount_cents) }}</td>
                    <td></td>
                </tr>
            </tbody>
        </table>
    </div>
</template>

<script setup>
    import { invoke } from '@tauri-apps/api/core';
    import { ref, computed, onMounted } from 'vue';
    import { useRoute } from 'vue-router';
    import { formatCurrency } from '../utils/utils.js';

    const route = useRoute();

    const accountId = computed(() => route.params.id);
    console.log(`AccountId: ${accountId.value}`);
    const accountName = ref('');
    const transactions = ref([]);
    const balance = ref(0);

    onMounted(async () => {
        const fetchAccountName = async () => {
            // TODO: Get account - single account, need to write that
        };

        const fetchTransactions = async () => {
            let result = await invoke('get_transactions', {
                accountId: parseInt(accountId.value),
            });
            console.log(transactions);
            transactions.value = result;
        };

        const fetchBalance = async () => {};

        // await fetchAccountName();
        await fetchTransactions();
        await fetchBalance();
    });
</script>

<style scoped>
    .account-details {
        padding: 32px;
        max-width: 1200px;
        margin: 0 auto;
    }

    .account-header {
        margin-bottom: 32px;
        padding-bottom: 24px;
        border-bottom: 1px solid var(--border-dim);
    }

    .account-header h1 {
        color: var(--text);
        margin-bottom: 8px;
    }

    .account-balance {
        color: var(--text-dim);
        font-size: 14px;
    }

    .transactions {
        width: 100%;
        border-collapse: collapse;
        background: var(--surface-1);
        border-radius: 8px;
        overflow: hidden;
    }

    .transactions th {
        background: var(--surface-2);
        color: var(--text);
        font-weight: 500;
        text-align: left;
        padding: 16px;
        font-size: 14px;
        border-bottom: 1px solid var(--border);
    }

    .transactions td {
        padding: 16px;
        color: var(--text);
        border-bottom: 1px solid var(--border-dim);
    }

    .transactions tr:last-child td {
        border-bottom: none;
    }

    .transactions tr:hover {
        background: var(--surface-2);
    }
</style>
