// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_main]

use risc0_zkvm::guest::env;
use risc0_zkvm::sha::{Impl, Sha256};
use wordle_core::{GameState, LetterFeedback, WordFeedback, WORD_LENGTH};

risc0_zkvm::guest::entry!(main);

#[repr(C)]
struct Hack {
    buffer: [u8; 8],
    point: *const fn(),
}

fn magic_function() {
    env::log("Magic function called! Should never be called under normal circumstances ...");
}

pub fn main() {
    let secret: String = env::read();
    let guess: String = env::read();

    let input_bytes: &[u8] = guess.as_bytes();

    let mut hack = Hack {
        buffer: [0; 8],
        point: 0 as *const fn() -> (),
    };

    unsafe {
        std::ptr::copy(
            input_bytes.as_ptr(),
            hack.buffer.as_mut_ptr(),
            input_bytes.len(),
        )
    }

    env::log(&format!(
        "MAGIC function address: x{:0x}",
        magic_function as usize
    ));

    env::log(&format!(
        "hack.point after strcpy: x{:0x}",
        hack.point as usize,
    ));
    // env::log(&format!(
    // "hack.point after strcpy (in chars): {:?}",
    // (hack.point as usize as u64)
    // .to_le_bytes()
    // .into_iter()
    // .map(|b| char::from(b))
    // .collect::<String>(),
    // ));
    if hack.point as usize != 0 {
        // env::log("Try again");
        //} else {
        let code: fn() = unsafe { std::mem::transmute(0x800009c) }; // this is hardcoded, should be taken from the overflown buffer but i am lazy
        code();
    }

    // if guess.eq("booom") {
    //    secret = guess.clone();
    //}
    // assert_eq!(
    // guess.chars().count(),
    // WORD_LENGTH,
    // "guess must have length 5!"
    // );
    //
    // assert_eq!(
    // secret.chars().count(),
    // WORD_LENGTH,
    // "secret must have length 5!"
    // );
    let mut feedback: WordFeedback = WordFeedback::default();
    for i in 0..WORD_LENGTH {
        feedback.0[i] = if secret.as_bytes()[i] == guess.as_bytes()[i] {
            LetterFeedback::Correct
        } else if secret.as_bytes().contains(&guess.as_bytes()[i]) {
            LetterFeedback::Present
        } else {
            LetterFeedback::Miss
        }
    }

    let correct_word_hash = *Impl::hash_bytes(&secret.as_bytes());
    let game_state = GameState {
        correct_word_hash,
        feedback,
    };
    env::commit(&game_state);
}
