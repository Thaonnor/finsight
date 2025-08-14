<template>
    <div>
        <h1>Account {{ accountId }} Transactions</h1>

        <button class="add-button" @click="showModal = true">
            Add Transaction
        </button>

        <div v-if="loading">Loading transactions...</div>

        <p v-else v-for="transaction in transactions" :key="transaction.id">
            {{ transaction.description }}
        </p>
    </div>

    <AddTransactionModal 
        v-if="showModal"
        :accountId="parseInt(accountId)"
        @close="showModal= false"
        @transactionAdded="handleTransactionAdded"
    />
</template>

<script setup>
    import { ref, onMounted } from 'vue';
    import { useRoute } from 'vue-router';
    import { invoke } from '@tauri-apps/api/core';
    import AddTransactionModal from '../components/AddTransactionModal.vue';

    const route = useRoute();
    const transactions = ref([]);
    const loading = ref(true);
    const accountId = route.params.id;
    const showModal = ref(false);

    const fetchTransactions = async () => {
        try {
            loading.value = true;
            const result = await invoke('get_transactions_by_account', {
                accountId: parseInt(accountId),
            });
            transactions.value = result;
        } catch (error) {
            console.error('Error fetching transactions:', error);
        } finally {
            loading.value = false;
        }
    };

    onMounted(() => {
        fetchTransactions();
    });

    const handleTransactionAdded = async () => {
        await fetchTransactions();
        showModal.value = false;
    }
</script>
