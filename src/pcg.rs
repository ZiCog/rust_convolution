pub struct Pcg32 {
    state: u64,
    inc: u64,
}

impl Pcg32 {
    pub fn new() -> Pcg32 {
        Pcg32 {
            state: 0xdeadbeef01234567,
            inc: 1,
        }
    }

    pub fn rand(&mut self) -> u32 {
        let old_state = self.state;

        // Advance internal state
        self.state = old_state.overflowing_mul(6364136223846793005).0 + (self.inc | 1);

        // Calculate output function (XSH RR), uses old state for max ILP
        let xorshifted: u32 = (((old_state >> 18) ^ old_state) >> 27) as u32;
        let rot: u32 = (old_state >> 59) as u32;
        (xorshifted >> rot) | (xorshifted << (((rot ^ 0xffffffff).overflowing_add(1).0) & 31))
    }

    pub fn frand(&mut self) -> f32 {
        let rnd = self.rand() as f32;
        let f = rnd / (0x100000000i64 as f32);
        f
    }
}
