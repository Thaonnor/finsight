<template>
    <router-link :to="`/accounts/${accountId}`" class="account-item">
        <span>{{ accountName }}</span>
        <span :class="accountBalance >= 0 ? 'positive' : 'negative'">{{
            formatCents(accountBalance)
        }}</span>
    </router-link>
</template>

<script setup>
    const props = defineProps({
        accountName: {
            type: String,
            required: true,
        },
        accountBalance: {
            type: Number,
            required: true,
        },
        accountId: {
            type: Number,
            required: true,
        }
    });

    const formatCents = (cents) => {
        return (cents / 100).toLocaleString('en-US', {
            style: 'currency',
            currency: 'USD',
        });
    };
</script>

<style scoped>
    .account-item {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 8px 12px;
        margin: 4px 0;
        border-radius: 4px;
        transition: background-color 0.2s ease;
        cursor: pointer;
        text-decoration: none;
        color: var(--text);
        width: 100%;
        box-sizing: border-box;
    }

    .account-item:hover {
        background: var(--surface-1);
    }

    .positive {
        color: var(--positive);
        font-variant-numeric: tabular-nums;
    }

    .negative {
        color: var(--negative);
        font-variant-numeric: tabular-nums;
    }
</style>
