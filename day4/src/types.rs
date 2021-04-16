use std::{collections::HashSet, str::FromStr};
use strum::EnumCount;

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumCount)]
pub(crate) enum Field {
    BirthYear(String),
    IssueYear(String),
    ExpirationYear(String),
    Height(String),
    HairColor(String),
    EyeColor(String),
    PassportId(String),
    CountryId(String),
}

impl Field {
    fn is_value_in_valid_range(&self) -> bool {
        use Field::*;

        // Suppose we deal with only ASCII strings for easier logic...
        match self {
            BirthYear(value) => {
                value.len() == 4 && {
                    let year = value.parse::<u32>().unwrap_or_default();
                    (1920..=2002).contains(&year)
                }
            }
            IssueYear(value) => {
                value.len() == 4 && {
                    let year = value.parse::<u32>().unwrap_or_default();
                    (2010..=2020).contains(&year)
                }
            }
            ExpirationYear(value) => {
                value.len() == 4 && {
                    let year = value.parse::<u32>().unwrap_or_default();
                    (2020..=2030).contains(&year)
                }
            }
            Height(value) => {
                // Guard against incorrect values
                value.len() > 2 && {
                    let height: u32 = value[..value.len() - 2].parse().unwrap_or_default();

                    match &value[value.len() - 2..] {
                        "cm" => (150..=193).contains(&height),
                        "in" => (59..=76).contains(&height),
                        _ => false,
                    }
                }
            }
            HairColor(value) => {
                value.len() == 7
                    && matches!(value.chars().next(), Some('#'))
                    && value
                        .chars()
                        .skip(1)
                        .all(|chr| matches!(chr, 'a'..='f' | '0'..='9'))
            }
            EyeColor(value) => {
                matches!(
                    value.as_str(),
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
                )
            }
            PassportId(value) => value.len() == 9 && value.parse::<u32>().is_ok(),
            CountryId(_) => true,
        }
    }
}

impl FromStr for Field {
    type Err = String;

    /// Example: "ecl:gry" -> Field::EyeColor("gry")
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Field::*;

        match s.split(':').collect::<Vec<&str>>()[..] {
            [a, b] => match a {
                "byr" => Ok(BirthYear(b.into())),
                "iyr" => Ok(IssueYear(b.into())),
                "eyr" => Ok(ExpirationYear(b.into())),
                "hgt" => Ok(Height(b.into())),
                "hcl" => Ok(HairColor(b.into())),
                "ecl" => Ok(EyeColor(b.into())),
                "pid" => Ok(PassportId(b.into())),
                "cid" => Ok(CountryId(b.into())),
                case => Err(format!("Field key is not recognized: {}", case)),
            },
            _ => Err(format!("Can not parse field entry {}", s)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Passport(HashSet<Field>);

impl Passport {
    const REQUIRED_FIELDS_NUM: usize = Field::COUNT - 1;
    fn is_field_required(field: &Field) -> bool {
        !matches!(field, Field::CountryId(_))
    }

    pub fn is_valid_1(&self) -> bool {
        // All but optinal fields are present -- valid passport
        self.0
            .iter()
            .filter(|&field| Passport::is_field_required(field))
            .count()
            == Passport::REQUIRED_FIELDS_NUM
    }

    pub fn is_valid_2(&self) -> bool {
        // All but optinal fields are present
        self.is_valid_1()
            // All field values are in their ranges
            && self
                .0
                .iter()
                .all(|field| field.is_value_in_valid_range())
    }
}

impl FromStr for Passport {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
            .split_whitespace()
            .map(|field_entry| field_entry.parse())
            .collect::<Result<Vec<Field>, String>>()
        {
            Ok(fields) => Ok(Passport {
                0: fields.into_iter().collect(),
            }),
            Err(err) => Err(err),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use pretty_assertions::assert_eq;
    use Field::*;

    #[test]
    fn test_field_from_str() {
        let cases = &[
            ("byr:123", Ok(BirthYear("123".into()))),
            ("iyr:123", Ok(IssueYear("123".into()))),
            ("eyr:123", Ok(ExpirationYear("123".into()))),
            ("hgt:123", Ok(Height("123".into()))),
            ("hcl:123", Ok(HairColor("123".into()))),
            ("ecl:123", Ok(EyeColor("123".into()))),
            ("pid:123", Ok(PassportId("123".into()))),
            ("cid:123", Ok(CountryId("123".into()))),
            ("cfg:hhfhff", Err("Field key is not recognized: cfg".into())),
            ("haha", Err("Can not parse field entry haha".into())),
            (
                "hoehe:hehe:123",
                Err("Can not parse field entry hoehe:hehe:123".into()),
            ),
        ];

        for case in cases {
            assert_eq!(case.0.parse(), case.1);
        }
    }

    #[test]
    fn test_field_value_in_range() {
        let cases = &[
            (BirthYear("1937".into()), true),
            (BirthYear("1910".into()), false),
            (BirthYear("3000".into()), false),
            (BirthYear("12".into()), false),
            (Height("152cm".into()), true),
            (Height("1524cm".into()), false),
            (Height("12cm".into()), false),
            (Height("1cm".into()), false),
            (Height("62in".into()), true),
            (Height("12in".into()), false),
            (Height("1in".into()), false),
            (HairColor("#fac9b0".into()), true),
            (HairColor("#fac9b04".into()), false),
            (HairColor("#fac9b".into()), false),
            (HairColor("#lollol".into()), false),
            (HairColor("fac9b0".into()), false),
            (HairColor("fac9b04".into()), false),
            (EyeColor("amb".into()), true),
            (EyeColor("hello".into()), false),
            (PassportId("012345678".into()), true),
            (PassportId("0000000010".into()), false),
            (PassportId("00a000001".into()), false),
            (CountryId("129".into()), true),
            (CountryId("russia".into()), true),
        ];

        for (index, case) in cases.iter().enumerate() {
            assert_eq!(
                case.0.is_value_in_valid_range(),
                case.1,
                "failed field_value_in_range with case {:?}\n----- at index {}",
                case,
                index
            );
        }
    }

    #[test]
    fn test_passport_from_str() {
        let cases = &[
            (
                "byr:123 ecl:gry",
                Ok(Passport {
                    0: vec![BirthYear("123".into()), EyeColor("gry".into())]
                        .into_iter()
                        .collect(),
                }),
            ),
            (
                "byr:123 ecl:gry",
                Ok(Passport {
                    0: vec![BirthYear("123".into()), EyeColor("gry".into())]
                        .into_iter()
                        .collect(),
                }),
            ),
            (
                "byr:123 ecl:gry",
                Ok(Passport {
                    0: vec![BirthYear("123".into()), EyeColor("gry".into())]
                        .into_iter()
                        .collect(),
                }),
            ),
            (
                "byr:123",
                Ok(Passport {
                    0: vec![BirthYear("123".into())].into_iter().collect(),
                }),
            ),
            (
                indoc! {"
                    ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
                    byr:1937 iyr:2017 cid:147 hgt:183cm
                "},
                Ok(Passport {
                    0: vec![
                        EyeColor("gry".into()),
                        PassportId("860033327".into()),
                        ExpirationYear("2020".into()),
                        HairColor("#fffffd".into()),
                        BirthYear("1937".into()),
                        IssueYear("2017".into()),
                        CountryId("147".into()),
                        Height("183cm".into()),
                    ]
                    .into_iter()
                    .collect(),
                }),
            ),
            (
                indoc! {"
                    hcl:#ae17e1 iyr:2013
                    eyr:2024
                    ecl:brn pid:760753108 byr:1931
                    hgt:179cm
                "},
                Ok(Passport {
                    0: vec![
                        EyeColor("brn".into()),
                        PassportId("760753108".into()),
                        ExpirationYear("2024".into()),
                        HairColor("#ae17e1".into()),
                        BirthYear("1931".into()),
                        IssueYear("2013".into()),
                        Height("179cm".into()),
                    ]
                    .into_iter()
                    .collect(),
                }),
            ),
            (
                indoc! {"
                    hcl:#ae17e1iyr:2013
                    ecl:brn pid:760753108 byr:1931
                "},
                Err("Can not parse field entry hcl:#ae17e1iyr:2013".into()),
            ),
        ];

        for case in cases {
            assert_eq!(case.0.parse(), case.1);
        }
    }

    #[test]
    fn test_is_passport_valid_1() {
        let cases = &[
            (
                Passport {
                    0: vec![
                        EyeColor("brn".into()),
                        PassportId("760753108".into()),
                        ExpirationYear("2024".into()),
                        HairColor("#ae17e1".into()),
                        BirthYear("1931".into()),
                        IssueYear("2013".into()),
                        Height("179cm".into()),
                    ]
                    .into_iter()
                    .collect(),
                },
                true,
            ),
            (
                Passport {
                    0: vec![
                        EyeColor("brn".into()),
                        PassportId("760753108".into()),
                        ExpirationYear("2024".into()),
                        HairColor("#ae17e1".into()),
                        BirthYear("1931".into()),
                        IssueYear("2013".into()),
                        CountryId("147".into()),
                        Height("179cm".into()),
                    ]
                    .into_iter()
                    .collect(),
                },
                true,
            ),
            (
                Passport {
                    0: vec![BirthYear("123".into()), EyeColor("gry".into())]
                        .into_iter()
                        .collect(),
                },
                false,
            ),
        ];

        for (index, case) in cases.iter().enumerate() {
            assert_eq!(
                case.0.is_valid_1(),
                case.1,
                "failed is_valid_1 with case {:?}\n------ at index {}",
                case,
                index
            )
        }
    }
}
