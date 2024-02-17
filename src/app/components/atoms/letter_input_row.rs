use std::borrow::Borrow;

use wasm_bindgen::JsCast;
use gloo::console::{log};
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub word_length: usize,
    pub row: usize,
    pub guessed: bool,
    pub word: Option<String>,
    pub submit: Callback<SubmitEvent>
}

#[function_component(LetterInputRow)]
pub fn letter_input_row(props: &Props) -> Html {
    html! {
        <div class="letterRow">
            {build_inputs(props.word_length, props.row, props.word.clone().unwrap_or_default())}
        </div> 
    }
}

fn build_inputs(word_length: usize, row: usize, word: String) -> Vec<Html>{
    let mut inputs: Vec<Html> = Vec::new();
    for n in 0..word_length {
        inputs.push(html!{<div id={"letterInput-".to_owned() + row.to_string().borrow() + "-".borrow() + n.to_string().borrow()} class="letter">
            {word.chars().nth(n).unwrap_or_default()}
        </div>})
    }
    return inputs
}