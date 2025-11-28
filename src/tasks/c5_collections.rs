// This chapter is dedicated to some collections: vectors, strings and hash maps

// VECTORS
// ================================================================================================

// ----- 1 --------------------------------------
// Write a function `second_largest(vec: &[i32]) -> Option<i32>` that returns the second largest
// element in the array. If the array has fewer than 2 elements, return `None`.

use std::collections::HashSet;

pub fn second_largest(vec: &[i32]) -> Option<i32> {
    if vec.len() < 2 {
        return None;
    }

    let mut largest = i32::MIN;
    let mut second_largest = i32::MIN;

    for &num in vec {
        if num > largest {
            second_largest = largest;
            largest = num;
        } else if num > second_largest && num < largest {
            second_largest = num;
        }
    }

    if second_largest == i32::MIN {
        None
    } else {
        Some(second_largest)
    }
}

// ----- 2 --------------------------------------
// Write a function `longest_increasing_subsequence(vec: &[i32]) -> Vec<i32>`` that finds the
// longest strictly increasing subsequence (not necessarily contiguous) in the array.
//
// For the simplicity, assume that there is only one longest increasing subsequence.

pub fn longest_increasing_subsequence(init_sequence: &[i32]) -> Vec<i32> {
    if init_sequence.is_empty() {
        return Vec::new();
    }

    let n = init_sequence.len();
    let mut dp = vec![1; n];
    let mut prev = vec![None; n];

    for i in 1..n {
        for j in 0..i {
            if init_sequence[i] > init_sequence[j] && dp[i] < dp[j] + 1 {
                dp[i] = dp[j] + 1;
                prev[i] = Some(j);
            }
        }
    }

    let mut max_length = dp[0];
    let mut max_index = 0;
    for i in 1..n {
        if dp[i] > max_length {
            max_length = dp[i];
            max_index = i;
        }
    }

    let mut result = Vec::new();
    let mut current = Some(max_index);
    while let Some(idx) = current {
        result.push(init_sequence[idx]);
        current = prev[idx];
    }
    result.reverse();

    result
}

// STRINGS
// ================================================================================================

// ----- 3 --------------------------------------
// Write a function `reverse_words(sentence: &str) -> String` that reverses the order of words in a
// sentence but does not reverse the characters inside each word.

pub fn reverse_words(sentence: &str) -> String {
    let words: Vec<&str> = sentence.split_whitespace().collect();
    words.iter().rev().map(|&s| s).collect::<Vec<&str>>().join(" ")
}

// ----- 4 --------------------------------------
// Write a function `normalize_and_capitalize(sentence: &str) -> String` that:
// - Trims extra spaces at the beginning and end.
// - Converts multiple spaces between words into a single space.
// - Makes the first letter of every word uppercase, and every other letter lowercase, for example
//   "пРеВеД МеДвЕд -> Превед Медвед"

pub fn normalize_and_capitalize(sentence: &str) -> String {
    let trimmed = sentence.trim();
    let words: Vec<&str> = trimmed.split_whitespace().collect();

    words
        .iter()
        .map(|&word| {
            if word.is_empty() {
                return String::new();
            }

            let mut chars: Vec<char> = word.chars().collect();

            if let Some(first) = chars.first_mut() {
                *first = first.to_uppercase().next().unwrap();
            }

            for i in 1..chars.len() {
                chars[i] = chars[i].to_lowercase().next().unwrap();
            }

            chars.into_iter().collect()
        })
        .collect::<Vec<String>>()
        .join(" ")
}

// HASH SET
// ================================================================================================

// ----- 5 --------------------------------------
// Write a function `unique_chars(s: &str) -> bool` that returns true if a string has all unique
// characters (ignoring case), and false otherwise.

pub fn unique_chars(s: &str) -> bool {
    let hash_set: HashSet<char> = s.chars().map(|c| c.to_lowercase().next().unwrap()).collect();
    hash_set.len() == s.len()
}

// HASH MAP
// ================================================================================================

// ----- 6 --------------------------------------
// Write a function `top_k_frequent(nums: Vec<i32>, k: usize) -> Vec<i32>` that returns the `k` most
// frequent numbers in the vector. If `k` is greater than the total number of unique elements in the
// vector, return all of them.

pub fn top_k_frequent(nums: Vec<i32>, k: usize) -> Vec<i32> {
    !unimplemented!()
}
