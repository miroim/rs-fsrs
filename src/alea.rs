use chrono::{Timelike, Utc};

#[derive(Debug, PartialEq)]
pub struct AleaState {
    pub c: f64,
    pub s0: f64,
    pub s1: f64,
    pub s2: f64,
}

#[derive(Debug)]
pub struct Alea {
    c: f64,
    s0: f64,
    s1: f64,
    s2: f64,
}

impl Alea {
    fn new(seed: Option<String>) -> Self {
        let mut mash = Mash::new();
        let mut alea = Alea {
            c: 1.0,
            s0: mash.mash(" ".to_string()),
            s1: mash.mash(" ".to_string()),
            s2: mash.mash(" ".to_string()),
        };

        let seed = seed.unwrap_or_else(|| Utc::now().nanosecond().to_string());
        alea.s0 -= mash.mash(seed.to_string());
        if alea.s0 < 0.0 {
            alea.s0 += 1.0;
        }
        alea.s1 -= mash.mash(seed.to_string());
        if alea.s1 < 0.0 {
            alea.s1 += 1.0;
        }
        alea.s2 -= mash.mash(seed.to_string());
        if alea.s2 < 0.0 {
            alea.s2 += 1.0;
        }

        alea
    }

    fn set_state(&mut self, state: AleaState) {
        self.c = state.c;
        self.s0 = state.s0;
        self.s1 = state.s1;
        self.s2 = state.s2;
    }

    fn get_state(&self) -> AleaState {
        AleaState {
            c: self.c,
            s0: self.s0,
            s1: self.s1,
            s2: self.s2,
        }
    }
}

impl Iterator for Alea {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        // 2.3283064365386963e-10: 2^-32
        let t = 2091639.0 * self.s0 + self.c * TWO_OF_THE_MINUS_32;
        self.s0 = self.s1;
        self.s1 = self.s2;
        self.c = t.floor();
        self.s2 = t - self.c;

        Some(self.s2)
    }
}

const INV_32: u64 = 0x100000000; // 2^32
const INV_21: u32 = 0x200000; // 2^21
const TWO_OF_THE_MINUS_32: f64 = 1.0 / ((1_u64 << 32) as f64);

struct Mash {
    n: u64,
}

impl Mash {
    fn new() -> Self {
        Mash { n: 0xefc8249d }
    }

    fn mash(&mut self, data: String) -> f64 {
        for c in data.chars() {
            self.n += c as u64;
            let mut h = 0.02519603282416938 * self.n as f64;
            self.n = h as u64;
            h -= self.n as f64;
            h *= self.n as f64;
            self.n = h as u64;
            h -= self.n as f64;
            self.n += (h * INV_32 as f64) as u64;
        }
        (self.n as f64) * 2.3283064365386963e-10 // 2^-32
    }
}

#[derive(Debug)]
pub struct Prng {
    pub xg: Alea,
}

impl Prng {
    fn new(seed: Option<String>) -> Self {
        Self {
            xg: Alea::new(seed),
        }
    }

    pub fn next(&mut self) -> f64 {
        self.xg.next().unwrap()
    }

    pub fn int32(&mut self) -> i32 {
        (self.next() * INV_32 as f64) as i32
    }

    pub fn double(&mut self) -> f64 {
        // 1.1102230246251565e-16: 2^-53
        self.next() + ((self.next() * INV_21 as f64) as u32 as f64) * 1.1102230246251565e-16
    }

    pub fn get_state(&self) -> AleaState {
        self.xg.get_state()
    }

    #[allow(dead_code)]
    fn import_state(&mut self, state: AleaState) {
        self.xg.set_state(state);
    }
}

pub fn alea(seed: Option<String>) -> Prng {
    Prng::new(seed)
}
