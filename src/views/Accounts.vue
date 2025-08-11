<template>
    <div>
        <h2>Accounts</h2>
        <div v-if="loading">Loading accounts...</div>
        <div v-else-if="accounts.length === 0">No accounts found</div>
        <div v-else>
            <ul>
                <li v-for="account in accounts" :key="account.id">
                    {{ account.name }} ({{ account.account_type }})
                </li>
            </ul>
            <AddAccountModal />
        </div>
    </div>
</template>

<script>
    import { invoke } from '@tauri-apps/api/core';
    import { ref, onMounted } from 'vue';
    import AddAccountModal from '../components/AddAccountModal.vue';

    export default {
        name: 'Accounts',
        components: { AddAccountModal },
        setup() {
            const accounts = ref([]);
            const loading = ref(true);

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

            return { accounts, loading };
        },
    };
</script>
