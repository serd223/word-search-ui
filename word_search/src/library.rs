use super::*;
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct SearchOptions {
    max_len_diff: i32,
    suggestion_count: usize,
    len_diff_over_weight_bias: f32,
    max_weight_diff: f32,
    forced_letter_multiplier: usize,
    weight_format_multiplier: f32,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            max_len_diff: 3,
            suggestion_count: 7,
            len_diff_over_weight_bias: 0.005,
            max_weight_diff: 0.79,
            forced_letter_multiplier: 4,
            weight_format_multiplier: (10u32).pow(7) as f32,
        }
    }
}

fn different_char_count(a: &str, b: &str) -> i32 {
    let n = a.chars().count().min(b.chars().count());
    let mut ac = a.chars();
    let mut bc = b.chars();
    let mut res = 0;
    for _ in 0..n {
        if ac.next() != bc.next() {
            res += 1;
        }
    }
    res
}

#[derive(Default)]
pub struct Library {
    source: String,
    options: SearchOptions,
}

impl Library {
    pub fn new(options: SearchOptions) -> Self {
        Self {
            options: options,
            ..Default::default()
        }
    }

    pub fn set_source(&mut self, new_source: String) {
        self.source.clear();
        self.source = new_source;
    }

    pub fn search(&self, input: &str) -> Vec<(&str, i32)> {
        // Sorry but I really don't want to refactor the search logic.

        let input_len = input.chars().count();
        if input_len < 1 {
            return (0..self.options.suggestion_count)
                .map(|_| ("", 0))
                .collect();
        }
        let forced_letter_count = match (input_len / self.options.forced_letter_multiplier) as usize
        {
            0 => 1,
            n => n,
        };
        let input_word = Word::from_str(input);

        let mut input_chars = input.chars().clone();
        let mut word_prefix = String::new();
        for _ in 0..forced_letter_count {
            word_prefix.push(input_chars.next().unwrap());
        }
        // { str: total_difference }
        let suggestions: HashMap<&str, f32> = HashMap::new();
        let sm = Arc::new(Mutex::new(suggestions));

        let words_list: Vec<Word> = self
            .source
            .par_lines()
            .filter(|s| {
                let len_diff = (input_len as i32 - s.chars().count() as i32).abs();
                !(len_diff > self.options.max_len_diff || !s.starts_with(&word_prefix))
            })
            .map(|s| Word::from_str(s))
            .collect::<Vec<Word>>();

        words_list.par_iter().for_each(|w| {
            let len_diff = (input_len as i32 - w.str_repr.chars().count() as i32).abs();

            let total: f32 = w
                .letters
                .iter()
                .map(|(c, w)| match input_word.letters.get(c) {
                    Some(iw) => (iw - w).abs(),
                    None => *w,
                })
                .sum::<f32>()
                + (len_diff as f32 / input_len as f32) / 100.
                + different_char_count(input, w.str_repr) as f32 / input_len as f32 * 1.35;

            if total <= self.options.max_weight_diff {
                let mut krm = ""; //  Key To Remove
                let mut should_add = false;
                let mut sg = sm.lock().unwrap();
                if sg.len() < self.options.suggestion_count {
                    should_add = true
                }

                if !should_add {
                    for (k, v) in sg.iter() {
                        let slendiff = (input_len as i32 - k.chars().count() as i32).abs();
                        if total < *v {
                            krm = k;
                            should_add = true;
                            break;
                        } else if len_diff < slendiff
                            && (total - v).abs() < self.options.len_diff_over_weight_bias
                        {
                            krm = k;
                            should_add = true;
                            break;
                        }
                    }
                }
                if should_add {
                    (*sg).remove(krm);
                    (*sg).insert(w.str_repr, total);
                }
                // drop(sg);
            }
        });

        let sg = sm.lock().unwrap();
        let mut sv: Vec<(&str, i32)> = (*sg)
            .iter()
            .map(|(k, v)| (*k, (*v * self.options.weight_format_multiplier) as i32))
            .collect();
        drop(sg);
        sv.sort_unstable_by_key(|(_, v)| *v);
        sv
    }
}
