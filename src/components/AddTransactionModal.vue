<template>
    <Dialog
        v-model:visible="visible"
        modal
        header="Add Transaction"
        :style="{ width: '25rem' }"
        @hide="handleClose"
    >
        <p>Hello</p>
    </Dialog>
    <!-- <div class="modal-overlay" @click="$emit('close')">
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
                    <label for="category">Category</label>
                    <select id="category" v-model="categoryId" required>
                        <option value="1">Uncategorized</option>
                        <option value="2">Groceries</option>
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
    </div> -->
</template>

<script setup>
    import { ref, onMounted } from 'vue';
    import { invoke } from '@tauri-apps/api/core';
    import Dialog from 'primevue/dialog';

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
    const visible = ref(false);

    onMounted(() => {
        visible.value = true;
    });

    const handleClose = () => {
        visible.value = false;
        emit('close');
    };

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
    /* .modal-overlay {
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
    } */

    :deep(.p-dialog-mask) {
        background: rgba(0, 0, 0, 0.5);
        display: flex;
        align-items: center;
        justify-content: center;
    }

    :deep(.p-dialog) {
        background: var(--surface-1);
        border-radius: 8px;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    }

    :deep(.p-dialog-header) {
        padding: 1.5rem 1.5rem 0 1.5rem;
        color: var(--accent);
        font-size: 1.25rem;
        font-weight: 600;
    }

    :deep(.p-dialog-content) {
        padding: 1.5rem;
        color: var(--text);
    }

    .p-dialog-mask {
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: rgba(0, 0, 0, 0.5);
        display: flex;
        align-items: center;
        justify-content: center;
    }
</style>
