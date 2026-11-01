> **Check if a token is safe before trading — one call, PASS/WARN/BITE.**

```toml
[dependencies]
guarddog = "0.1"
```

```rust
let r = guarddog::check_token("base", "0x4200...0006").await?;
if guarddog::is_bite(&r) {
    // BLOCK — abort trade
}
```

| verdict | agent action |
|---|---|
| PASS | proceed |
| WARN | read `reasons` |
| BITE (BLOCK) | **abort** |

Free 50 calls/day/IP. 23ms P95. ETH/BSC/Base/Solana.
Full spec: [llms.txt](https://api.guarddog.top/llms.txt)
