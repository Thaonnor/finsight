export const formatCurrency = (cents) => {
    if (typeof cents !== 'number') {
        throw new Error('Value must be a number');
    }

    if (cents == 0) {
        return '-';
    }

    let dollars = (cents / 100).toFixed(2);

    if (dollars < 0) {
        return `(${dollars})`;
    }

    return `${dollars}`;
};
