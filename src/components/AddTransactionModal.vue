<template>
    <div class="modal-overlay" @click="$emit('close')">
        <div class="modal-content" @click.stop>
            <h2>Add Transaction</h2>

            <form @submit.prevent="handleSubmit">
                <div class="form-field">
                    <label for="amount">Amount ($)</label>
                    <input
                        id="amount"
                        v-model="amount"
                        type="number"
                        step="0.01"
                        required
                        autocomplete="off"
                    />
                </div>

                <div class="form-field">
                    <label for="type">Type</label>
                    <select id="type" v-model="transactionType" required>
                        <option value="">Select type</option>
                        <option value="debit">Debit</option>
                        <option value="credit">Credit</option>
                    </select>
                </div>

                <div class="form-field">
                    <label for="description">Description</label>
                    <input
                        id="description"
                        v-model="description"
                        type="text"
                        required
                        autocomplete="off"
                    />
                </div>

                <div class="form-field">
                    <label for="date">Date</label>
                    <input
                        id="date"
                        v-model="transactionDate"
                        type="date"
                        required
                    />
                </div>

                <div class="form-actions">
                    <button type="button" @click="$emit('close')">
                        Cancel
                    </button>
                    <button type="submit">Add Transaction</button>
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
    const description = ref('');
    const transactionDate = ref(new Date().toISOString().split('T')[0]);

    const handleSubmit = async () => {
        try {
            const amountCents = Math.round(parseFloat(amount.value) * 100);

            await invoke('add_transaction', {
                accountId: props.accountId,
                amountCents: amountCents,
                transactionType: transactionType.value,
                description: description.value,
                transactionDate: transactionDate.value,
            });

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
