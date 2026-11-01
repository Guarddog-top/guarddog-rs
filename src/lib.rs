//! GuardDog token safety check for Rust agents.
//!
//! Check if a token is SAFE before trading — one call, PASS/WARN/BITE.
//! BITE (BLOCK) = abort trade immediately.
//!
//! ```no_run
//! # tokio_test::block_on(async {
//! let r = guarddog::check_token("eth", "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2").await.unwrap();
//! assert_eq!(r.verdict, "PASS");
//! if guarddog::is_bite(&r) { /* abort trade */ }
//! # })
//! ```

use serde::Deserialize;

pub const DEFAULT_BASE: &str = "https://api.guarddog.top";

#[derive(Debug, Deserialize, Clone)]
pub struct CheckResult {
    pub risk_score: u8,
    pub verdict: String,
    pub reasons: Vec<String>,
    pub covered: bool,
    #[serde(default)]
    pub data_limited: bool,
    #[serde(default)]
    pub unknown_token: bool,
}

/// Check if a token is safe to trade. Returns PASS/WARN/BLOCK verdict.
///
/// # Arguments
/// * `chain` - "eth" | "bsc" | "base" | "solana"
/// * `target` - token contract address
pub async fn check_token(chain: &str, target: &str)
    -> Result<CheckResult, Box<dyn std::error::Error>>
{
    check_token_with_base(DEFAULT_BASE, chain, target).await
}

/// Same as `check_token` but with custom API base URL.
pub async fn check_token_with_base(base: &str, chain: &str, target: &str)
    -> Result<CheckResult, Box<dyn std::error::Error>>
{
    let url = format!("{}/v1/check?chain={}&target={}",
                     base, chain, target);
    let resp = reqwest::get(&url).await?;
    Ok(resp.json().await?)
}

/// Returns true if the verdict is BITE (BLOCK) — abort trade.
pub fn is_bite(r: &CheckResult) -> bool {
    r.verdict == "BLOCK"
}

/// Returns true if the verdict is PASS — safe to proceed.
pub fn is_pass(r: &CheckResult) -> bool {
    r.verdict == "PASS"
}
