
#[cfg(test)]
mod tests{
    use UoI::storage_unit::StorageUnit;
    use UoI::conversion::bit;
    use UoI::conversion::hartley;
    use UoI::conversion::nat;
    use UoI::conversion::trit;

    #[test]
    fn test_hartley_nat_hartley(){
        let hartley: f64 = 1.0;
        let nat = hartley::to_nat(hartley);
        let hartley_out = nat::to_hartley(nat);
        assert!(hartley-hartley_out <= 0.00000000000000011102230246251565);
    }

    #[test]
    fn test_nat_hartley_nat(){
        let nat: f64 = 1.0;
        let hartley = nat::to_hartley(nat);
        let nat_out = hartley::to_nat(hartley);
        assert!(nat-nat_out <= 0.00000000000000011102230246251565);
    }

    #[test]
    fn test_bit_trit_bit(){
        let bit: f64 = 1.0;
        let trit = bit::to_trit(bit);
        let bit_out = trit::to_bit(trit);
        assert!(bit-bit_out <= 0.00000000000000011102230246251565);
    }


    #[test]
    fn test_bit_hartley_bit(){
        let bit: f64 = 1.0;
        let hartley = bit::to_hartley(bit);
        let bit_out = hartley::to_bit(hartley);
        assert!(bit-bit_out <= 0.00000000000000011102230246251565);
    }


    #[test]
    fn test_bit_nat_bit(){
        let bit: f64 = 1.0;
        let nat = bit::to_nat(bit);
        let bit_out = nat::to_bit(nat);
        assert!(bit-bit_out <= 0.00000000000000011102230246251565);
    }


    #[test]
    fn test_trit_nat_trit(){
        let trit: f64 = 1.0;
        let nat = trit::to_nat(trit);
        let trit_out = nat::to_trit(nat);
        assert!(trit-trit_out <= 0.00000000000000011102230246251565);
    }

    #[test]
    fn add_storage_units(){
        let s1: StorageUnit = StorageUnit::new(Some(1.0), None, None, None);
        let s2: StorageUnit = StorageUnit::new(None, Some(1.0), None, None);
        let s3: StorageUnit = s1 + s2;
        assert_eq!(s3.get_bits(), 1.0 + hartley::to_bit(1.0));
    }


    #[test]
    fn sub_storage_units(){
        let s1: StorageUnit = StorageUnit::new(Some(1.0), None, None, None);
        let s2: StorageUnit = StorageUnit::new(None, Some(1.0), None, None);
        let s3: StorageUnit = s1 - s2;
        assert_eq!(s3.get_bits(), 1.0 - hartley::to_bit(1.0));
    }


    #[test]
    fn mul_storage_units(){
        let s1: StorageUnit = StorageUnit::new(Some(1.0), None, None, None);
        let s2: StorageUnit = StorageUnit::new(None, Some(1.0), None, None);
        let s3: StorageUnit = s1 * s2;
        assert_eq!(s3.get_bits(), 1.0 * hartley::to_bit(1.0));
    }


    #[test]
    fn div_storage_units(){
        let s1: StorageUnit = StorageUnit::new(Some(1.0), None, None, None);
        let s2: StorageUnit = StorageUnit::new(None, Some(1.0), None, None);
        let s3: StorageUnit = s1 / s2;
        assert_eq!(s3.get_bits(), 1.0 / hartley::to_bit(1.0));
    }
}