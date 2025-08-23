export const formatCurrency = (cents) => {
    if (typeof cents !== 'number') {
        throw new Error('Value must be a number');
    }

    if (cents == 0) {
        return '-';
    }

    return (Math.abs(cents) / 100).toLocaleString('en-US', {
        style: 'currency',
        currency: 'USD',
    });
};

export const formatBalance = (cents) => {
    if (typeof cents !== 'number') {
        throw new Error('Value must be a number');
    }

    if (cents == 0) {
        return '$0.00';
    }

    return (cents / 100).toLocaleString('en-US', {
        style: 'currency',
        currency: 'USD',
    });
};

export const formatDate = (dateString) => {
    return new Date(dateString).toLocaleDateString();
};
