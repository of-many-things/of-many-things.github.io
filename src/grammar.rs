#[derive(Copy, Clone, Debug)]
pub enum Case {
    Nominative,
    Genitive,
    Dative,
    Accusative,
    Instrumental,
    Locative,
}

#[derive(Copy, Clone, Debug)]
pub enum Gender {
    Masculine,
    Feminine,
    Neuter,
}

#[derive(Copy, Clone, Debug)]
pub enum Animacy {
    Animate,
    Inanimate,
}

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone)]
pub enum NounDeclension {
    /// Несклоняемые слова
    T0,
    /// Слова с основой на твёрдый согласный (топор, комод, балда, кобра, олово, пекло; твёрдый, тусклый).
    T1, 
    /// Слова с основой на мягкий согласный (тюлень, искатель, цапля, Дуня, горе, поле; весенний).
    T2, 
    /// Слова с основой на г, к или х (сапог, коряга, парк, моллюск, золотко, петух, неряха; мягкий).
    T3, 
    /// Слова с основой на ж, ш, ч, щ (калач, лаваш, галоша, святоша, жилище, вече; вящий).
    T4, 
    /// Слова с основой на ц (немец, конец, девица, деревце; куцый).
    T5, 
    /// Слова с основой на гласный (кроме и) или й/j (бой, край; шея, здоровье).
    T6(char),
    /// Слова с основой на и (полоний, сложение, мания, удостоверение).
    T7, 
    /// Слова с традиционным «3-м склонением» (боль, тетрадь, зыбь; имя; путь).
    T8, 
}

#[derive(Copy, Clone)]
pub enum NounDeclensionException {
    Regular,
    FluentVowel(char),
    Plural(&'static str),
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
    use Number::*;

    match number {
        Plural => match declension {
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
        Singular => {
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

fn nom_singular(noun: &Noun) -> &'static str {
    use Gender::*;
    use NounDeclension::*;

    match (noun.gender, noun.declension) {
        (Masculine, T1) | (Masculine, T3) | (Masculine, T4) | (Masculine, T5) => "",
        (Masculine, _) => "ь",
        (Feminine, T1) | (Feminine, T2) | (Feminine, T3) | (Feminine, T4) => "а",
        (Feminine, _)  => "я",
        (Neuter, T1) => "о",
        (Neuter, _) => "е",
    }
}

fn nom_plural(noun: &Noun) -> &'static str {
    use Gender::*;
    use NounDeclension::*;
    use NounDeclensionException::*;

    match (noun.gender, noun.declension, noun.declension_exception) {
        (_, _, Plural(plural)) => plural,
        (Feminine, T1, _) | (Feminine, T4, _) | (Masculine, T1, _) | (Masculine, T4, _) => "ы",
        (Masculine, _, _) | (Feminine, _, _)  => "и",
        (Neuter, T1, _) | (Neuter, T4, _) => "а",
        (Neuter, _, _) => "я",
    }
}

fn gen_singular(noun: &Noun) -> &'static str {
    use Gender::*;
    use NounDeclension::*;

    match (noun.gender, noun.declension) {
        (Feminine, _) => "е",
        (_, T1) | (_, T3) | (_, T5) => "а",
        _ => "я",
    }
}

fn gen_plural(noun: &Noun) -> &'static str {
    use Gender::*;
    use NounDeclension::*;

    match (noun.gender, noun.declension) {
        (Feminine, _) | (Neuter, _) => "",
        (Masculine, T1) | (Masculine, T3) => "ов",
        (Masculine, T5) => "ев",
        _ => "ей",
    }
}

fn noun_stem(noun: &Noun) -> &'static str {
    let last = noun.base.chars().last().unwrap();
    match last {
        'ь' | 'а' | 'я' | 'о' | 'е' => &noun.base[.. noun.base.len() - last.len_utf8()],
        _ => noun.base,
    }
}

pub fn decline_noun(noun: &Noun, case: Case, number: Number) -> String {
    use Gender::*;
    use NounDeclension::*;
    use Case::*;
    use Number::*;
    use Animacy::*;
    let ending = match noun.declension {
        T0 => return noun.base.to_string(),
        T1 | T3 => match (case, number) {
            (Nominative, Singular) => nom_singular(noun),
            (Nominative, Plural) => nom_plural(noun),
            (Genitive, Singular) => gen_singular(noun),
            (Genitive, Plural) => gen_plural(noun),
            (Dative, Singular) => match noun.gender {
                Feminine => "е",
                _ => "у",
            }
            (Dative, Plural) => "ам",
            (Accusative, Singular) => match (noun.gender, noun.animacy) {
                (Feminine, _) => "у",
                (_, Animate) => gen_singular(noun),
                (_, Inanimate) => nom_singular(noun),
            }
            (Accusative, Plural) => match noun.animacy {
                Animate => gen_plural(noun),
                Inanimate => nom_plural(noun),
            }
            (Instrumental, Singular) => match noun.gender {
                Feminine => "ой",
                _ => "ом",
            }
            (Instrumental, Plural) => "ами",
            (Locative, Singular) => "е",
            (Locative, Plural) => "ах",
        }
        T2 => match (case, number) {
            (Nominative, Singular) => nom_singular(noun),
            (Nominative, Plural) => nom_plural(noun),
            (Genitive, Singular) => gen_singular(noun),
            (Genitive, Plural) => gen_plural(noun),
            (Dative, Singular) => match noun.gender {
                Feminine => "е",
                _ => "ю",
            }
            (Dative, Plural) => "ям",
            (Accusative, Singular) => match (noun.gender, noun.animacy) {
                (Feminine, _) => "ю",
                (_, Animate) => gen_singular(noun),
                (_, Inanimate) => nom_singular(noun),
            }
            (Accusative, Plural) => match noun.animacy {
                Animate => gen_plural(noun),
                Inanimate => nom_plural(noun),
            }
            (Instrumental, Singular) => match noun.gender {
                Feminine => "ей",
                _ => "ем",
            }
            (Instrumental, Plural) => "ями",
            (Locative, Singular) => "е",
            (Locative, Plural) => "ях",
        }
        _ => todo!(),
    };
    let mut word = noun_stem(noun).to_string();
    if let NounDeclensionException::FluentVowel(vowel) = noun.declension_exception {
        let second_last = word.chars().rev().nth(1);
        let null_ending = ending == "" || ending == "ь"; 
        if null_ending && second_last != Some(vowel) {
            let last = word.pop().unwrap();
            word.push(vowel);
            word.push(last);
        } else if !null_ending && second_last == Some(vowel) {
            let last = word.pop().unwrap();
            word.pop().unwrap();
            word.push(last);
        }
    }
    word.push_str(ending);
    word 
}

#[derive(Copy, Clone)]
pub struct Noun {
    pub base: &'static str,
    pub gender: Gender,
    pub animacy: Animacy,
    pub declension: NounDeclension,
    pub declension_exception: NounDeclensionException,
}

impl Noun {
    pub const fn new(
        base: &'static str, 
        gender: Gender, 
        animacy: Animacy,
        declension: NounDeclension, 
        declension_exception: NounDeclensionException, 
    ) -> Self {
        Self { base, declension, declension_exception, gender, animacy }
    }
}

#[derive(Copy, Clone)]
pub struct Adjective(pub &'static str);
