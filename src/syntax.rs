use std::ops::Deref;

use crate::grammar::{self, Adjective, Case, Noun, Number};

pub struct NounGroup {
    pub noun: Noun,
    pub modifiers: Vec<Adjective>,
}

impl NounGroup {
    pub fn to_string(&self, case: Case, number: Number) -> String {
        let mut parts = Vec::new();
        for modifier in self.modifiers.iter() {
            parts.push(grammar::decline_adjective(
                modifier,
                self.noun.gender,
                self.noun.animacy,
                case,
                number,
            ));
        }
        parts.push(grammar::decline_noun(&self.noun, case, number));
        build_string_from_parts(parts.iter().map(String::deref))
    }
}

pub enum SyntaxNode {
    Noun(NounGroup),
    With(NounGroup, NounGroup),
}

impl SyntaxNode {
    pub fn to_string(&self) -> String {
        match self {
            Self::Noun(group) => group.to_string(Case::Nominative, Number::Singular),
            Self::With(noun, with_noun) => {
                let mut string = noun.to_string(Case::Nominative, Number::Singular);
                let with_string = with_noun.to_string(Case::Instrumental, Number::Singular);
                string.push_str(" с "); // TODO со
                string.push_str(&with_string);
                string
            }
        }
    }
}

fn build_string_from_parts<'a, I: Iterator<Item = &'a str>>(parts: I) -> String {
    let mut string = String::new();
    for part in parts {
        if string.len() > 0 && !part.starts_with(",") {
            string.push(' ');
        }
        string.push_str(part);
    }
    string
}
