<template>
    <v-container fill-height class="d-flex flex-column">
        <v-row>
            <v-col>
                <h1>{{ accountName }}</h1>
            </v-col>
            <v-col cols="auto">
                <div class="balance-display">
                    <span class="balance-label">Balance</span>
                    <span
                        :class="[
                            'balance-amount',
                            balance >= 0 ? 'text-success' : 'text-error',
                        ]"
                        >{{ formatBalance(balance) }}</span
                    >
                </div>
                <v-btn color="primary" @click="showModal = true">
                    Add Transaction
                </v-btn>
            </v-col>
        </v-row>
        <v-row class="flex-grow-1 overflow-auto">
            <v-col v-if="loading">
                <v-skeleton-loader
                    type="table-heading, table-thead, table-row-divider@6"
                />
            </v-col>
            <v-col v-else-if="transactions.length === 0">
                <v-card variant="outlined" class="text-center pa-8">
                    <v-card-title>No transactions yet</v-card-title>
                    <v-card-text
                        >Add your first transaction to get started!</v-card-text
                    >
                    <v-card-actions class="justify-center">
                        <v-btn color="primary" @click="showModal = true"
                            >Add Transaction</v-btn
                        >
                    </v-card-actions>
                </v-card>
            </v-col>
            <v-col v-else>
                <v-data-table
                    :headers="headers"
                    :items="transactions"
                >
                    <template v-slot:item.transaction_date=" { item }">
                        {{ formatDate(item.transaction_date) }}
                    </template>
                    <template v-slot:item.amount_cents="{ item }">
                        <span :class="item.amount_cents >= 0 ? 'text-success' : 'text-error'">
                            {{ formatCurrency(item.amount_cents) }}
                        </span>
                    </template>
                    <template v-slot:item.actions="{ item }">
                        <v-btn icon="mdi-pencil" size="small" variant="text" />
                        <v-btn icon="mdi-delete" size="small" variant="text" />
                    </template>
                </v-data-table>
            </v-col>
        </v-row>
    </v-container>
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
    import {
        formatCurrency,
        formatDate,
        formatBalance,
    } from '../utils/utils.js';
    import AddTransactionModal from '../components/AddTransactionModal.vue';
    import { useAccounts } from '../composables/useAccounts.js';

    const route = useRoute();

    const accountId = computed(() => route.params.id);
    const accountName = ref('');
    const transactions = ref([]);
    const loading = ref(true);
    const showModal = ref(false);
    const { balance, refreshBalance } = useAccounts(parseInt(accountId.value));

    const headers = [
        { title: 'Date', key: 'transaction_date', value: 'transaction_date' },
        { title: 'Description', key: 'description', value: 'description' },
        { title: 'Category', key: 'category_id', value: 'category_id'},
        { title: 'Type', key: 'transaction_type', value: 'transaction_type'}, 
        { title: 'Amount', key: 'amount_cents', value: 'amount_cents', align: 'end' },
        { title: 'Actions', key: 'actions', sortable: false}
    ];

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

    onMounted(async () => {
        await fetchAccountName();
        await fetchTransactions();
        await refreshBalance(parseInt(accountId.value));
    });

    const handleTransactionAdded = async () => {
        await fetchTransactions();
        await refreshBalance(parseInt(accountId.value));
        showModal = false;
    };
</script>

<style scoped>
    .account-details {
        padding: 32px;
        max-width: 1200px;
        margin: 0 auto;
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
