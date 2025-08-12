<template>
    <div class="modal-overlay">
        <div class="modal-content">
            <h2>Add Account</h2>
            <form>
                <div class="form-field">
                    <label for="accountName">Account Name</label>
                    <input
                        id="accountName"
                        type="text"
                        placeholder="e.g., Chase Checking"
                        autocomplete="off"
                    />
                </div>

                <div class="form-field">
                    <label for="accountType">Account Type</label>
                    <select id="accountType" autocomplete="off">
                        <option value="checking">Checking</option>
                        <option value="savings">Savings</option>
                    </select>
                </div>

                <div class="form-actions">
                    <button type="button">Cancel</button>
                    <button type="submit">Add Account</button>
                </div>
            </form>
        </div>
    </div>
</template>

<script setup>
    // Script will go here
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
</style>
