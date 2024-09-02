use wasm_bindgen::prelude::*;
use rand::Rng;

#[wasm_bindgen]
pub struct Magic8Ball {
    responses: Vec<String>,
}

#[wasm_bindgen]
impl Magic8Ball {
    pub fn new() -> Magic8Ball {
        let responses = vec![
            String::from("Yes"),
            String::from("No"),
            String::from("Ask again later"),
            String::from("It is certain"),
            String::from("Very doubtful"),
        ];
        Magic8Ball { responses }
    }

    pub fn shake(&self) -> String {
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..self.responses.len());
        self.responses[random_index].clone()
    }
}

