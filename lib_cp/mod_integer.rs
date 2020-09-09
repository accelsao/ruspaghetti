// ███▓▓▓▓▓▓▓▓▓╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬▓▓▓╬╬╬╬╬╬▓█
// ███▓███████▓▓╬╬╬╬╬╬╬╬╬╬╬╬▓███▓▓▓▓█▓╬╬╬▓█
// ███████▓█████▓▓╬╬╬╬╬╬╬╬▓███▓╬╬╬╬╬╬╬▓╬╬▓█
// ████▓▓▓▓╬╬▓█████╬╬╬╬╬╬███▓╬╬╬╬╬╬╬╬╬╬╬╬╬█
// ███▓▓▓▓╬╬╬╬╬╬▓██╬╬╬╬╬╬▓▓╬╬╬╬╬╬╬╬╬╬╬╬╬╬▓█
// ████▓▓▓╬╬╬╬╬╬╬▓█▓╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬▓█
// ███▓█▓███████▓▓███▓╬╬╬╬╬╬▓███████▓╬╬╬╬▓█
// ████████████████▓█▓╬╬╬╬╬▓▓▓▓▓▓▓▓╬╬╬╬╬╬╬█
// ███▓▓▓▓▓▓▓╬╬▓▓▓▓▓█▓╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬▓█
// ████▓▓▓╬╬╬╬▓▓▓▓▓▓█▓╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬▓█
// ███▓█▓▓▓▓▓▓▓▓▓▓▓▓▓▓╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬▓█
// █████▓▓▓▓▓▓▓▓█▓▓▓█▓╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬▓█
// █████▓▓▓▓▓▓▓██▓▓▓█▓╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬██
// █████▓▓▓▓▓████▓▓▓█▓╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬██
// ████▓█▓▓▓▓██▓▓▓▓██╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬██
// ████▓▓███▓▓▓▓▓▓▓██▓╬╬╬╬╬╬╬╬╬╬╬╬█▓╬▓╬╬▓██
// █████▓███▓▓▓▓▓▓▓▓████▓▓╬╬╬╬╬╬╬█▓╬╬╬╬╬▓██
// █████▓▓█▓███▓▓▓████╬▓█▓▓╬╬╬▓▓█▓╬╬╬╬╬╬███
// ██████▓██▓███████▓╬╬╬▓▓╬▓▓██▓╬╬╬╬╬╬╬▓███
// ███████▓██▓▓▓▓▓▓▓▓▓▓▓▓▓▓▓╬╬╬╬╬╬╬╬╬╬╬████
// ███████▓▓██▓▓▓▓▓╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬▓████
// ████████▓▓▓█████▓▓╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬╬▓█████
// █████████▓▓▓█▓▓▓▓▓███▓╬╬╬╬╬╬╬╬╬╬╬▓██████
// ██████████▓▓▓█▓▓▓╬▓██╬╬╬╬╬╬╬╬╬╬╬▓███████
// ███████████▓▓█▓▓▓▓███▓╬╬╬╬╬╬╬╬╬▓████████
// ██████████████▓▓▓███▓▓╬╬╬╬╬╬╬╬██████████
// ███████████████▓▓▓██▓▓╬╬╬╬╬╬▓███████████
#![allow(unused)]

// //////////////
// Mod Integer //
// //////////////
const MOD: usize = 1_000_000_007;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
struct Modulo {
    value: usize,
}

impl Modulo {
    fn new(value: usize) -> Self {
        assert!(value < MOD);
        Self { value }
    }
    fn pow(&self, p: usize) -> Self {
        let mut ans = 1;
        let mut a = self.value;
        let mut p = p;
        while p > 0 {
            if p & 1 == 1 {
                ans = ans * a % MOD;
            }
            a = a * a % MOD;
            p = p / 2;
        }
        Modulo::new(ans)
    }
    fn inv(&self) -> Self {
        self.pow(MOD - 2)
    }
}

impl std::ops::Mul for Modulo {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Modulo::new(self.value * rhs.value % MOD)
    }
}

impl std::ops::MulAssign for Modulo {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl std::ops::Add for Modulo {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Modulo::new((self.value + rhs.value) % MOD)
    }
}

impl std::ops::AddAssign for Modulo {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for Modulo {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Modulo::new((self.value + MOD - rhs.value) % MOD)
    }
}

impl std::ops::SubAssign for Modulo {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::Div for Modulo {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}

impl std::ops::DivAssign for Modulo {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

impl std::ops::Neg for Modulo {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Modulo::new(0) - self
    }
}

impl std::fmt::Display for Modulo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

fn main() {
    let a = 3;
    let b = 4;
    let mut a = Modulo::new(a);
    let b = Modulo::new(b);
    let mut div = a / b;
    div *= b;
    assert_eq!(div, a);
    let mut c = a - b;
    assert_eq!(c + Modulo::new(1), Modulo::new(0));
    c += Modulo::new(2);
    c *= Modulo::new(3);
    c /= Modulo::new(3);
    c -= Modulo::new(10);
    assert_eq!(c, Modulo::new(0) - Modulo::new(9));
    let e = -Modulo::new(9);
    assert_eq!(c, e);
}
