pub struct Opcode {
    pub c: u8,    // nibble 1
    pub x: u8,    // nibble 2
    pub y: u8,    // nibble 3
    pub n: u8,    // nibble 4
    pub nn: u8,   // nibbles 3-4
    pub nnn: u16, // nibbles 2-4
}

impl Opcode {
    pub fn from(a: u8, b: u8) -> Self {
        Self {
            c: a >> 4,
            x: a & 0x0F,
            y: b >> 4,
            n: b & 0x0F,
            nn: b,
            nnn: ((a as u16 & 0x0F) << 8) + b as u16,
        }
    }
}

impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:X}{:X}{:X}{:X}", self.c, self.x, self.y, self.n)
    }
}

#[cfg(test)]
mod tests {
    use super::Opcode;
    #[test]
    fn optest() {
        let op = Opcode::from(0xAB, 0xCD);
        assert_eq!(op.c, 10);
        assert_eq!(op.x, 11);
        assert_eq!(op.y, 12);
        assert_eq!(op.n, 13);
        assert_eq!(op.nn, 205);
        assert_eq!(op.nnn, 3021);
    }
}
