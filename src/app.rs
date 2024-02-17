use std::borrow::{Borrow, BorrowMut};

use gloo::console::{log};
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;
use stylist::{yew::styled_component, Style};
use wasm_bindgen::{JsCast, JsValue};
use yew::UseStateHandle;

mod components;

use components::atoms::main_title::MainTitle;
use components::atoms::letter_input_row::LetterInputRow;

const STYLES: &str = include_str!("main.css");
const KEYBOARD_ROW_ONE: &str = "QWERTZUIOP";
const KEYBOARD_ROW_TWO: &str = "ASDFGHJKL";
const KEYBOARD_ROW_THREE: &str = "YXCVBNM";
const WORD: &str = "HELLO";


#[derive(Clone)]
pub struct InputState {
    guessed: bool,
    word: Option<String>,
}

#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet = Style::new(STYLES).unwrap();
    let rows = 5;
    let word_length = 5;

    let input_states: UseStateHandle<Vec<InputState>> = use_state(|| vec![InputState {guessed: false, word: None}, InputState {guessed: false, word: None}, InputState {guessed: false, word: None}, InputState {guessed: false, word: None}, InputState {guessed: false, word: None}]);
    let current_letter_row = use_state(|| 0);
    let current_letter_column = use_state(|| 0);
    let counter = use_state(|| 0);

    let on_keyboard_click = {
        let counter = counter.clone();
        let input_states = input_states.clone();
        let current_letter_row = current_letter_row.clone();
        let current_letter_column = current_letter_column.clone();
        Callback::from(move |event: MouseEvent| {
            let target = event.target().unwrap().unchecked_into::<HtmlElement>();
            let letter = target.text_content().unwrap();
            counter.set(*counter + 1);

            input_states.set(input_states.to_vec().iter().enumerate().map(|(index, input_state)| {
                if(*current_letter_row == index){
                    if(*current_letter_column > word_length - 1){
                        input_state.clone()   
                    }else{
                        InputState {guessed: true, word: Some(input_state.word.clone().unwrap_or_default() + &letter.clone())}
                    }
                }else{
                    input_state.clone()
                }
            }).collect::<Vec<InputState>>());
            
            /* input_states.set(input_states.to_vec().iter().enumerate().map(|(index, input_state)| {
                if(*current_row == index){
                    InputState {guessed: true, word: Some(letter.clone())}
                }else{
                    input_state.clone()
                }
            }).collect::<Vec<InputState>>()); */

            current_letter_column.set(*current_letter_column + 1);
            // Map each element to a string representation and join them into a single string
            let string_result = input_states.to_vec().into_iter().map(|x| x.word.unwrap_or_default().to_string()).collect::<Vec<_>>().join(", ");
            
            log!(letter);
            log!(counter.to_string());
            log!(string_result);
        })
    };

    html! {
    <>
        <div class={stylesheet}>
            <MainTitle title="Hello there"/>
            <div class="letters">
                {build_inputs(rows, word_length, input_states.clone())}
            </div>
            
            <div class="onScreenKeyboard">
                <div class="keyBoardRow">{build_letters(1, KEYBOARD_ROW_ONE, &on_keyboard_click)}</div>
                <div class="keyBoardRow">{build_letters(2, KEYBOARD_ROW_TWO, &on_keyboard_click)}</div>
                <div class="keyBoardRow">{build_letters(3, KEYBOARD_ROW_THREE, &on_keyboard_click)}</div>
            </div>
        </div>
    </>
    }
}

fn build_inputs(rows: usize, word_length: usize, input_states: UseStateHandle<Vec<InputState>>) -> Vec<Html>{
    let mut inputs = Vec::new();
    for n in 0..rows {
        inputs.push(html!{<LetterInputRow guessed={input_states.to_vec()[n].guessed} word={input_states.to_vec()[n].word.clone()} row={n} word_length={word_length} submit={onsubmit} />})
    }

    return inputs
}

fn build_letters(row: usize, letters: &str, onclick: &Callback<MouseEvent>) -> Vec<Html>{
    let mut keyboardRow = Vec::new();

    for letter in letters.chars() {
        keyboardRow.push(html!{<div onclick={onclick.clone()} class="keyBoardKey">{letter}</div>}); // Use the cloned state variable in the closure
    }
    
    return keyboardRow
}

fn onKeyboardKeyClick(event: MouseEvent, state: UseStateHandle<Vec<InputState>>) {
    
}

fn read_input_value(event: SubmitEvent) -> String {
    let form = event.target().unwrap().unchecked_into::<HtmlElement>();
    let mut child_inputs = Vec::new();

    log!(form.children());

    for n in 0..form.child_element_count() {
        child_inputs.push(form.children().item(n).unwrap().unchecked_into::<HtmlInputElement>())
    }

    let input_letters = child_inputs.iter().map(|input: &HtmlInputElement| input.value()).collect::<Vec<String>>();

    return input_letters.join("")
}

fn onsubmit(event: SubmitEvent) {
    event.prevent_default();
    let input_value = read_input_value(event);

    check_input_value_against_word(&input_value);

    log!(input_value);
}

fn check_input_value_against_word(guessed_word: &String){
    let letters = guessed_word.split("");
    let contained_letters = letters.clone().filter(|letter| WORD.contains(&letter.to_uppercase())).collect::<Vec<&str>>();
    let non_contained_letters = letters.clone().filter(|letter| !WORD.contains(&letter.to_uppercase())).collect::<Vec<&str>>();

    let contained = contained_letters.join("-");
    let non_contained = non_contained_letters.join("-");

    log!("contained: ", contained);
    log!("non contained: ", non_contained);            
}