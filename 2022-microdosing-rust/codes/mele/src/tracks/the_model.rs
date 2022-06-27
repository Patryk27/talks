use crate::*;

const BPM: u16 = 125;
const Q: u16 = bpm_to_quarter_ms(BPM);

track!([
    note("A3", Q + Q / 2, 200),
    note("A3", Q / 2, 1),
    note("C4", Q / 2, 1),
    note("A3", Q / 2, 1),
    note("C4", Q / 2, 1),
    note("A3", Q / 2, 1),
    note("E3", Q + Q / 2, 200),
    note("E3", Q / 2, 1),
    note("G3", Q / 2, 1),
    note("E3", Q / 2, 1),
    note("G3", Q, 100),
    //
    note("A3", Q + Q / 2, 200),
    note("A3", Q / 2, 1),
    note("C4", Q / 2, 1),
    note("A3", Q / 2, 1),
    note("C4", Q / 2, 1),
    note("A3", Q / 2, 1),
    note("E3", Q + Q / 2, 200),
    note("E3", Q / 2, 1),
    note("G3", Q / 2, 1),
    note("E3", Q / 2, 1),
    note("G3", Q, 100),
    //
    pause(200),
    //
    note("A5", 2 * Q + Q / 2, 400),
    note("C6", Q / 2, 10),
    note("B5", Q / 2, 10),
    note("A5", Q / 2, 10),
    note("B5", Q + Q / 2, 400),
    note("G5", Q / 2, 10),
    note("E5", Q + Q / 2, 200),
    note("E5", Q / 2, 10),
    note("A5", 2 * Q + Q / 2, 400),
    note("C6", Q / 2, 10),
    note("B5", Q / 2 + 40, 10),
    note("A5", Q / 2 + 80, 10),
    note("B5", Q + Q / 2 + 120, 400),
    note("G5", Q + Q / 2 + Q / 3, 200),
    note("E5", 3 * Q),
]);
