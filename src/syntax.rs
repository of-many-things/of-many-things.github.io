use crate::grammar::{Adjective, Case, Noun, Number};

pub struct NounGroup {
    pub noun: Noun,
    pub case: Case,
    pub number: Number,
    pub modifiers: Vec<Adjective>,
}
