<template>
    <div class="accounts-page">
        <div class="page-header">
            <h2>Accounts</h2>
            <button class="add-button" @click="showModal = true">
                Add Account
            </button>
        </div>

        <div class="loading-state" v-if="loading">
            <p>Loading accounts...</p>
        </div>

        <div class="empty-state" v-else-if="accounts.length === 0">
            <p>No accounts found.</p>
            <small>Add your first account to get started.</small>
        </div>

        <div class="accounts-grid" v-else>
            <div
                class="account-card"
                v-for="account in accounts"
                :key="account.id"
                @click="navigateToAccount(account.id)"
            >
                <h3>{{ account.name }}</h3>
                <span class="account-type">{{ account.account_type }}</span>
            </div>
        </div>

        <AddAccountModal
            v-if="showModal"
            @close="showModal = false"
            @accountAdded="handleAccountAdded"
        />
    </div>
</template>

<script setup>
    import { invoke } from '@tauri-apps/api/core';
    import { ref, onMounted } from 'vue';
    import AddAccountModal from '../components/AddAccountModal.vue';
    import { useRouter } from 'vue-router';

    const accounts = ref([]);
    const loading = ref(true);
    const showModal = ref(false);
    const router = useRouter();

    const loadAccounts = async () => {
        try {
            accounts.value = await invoke('get_accounts');
        } catch (error) {
            console.error('Failed to load accounts:', error);
        } finally {
            loading.value = false;
        }
    };

    onMounted(loadAccounts);

    const handleAccountAdded = async () => {
        await loadAccounts();
        showModal.value = false;
    };

    const navigateToAccount = (accountId) => {
        router.push(`/account/${accountId}`);
    };
</script>

<style scoped>
    .accounts-page {
        max-width: 1200px;
    }

    .page-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 32px;
    }

    .page-header h2 {
        margin: 0;
        color: var(--text);
    }

    .add-button {
        background: var(--accent);
        color: var(--on-accent);
        border: none;
        padding: 12px 24px;
        border-radius: 4px;
        font-size: 16px;
        font-weight: 500;
        cursor: pointer;
        transition: opacity 0.2s ease;
    }

    .add-button:hover {
        opacity: 0.9;
    }

    .loading-state,
    .empty-state {
        text-align: center;
        padding: 48px 24px;
        background: var(--surface-1);
        border-radius: 8px;
        border: 1px solid var(--surface-2);
    }

    .empty-state small {
        display: block;
        margin-top: 8px;
        color: var(--text-dim);
    }

    .accounts-grid {
        display: grid;
        grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
        gap: 20px;
    }

    .account-card {
        background: var(--surface-1);
        border: 1px solid var(--surface-2);
        border-radius: 8px;
        padding: 24px;
        transition: border-color 0.2s ease;
    }

    .account-card:hover {
        border-color: var(--accent);
    }

    .account-card h3 {
        margin: 0 0 8px 0;
        color: var(--text);
        font-size: 18px;
        font-weight: 600;
    }

    .account-type {
        display: inline-block;
        background: var(--surface-2);
        color: var(--text-dim);
        padding: 4px 12px;
        border-radius: 16px;
        font-size: 14px;
        text-transform: capitalize;
    }
</style>
