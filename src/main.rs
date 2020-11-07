#![recursion_limit="256"]

use rand::prelude::*;
//use yew::services::ConsoleService;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

#[derive(Default)]
pub struct Oddity {
    pub weird: Option<String>,
    pub size: Option<String>,
    pub feel: Option<String>,
    pub shape_prefix: Option<String>,
    pub color: Option<String>,
    pub material_prefix: Option<String>,
    pub object: String,
    pub material_postfix: Option<String>,
    pub shape_postfix: Option<String>,
    pub with: Option<String>,
    pub that: Option<String>,
}

fn build_string_from_parts(parts: &[Option<&str>]) -> String {
    let mut string = String::new();
    for part in parts {
        if let Some(part) = part {
            if !part.starts_with(",") {
                string.push(' ');
            }
            string.push_str(part);
        }
    }
    string
}

fn build_oddity_string(oddity: &Oddity) -> String {
    build_string_from_parts(&[
        oddity.weird.as_deref(),
        oddity.size.as_deref(),
        oddity.feel.as_deref(),
        oddity.shape_prefix.as_deref(),
        oddity.color.as_deref(),
        oddity.material_prefix.as_deref(),
        Some(&oddity.object),
        oddity.material_postfix.as_deref(),
        oddity.shape_postfix.as_deref(),
        oddity.with.as_deref(),
        oddity.that.as_deref(),
    ])
}

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

const WEIRD: &[&str] = &[
    "непонятный",
    "странный",
    "загадочный",
    "необычный",
    "непривычный",
    "удивительный",
    "подозрительный",
    "неестественный",
    "таинственный",
    "непостижимый",
    "любопытный",
];

const OBJECTS: &[Noun] = &[
    Noun::new("птица", Gender::Feminine, Animacy::Animate),
    Noun::new("лягушка", Gender::Feminine, Animacy::Animate),
    Noun::new("осьминог", Gender::Masculine, Animacy::Inanimate),
    Noun::new("череп", Gender::Feminine, Animacy::Inanimate),
    Noun::new("рог", Gender::Masculine, Animacy::Inanimate),
    Noun::new("игрушка", Gender::Feminine, Animacy::Inanimate),
    Noun::new("орнитоптер", Gender::Masculine, Animacy::Inanimate),
    Noun::new("карточка", Gender::Feminine, Animacy::Inanimate),
    Noun::new("подушка", Gender::Feminine, Animacy::Inanimate),
    Noun::new("одеяло", Gender::Neuter, Animacy::Inanimate),
    Noun::new("седло", Gender::Neuter, Animacy::Inanimate),
    Noun::new("стул", Gender::Masculine, Animacy::Inanimate),
    Noun::new("прут", Gender::Masculine, Animacy::Inanimate),
];

pub struct Context {
    pub gender: Gender,
    pub rng: ThreadRng,
}

#[derive(Debug)]
pub enum InterpreterError {
    UnknownFunction,
}

fn interpret(call: &str, context: &mut Context) -> Result<String, InterpreterError> {
    let func_and_args: Vec<_> = call.split(":").collect();
    match func_and_args[0] {
        "adj" => {
            Ok(WEIRD.choose(&mut context.rng).unwrap().to_string())
        }
        _ => Err(InterpreterError::UnknownFunction)
    }
}

fn run(text: &str, context: &mut Context) -> String {
    let regex = regex::Regex::new("\\[([^\\]]+)\\]").unwrap();
    regex.replace_all(text, |captures: &regex::Captures| interpret(captures.get(1).unwrap().as_str(), context).unwrap()).into_owned()
}

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
                let mut oddity = Oddity::default();
                let object = OBJECTS.choose(&mut rng).unwrap();
                let gender = object.gender;
                oddity.object = String::from(object.text);
                oddity.weird = Some(String::from("[adj]"));
                self.value = run(&build_oddity_string(&oddity), &mut Context { gender, rng });
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
