use relm::{Relm, Widget, connect};
use std::time::{Duration, Instant};
use std::convert::TryFrom;

/**
 * CPM : chars per minute
 * WPM : word per minute
 */
pub struct Model {
    // algorithm to count WPM
    cps: f32,
    wpm: i32,

    // assuming average lenght of word x, count how many chars have been
    // written since last time
    count_char_for_word: i32,
    last_input_char: Instant,
    start_typ_time: Instant,
    in_typing: bool,
    chars_count: i32,
}



impl Model {
    pub fn new() -> Model {
        Model {
            cps: 0.0,
            wpm: 0,
            count_char_for_word: 0,
            start_typ_time: Instant::now(),
            last_input_char: Instant::now(),
            in_typing: false,
            chars_count: 0,
        }
    }

    pub fn chars_per_sec(&self) -> f32 {
        self.cps
    }

    pub fn key_pressed(&mut self) {
        let elaps_sec_from_last_typ = elapsed_sec(self.last_input_char, Instant::now());
        self.last_input_char = Instant::now();
        
        if !self.in_typing {
            self.start_typ_time = Instant::now();
            self.chars_count = 0;
            self.in_typing = true;
        }
        else {
            if elaps_sec_from_last_typ > 1.0 {
                self.flush_cps();
            }
        }

        self.chars_count += 1;
    }

    pub fn flush_cps(&mut self) {
        if self.in_typing {
            self.cps = cps_alg(self.chars_count, self.start_typ_time, self.last_input_char);
            self.in_typing = false;
        }
    }

}

fn cps_alg(chars_count: i32, start: Instant, end: Instant) -> f32 {
    let elapsed_sec = elapsed_sec(start, end);
    chars_count as f32 / elapsed_sec as f32
}

fn elapsed_sec(start: Instant, end: Instant) -> f64{ 
    let elapsed_time = end - start;
    elapsed_time.as_secs() as f64 + elapsed_time.subsec_nanos() as f64 * 1e-9
}
