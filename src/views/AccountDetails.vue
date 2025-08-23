<template>
    <div class="account-details">
        <div class="account-header">
            <div class="header-left">
                <h1>{{ accountName }}</h1>
            </div>
            <div class="header-right">
                <div class="balance-display">
                    <span class="balance-label">Balance</span>
                    <span class="balance-amount">{{
                        formatBalance(balance)
                    }}</span>
                </div>
                <button class="add-transaction-btn" @click="showModal = true">
                    Add Transaction
                </button>
            </div>
        </div>
        <div v-if="loading">Loading transactions...</div>

        <div v-else-if="transactions.length === 0" class="empty-state">
            <p>
                No transactions yet. Add your first transaction to get started!
            </p>
        </div>

        <table v-else class="transactions">
            <thead>
                <tr>
                    <th>Date</th>
                    <th>Description</th>
                    <th>Category</th>
                    <th>Type</th>
                    <th class="amount-column">Amount</th>
                    <th>Options</th>
                </tr>
            </thead>
            <tbody>
                <tr v-for="transaction in transactions" :key="transaction.id">
                    <td>{{ formatDate(transaction.transaction_date) }}</td>
                    <td>{{ transaction.description }}</td>
                    <td>{{ transaction.category_id }}</td>
                    <td>{{ transaction.transaction_type }}</td>
                    <td
                        class="amount-column"
                        :class="
                            transaction.amount_cents >= 0
                                ? 'amount-positive'
                                : 'amount-negative'
                        "
                    >
                        {{ formatCurrency(transaction.amount_cents) }}
                    </td>
                    <td></td>
                </tr>
            </tbody>
        </table>
    </div>

    <AddTransactionModal
        v-if="showModal"
        :accountId="Number(route.params.id)"
        @close="showModal = false"
        @transactionAdded="handleTransactionAdded"
    />
</template>

<script setup>
    import { invoke } from '@tauri-apps/api/core';
    import { ref, computed, onMounted } from 'vue';
    import { useRoute } from 'vue-router';
    import { formatCurrency, formatDate, formatBalance } from '../utils/utils.js';
    import AddTransactionModal from '../components/AddTransactionModal.vue';

    const route = useRoute();

    const accountId = computed(() => route.params.id);
    const accountName = ref('');
    const transactions = ref([]);
    const balance = ref(0);
    const loading = ref(true);
    const showModal = ref(false);

    const fetchAccountName = async () => {
        try {
            let result = await invoke('get_account', {
                accountId: parseInt(accountId.value),
            });
            console.log(result);
            accountName.value = result.name;
        } catch (error) {
            console.error('Error fetching transactions:', error);
        }
    };

    const fetchTransactions = async () => {
        try {
            loading.value = true;
            let result = await invoke('get_transactions', {
                accountId: parseInt(accountId.value),
            });
            transactions.value = result;
        } catch (error) {
            console.error('Error fetching transactions:', error);
        } finally {
            loading.value = false;
        }
    };

    const fetchBalance = async () => {
        try {
            let result = await invoke('get_balance', {
                accountId: parseInt(accountId.value),
            });
            balance.value = result;
        } catch (error) {
            console.error('Error fetching balance:', error);
        }
    };

    onMounted(async () => {
        await fetchAccountName();
        await fetchTransactions();
        await fetchBalance();
    });

    const handleTransactionAdded = async () => {
        await fetchTransactions();
        await fetchBalance();
        showModal = false;
    };
</script>

<style scoped>
    .account-details {
        padding: 32px;
        max-width: 1200px;
        margin: 0 auto;
    }

    .account-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 32px;
        padding-bottom: 24px;
        border-bottom: 1px solid var(--border-dim);
    }

    .balance-display {
        text-align: right;
        margin-bottom: 12px;
    }

    .balance-label {
        display: block;
        font-size: 14px;
        color: var(--text-dim);
    }

    .balance-amount {
        display: block;
        font-size: 24px;
        font-weight: 600;
        color: var(--text);
    }

    .add-transaction-btn {
        background: var(--accent);
        color: var(--on-accent);
        border: none;
        padding: 12px 20px;
        border-radius: 6px;
        font-size: 14px;
        font-weight: 500;
        cursor: pointer;
        transition: background-color 0.2s ease;
    }

    .add-transaction-btn:hover {
        background: var(--accent-hover);
    }

    .add-transaction-btn:active {
        background: var(--accent-active);
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

    .amount-column {
        text-align: right;
        font-variant: tabular-nums;
    }

    .transactions th:nth-child(1) {
        width: 100px;
    }
    .transactions th:nth-child(2) {
        width: 40%;
    }
    .transactions th:nth-child(3) {
        width: 120px;
    }
    .transactions th:nth-child(4) {
        width: 80px;
    }
    .transactions th:nth-child(5) {
        width: 120px;
    }
    .transactions th:nth-child(6) {
        width: 80px;
    }

    .transactions .amount-positive {
        color: var(--positive);
    }

    .transactions .amount-negative {
        color: var(--negative);
    }
</style>
