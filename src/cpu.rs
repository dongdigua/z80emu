// the registers and memory or z80
// s means shadow
#[derive (Debug)]
struct Z80 {
    mem: [u8; 0xfff],
    A : u8,
    As: u8,
    B : u8,
    Bs: u8,
    C : u8,
    Cs: u8,
    D : u8,
    Ds: u8,
    E : u8,
    Es: u8,
    H : u8,
    Hs: u8,
    L : u8,
    Ls: u8,
    F : u8,
    Fs: u8,
    I : u8,
    R : u8,
    SP: u16,
    PC: u16,
    IX: u16,
    IY: u16,
}
