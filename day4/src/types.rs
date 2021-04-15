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

impl FromStr for Field {
    type Err = String;

    /// Example: "ecl:gry" -> Field::EyeColor("gry")
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Field::*;

        match s.split(":").collect::<Vec<&str>>()[..] {
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
        match field {
            Field::CountryId(_) => false,
            _ => true,
        }
    }

    pub fn is_valid(&self) -> bool {
        // All but optinal fields are present -- valid passport
        self.0
            .iter()
            .filter(|&field| Passport::is_field_required(field))
            .count()
            == Passport::REQUIRED_FIELDS_NUM
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
    fn test_is_passport_valid() {
        let cases = [
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
    }
}
