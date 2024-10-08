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
    pub fn new(seed: Option<u64>) -> Self {
        let mut mash = Mash::new();
        let mut alea = Alea {
            c: 1.0,
            s0: mash.mash(" "),
            s1: mash.mash(" "),
            s2: mash.mash(" "),
        };

        let seed = seed.unwrap_or_else(|| Utc::now().nanosecond() as u64);

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

struct Mash {
    n: u32,
}

impl Mash {
    const INV_32: u32 = 0x100000000;
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
            self.n = self.n.wrapping_add((h * Self::INV_32 as f64) as u32);
        }
        (self.n as f64) * 2.3283064365386963e-10
    }
}
