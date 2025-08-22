<template>
    <div class="modal-overlay" @click="$emit('close')">
        <div class="modal-content" @click.stop>
            <v-card-title>Add Transaction</v-card-title>

            <form @submit.prevent="handleSubmit">
                <v-text-field
                    v-model="amount"
                    label="Amount ($)"
                    type="number"
                    step="0.01"
                />

                <v-select
                    v-model="transactionType"
                    label="Type"
                    :items="['Debit', 'Credit']"
                />

                <v-select
                    v-model="categoryId"
                    label="Category"
                    :items="categoryItems"
                    item-title="name"
                    item-value="id"
                />

                <v-text-field v-model="description" label="Description" />

                <v-text-field
                    v-model="transactionDate"
                    label="Date"
                    placeholder="MM/DD/YYYY"
                />

                <div class="form-actions">
                    <v-btn variant="outlined" @click="$emit('close')"
                        >Cancel</v-btn
                    >
                    <v-btn
                        variant="elevated"
                        color="primary"
                        @click="handleSubmit"
                        >Add Transaction</v-btn
                    >
                </div>
            </form>
        </div>
    </div>
</template>

<script setup>
    import { ref } from 'vue';
    import { invoke } from '@tauri-apps/api/core';

    const props = defineProps({
        accountId: {
            type: Number,
            required: true,
        },
    });

    const emit = defineEmits(['close', 'transactionAdded']);

    const amount = ref('');
    const transactionType = ref('');
    const categoryId = ref(1);
    const description = ref('');
    const transactionDate = ref(new Date().toISOString().split('T')[0]);

    const handleSubmit = async () => {
        try {
            const amountCents = Math.round(parseFloat(amount.value) * 100);
            const signedAmount =
                transactionType.value === 'debit' ? -amountCents : amountCents;

            const payload = {
                accountId: parseInt(props.accountId),
                amountCents: signedAmount,
                transactionType: transactionType.value,
                description: description.value,
                transactionDate: transactionDate.value,
                categoryId: categoryId.value,
            };

            console.log(payload);

            await invoke('add_transaction', payload);

            emit('transactionAdded');
        } catch (error) {
            console.error('Failed to add transaction:', error);
        }
    };
</script>

<style scoped>
    .modal-overlay {
        position: fixed;
        inset: 0;
        background: rgba(0, 0, 0, 0.5);
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .modal-content {
        background: var(--surface-1);
        color: var(--text);
        padding: 2rem;
        border-radius: 8px;
        min-width: 400px;
    }

    .form-actions {
        display: flex;
        gap: 12px;
        justify-content: flex-end;
        margin-top: 24px;
    }
</style>
