#![recursion_limit="256"]

pub mod grammar;
pub mod lexicon;
pub mod syntax;

use std::ops::Deref;

use grammar::{Case, Number};
use rand::prelude::*;
//use yew::services::ConsoleService;
use syntax::NounGroup;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

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

fn build_string(group: &NounGroup) -> String {
    let gender = group.noun.gender;
    let animacy = group.noun.animacy;
    let mut parts = Vec::new();
    for modifier in group.modifiers.iter() {
        parts.push(grammar::decline_adjective(modifier, gender, animacy, group.case, group.number));
    }
    parts.push(group.noun.text.to_string());
    build_string_from_parts(parts.iter().map(String::deref))
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(ch) => {
            ch.to_uppercase().chain(chars).collect()
        }
    }
}

const PROBABILITY_WEIRD: f64 = 0.2;
const PROBABILITY_FEEL: f64 = 0.1;
const PROBABILITY_SIZE: f64 = 0.3;
const PROBABILITY_SHAPE: f64 = 0.3;
const PROBABILITY_COLOR: f64 = 0.2;
const PROBABILITY_MATERIAL: f64 = 0.7;

pub enum Msg {
    Oddity,
}

pub struct Model {
    link: ComponentLink<Self>,
    value: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, value: String::new() }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Oddity => {
                let mut rng = rand::thread_rng();
                let object = *lexicon::OBJECTS.choose(&mut rng).unwrap();
                let mut oddity = NounGroup {
                    noun: object,
                    case: Case::Nominative,
                    number: Number::Singular,
                    modifiers: Vec::new(),
                };
                if rng.gen_bool(PROBABILITY_WEIRD) {
                    oddity.modifiers.push(*lexicon::WEIRD.choose(&mut rng).unwrap());
                }
                if rng.gen_bool(PROBABILITY_SIZE) {
                    oddity.modifiers.push(*lexicon::SIZE.choose(&mut rng).unwrap());
                }
                if rng.gen_bool(PROBABILITY_SHAPE) {
                    oddity.modifiers.push(*lexicon::SHAPE.choose(&mut rng).unwrap());
                }
                if rng.gen_bool(PROBABILITY_FEEL) {
                    oddity.modifiers.push(*lexicon::FEEL.choose(&mut rng).unwrap());
                }
                if rng.gen_bool(PROBABILITY_COLOR) {
                    oddity.modifiers.push(*lexicon::COLOR.choose(&mut rng).unwrap());
                }
                if rng.gen_bool(PROBABILITY_MATERIAL) {
                    oddity.modifiers.push(*lexicon::MATERIAL.choose(&mut rng).unwrap());
                }
                self.value = build_string(&oddity);
                self.value = capitalize(&self.value);
                self.value.push('.');
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <nav class="menu">
                    <button onclick=self.link.callback(|_| Msg::Oddity)>
                        { "Диковина" }
                    </button>
                </nav>
                <p>
                    { self.value.clone() }
                </p>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
