import { reactive, computed, readonly } from 'vue';
import { invoke } from '@tauri-apps/api/core';

// Shared reactive state
const balances = reactive({});

export function useAccounts(accountId = null) {
    const getBalance = async (id) => {
        try {
        const balance = await invoke('get_balance', { accountId: id });
        balances[id] = balance;
        return balance;
        } catch (error) {
            console.error('Failed fetching balance:', error);
            return 0;
        }
    }

    const refreshBalance = (id) => getBalance(id);

    if (accountId) {
        const balance = computed(() => balances[accountId] || 0);
        return { balance, refreshBalance };
    } else {
        return {
            balances: readonly(balances),
            getBalance,
            refreshBalance,
        }
    }
}
