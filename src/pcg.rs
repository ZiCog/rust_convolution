#[derive(Default, Debug, Copy, Clone)]
pub struct Pcg32 {
    state: u64,
    inc: u64,
}

// Implement `Iterator` for `Pcg32`.
// The `Iterator` trait only requires a method to be defined for the `next` element.
impl Iterator for Pcg32 {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let old_state = self.state;

        // Advance internal state
        self.state = old_state.overflowing_mul(6364136223846793005).0 + (self.inc | 1);

        // Calculate output function (XSH RR), uses old state for max ILP
        let xorshifted: u32 = (((old_state >> 18) ^ old_state) >> 27) as u32;
        let rot: u32 = (old_state >> 59) as u32;
        Some((xorshifted >> rot) | (xorshifted << (((rot ^ 0xffffffff).overflowing_add(1).0) & 31)))
    }
}

impl Pcg32 {
    pub fn new() -> Pcg32 {
        Pcg32 {
            state: 0xdeadbeef01234567,
            inc: 1,
        }
    }

    pub fn rand(&mut self) -> u32 {
        self.next().unwrap()
    }

    pub fn frand(&mut self) -> f32 {
        let rnd = self.next().unwrap() as f32;
        rnd / (0x100000000i64 as f32)
    }
}

#[cfg(test)]
mod tests {
    use crate::pcg::Pcg32;
    #[test]
    fn test_rand() {
        let mut pcg32 = Pcg32::new();

        let mut r = pcg32.next().unwrap();
        assert_eq!(r, 3055882682);

        r = pcg32.next().unwrap();
        assert_eq!(r, 0300354198);

        r = pcg32.next().unwrap();
        assert_eq!(r, 3539460393);
    }
    /*
        let mut skipped = pcg32.by_ref().skip(1000);

        for r in skipped.by_ref().take(10) {
            println! ("random u32 = {0:010}", r);
        }


        println!("{:?}", pcg32.frand());
        println!("{:?}", pcg32.frand());
    */
}
