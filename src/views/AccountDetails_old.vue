<template>
    <div>
        <h1>Account {{ accountId }} Transactions</h1>

        <button class="add-button" @click="showModal = true">
            Add Transaction
        </button>

        <div v-if="loading">Loading transactions...</div>

        <div v-else-if="transactions.length === 0" class="empty-state">
            <p>
                No transactions yet. Add your first transaction to get started!
            </p>
        </div>

        <table v-else class="transactions-table">
            <thead>
                <tr>
                    <th>Date</th>
                    <th>Description</th>
                    <th>Type</th>
                    <th>Amount</th>
                </tr>
            </thead>
            <tbody>
                <tr
                    v-for="transaction in transactionsWithBalance"
                    :key="transaction.id"
                >
                    <td>{{ formatDate(transaction.date) }}</td>
                    <td>{{ transaction.description }}</td>
                    <td>{{ transaction.transaction_type }}</td>
                    <td
                        class="amount"
                        :class="getAmountClass(transaction.transaction_type)"
                    >
                        {{
                            formatAmount(
                                transaction.amount_cents,
                                transaction.transaction_type
                            )
                        }}
                    </td>
                    <td class="balance">
                        {{ formatCurrency(transaction.running_balance) }}
                    </td>
                </tr>
            </tbody>
        </table>
    </div>

    <AddTransactionModal
        v-if="showModal"
        :accountId="parseInt(accountId)"
        @close="showModal = false"
        @transactionAdded="handleTransactionAdded"
    />
</template>

<script setup>
    import { ref, onMounted, computed } from 'vue';
    import { useRoute } from 'vue-router';
    import { invoke } from '@tauri-apps/api/core';
    import AddTransactionModal from '../components/AddTransactionModal.vue';

    const route = useRoute();

    const transactions = ref([]);
    const loading = ref(true);
    const showModal = ref(false);

    const accountId = computed(() => route.params.id);

    const transactionsWithBalance = computed(() => {
        if (transactions.value.length === 0) return [];

        const sorted = [...transactions.value].sort(
            (a, b) => new Date(a.date) - new Date(b.date)
        );

        let runningBalance = 0;
        const withBalances = sorted.map((transaction) => {
            if (transaction.transaction_type === 'debit') {
                runningBalance -= transaction.amount_cents;
            } else {
                runningBalance += transaction.amount_cents;
            }

            return {
                ...transaction,
                running_balance: runningBalance,
            };
        });

        return withBalances.reverse();
    });

    const formatDate = (dateString) => {
        return new Date(dateString).toLocaleDateString();
    };

    const formatAmount = (amountCents, transactionType) => {
        const amount = Math.abs(amountCents) / 100;
        const sign = transactionType === 'debit' ? '-' : '+';
        return `${sign}$${amount.toFixed(2)}`;
    };

    const formatCurrency = (amountCents) => {
        const amount = amountCents / 100;
        return `$${amount.toFixed(2)}`;
    };

    const getAmountClass = (transactionType) => {
        return transactionType === 'debit' ? 'negative' : 'positive';
    };

    const fetchTransactions = async () => {
        try {
            loading.value = true;
            const result = await invoke('get_transactions', {
                accountId: parseInt(accountId),
            });
            transactions.value = result;
        } catch (error) {
            console.error('Error fetching transactions:', error);
        } finally {
            loading.value = false;
        }
    };

    const handleTransactionAdded = async () => {
        await fetchTransactions();
        showModal.value = false;
    };

    onMounted(() => {
        fetchTransactions();
    });
</script>
