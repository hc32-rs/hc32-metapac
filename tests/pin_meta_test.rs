//! pins 元数据验收（host 测试）：JCTA 539 / PCTB 1048 / PE6 仅 PCTB。
//! 门控：只在「F460 两封装样本 + metadata」都启用时编译/运行——
//! 未启用 metadata 时编译为空（此前普通 `cargo test` 直接触发编译失败）。
//! 单 feature 运行（build.rs 限制一次一芯片）：
//!   cargo test --no-default-features --features metadata,hc32f460jcta --test pin_meta_test
//!   cargo test --no-default-features --features metadata,hc32f460pctb --test pin_meta_test
#![cfg(all(
    feature = "metadata",
    any(feature = "hc32f460jcta", feature = "hc32f460pctb")
))]
#[test]
fn pin_metadata() {
    let m = &hc32_metapac::metadata::METADATA;
    let n: usize = m.peripherals.iter().map(|p| p.pins.len()).sum();
    let pe6: Vec<&str> = m
        .peripherals
        .iter()
        .filter(|p| p.pins.iter().any(|pin| pin.pin == "PE6"))
        .map(|p| p.name)
        .collect();
    if m.name == "HC32F460JCTA" {
        assert_eq!(n, 539, "JCTA pins 总数");
        assert!(pe6.is_empty(), "JCTA 不得有 PE6 路由，实测 {pe6:?}");
    } else if m.name == "HC32F460PCTB" {
        assert_eq!(n, 1048, "PCTB pins 总数");
        assert_eq!(pe6, ["SPI3", "SPI4", "USART3", "USART4"], "PE6 路由归属");
    }
}
