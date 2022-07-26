use super::{
    Bitwise, Felt, StarkField, TraceFragment, BITWISE_AND, BITWISE_OR, BITWISE_XOR, TRACE_WIDTH,
};
use rand_utils::rand_value;
use vm_core::{
    chiplets::bitwise::{A_COL_IDX, B_COL_IDX, OP_CYCLE_LEN, OUTPUT_COL_IDX, PREV_OUTPUT_COL_IDX},
    ZERO,
};

#[test]
fn bitwise_init() {
    let bitwise = Bitwise::new();
    assert_eq!(0, bitwise.trace_len());
}

#[test]
fn bitwise_and() {
    let mut bitwise = Bitwise::new();

    let a = rand_u32();
    let b = rand_u32();

    let result = bitwise.u32and(a, b).unwrap();
    assert_eq!(a.as_int() & b.as_int(), result.as_int());

    // --- check generated trace ----------------------------------------------
    let num_rows = OP_CYCLE_LEN;
    let mut trace = (0..TRACE_WIDTH)
        .map(|_| vec![ZERO; num_rows])
        .collect::<Vec<_>>();
    let mut fragment = TraceFragment::trace_to_fragment(&mut trace);

    bitwise.fill_trace(&mut fragment);

    // make sure the selector values specify bitwise AND at each step in the trace
    for row in 0..OP_CYCLE_LEN {
        assert_eq!([trace[0][row], trace[1][row]], BITWISE_AND);
    }

    // make sure result and result from the trace are the same
    assert_eq!(result, trace[OUTPUT_COL_IDX][OP_CYCLE_LEN - 1]);

    // make sure values a and b were decomposed correctly
    check_decomposition(&trace, 0, a.as_int(), b.as_int());

    // make sure the result was re-composed correctly
    let mut prev_result = ZERO;

    for i in 0..OP_CYCLE_LEN {
        let c0 = binary_and(trace[4][i], trace[8][i]);
        let c1 = binary_and(trace[5][i], trace[9][i]);
        let c2 = binary_and(trace[6][i], trace[10][i]);
        let c3 = binary_and(trace[7][i], trace[11][i]);

        let result_4_bit = c0 + Felt::new(2) * c1 + Felt::new(4) * c2 + Felt::new(8) * c3;
        let result = prev_result * Felt::new(16) + result_4_bit;

        assert_eq!(prev_result, trace[PREV_OUTPUT_COL_IDX][i]);
        assert_eq!(result, trace[OUTPUT_COL_IDX][i]);

        prev_result = result;
    }
}

#[test]
fn bitwise_or() {
    let mut bitwise = Bitwise::new();

    let a = rand_u32();
    let b = rand_u32();

    let result = bitwise.u32or(a, b).unwrap();
    assert_eq!(a.as_int() | b.as_int(), result.as_int());

    // --- check generated trace ----------------------------------------------
    let num_rows = 8;
    let mut trace = (0..TRACE_WIDTH)
        .map(|_| vec![ZERO; num_rows])
        .collect::<Vec<_>>();
    let mut fragment = TraceFragment::trace_to_fragment(&mut trace);

    bitwise.fill_trace(&mut fragment);

    // make sure the selector values specify bitwise OR at each step in the trace
    for row in 0..OP_CYCLE_LEN {
        assert_eq!([trace[0][row], trace[1][row]], BITWISE_OR);
    }

    // make sure result and result from the trace are the same
    assert_eq!(result, trace[OUTPUT_COL_IDX][OP_CYCLE_LEN - 1]);

    // make sure values a and b were decomposed correctly
    check_decomposition(&trace, 0, a.as_int(), b.as_int());

    // make sure the result was re-composed correctly
    let mut prev_result = ZERO;

    for i in 0..OP_CYCLE_LEN {
        let c0 = binary_or(trace[4][i], trace[8][i]);
        let c1 = binary_or(trace[5][i], trace[9][i]);
        let c2 = binary_or(trace[6][i], trace[10][i]);
        let c3 = binary_or(trace[7][i], trace[11][i]);

        let result_4_bit = c0 + Felt::new(2) * c1 + Felt::new(4) * c2 + Felt::new(8) * c3;
        let result = prev_result * Felt::new(16) + result_4_bit;

        assert_eq!(prev_result, trace[PREV_OUTPUT_COL_IDX][i]);
        assert_eq!(result, trace[OUTPUT_COL_IDX][i]);

        prev_result = result;
    }
}

#[test]
fn bitwise_xor() {
    let mut bitwise = Bitwise::new();

    let a = rand_u32();
    let b = rand_u32();

    let result = bitwise.u32xor(a, b).unwrap();
    assert_eq!(a.as_int() ^ b.as_int(), result.as_int());

    // --- check generated trace ----------------------------------------------
    let num_rows = 8;
    let mut trace = (0..TRACE_WIDTH)
        .map(|_| vec![ZERO; num_rows])
        .collect::<Vec<_>>();
    let mut fragment = TraceFragment::trace_to_fragment(&mut trace);

    bitwise.fill_trace(&mut fragment);

    // make sure the selector values specify bitwise XOR at each step in the trace
    for row in 0..OP_CYCLE_LEN {
        assert_eq!([trace[0][row], trace[1][row]], BITWISE_XOR);
    }

    // make sure result and result from the trace are the same
    assert_eq!(result, trace[OUTPUT_COL_IDX][OP_CYCLE_LEN - 1]);

    // make sure values a and b were decomposed correctly
    check_decomposition(&trace, 0, a.as_int(), b.as_int());

    // make sure the result was re-composed correctly
    let mut prev_result = ZERO;

    for i in 0..8 {
        let c0 = binary_xor(trace[4][i], trace[8][i]);
        let c1 = binary_xor(trace[5][i], trace[9][i]);
        let c2 = binary_xor(trace[6][i], trace[10][i]);
        let c3 = binary_xor(trace[7][i], trace[11][i]);

        let result_4_bit = c0 + Felt::new(2) * c1 + Felt::new(4) * c2 + Felt::new(8) * c3;
        let result = prev_result * Felt::new(16) + result_4_bit;

        assert_eq!(prev_result, trace[PREV_OUTPUT_COL_IDX][i]);
        assert_eq!(result, trace[OUTPUT_COL_IDX][i]);

        prev_result = result;
    }
}

#[test]
fn bitwise_multiple() {
    let mut bitwise = Bitwise::new();

    let a = [rand_u32(), rand_u32(), rand_u32(), rand_u32()];
    let b = [rand_u32(), rand_u32(), rand_u32(), rand_u32()];

    // first operation: AND
    let result0 = bitwise.u32and(a[0], b[0]).unwrap();
    assert_eq!(a[0].as_int() & b[0].as_int(), result0.as_int());

    // second operation: OR
    let result1 = bitwise.u32or(a[1], b[1]).unwrap();
    assert_eq!(a[1].as_int() | b[1].as_int(), result1.as_int());

    // third operation: XOR
    let result2 = bitwise.u32xor(a[2], b[2]).unwrap();
    assert_eq!(a[2].as_int() ^ b[2].as_int(), result2.as_int());

    // fourth operation: AND
    let result3 = bitwise.u32and(a[3], b[3]).unwrap();
    assert_eq!(a[3].as_int() & b[3].as_int(), result3.as_int());

    // --- check generated trace ----------------------------------------------
    let num_rows = 4 * OP_CYCLE_LEN;
    let mut trace = (0..TRACE_WIDTH)
        .map(|_| vec![ZERO; num_rows])
        .collect::<Vec<_>>();
    let mut fragment = TraceFragment::trace_to_fragment(&mut trace);

    bitwise.fill_trace(&mut fragment);

    // make sure results and results from the trace are the same
    assert_eq!(result0, trace[OUTPUT_COL_IDX][OP_CYCLE_LEN - 1]);
    assert_eq!(result1, trace[OUTPUT_COL_IDX][2 * OP_CYCLE_LEN - 1]);
    assert_eq!(result2, trace[OUTPUT_COL_IDX][3 * OP_CYCLE_LEN - 1]);
    assert_eq!(result3, trace[OUTPUT_COL_IDX][4 * OP_CYCLE_LEN - 1]);

    // make sure input values were decomposed correctly
    check_decomposition(&trace, 0, a[0].as_int(), b[0].as_int());
    check_decomposition(&trace, OP_CYCLE_LEN, a[1].as_int(), b[1].as_int());
    check_decomposition(&trace, 2 * OP_CYCLE_LEN, a[2].as_int(), b[2].as_int());
    check_decomposition(&trace, 3 * OP_CYCLE_LEN, a[3].as_int(), b[3].as_int());

    // make sure the results was re-composed correctly

    let mut prev_result = ZERO;
    for i in 0..OP_CYCLE_LEN {
        let c0 = binary_and(trace[4][i], trace[8][i]);
        let c1 = binary_and(trace[5][i], trace[9][i]);
        let c2 = binary_and(trace[6][i], trace[10][i]);
        let c3 = binary_and(trace[7][i], trace[11][i]);

        let result_4_bit = c0 + Felt::new(2) * c1 + Felt::new(4) * c2 + Felt::new(8) * c3;
        let result = prev_result * Felt::new(16) + result_4_bit;

        assert_eq!(prev_result, trace[PREV_OUTPUT_COL_IDX][i]);
        assert_eq!(result, trace[OUTPUT_COL_IDX][i]);

        prev_result = result;
    }

    let mut prev_result = ZERO;
    for i in OP_CYCLE_LEN..(2 * OP_CYCLE_LEN) {
        let c0 = binary_or(trace[4][i], trace[8][i]);
        let c1 = binary_or(trace[5][i], trace[9][i]);
        let c2 = binary_or(trace[6][i], trace[10][i]);
        let c3 = binary_or(trace[7][i], trace[11][i]);

        let result_4_bit = c0 + Felt::new(2) * c1 + Felt::new(4) * c2 + Felt::new(8) * c3;
        let result = prev_result * Felt::new(16) + result_4_bit;

        assert_eq!(prev_result, trace[PREV_OUTPUT_COL_IDX][i]);
        assert_eq!(result, trace[OUTPUT_COL_IDX][i]);

        prev_result = result;
    }

    let mut prev_result = ZERO;
    for i in (2 * OP_CYCLE_LEN)..(3 * OP_CYCLE_LEN) {
        let c0 = binary_xor(trace[4][i], trace[8][i]);
        let c1 = binary_xor(trace[5][i], trace[9][i]);
        let c2 = binary_xor(trace[6][i], trace[10][i]);
        let c3 = binary_xor(trace[7][i], trace[11][i]);

        let result_4_bit = c0 + Felt::new(2) * c1 + Felt::new(4) * c2 + Felt::new(8) * c3;
        let result = prev_result * Felt::new(16) + result_4_bit;

        assert_eq!(prev_result, trace[PREV_OUTPUT_COL_IDX][i]);
        assert_eq!(result, trace[OUTPUT_COL_IDX][i]);

        prev_result = result;
    }

    let mut prev_result = ZERO;
    for i in (3 * OP_CYCLE_LEN)..(4 * OP_CYCLE_LEN) {
        let c0 = binary_and(trace[4][i], trace[8][i]);
        let c1 = binary_and(trace[5][i], trace[9][i]);
        let c2 = binary_and(trace[6][i], trace[10][i]);
        let c3 = binary_and(trace[7][i], trace[11][i]);

        let result_4_bit = c0 + Felt::new(2) * c1 + Felt::new(4) * c2 + Felt::new(8) * c3;
        let result = prev_result * Felt::new(16) + result_4_bit;

        assert_eq!(prev_result, trace[PREV_OUTPUT_COL_IDX][i]);
        assert_eq!(result, trace[OUTPUT_COL_IDX][i]);

        prev_result = result;
    }
}

// HELPER FUNCTIONS
// ================================================================================================

fn check_decomposition(trace: &[Vec<Felt>], start: usize, a: u64, b: u64) {
    let mut bit_offset = 28;

    for i in start..start + 8 {
        let a = a >> bit_offset;
        let b = b >> bit_offset;

        assert_eq!(Felt::new(a), trace[A_COL_IDX][i]);
        assert_eq!(Felt::new(b), trace[B_COL_IDX][i]);

        assert_eq!(Felt::new(a & 1), trace[4][i]);
        assert_eq!(Felt::new((a >> 1) & 1), trace[5][i]);
        assert_eq!(Felt::new((a >> 2) & 1), trace[6][i]);
        assert_eq!(Felt::new((a >> 3) & 1), trace[7][i]);

        assert_eq!(Felt::new(b & 1), trace[8][i]);
        assert_eq!(Felt::new((b >> 1) & 1), trace[9][i]);
        assert_eq!(Felt::new((b >> 2) & 1), trace[10][i]);
        assert_eq!(Felt::new((b >> 3) & 1), trace[11][i]);

        bit_offset -= 4;
    }
}

fn binary_and(a: Felt, b: Felt) -> Felt {
    a * b
}

fn binary_or(a: Felt, b: Felt) -> Felt {
    a + b - a * b
}

fn binary_xor(a: Felt, b: Felt) -> Felt {
    a + b - Felt::new(2) * a * b
}

fn rand_u32() -> Felt {
    let value = rand_value::<u64>() as u32 as u64;
    Felt::new(value)
}
