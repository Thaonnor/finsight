<template>
    <v-dialog v-model="dialog" max-width="500px">
        <v-card elevation="8">
            <v-card-title>Add Transaction</v-card-title>
            <v-card-text class="pb-0">
                <v-form @submit.prevent="handleSubmit">
                    <v-number-input
                        v-model="amount"
                        label="Amount ($)"
                        variant="outlined"
                        :step="0.01"
                        :min="0"
                        :precision="2"
                        prefix="$"
                        required
                        control-variant="hidden"
                        persistent-placeholder
                        autocomplete="off"
                    />

                    <v-select
                        v-model="transactionType"
                        label="Type"
                        variant="outlined"
                        :items="[
                            { title: 'Debit', value: 'debit' },
                            { title: 'Credit', value: 'credit' },
                        ]"
                        required
                    />

                    <v-select
                        v-model="categoryId"
                        label="Category"
                        variant="outlined"
                        :items="[
                            { title: 'Uncategorized', value: 1 },
                            { title: 'Groceries', value: 2 },
                        ]"
                        required
                    />

                    <v-text-field
                        v-model="description"
                        label="Description"
                        variant="outlined"
                        persistent-placeholder
                        required
                    />

                    <v-date-input
                        v-model="transactionDate"
                        label="Date"
                        variant="outlined"
                        prepend-icon=""
                        required
                    />
                </v-form>
            </v-card-text>
            <v-card-actions>
                <v-spacer/>
                <v-btn variant="outlined" @click="$emit('close')">Cancel</v-btn>
                <v-btn color="primary" variant="elevated" @click="handleSubmit">Add Transaction</v-btn>
            </v-card-actions>
        </v-card>
    </v-dialog>
</template>

<script setup>
    import { ref, onMounted } from 'vue';
    import { invoke } from '@tauri-apps/api/core';

    const props = defineProps({
        accountId: {
            type: Number,
            required: true,
        },
    });

    const emit = defineEmits(['close', 'transactionAdded']);

    const dialog = ref(false);
    const amount = ref(null);
    const transactionType = ref('debit');
    const categoryId = ref(1);
    const description = ref('');
    const transactionDate = ref(new Date().toISOString().split('T')[0]);

    onMounted(() => {
        dialog.value = true;
    });

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
            emit('close');
        } catch (error) {
            console.error('Failed to add transaction:', error);
        }
    };
</script>

<style scoped>
</style>
