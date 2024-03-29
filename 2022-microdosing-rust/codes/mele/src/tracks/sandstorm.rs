use crate::*;

const BPM: u16 = 137;
const Q: u16 = bpm_to_quarter_ms(BPM);

track!([
    // ---
    // 11:
    note("E2", Q / 16),
    note("B3", Q / 8 - Q / 16),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 2),
    pause(Q + Q / 2),
    // ---
    // 12:
    pause(Q + Q / 2),
    note("D5", Q / 3),
    pause(Q / 2),
    // ---
    // 13:
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 2),
    pause(Q + Q / 2),
    // ---
    // 14:
    pause(2 * Q),
    // ---
    // 15:
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 2),
    pause(Q + Q / 2),
    // ---
    // 16:
    note("E2", Q / 16),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 2),
    pause(Q + Q / 2),
    // ---
    // 17:
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 4),
    pause(Q / 4),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 4),
    pause(Q / 4),
    // ---
    // 18:
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", 3 * Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(3 * Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", 3 * Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(3 * Q / 8),
    // ---
    // 19:
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", 3 * Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", 3 * Q / 8),
    pause(Q / 8),
    // ------------
    // Main melody:
    note("E5", Q / 8),
    pause(Q / 8),
    note("E5", Q / 8),
    pause(Q / 8),
    note("E5", Q / 8),
    pause(Q / 8),
    note("E5", Q / 8),
    pause(Q / 8),
    note("E5", Q / 8),
    pause(Q / 8),
    note("E5", Q / 8),
    pause(Q / 8),
    note("E5", 3 * Q / 8),
    pause(Q / 8),
    note("D5", Q / 8),
    pause(Q / 8),
    note("D5", Q / 8),
    pause(Q / 8),
    note("D5", Q / 8),
    pause(Q / 8),
    note("D5", Q / 8),
    pause(Q / 8),
    note("D5", Q / 8),
    pause(Q / 8),
    note("D5", Q / 8),
    pause(Q / 8),
    note("D5", 3 * Q / 8),
    pause(Q / 8),
    note("A4", Q / 8),
    pause(Q / 8),
    note("A4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", 3 * Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", 3 * Q / 8),
    pause(Q / 8),
    note("E5", Q / 8),
    pause(Q / 8),
    note("E5", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", 3 * Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", 3 * Q / 8),
    pause(Q / 8),
    note("E5", Q / 8),
    pause(Q / 8),
    note("E5", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", 3 * Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", 3 * Q / 8),
    pause(Q / 8),
    note("E5", Q / 8),
    pause(Q / 8),
    note("E5", Q / 8),
    pause(Q / 8),
    note("E5", Q / 8),
    pause(Q / 8),
    note("E5", Q / 8),
    pause(Q / 8),
    note("E5", Q / 8),
    pause(Q / 8),
    note("E5", Q / 8),
    pause(Q / 8),
    note("E5", 3 * Q / 8),
    pause(Q / 8),
    note("D5", Q / 8),
    pause(Q / 8),
    note("D5", Q / 8),
    pause(Q / 8),
    note("D5", Q / 8),
    pause(Q / 8),
    note("D5", Q / 8),
    pause(Q / 8),
    note("D5", Q / 8),
    pause(Q / 8),
    note("D5", Q / 8),
    pause(Q / 8),
    note("D5", 3 * Q / 8),
    pause(Q / 8),
    note("A4", Q / 8),
    pause(Q / 8),
    note("A4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(Q / 8),
    note("B4", 3 * Q / 8),
    pause(Q / 8),
    note("B4", Q / 8),
    pause(40),
    note("B4", Q / 8),
    pause(60),
    note("B4", Q / 8),
    pause(80),
    note("B4", Q / 8),
    pause(100),
    note("B4", Q / 8),
    pause(120),
    note("B4", Q / 8),
    pause(140),
    note("B4", 180),
    pause(160),
    note("B4", 350),
]);
