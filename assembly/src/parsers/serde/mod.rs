use num_enum::TryFromPrimitive;

mod serialization;
pub use serialization::{ByteWriter, Serializable};

mod deserialization;
pub use deserialization::{ByteReader, Deserializable};

const IF_ELSE_OPCODE: u8 = 253;
const REPEAT_OPCODE: u8 = 254;
const WHILE_OPCODE: u8 = 255;

// OPERATION CODES ENUM
// ================================================================================================

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, TryFromPrimitive)]
pub enum OpCode {
    Assert = 0,
    AssertEq = 1,
    Assertz = 2,
    Add = 3,
    AddImm = 4,
    Sub = 5,
    SubImm = 6,
    Mul = 7,
    MulImm = 8,
    Div = 9,
    DivImm = 10,
    Neg = 11,
    Inv = 12,
    Pow2 = 13,
    Exp = 14,
    ExpImm = 15,
    ExpBitLength = 16,
    Not = 17,
    And = 18,
    Or = 19,
    Xor = 20,
    Eq = 21,
    EqImm = 22,
    Neq = 23,
    NeqImm = 24,
    Eqw = 25,
    Lt = 26,
    Lte = 27,
    Gt = 28,
    Gte = 29,

    // ----- u32 manipulation ---------------------------------------------------------------
    U32Test = 30,
    U32TestW = 31,
    U32Assert = 32,
    U32Assert2 = 33,
    U32AssertW = 34,
    U32Split = 35,
    U32Cast = 36,
    U32CheckedAdd = 37,
    U32CheckedAddImm = 38,
    U32WrappingAdd = 39,
    U32WrappingAddImm = 40,
    U32OverflowingAdd = 41,
    U32OverflowingAddImm = 42,
    U32OverflowingAdd3 = 43,
    U32WrappingAdd3 = 44,
    U32CheckedSub = 45,
    U32CheckedSubImm = 46,
    U32WrappingSub = 47,
    U32WrappingSubImm = 48,
    U32OverflowingSub = 49,
    U32OverflowingSubImm = 50,
    U32CheckedMul = 51,
    U32CheckedMulImm = 52,
    U32WrappingMul = 53,
    U32WrappingMulImm = 54,
    U32OverflowingMul = 55,
    U32OverflowingMulImm = 56,
    U32OverflowingMadd = 57,
    U32WrappingMadd = 58,
    U32CheckedDiv = 59,
    U32CheckedDivImm = 60,
    U32UncheckedDiv = 61,
    U32UncheckedDivImm = 62,
    U32CheckedMod = 63,
    U32CheckedModImm = 64,
    U32UncheckedMod = 65,
    U32UncheckedModImm = 66,
    U32CheckedDivMod = 67,
    U32CheckedDivModImm = 68,
    U32UncheckedDivMod = 69,
    U32UncheckedDivModImm = 70,
    U32CheckedAnd = 71,
    U32CheckedOr = 72,
    U32CheckedXor = 73,
    U32CheckedNot = 74,
    U32CheckedShr = 75,
    U32CheckedShrImm = 76,
    U32UncheckedShr = 77,
    U32UncheckedShrImm = 78,
    U32CheckedShl = 79,
    U32CheckedShlImm = 80,
    U32UncheckedShl = 81,
    U32UncheckedShlImm = 82,
    U32CheckedRotr = 83,
    U32CheckedRotrImm = 84,
    U32UncheckedRotr = 85,
    U32UncheckedRotrImm = 86,
    U32CheckedRotl = 87,
    U32CheckedRotlImm = 88,
    U32UncheckedRotl = 89,
    U32UncheckedRotlImm = 90,
    U32CheckedEq = 91,
    U32CheckedEqImm = 92,
    U32CheckedNeq = 93,
    U32CheckedNeqImm = 94,
    U32CheckedLt = 95,
    U32UncheckedLt = 96,
    U32CheckedLte = 97,
    U32UncheckedLte = 98,
    U32CheckedGt = 99,
    U32UncheckedGt = 100,
    U32CheckedGte = 101,
    U32UncheckedGte = 102,
    U32CheckedMin = 103,
    U32UncheckedMin = 104,
    U32CheckedMax = 105,
    U32UncheckedMax = 106,

    // ----- stack manipulation ---------------------------------------------------------------
    Drop = 107,
    DropW = 108,
    PadW = 109,
    Dup0 = 110,
    Dup1 = 111,
    Dup2 = 112,
    Dup3 = 113,
    Dup4 = 114,
    Dup5 = 115,
    Dup6 = 116,
    Dup7 = 117,
    Dup8 = 118,
    Dup9 = 119,
    Dup10 = 120,
    Dup11 = 121,
    Dup12 = 122,
    Dup13 = 123,
    Dup14 = 124,
    Dup15 = 125,
    DupW0 = 126,
    DupW1 = 127,
    DupW2 = 128,
    DupW3 = 129,
    Swap1 = 130,
    Swap2 = 131,
    Swap3 = 132,
    Swap4 = 133,
    Swap5 = 134,
    Swap6 = 135,
    Swap7 = 136,
    Swap8 = 137,
    Swap9 = 138,
    Swap10 = 139,
    Swap11 = 140,
    Swap12 = 141,
    Swap13 = 142,
    Swap14 = 143,
    Swap15 = 144,
    SwapW1 = 145,
    SwapW2 = 146,
    SwapW3 = 147,
    SwapDW = 148,
    MovUp2 = 149,
    MovUp3 = 150,
    MovUp4 = 151,
    MovUp5 = 152,
    MovUp6 = 153,
    MovUp7 = 154,
    MovUp8 = 155,
    MovUp9 = 156,
    MovUp10 = 157,
    MovUp11 = 158,
    MovUp12 = 159,
    MovUp13 = 160,
    MovUp14 = 161,
    MovUp15 = 162,
    MovUpW2 = 163,
    MovUpW3 = 164,
    MovDn2 = 165,
    MovDn3 = 166,
    MovDn4 = 167,
    MovDn5 = 168,
    MovDn6 = 169,
    MovDn7 = 170,
    MovDn8 = 171,
    MovDn9 = 172,
    MovDn10 = 173,
    MovDn11 = 174,
    MovDn12 = 175,
    MovDn13 = 176,
    MovDn14 = 177,
    MovDn15 = 178,
    MovDnW2 = 179,
    MovDnW3 = 180,
    CSwap = 181,
    CSwapW = 182,
    CDrop = 183,
    CDropW = 184,

    // ----- input / output operations --------------------------------------------------------
    PushConstants = 185,

    Locaddr = 186,
    Sdepth = 187,
    Caller = 188,

    MemLoad = 189,
    MemLoadImm = 190,
    MemLoadW = 191,
    MemLoadWImm = 192,
    LocLoad = 193,
    LocLoadW = 194,
    MemStore = 195,
    MemStoreImm = 196,
    LocStore = 197,
    MemStoreW = 198,
    MemStoreWImm = 199,
    LocStoreW = 200,

    MemStream = 201,
    AdvPipe = 202,

    AdvPush = 203,
    AdvLoadW = 204,

    AdvU64Div = 205,
    AdvKeyval = 206,
    AdvMem = 207,

    // ----- cryptographic operations ---------------------------------------------------------
    RPHash = 208,
    RPPerm = 209,
    MTreeGet = 210,
    MTreeSet = 211,
    MTreeCwm = 212,

    // ----- exec / call ----------------------------------------------------------------------
    ExecLocal = 213,
    ExecImported = 214,
    CallLocal = 215,
    CallImported = 216,
    SysCall = 217,
}
