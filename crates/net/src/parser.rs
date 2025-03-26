use regex::Regex;

#[allow(dead_code)]
pub fn hashtag_num_parser(result: &str) -> usize {
    if let Ok(re) = Regex::new(r"#(\d+)") {
        let original_idx = re
            .captures(result)
            .and_then(|caps| caps.get(1))
            .and_then(|m| m.as_str().parse::<usize>().ok())
            .unwrap_or(0);

        original_idx
    }
    else {
        0
    }
}

#[allow(dead_code)]
pub fn parse_asset_info(result: &str) -> Vec<(String, String)> {
    let mut assets = Vec::new();


    if let Ok(re) = Regex::new(r"Asset#1_name:\s*([\w-]+).*?Asset#1_amount:\s*([\d\.]+)") {
        if let Some(caps) = re.captures(result) {
            if caps.len() >= 3 {
                let asset_name = caps.get(1).map_or("", |m| m.as_str()).to_string();
                let asset_amount = caps.get(2).map_or("", |m| m.as_str()).to_string();
                assets.push((asset_name, asset_amount));
            }
        }
    }

    if let Ok(re) = Regex::new(r"Amount#2_name:\s*([\w-]+).*?Asset#2_amount:\s*([\d\.]+)") {
        if let Some(caps) = re.captures(result) {
            if caps.len() >= 3 {
                let asset_name = caps.get(1).map_or("", |m| m.as_str()).to_string();
                let asset_amount = caps.get(2).map_or("", |m| m.as_str()).to_string();
                assets.push((asset_name, asset_amount));
            }
        }
    }

    assets
}

#[allow(dead_code)]
pub fn parse_strategy_name(result: &str) -> String {
    // 전체 전략 이름을 캡처하도록 수정 (공백 포함)
    if let Ok(re) = Regex::new(r"Strategy\s+-\s+(.+?)\s+for\s+Asset") {
        re.captures(result)
            .and_then(|caps| caps.get(1))
            .map_or("".to_string(), |m| m.as_str().to_string())
    } else {
        "".to_string()
    }
}

// 디버그용 테스트 함수 추가
#[allow(dead_code)]
pub fn test_parsing() {
    let test_str = "I recommend following combination, #12 Strategy - Earn SWPx by SwapX classic wS-MOON vLP for Asset#1_name: MOON Asset#1_amount: 379.5924";

    let idx = hashtag_num_parser(test_str);


    let assets = parse_asset_info(test_str);

    let strategy_name = parse_strategy_name(test_str);
}
