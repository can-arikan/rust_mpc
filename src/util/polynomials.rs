use std::{fmt, str::FromStr};

use bigdecimal::{BigDecimal};
use primitive_types::U256;

/// A simple polynomial representation with `coefficients` and an `indeterminate`. 
pub struct Polynomial {
    /// Coefficients of Polynomial. The index of each coefficient indicates its degree, for example in `vec![1, 2]`, the first value is explicitly `1x^0`, the second is `2x^1`, etc.
    pub coefficients: Vec<BigDecimal>,
    /// The `char` representation of the indeterminate, eg. _f(**x**) = 1 + 2x_
    pub indeterminate: char,
}

impl fmt::Debug for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Polynomial {{ coefficients: {:?}, indeterminate: '{ind}', as_string: {} }}",
            self.coefficients,
            self.as_string(),
            ind = self.indeterminate
        )
    }
}

#[allow(dead_code)]
impl Polynomial {
    fn strip_from_end<T: PartialEq + Clone + Default>(list: Vec<T>, object: T) -> Vec<T> {
        let mut new_list = list.clone();
        let mut strip_amount: usize = 0;
        for item in list.iter().rev() {
            if *item == object {
                strip_amount += 1;
            } else {
                break;
            }
        }
        let default: T = Default::default();
        new_list.resize(list.len() - strip_amount, default);
        new_list
    }

    /// Returns a Polynomial from a vector of floats and an indeterminate
    /// # Example
    /// ```
    /// use polynom::polynomial::Polynomial;
    ///
    /// let polynomial = Polynomial::new(vec![1BigDecimal, 2BigDecimal, 3BigDecimal], 'x');
    /// assert_eq!(polynomial.coefficients, vec![1BigDecimal, 2BigDecimal, 3BigDecimal]);
    /// ```
    pub fn new(coefficients: Vec<BigDecimal>, indeterminate: char) -> Polynomial {
        let stripped_coefficients = self::Polynomial::strip_from_end(coefficients, BigDecimal::from(0));
        // Zero degree special case
        if stripped_coefficients.len() == 0 {
            return Polynomial {
                coefficients: vec![BigDecimal::from(0)],
                indeterminate,
            };
        }

        Polynomial {
            coefficients: stripped_coefficients,
            indeterminate,
        }
    }

    /// Returns a Polynomial from a vector of integers and an indeterminate
    /// # Example
    /// ```
    /// use polynom::polynomial::Polynomial;
    ///
    /// let polynomial = Polynomial::from_ints(vec![1, 2, 3], 'x');
    /// assert_eq!(polynomial.coefficients, vec![1BigDecimal, 2BigDecimal, 3BigDecimal]);
    /// ```
    pub fn from_ints(coefficients: Vec<BigDecimal>, indeterminate: char) -> Polynomial {
        let stripped_coefficients = self::Polynomial::strip_from_end(coefficients, BigDecimal::from(0));
        // Zero degree special case
        if stripped_coefficients.len() == 0 {
            return Polynomial {
                coefficients: vec![BigDecimal::from(0)],
                indeterminate,
            };
        }

        let float_coefficients = stripped_coefficients.iter().map(|x| x.to_owned()).collect();

        Polynomial {
            coefficients: float_coefficients,
            indeterminate,
        }
    }

    /// Adds the same-degree coefficients of `other: Polynomial` to the coefficients of `self`, and returns a new Polynomial with the summed coefficients.
    /// # Example
    /// ```
    /// use polynom::polynomial::Polynomial;
    ///
    /// let a_polynomial = Polynomial::from_ints(vec![1, 2, 3], 'x');
    /// let b_polynomial = Polynomial::from_ints(vec![1, 2, 3], 'x');
    /// 
    /// assert_eq!(a_polynomial.add(b_polynomial).coefficients, vec![2BigDecimal, 4BigDecimal, 6BigDecimal]);
    /// ``` 
    pub fn add(&self, other: Polynomial) -> Polynomial {
        let mut a_coefficients = self.coefficients.clone();
        let mut b_coefficients = other.coefficients.clone();

        // Resize coeff vectors to the longer size
        if a_coefficients.len() < b_coefficients.len() {
            a_coefficients.resize(b_coefficients.len(), BigDecimal::from(0))
        } else {
            b_coefficients.resize(a_coefficients.len(), BigDecimal::from(0))
        }

        let new_coefficients: Vec<BigDecimal> = a_coefficients
            .iter()
            .zip(b_coefficients)
            .map(|pair| pair.0 + pair.1)
            .collect();

        Polynomial::new(new_coefficients, 'x')
    }  

    /// Adds the same-degree coefficients of `other: Polynomial` to the coefficients of `self`, and returns a new Polynomial with the summed coefficients.
    /// # Example
    /// ```
    /// use polynom::polynomial::Polynomial;
    ///
    /// let a_polynomial = Polynomial::from_ints(vec![1, 2], 'x');
    /// let b_polynomial = Polynomial::from_ints(vec![2, 4], 'x');
    /// 
    /// assert_eq!(a_polynomial.sub(b_polynomial).coefficients, vec![BigDecimal::from(-1), BigDecimal::from(-2)]);
    /// ```

    pub fn sub(&self, other: Polynomial) -> Polynomial {
        let negative_coefficients: Vec<BigDecimal> = other
            .coefficients
            .iter()
            .map(|coeff| coeff * BigDecimal::from(-1))
            .collect();
        let negative = Polynomial::new(negative_coefficients, 'x');

        self.add(negative)
    }

    /// Multiplies the same-degree coefficients of `self` and `other`, and returns a Polynomial with the new coefficients.
    /// # Example
    /// ```
    /// use polynom::polynomial::Polynomial;
    ///
    /// let a_polynomial = Polynomial::from_ints(vec![1, 2], 'x');
    /// let b_polynomial = Polynomial::from_ints(vec![2, 4], 'x');
    /// 
    /// assert_eq!(a_polynomial.multiply(b_polynomial).coefficients, vec![BigDecimal::from(2), BigDecimal::from(8), BigDecimal::from(8)]);
    /// ```
    pub fn multiply(&self, other: Polynomial) -> Polynomial {
        let mut new_coefficients: Vec<BigDecimal> =
            vec![BigDecimal::from(0); self.coefficients.len() * other.coefficients.len()];

        for (i, self_coeff) in self.coefficients.iter().enumerate() {
            for (j, other_coeff) in other.coefficients.iter().enumerate() {
                new_coefficients[i + j] += self_coeff * other_coeff;
            }
        }

        Polynomial::new(new_coefficients, 'x')
    }

    /// Return the result of evaluating a Polynomial at value `determinate`
    /// # Example
    /// ```
    /// use polynom::polynomial::Polynomial;
    ///
    /// let polynomial = Polynomial::new(vec![BigDecimal::from(1), BigDecimal::from(2), BigDecimal::from(3)], 'x');
    /// assert_eq!(polynomial.evaluate_at(BigDecimal::from(1)), BigDecimal::from(6))
    /// ```
    pub fn evaluate_at(&self, determinate: BigDecimal) -> BigDecimal {
        let mut sum = BigDecimal::from(0);
        for (degree, coeff) in self.coefficients.iter().enumerate() {
            let str_determinate = determinate.to_string();
            let str_coeff =  coeff.to_string();
            let tmp = U256::from_str_radix(&str_determinate, 10).unwrap();
            let tmp2 = U256::from_str_radix(&str_coeff, 10).unwrap();
            let add = tmp.pow(U256::from(degree)) * tmp2;
            sum += BigDecimal::from_str(add.to_string().as_str()).unwrap();
        }
        sum
    }

    /// Return the polynomial represented as a String
    /// # Example
    /// ```
    /// use polynom::polynomial::Polynomial;
    ///
    /// let polynomial = Polynomial::new(vec![1BigDecimal, 2BigDecimal, 3BigDecimal], 'x');
    /// assert_eq!(polynomial.as_string(), String::from("f(x) = 1 + 2x + 3x^2"))
    /// ```
    pub fn as_string(&self) -> String {
        let mut terms = String::new();
        for (degree, coeff) in self.coefficients.iter().enumerate() {
            if degree == 0 {
                terms = format!("{}", coeff);
                continue;
            }

            if degree == 1 {
                terms = format!("{} + {}{}", terms, coeff, self.indeterminate);
                continue;
            }

            if *coeff == BigDecimal::from(0) {
                continue;
            }

            terms = format!("{} + {}{}^{}", terms, coeff, self.indeterminate, degree);
        }

        format!("f({}) = {}", self.indeterminate, terms)
    }

    /// Return an integer representation of the degree of the Polynomial
    /// # Example
    /// ```
    /// use polynom::polynomial::Polynomial;
    ///
    /// let polynomial = Polynomial::new(vec![1BigDecimal, 2BigDecimal, 3BigDecimal], 'x');
    /// assert_eq!(polynomial.degree(), 2)
    /// ```
    pub fn degree(&self) -> isize {
        // Special case zero polynomial
        if self.coefficients == vec![BigDecimal::from(0)] {
            return -1;
        }

        (self.coefficients.len() - 1) as isize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_from_end() {
        assert_eq!(
            self::Polynomial::strip_from_end(vec![1, 2, 0, 3, 0, 0, 0], 0),
            vec![1, 2, 0, 3]
        );
    }
    #[test]
    fn test_strip_from_end_on_polynomial() {
        let polynomial = Polynomial::new(vec![BigDecimal::from(1), BigDecimal::from(2), BigDecimal::from(0), BigDecimal::from(3), BigDecimal::from(0)], 'x');
        assert_eq!(polynomial.coefficients, vec![BigDecimal::from(1), BigDecimal::from(2), BigDecimal::from(0), BigDecimal::from(3)]);
    }

    #[test]
    fn test_zero_polynomial() {
        let polynomial = Polynomial::new(vec![BigDecimal::from(0)], 'x');
        assert_eq!(polynomial.coefficients, vec![BigDecimal::from(0)]);
    }

    #[test]
    fn test_zero_special_case_degree() {
        let polynomial = Polynomial::new(vec![BigDecimal::from(0)], 'x');
        assert_eq!(polynomial.degree(), -1)
    }

    #[test]
    fn test_degree() {
        let polynomial = Polynomial::new(vec![BigDecimal::from(1), BigDecimal::from(2), BigDecimal::from(0), BigDecimal::from(3)], 'x');
        assert_eq!(polynomial.degree(), 3)
    }

    #[test]
    fn test_string_representation() {
        let polynomial = Polynomial::new(vec![BigDecimal::from(1), BigDecimal::from(2), BigDecimal::from(0), BigDecimal::from(3)], 'x');
        assert_eq!(polynomial.as_string(), String::from("f(x) = 1 + 2x + 3x^3"))
    }

    #[test]
    fn test_add() {
        let a_polynomial = Polynomial::new(vec![BigDecimal::from(1), BigDecimal::from(2), BigDecimal::from(0), BigDecimal::from(3)], 'x');
        let b_polynomial = Polynomial::new(vec![BigDecimal::from(1), BigDecimal::from(2), BigDecimal::from(0), BigDecimal::from(3), BigDecimal::from(4)], 'x');

        assert_eq!(
            a_polynomial.add(b_polynomial).coefficients,
            vec![BigDecimal::from(2), BigDecimal::from(4), BigDecimal::from(0), BigDecimal::from(6), BigDecimal::from(4)]
        )
    }

    #[test]
    fn test_add_negative_coefficients() {
        let a_polynomial = Polynomial::new(vec![BigDecimal::from(1), BigDecimal::from(2), BigDecimal::from(0), BigDecimal::from(3)], 'x');
        let b_polynomial = Polynomial::new(vec![BigDecimal::from(-1), BigDecimal::from(-2), BigDecimal::from(0), BigDecimal::from(-3)], 'x');

        assert_eq!(a_polynomial.add(b_polynomial).coefficients, vec![BigDecimal::from(0)])
    }

    #[test]
    fn test_multiply_simple() {
        let a_polynomial = Polynomial::new(vec![BigDecimal::from(1), BigDecimal::from(2), BigDecimal::from(3)], 'x');
        let b_polynomial = Polynomial::new(vec![BigDecimal::from(1)], 'x');

        assert_eq!(
            a_polynomial.multiply(b_polynomial).coefficients,
            vec![BigDecimal::from(1), BigDecimal::from(2), BigDecimal::from(3)]
        );
    }

    #[test]
    fn test_multiply() {
        let a_polynomial = Polynomial::new(vec![BigDecimal::from(1), BigDecimal::from(2), BigDecimal::from(3)], 'x');
        let b_polynomial = Polynomial::new(vec![BigDecimal::from(3), BigDecimal::from(2), BigDecimal::from(1)], 'x');

        assert_eq!(
            a_polynomial.multiply(b_polynomial).coefficients,
            vec![BigDecimal::from(3), BigDecimal::from(8), BigDecimal::from(14), BigDecimal::from(8), BigDecimal::from(3)]
        )
    }

    #[test]
    fn test_multiply_negative() {
        let a_polynomial = Polynomial::new(vec![BigDecimal::from(1), BigDecimal::from(2), BigDecimal::from(3)], 'x');
        let b_polynomial = Polynomial::new(vec![BigDecimal::from(-3), BigDecimal::from(-2), BigDecimal::from(-1)], 'x');

        assert_eq!(
            a_polynomial.multiply(b_polynomial).coefficients,
            vec![BigDecimal::from(-3), BigDecimal::from(-8), BigDecimal::from(-14), BigDecimal::from(-8), BigDecimal::from(-3)]
        )
    }

    #[test]
    fn test_evaluate_at_zero() {
        let polynomial = Polynomial::new(vec![BigDecimal::from(1), BigDecimal::from(2), BigDecimal::from(3)], 'x');

        assert_eq!(polynomial.evaluate_at(BigDecimal::from(0)), BigDecimal::from(1))
    }

    #[test]
    fn test_evaluate_at_five() {
        let polynomial = Polynomial::new(vec![BigDecimal::from(1), BigDecimal::from(2), BigDecimal::from(3), BigDecimal::from(4)], 'x');

        assert_eq!(polynomial.evaluate_at(BigDecimal::from(5)), BigDecimal::from(586))
    }

    #[test]
    fn test_evaluate_at_negative() {
        let polynomial = Polynomial::new(vec![BigDecimal::from(-1), BigDecimal::from(2), BigDecimal::from(-3), BigDecimal::from(4)], 'x');

        assert_eq!(polynomial.evaluate_at(BigDecimal::from(-5)), BigDecimal::from(-586))
    }

    #[test]
    fn test_subtract() {
        let a_polynomial = Polynomial::new(vec![BigDecimal::from(1), BigDecimal::from(2), BigDecimal::from(3)], 'x');
        let b_polynomial = Polynomial::new(vec![BigDecimal::from(1), BigDecimal::from(2), BigDecimal::from(3)], 'x');

        assert_eq!(a_polynomial.sub(b_polynomial).coefficients, vec![BigDecimal::from(0)])
    }

    #[test]
    fn test_double_negative_subtract() {
        let a_polynomial = Polynomial::new(vec![BigDecimal::from(-1), BigDecimal::from(-2), BigDecimal::from(-3)], 'x');
        let b_polynomial = Polynomial::new(vec![BigDecimal::from(-3), BigDecimal::from(-2), BigDecimal::from(-1)], 'x');

        assert_eq!(
            a_polynomial.sub(b_polynomial).coefficients,
            vec![BigDecimal::from(2), BigDecimal::from(0), BigDecimal::from(-2)]
        )
    }

    #[test]
    fn test_negative_subtract() {
        let a_polynomial = Polynomial::new(vec![BigDecimal::from(-1), BigDecimal::from(-2), BigDecimal::from(-3)], 'x');
        let b_polynomial = Polynomial::new(vec![BigDecimal::from(3), BigDecimal::from(2), BigDecimal::from(1)], 'x');

        assert_eq!(
            a_polynomial.sub(b_polynomial).coefficients,
            vec![BigDecimal::from(-4), BigDecimal::from(-4), BigDecimal::from(-4)]
        )
    }
    #[test]
    fn test_new_polynomial_from_ints() {
        let polynomial = Polynomial::from_ints(vec![BigDecimal::from(1), BigDecimal::from(2), BigDecimal::from(3)], 'x');

        assert_eq!(polynomial.coefficients, vec![BigDecimal::from(1), BigDecimal::from(2), BigDecimal::from(3)]);
    }

}
