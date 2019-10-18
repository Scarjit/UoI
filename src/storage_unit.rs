use crate::conversion::{hartley, trit, nat, bit};
use std::ops::{Add, Sub, Div, Mul};

#[derive(Debug)]
pub struct StorageUnit {
    bit: f64,
    hartley: f64,
    nat: f64,
    trit: f64
}

impl Add for StorageUnit{
    type Output = StorageUnit;

    fn add(self, rhs: Self) -> StorageUnit {
        StorageUnit{
            bit: self.bit + rhs.bit,
            hartley: self.hartley + rhs.hartley,
            nat: self.nat + rhs.nat,
            trit: self.trit + rhs.trit
        }
    }
}

impl Sub for StorageUnit{
    type Output = StorageUnit;

    fn sub(self, rhs: Self) -> StorageUnit{
        StorageUnit{
            bit: self.bit - rhs.bit,
            hartley: self.hartley - rhs.hartley,
            nat: self.nat - rhs.nat,
            trit: self.trit - rhs.trit
        }
    }
}

impl Div for StorageUnit{
    type Output = StorageUnit;

    fn div(self, rhs: Self) -> StorageUnit {
        StorageUnit::new(Some(self.get_bits() / rhs.get_bits()), None, None, None)
    }
}

impl Mul for StorageUnit{
    type Output = StorageUnit;

    fn mul(self, rhs: Self) -> StorageUnit {
        StorageUnit::new(Some(self.get_bits() * rhs.get_bits()), None, None, None)
    }
}

impl StorageUnit {
    pub fn new(bits: Option<f64>, hartleys: Option<f64>, nats: Option<f64>, trits: Option<f64>) -> StorageUnit{
        StorageUnit{
            bit: bits.unwrap_or_default(),
            hartley: hartleys.unwrap_or_default(),
            nat: nats.unwrap_or_default(),
            trit: trits.unwrap_or_default()
        }
    }

    pub fn get_bits(&self) -> f64{
        self.bit + trit::to_bit(self.trit) + nat::to_bit(self.nat) + hartley::to_bit(self.hartley)
    }

    pub fn get_hartleys(&self) -> f64{
        bit::to_hartley(self.bit) + trit::to_hartley(self.trit) + nat::to_hartley(self.nat) + self.hartley
    }

    pub fn get_nats(&self) -> f64{
        bit::to_nat(self.bit) + trit::to_nat(self.trit) + self.nat + hartley::to_nat(self.hartley)
    }

    pub fn get_trits(&self) -> f64{
        bit::to_trit(self.bit) + self.trit + nat::to_trit(self.nat) + hartley::to_trit(self.hartley)
    }
}