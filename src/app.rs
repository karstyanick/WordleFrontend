use std::borrow::{Borrow, BorrowMut};
use gloo::console::{log};
use rand::seq::SliceRandom;
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;
use stylist::{yew::styled_component, Style};
use wasm_bindgen::{JsCast, JsValue};
use yew::UseStateHandle;

mod components;
use components::atoms::letter_input_row::LetterInputRow;

const STYLES: &str = include_str!("main.css");
const KEYBOARD_ROW_ONE: &str = "QWERTZUIOP";
const KEYBOARD_ROW_TWO: &str = "ASDFGHJKL";
const KEYBOARD_ROW_THREE: &str = "YXCVBNM";
const WORDS: [&str; 121] = [
    "APPLE", "TABLE", "CHAIR", "HOUSE", "BEACH", "MOUSE", "HAPPY", "HELLO", "WORLD", "GRASS",
    "LIGHT", "DRINK", "PLANT", "TIGER", "LEMON", "CLOCK", "RIVER", "MUSIC", "PAPER", "MONEY",
    "NIGHT", "EARTH", "PEACE", "PHONE", "SMILE", "PIANO", "HONEY", "DREAM", "SWORD", "SPOON",
    "KNIFE", "SHOES", "PUPPY", "SUNNY", "CLOUD", "BREAD", "PIZZA", "CANDY", "SUGAR", "CREAM",
    "MELON", "APPLE", "FANCY", "MAGIC", "ROBOT", "QUEEN", "HEART", "DREAM", "STORY", "GIANT",
    "OCEAN", "TABLE", "CHAIR", "KNIFE", "SPOON", "FORKS", "WINGS", "PEACE", "HAPPY", "LAUGH",
    "SMILE", "NIGHT", "STARS", "LIGHT", "WATER", "EARTH", "PLANTS", "TIGER", "LION", "ZEBRA",
    "HORSE", "BEACH", "OCEAN", "COAST", "SHORE", "LEMON", "GRAPE", "KIWI", "MELON", "GUAVA",
    "PEACH", "JUICE", "CANDY", "CHIPS", "TOAST", "BURGER", "PIZZA", "STEAK", "BACON", "SAUCE",
    "FRIES", "CHEESE", "CREAM", "HONEY", "MONEY", "COINS", "DOLLAR", "EUROS", "POUND", "SENSE",
    "CENTS", "NICKEL", "PENNY", "PHONE", "MUSIC", "RADIO", "TEARS", "SONGS", "VOICE", "SUNNY",
    "CLOUD", "STORM", "RAINY", "SNOWY", "ICY", "COLD", "WINDY", "SUNNY", "CLOUD", "STORM",
    "THUND"
];

#[derive(Clone)]
pub struct InputState {
    guessed: bool,
    word: String,
}

pub struct LetterStatus {
    contained: bool,
    right_position: bool
}
#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
enum GuessStatus {
    NotGuessed,
    Right,
    Contained,
    NotContained,
}
#[derive(Debug)]
#[derive(Clone)]
struct KeyboardLetter {
    letter: char,
    guess_status: GuessStatus
}

#[styled_component(App)]
pub fn app() -> Html {
    let stylesheet = Style::new(STYLES).unwrap();
    let input_states: UseStateHandle<Vec<InputState>> = use_state(|| vec![InputState {guessed: false, word: "".to_owned()}, InputState {guessed: false, word: "".to_owned()}, InputState {guessed: false, word: "".to_owned()}, InputState {guessed: false, word: "".to_owned()}, InputState {guessed: false, word: "".to_owned()}, InputState {guessed: false, word: "".to_owned()}]);
    let current_letter_row: UseStateHandle<usize> = use_state(|| 0);
    let current_letter_column: UseStateHandle<usize> = use_state(|| 0);
    let rows = use_state(|| 6);
    let word_length = use_state(|| 5);
    let chosen_word = use_state(|| *WORDS.choose(&mut rand::thread_rng()).unwrap());
    let keyboard_row_one = use_state(|| KEYBOARD_ROW_ONE.chars().map(|c| { KeyboardLetter {letter: c, guess_status: GuessStatus::NotGuessed}}).collect::<Vec<KeyboardLetter>>());
    let keyboard_row_two = use_state(|| KEYBOARD_ROW_TWO.chars().map(|c| { KeyboardLetter {letter: c, guess_status: GuessStatus::NotGuessed}}).collect::<Vec<KeyboardLetter>>());
    let keyboard_row_three = use_state(|| KEYBOARD_ROW_THREE.chars().map(|c| { KeyboardLetter {letter: c, guess_status: GuessStatus::NotGuessed}}).collect::<Vec<KeyboardLetter>>());

    let on_keyboard_click = {
        
        let input_states = input_states.clone();
        let current_letter_row = current_letter_row.clone();
        let current_letter_column = current_letter_column.clone();
        let word_length = word_length.clone();

        Callback::from(move |event: MouseEvent| {
            let target = event.target().unwrap().unchecked_into::<HtmlElement>();
            let letter = target.text_content().unwrap();
            input_states.set(input_states.to_vec().iter().enumerate().map(|(index, input_state)| {
                
                let current_row_word = input_state.word.clone();
                
                if(*current_letter_row == index){
                    if(*current_letter_column > *word_length.clone() - 1){
                        return input_state.clone()
                    }

                    current_letter_column.set(*current_letter_column + 1);
                    let concatenated = format!("{}{}", current_row_word, letter.clone());
                    return InputState {guessed: false, word: concatenated}
                }else{
                    input_state.clone()
                }
            }).collect::<Vec<InputState>>());            
        })
    };

    let on_backspace_click = {
        let input_states = input_states.clone();
        let current_letter_row = current_letter_row.clone();
        let current_letter_column = current_letter_column.clone();

        Callback::from(move |event: MouseEvent| {
            input_states.set(input_states.to_vec().iter().enumerate().map(|(index, input_state)| {
                let current_row_word = input_state.word.clone();
                if(*current_letter_row == index && current_row_word.len() > 0){
                    let last_removed = current_row_word.split_at(current_row_word.len() - 1).0.to_owned();
                    current_letter_column.set(*current_letter_column - 1);
                    return InputState {guessed: false, word: last_removed}
                }else{
                    input_state.clone()
                }
            }).collect::<Vec<InputState>>());
        })
    };

    let on_enter_click = {
        let input_states = input_states.clone();
        let current_letter_row = current_letter_row.clone();
        let current_letter_column = current_letter_column.clone();
        let word_length = word_length.clone();
        let chosen_word = chosen_word.clone();
        let keyboard_row_one = keyboard_row_one.clone();
        let keyboard_row_two = keyboard_row_two.clone();
        let keyboard_row_three = keyboard_row_three.clone();
        Callback::from(move |event: MouseEvent| {
            if(input_states[*current_letter_row].word.len() == *word_length){
                input_states.set(input_states.to_vec().iter().enumerate().map(|(index, input_state)| {
                    if(*current_letter_row == index){
                        return InputState {guessed: true, word: input_state.word.clone()}
                    }else{
                        input_state.clone()
                    }
                }).collect::<Vec<InputState>>());

                let submitted_word = &input_states[*current_letter_row].word;
                let letter_status = check_input_value_against_word(submitted_word, (*chosen_word).to_owned());


                keyboard_row_one.set(keyboard_row_one.iter().map(|key| {
                    let mut key = key.clone();
                    submitted_word.chars().enumerate().for_each(|(index, c)| {
                        
                        let guess_status = match letter_status[index] {
                            LetterStatus {contained: true, right_position: true} => GuessStatus::Right,
                            LetterStatus {contained: true, right_position: false} => GuessStatus::Contained,
                            LetterStatus {contained: false, right_position: false} => GuessStatus::NotContained,
                            LetterStatus {contained: false, right_position: true} => GuessStatus::NotGuessed,
                        };

                        if (key.letter == c){
                            log!(format!("{:?}", guess_status));
                            log!(format!("{:?}", c));
                            key = KeyboardLetter {letter: key.letter, guess_status: guess_status.clone()}
                        }
                    });
                    key
                }).collect::<Vec<KeyboardLetter>>());

                keyboard_row_two.set(keyboard_row_two.iter().map(|key| {
                    let mut key = key.clone();
                    submitted_word.chars().enumerate().for_each(|(index, c)| {
                        
                        let guess_status = match letter_status[index] {
                            LetterStatus {contained: true, right_position: true} => GuessStatus::Right,
                            LetterStatus {contained: true, right_position: false} => GuessStatus::Contained,
                            LetterStatus {contained: false, right_position: false} => GuessStatus::NotContained,
                            LetterStatus {contained: false, right_position: true} => GuessStatus::NotGuessed,
                        };

                        if (key.letter == c){
                            log!(format!("{:?}", guess_status));
                            log!(format!("{:?}", c));
                            key = KeyboardLetter {letter: key.letter, guess_status: guess_status.clone()}
                        }
                    });
                    key
                }).collect::<Vec<KeyboardLetter>>());
                
                keyboard_row_three.set(keyboard_row_three.iter().map(|key| {
                    let mut key = key.clone();
                    submitted_word.chars().enumerate().for_each(|(index, c)| {
                        
                        let guess_status = match letter_status[index] {
                            LetterStatus {contained: true, right_position: true} => GuessStatus::Right,
                            LetterStatus {contained: true, right_position: false} => GuessStatus::Contained,
                            LetterStatus {contained: false, right_position: false} => GuessStatus::NotContained,
                            LetterStatus {contained: false, right_position: true} => GuessStatus::NotGuessed,
                        };

                        if (key.letter == c){
                            log!(format!("{:?}", guess_status));
                            log!(format!("{:?}", c));
                            key = KeyboardLetter {letter: key.letter, guess_status: guess_status.clone()}
                        }
                    });
                    key
                }).collect::<Vec<KeyboardLetter>>());

                current_letter_row.set(*current_letter_row + 1);
                current_letter_column.set(0);
            }
        })
    };

    html! {
    <>
        <div class={stylesheet}>
            <div class="letters">
                {build_inputs(*chosen_word, *rows, *word_length, input_states.clone())}
            </div>
            
            <div class="onScreenKeyboard">
                <div class="keyBoardRow">{build_letters(1, keyboard_row_one.clone(), &on_keyboard_click)}</div>
                <div class="keyBoardRow">{build_letters(2, keyboard_row_two.clone(), &on_keyboard_click)}</div>
                <div class="keyBoardRow">
                    <div onclick={&on_enter_click} class="keyBoardKey enterKey">{"ENTER"}</div>
                    {build_letters(3, keyboard_row_three.clone(), &on_keyboard_click)}
                    <div onclick={&on_backspace_click} class="keyBoardKey backspaceKey">{"<"}</div>
                </div>
            </div>
        </div>
    </>
    }
}

fn build_inputs(chosen_word: &str, rows: usize, word_length: usize, input_states: UseStateHandle<Vec<InputState>>) -> Vec<Html>{
    let mut inputs = Vec::new();
    for n in 0..rows {
        inputs.push(html!{<LetterInputRow guessed={input_states.to_vec()[n].guessed} word={input_states.to_vec()[n].word.clone()} row={n} word_length={word_length} word_to_guess={chosen_word.to_owned()} />})
    }

    return inputs
}

fn build_letters(row: usize, letters: UseStateHandle<Vec<KeyboardLetter>>, onclick: &Callback<MouseEvent>) -> Vec<Html>{
    let mut keyboardRow = Vec::new();
    
    for letter in &*letters {
        let class = if letter.guess_status == GuessStatus::Right {
            "right"
        } else if letter.guess_status == GuessStatus::Contained {
            "contained"
        } else if letter.guess_status == GuessStatus::NotContained {
            "notContained"
        } else {
            ""
        };
        
        keyboardRow.push(html!{<div onclick={onclick} class={classes!("keyBoardKey", class)} >{letter.letter}</div>});
    }
    
    return keyboardRow
}

pub fn check_input_value_against_word(guessed_word: &String, word: String) -> Vec<LetterStatus>{

    let letter_status = guessed_word.chars().enumerate().map(|(index, letter)| {
        let contained = word.contains(&letter.to_uppercase().collect::<String>());
        
        if(!contained){
            return LetterStatus {contained: false, right_position: false}
        }

        if(letter == word.chars().nth(index).unwrap()){
            return LetterStatus {contained: true, right_position: true}
        }

        LetterStatus {contained, right_position: false}
    }).collect::<Vec<LetterStatus>>();

    return letter_status;    
}