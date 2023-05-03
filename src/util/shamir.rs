use actix_web::middleware::Logger;
use bigdecimal::{BigDecimal, num_bigint::ToBigInt};
use rand::Rng;

use super::polynomials::Polynomial;

pub struct ShamirAlgorithm {
    pub degree: u8
}

#[allow(dead_code)]
impl ShamirAlgorithm {
    pub fn new(degree: Option<u8>) -> Self {
        let x = if degree.is_some() {
            degree.unwrap()
        } else {
            2
        };
        assert!(!(x.lt(&u8::from(2))), "Degree must be greater than or equal to 2");
        Self { degree: x }
    }

    pub fn polynomialGenerator(self, value: BigDecimal) -> Polynomial {
        let mut polynom: Vec<BigDecimal> = vec![];
        polynom.push(value);
        for _i in 1..=self.degree {
            let c: u128 = rand::thread_rng().gen();
            polynom.push(c.to_bigint().unwrap().into());
        }
        Polynomial::new(polynom, 'x')
    }

    pub fn fromValues(self, values: Vec<Vec<BigDecimal>>) -> Polynomial {
        assert!(values.len() <= self.degree as usize, "Size must be greater than degree {}", self.degree);
        let mut polynom: Polynomial = Polynomial::new(vec![BigDecimal::from(0)], 'x');
        for i in 0..=self.degree as usize {
            let mut inner_polynom: Option<Polynomial> = None;
            for j in 0..=self.degree as usize {
                if i != j {
                    let c1 = (BigDecimal::from(-1) * &values[j][0]) / (&values[i][0] - &values[j][0]);
                    let c2 = (BigDecimal::from(1)) / (&values[i][0] - &values[j][0]);
                    let coefs = vec![(&values[i][0] * c1), (&values[i][0] * c2)];
                    let tmp = Polynomial::new(coefs, 'x');
                    Logger::default().log_target(tmp.as_string());
                    if inner_polynom.is_none() { inner_polynom = Some(Polynomial::new(vec![BigDecimal::from(1)], 'x')) }
                    inner_polynom = Some(inner_polynom.unwrap().multiply(tmp));
                }
            }
            if inner_polynom.is_some() { polynom = polynom.add(inner_polynom.unwrap()); }
        }
        return polynom;
    }
}