export const formatCurrency = (cents) => {
    if (typeof cents !== 'number') {
        throw new Error('Value must be a number');
    }

    if (cents == 0) {
        return '-';
    }

    let dollars = (Math.abs(cents) / 100).toFixed(2);

    return `$${dollars}`;
};

export const formatDate = (dateString) => {
    return new Date(dateString).toLocaleDateString();
};
