/**
 * CPM : chars per minute
 * WPM : word per minute
 */
pub struct Model {
    // algorithm to count WPM
    cpm: i32,
    wpm: i32,

    // assuming average lenght of word x, count how many chars have been
    // written since last time
    count_char_for_word: i32,
    elapsed_min: i32,
    start_time: i32,
}

impl Model {
    pub fn new() -> Model {
        Model {
            cpm: 0,
            wpm: 0,
            count_char_for_word: 0,
            elapsed_min: 0,
            start_time: 0,
        }
    }

    pub fn on_key_pressed(&self) {
        /* if char of interest count 
            if count_char == x 
                then call algorithm
        */
    
        /* if char is of interest
            then call algorithm for char
        */
    }
}
