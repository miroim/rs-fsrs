use chrono::{Timelike, Utc};
struct AleaState {
    c: f64,
    s0: f64,
    s1: f64,
    s2: f64,
}

pub struct Alea {
    c: f64,
    s0: f64,
    s1: f64,
    s2: f64,
}

impl Alea {
    pub fn new(seed: Option<String>) -> Self {
        let mut mash = Mash::new();
        let mut alea = Alea {
            c: 1.0,
            s0: mash.mash(" "),
            s1: mash.mash(" "),
            s2: mash.mash(" "),
        };

        let seed = seed.unwrap_or_else(|| Utc::now().nanosecond().to_string());

        alea.s0 -= mash.mash(&seed.to_string());
        if alea.s0 < 0.0 {
            alea.s0 += 1.0;
        }
        alea.s1 -= mash.mash(&seed.to_string());
        if alea.s1 < 0.0 {
            alea.s1 += 1.0;
        }
        alea.s2 -= mash.mash(&seed.to_string());
        if alea.s2 < 0.0 {
            alea.s2 += 1.0;
        }

        alea
    }

    fn next(&mut self) -> f64 {
        // 2.3283064365386963e-10: 2^-32
        let t = 2091639.0 * self.s0 + self.c * 2.3283064365386963e-10;
        self.s0 = self.s1;
        self.s1 = self.s2;
        self.c = t.floor();
        self.s2 = t - self.c;
        self.s2
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

const INV_32: u64 = 0x100000000; // 2^32
const INV_21: u32 = 0x200000; // 2^21

struct Mash {
    n: u32,
}

impl Mash {
    fn new() -> Self {
        Mash { n: 0xefc8249d }
    }

    fn mash(&mut self, data: &str) -> f64 {
        for c in data.chars() {
            self.n = self.n.wrapping_add(c as u32);
            let mut h = 0.02519603282416938 * self.n as f64;
            self.n = h as u32;
            h -= self.n as f64;
            h *= self.n as f64;
            self.n = h as u32;
            h -= self.n as f64;
            self.n = self.n.wrapping_add((h * INV_32 as f64) as u32);
        }
        (self.n as f64) * 2.3283064365386963e-10 // 2^-32
    }
}

pub struct Prng {
    xg: Alea,
}

impl Prng {
    fn new(seed: Option<String>) -> Self {
        Prng {
            xg: Alea::new(seed),
        }
    }

    fn next(&mut self) -> f64 {
        self.xg.next()
    }

    fn int32(&mut self) -> i32 {
        (self.next() * INV_32 as f64) as i32
    }

    pub fn double(&mut self) -> f64 {
        // 1.1102230246251565e-16: 2^-53
        self.next() + ((self.next() * INV_21 as f64) as u32 as f64) * 1.1102230246251565e-16
    }

    fn get_state(&self) -> AleaState {
        self.xg.get_state()
    }

    fn import_state(&mut self, state: AleaState) {
        self.xg.set_state(state);
    }
}

pub fn alea(seed: Option<String>) -> Prng {
    Prng::new(seed)
}
