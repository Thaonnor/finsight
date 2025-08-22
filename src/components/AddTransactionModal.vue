<template>
    <v-dialog v-model="dialog" max-width="500px">
        <v-card>
            <v-card-title>Add Transaction</v-card-title>
            <v-card-text>
                <v-form @submit.prevent="handleSubmit">
                    <v-number-input
                        v-model="amount"
                        label="Amount ($)"
                        :step="0.01"
                        :min="0"
                        :precision="2"
                        required
                        control-variant="hidden"
                    />

                    <v-select 
                        v-model="transactionType"
                        label="Type"
                        :items="[
                            {title: 'Debit', value: 'debit'},
                            {title: 'Credit', value: 'credit'}
                        ]"
                        required
                    />
                </v-form>
            </v-card-text>
            <v-card-actions>
                <!-- buttons will go here -->
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
    const transactionType = ref('');
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

    .modal-content h2 {
        margin-top: 0;
        color: var(--accent);
    }

    .form-field {
        margin-bottom: 16px;
    }

    .form-field label {
        display: block;
        margin-bottom: 4px;
        font-size: 14px;
    }

    .form-field input,
    .form-field select {
        width: 100%;
        padding: 8px 12px;
        background: var(--bg);
        border: 1px solid var(--text-disabled);
        border-radius: 4px;
        color: var(--text);
        font-size: 16px;
        box-sizing: border-box;
    }

    .form-field input:focus,
    .form-field select:focus {
        border-color: var(--accent);
        outline: none;
    }
    .form-actions {
        display: flex;
        gap: 12px;
        justify-content: flex-end;
        margin-top: 24px;
    }

    button {
        padding: 8px 16px;
        border: none;
        border-radius: 4px;
        font-size: 16px;
        cursor: pointer;
    }

    button[type='button'] {
        background: var(--bg);
        color: var(--text);
    }

    button[type='submit'] {
        background: var(--accent);
        color: var(--on-accent);
    }

    input[type='number']::-webkit-outer-spin-button,
    input[type='number']::-webkit-inner-spin-button {
        -webkit-appearance: none;
        margin: 0;
    }

    input[type='number'] {
        -moz-appearance: textfield;
    }
</style>
