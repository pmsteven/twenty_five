// twenty_five.rs  --  Find five 5-letter words using 25 of the 26 letters
//                     of the ASCII alphabet. Try to do it fast...
// (c) Peter M. Steven 2022

use std::env;
use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::time::Instant;

fn main() {
    let now = Instant::now();   // start the clock...now!
    let args: Vec<String> = env::args().collect();
    let file: &str;
    if args.len() < 2 {
        println!("Defaulting to the Wordle word list.");
        file = "../../Assets/original-wordle.txt";
    }
    else {
        file = &args[1];
    }

    // read words from file. encode them and remove anagrams
    let (full_list, anagrams) = load_word_file(file);
    println!("{} 5-letter unique words with {} anagrams ", full_list.len(), anagrams.len());
    println!("{:.3} seconds elapsed.", now.elapsed().as_millis() as f64/1000.0);

    // bool table indicating whether any given word can be paired with another without overlap
    let allowed = build_allowed_table(&full_list);
    println!("{0} x {0} table of allowed combos built.", full_list.len());
    println!("{:.3} seconds elapsed.", now.elapsed().as_millis() as f64/1000.0);

    let mut victory = 0;
    // allocate once for efficiency, even though the later Vecs are needlessly large
    let mut cand_a: Vec<usize> = Vec::with_capacity(full_list.len());
    let mut cand_ab: Vec<usize> = Vec::with_capacity(full_list.len());
    let mut cand_abc: Vec<usize> = Vec::with_capacity(full_list.len());
    let mut cand_abcd: Vec<usize> = Vec::with_capacity(full_list.len());

    if full_list.len() < 5 {
        println!("Not enough words to test!");
        return;
    }
    for a in 0..full_list.len()-4 {
        // note that clear is very fast. it just zeros the length, it doesn't change memory.
        cand_a.clear();
        // build a list of non-overlapping words to start with
        let allow_a = &allowed[a];      // load the row first for single index access
        for i in 0..full_list.len() {
            // only allow words later in the list in avoid list permutations
            if i > a && allow_a[i] {    // i > a was added to the allow table...
                cand_a.push(i)          // nevertheless, faster to avoid the lookup all together
            }
        }
        for &b in cand_a.iter() {
            cand_ab.clear();
            // edit the list to non-overlapping words of a and b
            let allow_b = &allowed[b];
            for &i in cand_a.iter() {
                if i > b && allow_b[i] {
                    cand_ab.push(i)     // note that push is very fast as long as
                                        // you don't need to allocate space, hence
                                        // the inital declaration.
                }
            }
            for &c in cand_ab.iter() {
                // note these inner loops are getting much faster
                // because the lists are getting much shorter
                cand_abc.clear();
                // now non-overlapping words of a, b, and c
                let allow_c = &allowed[c];
                for &i in cand_ab.iter() {
                    if i > c && allow_c[i] {
                        cand_abc.push(i)
                    }
                }
                for &d in cand_abc.iter() {
                    cand_abcd.clear();
                    // finally non-overlapping words of a, b, c, and d
                    for &i in cand_abc.iter() {
                        if i > d && allowed[d][i] {
                            cand_abcd.push(i)
                        }
                    }
                    // everything in this list completes the set of 5 non-overlapping words
                    for &e in cand_abcd.iter() {
                        victory += 1;
                        print!("Found {}-{}-{}-{}-{}", full_list[a].1,
                            full_list[b].1, full_list[c].1,
                            full_list[d].1, full_list[e].1);
                        let mut labeled = false;
                        // check if any of these words have anagrams
                        // not super fast, but it doesn't happen much
                        for n in 0..anagrams.len() {
                            if anagrams[n].0 == full_list[a].0 ||
                                anagrams[n].0 == full_list[b].0 ||
                                anagrams[n].0 == full_list[c].0 ||
                                anagrams[n].0 == full_list[d].0 ||
                                anagrams[n].0 == full_list[e].0 {
                                if !labeled {
                                    print!(" with anagrams: ");
                                    labeled = true;
                                }
                                print!(" {}", anagrams[n].1);
                            }
                        }
                        println!("{}", if labeled {"."} else {""})
                    }
                }
            }
        }
    }
    if victory == 0 {
        println!("No 5-word sets found.");
    }
    else {
        println!("{victory} unique solutions found.")
    }
    println!("Total elapsed time {:.3} seconds.", now.elapsed().as_millis() as f64/1000.0);
}

fn load_word_file(filename: &str)
        -> (Vec<(usize, String)>, Vec<(usize, String)>) {
    let mut full_list: Vec<(usize, String)> = Vec::with_capacity(6000);
    let mut anagrams: Vec<(usize, String)> = Vec::with_capacity(4000);

    let path = Path::new(filename);
    let display = path.display();

    let file = match File::open(&path) {
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(ok_file) => ok_file,
    };

    'main_loop: for line in io::BufReader::new(file).lines() {
        if let Ok(word) = line {
            let trim_word = word.trim().to_ascii_lowercase();
            if trim_word.len() != 5 || !trim_word.is_ascii() {
                continue;
            }

            // make a bit mask for each word.
            // mask a & mask b == 0 means no common letters
            let mut word_code: usize = 0;
            for &c in trim_word.as_bytes() {
                word_code |= 1 << (c - 'a' as u8);
            }

            // check for five unique letters
            if word_code.count_ones() == 5 {
                for i in 0..full_list.len() {
                    if word_code == full_list[i].0 {
                        anagrams.push((word_code, trim_word));
                        continue 'main_loop;    // don't blame me for the jump,...
                    }                           // blame Rust for lacking the for...else of python
                }
                full_list.push((word_code, trim_word));
            }
        }
    }
    (full_list, anagrams)
}

fn build_allowed_table (full_list: &Vec<(usize, String)>) -> Vec<Vec<bool>> {
    let mut allowed: Vec<Vec<bool>> = Vec::with_capacity(full_list.len());
    for i in 0..full_list.len() {
        let mut allow: Vec<bool> = Vec::with_capacity(full_list.len());
        let word_code = full_list[i].0;
        for j in 0..full_list.len() {
            // i < j disallows permutations.
            // word sets are always in the order they were read.
            allow.push(i < j && word_code & full_list[j].0 == 0);
        }
        // build each row then make a table of rows. Might be faster to
        // have a true 2D table, but the code prefetches the row address
        // already, so it probably doesn't matter.
        allowed.push(allow);
    }
    allowed
}