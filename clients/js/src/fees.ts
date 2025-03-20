function bpsMul(amount: number, numerator: number, denominator: number): number {
    return Math.floor((amount * numerator) / denominator);
}

export function calculateFee(amount: number, currentSlot: number, startSlot: number): { solFee: number, feeBps: number } {
    const slotsPassed = currentSlot - startSlot;
    let solFee = 0;
    let feeBps = 0

    if (slotsPassed < 150) {
        // Phase 1: 99% fees between slot 0 - 150
        feeBps = 9999
        solFee = bpsMul(amount, feeBps, 10000);
    } else if (slotsPassed >= 150 && slotsPassed <= 250) {
        // Phase 2: Linear decrease between 150 - 250
        // Calculate the minimum fee bps (at slot 250) scaled by 100,000 for precision
        feeBps = Math.floor((-9704901 * slotsPassed + 2445930392) / 100000);
        solFee = bpsMul(amount, feeBps, 10000);
    } else if (slotsPassed > 250) {
        // Phase 3: 1% fees after 250
        feeBps = 100
        solFee = bpsMul(amount, feeBps, 10000);
    }

    return { solFee, feeBps };
}
