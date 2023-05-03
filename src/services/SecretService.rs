use std::{str::FromStr};

use bigdecimal::{BigDecimal, num_bigint::{ToBigInt}};
use rand::Rng;
use primitive_types::U256;

use crate::util::shamir::ShamirAlgorithm;

pub struct SecretService;

#[allow(dead_code)]
impl SecretService {
    fn getRandomDifferentNumbers(amount: u8) -> Vec<BigDecimal> {
        assert!(!(amount <= 2_u8), "Amount: {} must be lower than {}", amount, 2);
        let mut result: Vec<BigDecimal> = vec![];
        while result.len() != amount as usize {
            let rand = rand::thread_rng().gen_range(1..255);
            if !result.contains(&BigDecimal::from(rand.to_bigint().unwrap())) {
                result.push(BigDecimal::from(rand.to_bigint().unwrap()));
            }
        }
        result
    }

    pub fn secretPartition(degree: u8, secret: String, parties: u8) -> Vec<Vec<BigDecimal>> {
        let shamir = ShamirAlgorithm::new(Some(degree));
        let rand_nums = self::SecretService::getRandomDifferentNumbers(parties);
        let secret = U256::from_str_radix(secret.as_str(), 16).unwrap().to_string();
        let polynomial = shamir.polynomialGenerator(BigDecimal::from_str(&secret).unwrap());
        let mut result: Vec<Vec<BigDecimal>> = vec![];
        for i in 0..rand_nums.len() {
            let evaluation = polynomial.evaluate_at(rand_nums[i].to_owned());
            result.push(vec![rand_nums[i].to_owned(), evaluation])
        }
        result
    }

    pub fn getSecret(degree: u8, values: Vec<Vec<BigDecimal>>) -> String {
        let shamir = ShamirAlgorithm::new(Some(degree));
        shamir.fromValues(values).as_string()
    }
}