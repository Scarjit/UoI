
#[cfg(test)]
mod tests{
    use uoi::storage_unit::StorageUnit;
    use uoi::conversion::bit;
    use uoi::conversion::hartley;
    use uoi::conversion::nat;
    use uoi::conversion::trit;
    use std::f64::EPSILON;

    const MAX_FLOAT_ROUNDING_DEVIATION: f64 = EPSILON/2.0;
    #[test]
    fn test_hartley_nat_hartley(){
        let hartley: f64 = 1.0;
        let nat = hartley::to_nat(hartley);
        let hartley_out = nat::to_hartley(nat);
        assert!(hartley-hartley_out <= MAX_FLOAT_ROUNDING_DEVIATION);
    }

    #[test]
    fn test_nat_hartley_nat(){
        let nat: f64 = 1.0;
        let hartley = nat::to_hartley(nat);
        let nat_out = hartley::to_nat(hartley);
        assert!(nat-nat_out <= MAX_FLOAT_ROUNDING_DEVIATION);
    }

    #[test]
    fn test_bit_trit_bit(){
        let bit: f64 = 1.0;
        let trit = bit::to_trit(bit);
        let bit_out = trit::to_bit(trit);
        assert!(bit-bit_out <= MAX_FLOAT_ROUNDING_DEVIATION);
    }


    #[test]
    fn test_bit_hartley_bit(){
        let bit: f64 = 1.0;
        let hartley = bit::to_hartley(bit);
        let bit_out = hartley::to_bit(hartley);
        assert!(bit-bit_out <= MAX_FLOAT_ROUNDING_DEVIATION);
    }


    #[test]
    fn test_bit_nat_bit(){
        let bit: f64 = 1.0;
        let nat = bit::to_nat(bit);
        let bit_out = nat::to_bit(nat);
        assert!(bit-bit_out <= MAX_FLOAT_ROUNDING_DEVIATION);
    }


    #[test]
    fn test_trit_nat_trit(){
        let trit: f64 = 1.0;
        let nat = trit::to_nat(trit);
        let trit_out = nat::to_trit(nat);
        assert!(trit-trit_out <= MAX_FLOAT_ROUNDING_DEVIATION);
    }

    #[test]
    fn add_storage_units(){
        let s1: StorageUnit = StorageUnit::new(Some(1.0), None, None, None);
        let s2: StorageUnit = StorageUnit::new(None, Some(1.0), None, None);
        let s3: StorageUnit = s1 + s2;
        //s3.get_bits(), 1.0 + hartley::to_bit(1.0)

        assert!(s3.get_bits() - (1.0 + hartley::to_bit(1.0)) <= MAX_FLOAT_ROUNDING_DEVIATION);
    }


    #[test]
    fn sub_storage_units(){
        let s1: StorageUnit = StorageUnit::new(Some(1.0), None, None, None);
        let s2: StorageUnit = StorageUnit::new(None, Some(1.0), None, None);
        let s3: StorageUnit = s1 - s2;

        assert!(s3.get_bits() - ( 1.0 - hartley::to_bit(1.0)) <= MAX_FLOAT_ROUNDING_DEVIATION);
    }


    #[test]
    fn mul_storage_units(){
        let s1: StorageUnit = StorageUnit::new(Some(1.0), None, None, None);
        let s2: StorageUnit = StorageUnit::new(None, Some(1.0), None, None);
        let s3: StorageUnit = s1 * s2;
        assert!(s3.get_bits() - ( 1.0 * hartley::to_bit(1.0)) <= MAX_FLOAT_ROUNDING_DEVIATION);
    }


    #[test]
    fn div_storage_units(){
        let s1: StorageUnit = StorageUnit::new(Some(1.0), None, None, None);
        let s2: StorageUnit = StorageUnit::new(None, Some(1.0), None, None);
        let s3: StorageUnit = s1 / s2;
        assert!(s3.get_bits() - ( 1.0 / hartley::to_bit(1.0)) <= MAX_FLOAT_ROUNDING_DEVIATION);
    }
}