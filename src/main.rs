#![recursion_limit = "256"]

pub mod grammar;
pub mod lexicon;
pub mod syntax;

use rand::prelude::*;
//use yew::services::ConsoleService;
use syntax::NounGroup;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(ch) => ch.to_uppercase().chain(chars).collect(),
    }
}

const PROBABILITY_WEIRD: f64 = 0.2;
const PROBABILITY_FEEL: f64 = 0.1;
const PROBABILITY_SIZE: f64 = 0.3;
const PROBABILITY_SHAPE: f64 = 0.3;
const PROBABILITY_COLOR: f64 = 0.2;
const PROBABILITY_MATERIAL: f64 = 0.7;
const PROBABILITY_WITH: f64 = 0.1;

pub enum Msg {
    Oddity,
}

pub struct Model {
    link: ComponentLink<Self>,
    value: String,
}

fn gen_oddity(rng: &mut ThreadRng) -> NounGroup {
    let object = *lexicon::OBJECTS.choose(rng).unwrap();
    let mut oddity = NounGroup {
        noun: object,
        modifiers: Vec::new(),
    };
    if rng.gen_bool(PROBABILITY_WEIRD) {
        oddity.modifiers.push(*lexicon::WEIRD.choose(rng).unwrap());
    }
    if rng.gen_bool(PROBABILITY_SIZE) {
        oddity.modifiers.push(*lexicon::SIZE.choose(rng).unwrap());
    }
    if rng.gen_bool(PROBABILITY_SHAPE) {
        oddity.modifiers.push(*lexicon::SHAPE.choose(rng).unwrap());
    }
    if rng.gen_bool(PROBABILITY_FEEL) {
        oddity.modifiers.push(*lexicon::FEEL.choose(rng).unwrap());
    }
    if rng.gen_bool(PROBABILITY_COLOR) {
        oddity.modifiers.push(*lexicon::COLOR.choose(rng).unwrap());
    }
    if rng.gen_bool(PROBABILITY_MATERIAL) {
        oddity
            .modifiers
            .push(*lexicon::MATERIAL.choose(rng).unwrap());
    }
    oddity
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Oddity => {
                let mut rng = rand::thread_rng();
                let syntax = gen_oddity(&mut rng);
                let syntax = if rng.gen_bool(PROBABILITY_WITH) {
                    syntax::SyntaxNode::With(syntax, gen_oddity(&mut rng))
                } else {
                    syntax::SyntaxNode::Noun(syntax)
                };
                self.value = syntax.to_string();
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
