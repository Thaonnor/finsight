<template>
    <div class="modal-overlay">
        <div class="modal-content">
            <h2>Add Account</h2>
            <p>Form will go here</p>
            <button>Close</button>
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
    background: rgba(0,0,0,0.5);
    display: flex;
    justify-content: center;
    align-items: center;
}

.modal-content {
    background: white;
    padding: 2rem;
    border-radius: 8px;
    min-width: 400px;
}
</style>
