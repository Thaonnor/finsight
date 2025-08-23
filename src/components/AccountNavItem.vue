<template>
    <v-list-item :to="`/accounts/${accountId}`" class="account-nav-item">
        <v-list-item-title>
            {{ accountName }}
        </v-list-item-title>
        <template v-slot:append>
            <span
                :class="accountBalance >= 0 ? 'text-success' : 'text-error'"
                >{{ formatBalance(accountBalance) }}</span
            >
        </template>
    </v-list-item>
</template>

<script setup>
  import { formatBalance } from '../utils/utils.js';

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
        },
    });
</script>

<style scoped>
    .account-nav-item {
        font-variant-numeric: tabular-nums;
    }
</style>
