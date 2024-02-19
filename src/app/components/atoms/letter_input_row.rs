use std::borrow::Borrow;

use wasm_bindgen::JsCast;
use gloo::console::{log};
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;

use crate::app::check_input_value_against_word;

use self::_Props::word_to_guess;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub word_length: usize,
    pub row: usize,
    pub guessed: bool,
    pub word: String,
    pub word_to_guess: String
}

#[function_component(LetterInputRow)]
pub fn letter_input_row(props: &Props) -> Html {
    html! {
        <div class="letterRow">
            {build_inputs(props.word_length, props.row, props.word.clone(), props.guessed, props.word_to_guess.clone())}
        </div> 
    }
}

fn build_inputs(word_length: usize, row: usize, guessed_word: String, guessed: bool, word: String) -> Vec<Html>{
    let mut inputs: Vec<Html> = Vec::new();

    for n in 0..word_length {
        let mut class = "";
        
        if(guessed){
            let letter_status = check_input_value_against_word(&guessed_word, word.clone());

            if(letter_status[n].contained && letter_status[n].right_position){
                class = "right";
            }else if(letter_status[n].contained && !letter_status[n].right_position){
                class = "contained";
            }else if(!letter_status[n].contained){
                class = "notContained";
            }
        }

        inputs.push(html!{<div id={"letterInput-".to_owned() + row.to_string().borrow() + "-".borrow() + n.to_string().borrow()} class={classes!("letter", class)}>
            {guessed_word.chars().nth(n).unwrap_or_else(|| ' ')}
        </div>})
    }
    return inputs
}