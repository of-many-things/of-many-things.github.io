#[derive(Copy, Clone)]
pub enum Case {
    Nominative,
    Genitive,
    Dative,
    Accusative,
    Instrumental,
    Locative,
}

#[derive(Copy, Clone)]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
}

#[derive(Copy, Clone)]
pub enum Animacy {
    Animate,
    Inanimate,
}

#[derive(Copy, Clone)]
pub enum Number {
    Singular,
    Plural,
}

#[derive(Copy, Clone)]
pub enum HardAdjDeclension {
    Y,
    O,
}

#[derive(Copy, Clone)]
pub enum MixedAdjDeclension {
    I,
    O,
}

#[derive(Copy, Clone)]
pub enum AdjDeclension {
    Hard(HardAdjDeclension),
    Mixed(MixedAdjDeclension),
    Soft,
}

fn adjective_declension(adjective: &Adjective) -> AdjDeclension {
    const MIXED_STEM_ENDS: &[char] = &['г', 'к', 'х', 'ш', 'ч', 'щ', 'ц'];

    let mut chars = adjective.0.chars().rev();
    let ending_start = chars.nth(1).unwrap();
    if ending_start == 'ы' {
        return AdjDeclension::Hard(HardAdjDeclension::Y);
    }
    let is_mixed = MIXED_STEM_ENDS.contains(&chars.next().unwrap());
    if ending_start == 'о' {
        if is_mixed { 
            AdjDeclension::Mixed(MixedAdjDeclension::O)
        } else { 
            AdjDeclension::Hard(HardAdjDeclension::O)
        }
    } else if is_mixed {
        AdjDeclension::Mixed(MixedAdjDeclension::I)
    } else {
        AdjDeclension::Soft
    }
}

pub fn adjective_ending(declension: AdjDeclension, gender: Gender, animacy: Animacy, case: Case, number: Number) -> &'static str {
    use AdjDeclension::*;
    use Animacy::*;
    use Case::*;
    use Gender::*;

    match number {
        Number::Plural => match declension {
            Hard(_) => match case {
                Nominative => "ые",
                Genitive => "ых",
                Dative => "ым",
                Accusative => match animacy {
                    Animate => "ых",
                    Inanimate => "ые",
                }
                Instrumental => "ыми",
                Locative => "ых",
            } 
            Mixed(_) | Soft => match case {
                Nominative => "ие",
                Genitive => "их",
                Dative => "им",
                Accusative => match animacy {
                    Animate => "их",
                    Inanimate => "ие",
                }
                Instrumental => "ими",
                Locative => "их",
            }
        }
        Number::Singular => {
            let (nominative, instrumental) = match declension {
                Hard(HardAdjDeclension::Y) => ("ый", "ым"),
                Hard(HardAdjDeclension::O) => ("ой", "ым"),
                Mixed(MixedAdjDeclension::O) => ("ой", "им"),
                Mixed(MixedAdjDeclension::I) | Soft => ("ий", "им"),
            };
            match gender {
                Masculine => match declension {
                    Hard(_) | Mixed(_) => match case {
                        Nominative => nominative,
                        Genitive => "ого",
                        Dative => "ому",
                        Accusative => match animacy {
                            Animate => "ого",
                            Inanimate => nominative,
                        }
                        Instrumental => instrumental,
                        Locative => "ом",
                    }
                    Soft => match case {
                        Nominative => nominative,
                        Genitive => "его",
                        Dative => "ему",
                        Accusative => match animacy {
                            Animate => "его",
                            Inanimate => nominative,
                        }
                        Instrumental => instrumental,
                        Locative => "им",
                    }
                }
                Feminine => match declension {
                    Hard(_) | Mixed(_) => match case {
                        Nominative => "ая",
                        Accusative => "ую",
                        _ => "ой",
                    }
                    Soft => match case {
                        Nominative => "яя",
                        Accusative => "юю",
                        _ => "ей",
                    }
                }
                Neuter => match declension {
                    Hard(_) | Mixed(_) => match case {
                        Nominative => "ое",
                        Genitive => "ого",
                        Dative => "ому",
                        Accusative => "ое",
                        Instrumental => instrumental,
                        Locative => "ом",
                    }
                    Soft => match case {
                        Nominative => "ее",
                        Genitive => "его",
                        Dative => "ему",
                        Accusative => "ее",
                        Instrumental => instrumental,
                        Locative => "ем",
                    }
                }
            }
        }
    }
}

pub fn decline_adjective(adjective: &Adjective, gender: Gender, animacy: Animacy, case: Case, number: Number) -> String {
    let len = adjective.0.chars().count();
    let mut string: String = adjective.0.chars().take(len - 2).collect();
    string.push_str(adjective_ending(adjective_declension(adjective), gender, animacy, case, number));
    string
}

#[derive(Copy, Clone)]
pub struct Noun {
    pub text: &'static str,
    pub gender: Gender,
    pub animacy: Animacy,
}

impl Noun {
    pub const fn new(text: &'static str, gender: Gender, animacy: Animacy) -> Self {
        Self { text, gender, animacy }
    }
}

#[derive(Copy, Clone)]
pub struct Adjective(pub &'static str);