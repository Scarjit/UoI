pub mod hartley{
    use crate::conversion::bit;
    #[inline]
    pub const fn to_bit(hartleys: f64) -> f64{
        std::f64::consts::LOG2_10 * hartleys
    }

    #[inline]
    pub const fn to_nat(hartleys: f64) -> f64{
        std::f64::consts::LN_10 * hartleys
    }

    pub fn to_trit(hartleys: f64) -> f64 {
        bit::to_trit(to_bit(hartleys))
    }
}

pub mod nat {
    use crate::conversion::bit;

    #[inline]
    pub const fn to_bit(nat: f64) -> f64 {
        (1.0 / std::f64::consts::LN_2) * nat
    }

    #[inline]
    pub const fn to_hartley(nat: f64) -> f64 {
        (1.0 / std::f64::consts::LN_10) * nat
    }

    pub fn to_trit(nat: f64) -> f64 {
        bit::to_trit(to_bit(nat))
    }
}

pub mod trit{
    use crate::conversion::bit;

    pub fn to_bit(trit: f64) -> f64{
        let three: f64 = 3.0;
        three.log2() * trit
    }

    pub fn to_hartley(trit: f64) -> f64{
        bit::to_hartley(to_bit(trit))
    }

    pub fn to_nat(trit: f64) -> f64{
        bit::to_nat(to_bit(trit))
    }
}

pub mod bit {
    pub fn to_trit(bit: f64) -> f64{
        let two: f64 = 2.0;
        bit * two.log(3.0)
    }

    pub const fn to_hartley(bit: f64) -> f64{
        std::f64::consts::LOG10_2 * bit
    }

    pub const fn to_nat(bit: f64) -> f64{
        std::f64::consts::LN_10 * bit
    }
}