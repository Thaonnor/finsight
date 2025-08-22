<template>
    <div class="modal-overlay" @click="$emit('close')">
        <div class="modal-content" @click.stop>
            <el-card>
                <h2>Add Transaction</h2>
            

            <form @submit.prevent="handleSubmit">
                <el-form-item label="Amount ($)">
                    <el-input
                        v-model="amount"
                        placeholder="Amount ($)"
                        type="number"
                        step="0.01"
                    />
                </el-form-item>

                <el-form-item label="Type">
                    <el-select v-model="transactionType" placeholder="Select type">
                        <el-option label="Debit" value="debit" />
                        <el-option label="Credit" value="credit" />
                    </el-select>
                </el-form-item>

                <el-form-item label="Category">
                    <el-select v-model="categoryId" placeholder="Select category">
                        <el-option label="Uncategorize" value="1" />
                        <el-option label="Groceries" value="2" />
                    </el-select>
                </el-form-item>

                <el-form-item label="Description">
                    <el-input v-model="description" placeholder="Description"/>
                </el-form-item>

                <el-form-item label="Date">
                    <el-date-picker v-model="transactionDate" type="date" placeholder="Pick a date" />
                </el-form-item>

                <div class="form-actions">
                    <el-button @click="$emit('close')">Cancel</el-button>
                    <el-button type="primary" @click="handleSubmit">Add Transaction</el-button>
                </div>
            </form>
            </el-card>
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

    .modal-content h2 {
        margin-top: 0;
        color: var(--accent);
    }

    .form-field {
        margin-bottom: 16px;
    }

    .form-actions {
        display: flex;
        gap: 12px;
        justify-content: flex-end;
        margin-top: 24px;
    }

    :deep(.el-form-item) {
        display: block;
    }

    :deep(.el-form-item__label) {
        display: block;
        margin-bottom: 8px;
        width: 100% !important;
    }

    :deep(.el-form-item__content) {
        margin-left: 0 !important;
    }
</style>
