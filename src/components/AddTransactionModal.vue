<template>
    <div class="modal-overlay" @click="$emit('close')">
        <div class="modal-content" @click.stop>
            <Panel header="Add Transaction">

            <form @submit.prevent="handleSubmit">
                <FloatLabel variant="in">
                    <InputText
                        id="amount"
                        v-model="amount"
                        type="number"
                        step="0.01"
                    />
                    <label for="amount">Amount ($)</label>
                </FloatLabel>

                <FloatLabel variant="in">
                    <Select
                        id="type"
                        v-model="transactionType"
                        :options="typeOptions"
                        optionLabel="label"
                        optionValue="value"
                    />
                    <label for="type">Type</label>
                </FloatLabel>

                <FloatLabel variant="in">
                    <Select
                        id="category"
                        v-model="categoryId"
                        :options="categoryOptions"
                        optionLabel="label"
                        optionValue="value"
                    />
                    <label for="category">Category</label>
                </FloatLabel>

                <FloatLabel variant="in">
                    <InputText id="description" v-model="description"/>
                    <label for="description">Description</label>
                </FloatLabel>

                <FloatLabel variant="in">
                    <InputText id="date" v-model="transactionDate" placeholder="MM/DD/YYYY"/>
                    <label for="date">Date</label>
                </FloatLabel>

                <div class="form-actions">
                    <Button label="Cancel" severity="contrast" @click="$emit('close')"/>
                    <Button label="Add Transaction" @click="handleSubmit"/>
                </div>
            </form>
            </Panel>
        </div>
    </div>
</template>

<script setup>
    import { ref } from 'vue';
    import { invoke } from '@tauri-apps/api/core';
    import InputText from 'primevue/inputtext';
    import FloatLabel from 'primevue/floatlabel';
    import Select from 'primevue/select';
    import Button from 'primevue/button';
    import Panel from 'primevue/panel';

    const typeOptions = ref([
        { label: 'Debit', value: 'debit' },
        { label: 'Credit', value: 'credit' },
    ]);

    const categoryOptions = ref([
        { label: 'Uncategorized', value: 1 },
        { label: 'Groceries', value: 2 },
    ]);

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
    .p-floatlabel {
        margin-bottom: 1.5rem;
    }

    :deep(.p-inputtext) {
        width: 100%;
        padding: 12px 16px;
        font-size: 16px;
    }

    :deep(.p-select) {
        width: 100%;
        min-height: 48px;
        font-size: 16px;
    }

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

    .form-actions {
        display: flex;
        gap: 12px;
        justify-content: flex-end;
        margin-top: 2rem;
    }
</style>
