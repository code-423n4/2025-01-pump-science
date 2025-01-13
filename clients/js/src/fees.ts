function bpsMul(amount: number, numerator: number, denominator: number): number {
    return Math.floor((amount * numerator) / denominator);
}

export function calculateFee(amount: number, currentSlot: number, startSlot: number): { solFee: number, feeBps: number } {
    const slotsPassed = currentSlot - startSlot;
    let solFee = 0;
    let feeBps = 0

    if (slotsPassed < 150) {
        // Phase 1: 99% fees between slot 0 - 150
        solFee = bpsMul(amount, 9900, 10000);
        feeBps = 9900
    } else if (slotsPassed >= 150 && slotsPassed <= 250) {
        // Phase 2: Linear decrease between 150 - 250
        // Calculate the minimum fee bps (at slot 250) scaled by 10000 for precision
        feeBps = Math.floor((-8300000 * slotsPassed + 2162600000) / 1000000);
        solFee = bpsMul(amount, feeBps, 10000);
    } else if (slotsPassed > 250) {
        // Phase 3: 1% fees after 250
        solFee = bpsMul(amount, 100, 10000);
        feeBps = 100
    }

    return { solFee, feeBps };
}
