use regex::Regex;

// for parsing strategy number;
#[allow(dead_code)]
pub fn hashtag_num_parser(result: &str, re: Regex)-> usize{
    let original_idx = re
        .captures(result)
        .and_then(|caps| caps.get(1))
        .and_then(|m| m.as_str().parse::<usize>().ok())
        .unwrap_or(0);

    original_idx
}