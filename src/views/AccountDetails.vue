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
                        <v-btn icon="mdi-delete" size="small" variant="text" @click="confirmDelete(item)"/>
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
    <v-dialog v-model="deleteDialog" max-width="400px">
        <v-card>
            <v-card-title>Delete Transaction</v-card-title>
            <v-card-text v-if="transactionToDelete">
                Are you sure you want to delete this transaction?
                <br><br>
                <strong>{{ transactionToDelete?.description }}</strong><br>
                <strong>{{ formatCurrency(transactionToDelete.amount_cents) }}</strong>
            </v-card-text>
            <v-card-actions>
                <v-spacer/>
                <v-btn variant="outlined" @click="cancelDelete">Cancel</v-btn>
                <v-btn color="error" variant="elevated" @click="deleteTransaction">Delete</v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>
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
    const deleteDialog = ref(false);
    const transactionToDelete = ref(null);

    const headers = [
        { title: 'Date', key: 'transaction_date', value: 'transaction_date' },
        { title: 'Description', key: 'description', value: 'description' },
        { title: 'Category', key: 'category_name', value: 'category_name'},
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

    const confirmDelete = (transaction) => {
        transactionToDelete.value = transaction;
        deleteDialog.value = true;
    }

    const deleteTransaction = async () => {
        try {
            await invoke('delete_transaction', {
                transactionId: transactionToDelete.value.id
            });

            await fetchTransactions();
            await refreshBalance(parseInt(accountId.value));

            deleteDialog.value = false;
            transactionToDelete.value = null;
        } catch (error) {
            console.error('Error deleting transaction:', error);
        }
    }

    const cancelDelete = () => {
        deleteDialog.value = false;
        transactionToDelete.value = null;
    }

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
    
</style>
