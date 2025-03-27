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


    if let Ok(re) = Regex::new(r"Asset#1_name:\s*([\w-]+).*?Asset#1_balance:\s*([\d\.]+)") {
        if let Some(caps) = re.captures(result) {
            if caps.len() >= 3 {
                let asset_name = caps.get(1).map_or("", |m| m.as_str()).to_string();
                let asset_amount = caps.get(2).map_or("", |m| m.as_str()).to_string();
                assets.push((asset_name, asset_amount));
            }
        }
    }

    if let Ok(re) = Regex::new(r"Asset#2_name:\s*([\w-]+).*?Asset#2_balance:\s*([\d\.]+)") {
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



pub fn parse_asset_single(result: &str) -> Vec<(String, String)> {
    let mut assets = Vec::new();

    if let Ok(re) = Regex::new(r"#Asset_name:\s*([\w-]+).*?#Asset_balance:\s*([\d\.]+)") {
        if let Some(caps) = re.captures(result) {
            if caps.len() >= 3 {
                let asset_name = caps.get(1).map_or("", |m| m.as_str()).to_string();
                let asset_amount = caps.get(2).map_or("", |m| m.as_str()).to_string();
                assets.push((asset_name, asset_amount));
                return assets
            }
        }
    }

    assets
}


// 디버그용 테스트 함수 추가
#[cfg(test)]
mod tests {
    use super::*;  // 상위 모듈의 모든 항목을 가져옵니다

    #[test]
    pub fn test_parsing() {
        let test_str = "# DeFi High-Risk Investment Strategy Analysis\n\n## 1. Main Recommendation\n\nI recommend following combination, #54 Strategy - Earn SWPx and fees on SwapX pool wS-SACRA by Ichi IV-SWAPX-12-SACRA-wS and corresponding depositable_assets are [\"wS\", \"SACRA\", \"USDC\", \"stS\"] from them, I can pick asset(s) which is also included in user asset state; Asset#1_name: wS Asset#1_balance: 120.5 and Asset#2_name: SACRA Asset#2_balance: 50203.25\n\nThis strategy offers an exc";

        // 전략 번호 파싱 테스트
        let idx = hashtag_num_parser(test_str);
        assert_eq!(idx, 54, "전략 번호가 올바르게 파싱되지 않았습니다");

        // 자산 정보 파싱 테스트
        let assets = parse_asset_info(test_str);
        assert_eq!(assets.len(), 2, "자산 정보 개수가 올바르지 않습니다");

        // 첫 번째 자산 검증
        assert_eq!(assets[0].0, "wS", "첫 번째 자산 이름이 올바르지 않습니다");
        assert_eq!(assets[0].1, "120.5", "첫 번째 자산 금액이 올바르지 않습니다");

        // 두 번째 자산 검증
        assert_eq!(assets[1].0, "SACRA", "두 번째 자산 이름이 올바르지 않습니다");
        assert_eq!(assets[1].1, "50203.25", "두 번째 자산 금액이 올바르지 않습니다");

        // 전략 이름 파싱 테스트

        println!("전략 번호: {}", idx);
        println!("자산 정보: {:?}", assets);
    }
}
