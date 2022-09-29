// In this exercise you will create a model of and gives an API to
// deal with blood types

// Start creating the data representation of the blood types
// Create the enumerator `Antigen` that has 4 possibilities: A, B, O and AB
// And the enumerator `RhFactor` that has two possible values: Positive
// and Negative

// After, create the struct BloodType that contains two fields with the
// names antigen and rh_factor

// To provide a simple way to create blood types implement the trait
// FromStr for BloodType (which will allow us to use the `parse`
// method and the associated function from_str, so we can do:
// ```rust
//    let a_neg: BloodType = "A-".parse();
// ```
//)

// Implement the std::cmp::Ord trait to make possible to sort a vector
// or array of BloodType's

// Implement the trait std::Debug for BloodType allowing to print a
// vector such as [BloodType { antigen: A, rh_factor: Positive},
// BloodType{ antigen: B, rh_factor: Negative}] as [ A+, A-] using the
// formatting {:?}

// Write three methods for BloodType:
// - can_receive_from(&self, other: BloodType) -> bool {}: which
// returns true if self can receive blood from `other` blood type
// - donors(&self) -> Vec<BloodType>: which returns
// all the blood types that can give blood to self
// - recipients(&self) -> Vec<BloodType>: which returns all the blood
// types that can receive blood from self

#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

use std::cmp::{Ord, Ordering};

use std::str::FromStr;

impl FromStr for Antigen {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "AB" => Ok(Antigen::AB),
            "B" => Ok(Antigen::B),
            "O" => Ok(Antigen::O),
            other => Err(format!("`{}` is not a valid antigen", other)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

impl FromStr for RhFactor {
    type Err = String;
    fn from_str(rhf: &str) -> Result<Self, Self::Err> {
        match rhf {
            "+" => Ok(RhFactor::Positive),
            "-" => Ok(RhFactor::Negative),
            o => Err(format!("`{}` is not a valid Rh Factor", o)),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.rh_factor == other.rh_factor {
            return self.antigen.cmp(&other.antigen);
        }
        self.antigen.cmp(&other.antigen)
    }
}

impl FromStr for BloodType {
    type Err = String;

    fn from_str(bt: &str) -> Result<Self, Self::Err> {
        if bt.len() > 3 || bt.len() < 2 {
            return Err(format!(
                "Invalid antigen: `{}` invalid length: {}",
                bt,
                bt.len()
            ));
        }

        let rh_fac_str = bt.get(bt.len() - 1..);

        if let None = rh_fac_str {
            return Err(format!("Invalid suffix {:?}", rh_fac_str));
        }

        let rh_factor = rh_fac_str.unwrap().parse()?;
        let antigen = bt.get(..bt.len() - 1).unwrap().parse()?;

        Ok(BloodType { antigen, rh_factor })
    }
}

use std::fmt::{self, Debug};

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.antigen)?;

        if self.rh_factor == RhFactor::Positive {
            return write!(f, "+");
        }

        write!(f, "-")
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        // Positive can only receive from positive
        // A can only give from A
        // And B can only give to B
        if self.rh_factor != other.rh_factor && self.rh_factor == RhFactor::Negative {
            return false;
        }

        if other.antigen == Antigen::O {
            return true;
        }

        // if self.rh_factor contains one of the antigens of other
        // then it can receive from it
        self.antigen == Antigen::AB || other.antigen == self.antigen
    }

    // who are the donors of self
    pub fn donors(&self) -> Vec<Self> {
        // all blood types A, B, AB, O
        let mut blood_types = Vec::new();
        let mut antigens = if self.antigen == Antigen::O {
            vec![Antigen::O]
        } else {
            vec![Antigen::O, self.antigen.clone()]
        };

        let rh_factors = if self.rh_factor == RhFactor::Negative {
            vec![RhFactor::Negative]
        } else {
            vec![RhFactor::Positive, RhFactor::Negative]
        };

        if self.antigen == Antigen::AB {
            antigens.extend(vec![Antigen::A, Antigen::B]);
        }

        for factor in rh_factors.iter() {
            for ant in antigens.iter() {
                blood_types.push(BloodType {
                    rh_factor: (*factor).clone(),
                    antigen: (*ant).clone(),
                })
            }
        }

        blood_types
    }

    // who are the recipients of self
    pub fn recipients(&self) -> Vec<BloodType> {
        let mut blood_types = Vec::new();
        let mut antigens = if self.antigen != Antigen::AB {
            vec![Antigen::AB, self.antigen.clone()]
        } else {
            vec![Antigen::AB]
        };

        let rh_factors = if self.rh_factor == RhFactor::Negative {
            vec![RhFactor::Positive, RhFactor::Negative]
        } else {
            vec![RhFactor::Positive]
        };

        if self.antigen == Antigen::O {
            antigens.extend(vec![Antigen::A, Antigen::B]);
        }

        for factor in rh_factors.iter() {
            for ant in antigens.iter() {
                blood_types.push(BloodType {
                    rh_factor: (*factor).clone(),
                    antigen: (*ant).clone(),
                })
            }
        }

        blood_types
    }
}

// fn main() {
// 	let blood_type: BloodType = "O+".parse().unwrap();
// 	println!("recipients of O+ {:?}", blood_type.recipients());
// 	println!("donors of O+ {:?}", blood_type.donors());
// 	let another_blood_type: BloodType = "A-".parse().unwrap();
// 	println!(
// 		"donors of O+ can receive from {:?} {:?}",
// 		&another_blood_type,
// 		blood_type.can_receive_from(&another_blood_type)
// 	);
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compatible_ab_neg_with_a_pos() {
        let blood_type: BloodType = "AB-".parse().unwrap();
        let other_bt: BloodType = "A+".parse().unwrap();
        assert!(!blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn compatible_a_neg_with_a_pos() {
        let blood_type: BloodType = "A-".parse().unwrap();
        let other_bt: BloodType = "A+".parse().unwrap();
        assert!(!blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn compatible_a_neg_with_ab_neg() {
        let blood_type: BloodType = "AB-".parse().unwrap();
        let other_bt: BloodType = "A-".parse().unwrap();
        assert!(blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn compatible_ab_neg_with_o_pos() {
        let blood_type: BloodType = "AB-".parse().unwrap();
        let other_bt: BloodType = "O+".parse().unwrap();
        assert!(!blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn compatible_ab_pos_with_o_pos() {
        let blood_type: BloodType = "AB+".parse().unwrap();
        let other_bt: BloodType = "O+".parse().unwrap();
        assert!(blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn test_compatible_ab_neg_with_o_neg() {
        let blood_type: BloodType = "AB-".parse().unwrap();
        let other_bt: BloodType = "O-".parse().unwrap();
        assert!(blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn test_antigen_ab_from_str() {
        let blood = "AB+";
        let blood_type: BloodType = blood.parse().unwrap();
        assert_eq!(blood_type.antigen, Antigen::AB);
        assert_eq!(blood_type.rh_factor, RhFactor::Positive);
    }

    #[test]
    fn test_antigen_a_from_str() {
        let blood = "A-";
        let blood_type = blood.parse::<BloodType>().unwrap();
        assert_eq!(blood_type.antigen, Antigen::A);
        assert_eq!(blood_type.rh_factor, RhFactor::Negative);
    }

    #[test]
    #[should_panic]
    fn test_unexistent_blood_type() {
        let _blood_type: BloodType = "AO-".parse().unwrap();
    }

    #[test]
    fn test_donors() {
        let mut givers = "AB+".parse::<BloodType>().unwrap().donors();
        println!("Before sorting {:?}", &givers);
        givers.sort();
        println!("{:?}", &givers);
        let mut expected = vec![
            "AB-".parse::<BloodType>().unwrap(),
            "A-".parse().unwrap(),
            "B-".parse().unwrap(),
            "O-".parse().unwrap(),
            "AB+".parse().unwrap(),
            "A+".parse().unwrap(),
            "B+".parse().unwrap(),
            "O+".parse().unwrap(),
        ];
        expected.sort();
        assert_eq!(givers, expected);
    }

    #[test]
    fn test_a_neg_donors() {
        let mut givers = "A-".parse::<BloodType>().unwrap().donors();
        givers.sort();
        let mut expected: Vec<BloodType> = vec!["A-".parse().unwrap(), "O-".parse().unwrap()];
        expected.sort();
        assert_eq!(givers, expected);
    }

    #[test]
    fn test_o_neg_donors() {
        let mut givers = "O-".parse::<BloodType>().unwrap().donors();
        givers.sort();
        let mut expected: Vec<BloodType> = vec!["O-".parse().unwrap()];
        expected.sort();
        assert_eq!(givers, expected);
    }

    #[test]
    fn test_ab_pos_recipients() {
        let mut recipients: Vec<BloodType> = "AB+".parse::<BloodType>().unwrap().recipients();
        recipients.sort();
        let mut expected: Vec<BloodType> = vec!["AB+".parse().unwrap()];
        expected.sort();
        assert_eq!(recipients, expected);
    }

    #[test]
    fn test_a_neg_recipients() {
        let mut recipients = "A-".parse::<BloodType>().unwrap().recipients();
        recipients.sort();
        let mut expected: Vec<BloodType> = vec![
            "A-".parse().unwrap(),
            "AB+".parse().unwrap(),
            "A+".parse().unwrap(),
            "AB-".parse().unwrap(),
        ];
        expected.sort();
        assert_eq!(recipients, expected);
    }

    #[test]
    fn test_output() {
        let blood_type: BloodType = "O+".parse().unwrap();
        println!("recipients of O+ {:?}", blood_type.recipients());
        println!("donors of O+ {:?}", blood_type.donors());
        let another_blood_type: BloodType = "A-".parse().unwrap();
        println!(
            "donors of O+ can receive from {:?} {:?}",
            &another_blood_type,
            blood_type.can_receive_from(&another_blood_type)
        );
    }
}
