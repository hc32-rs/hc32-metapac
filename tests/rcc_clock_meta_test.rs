//! rcc/bus/kernel_clock 元数据验收（host 测试）。
//! 验证 HAL 两个核心消费路径，全部走生成元数据，禁止硬编码：
//!   1) HAL 找门控位：peripheral.rcc.enable{register,field} + polarity
//!      （DDL high=写 1 使能；LL low=写 0 使能）；
//!   2) HAL 算 USART 输入频率：peripheral.kernel_clock 固定源（pclk0/pclk1）
//!      >> (2 × PSC)，PSC 为运行时寄存器值（BRR.PSC / PR.PSC，2 位）。
//! PSC=1 覆盖重复分频回归：pclk0 与 pclk1 同为 8MHz、PSC=1 时，
//! F420 USART0 与 F460 USART1 必须算出同一输入频率（8M/4=2MHz），
//! 防止把 4^psc 写成 2^psc 或读错位域。
//!
//! 门控：只在「样本芯片 + metadata」都启用时编译/运行——
//! `cargo test` 默认 feature（pac、无芯片）下整个文件编译为空，
//! `./d ci` 的 F003C4PA/F460PCTB 样本不会撞进样本分支 panic。
//! 单 feature 运行（build.rs 限制一次一芯片）：
//!   cargo test --no-default-features --features metadata,hc32f420jatb --test rcc_clock_meta_test
//!   cargo test --no-default-features --features metadata,hc32f460jcta --test rcc_clock_meta_test

#![cfg(all(
    feature = "metadata",
    any(feature = "hc32f420jatb", feature = "hc32f460jcta")
))]

/// HAL 侧门控动作：按元数据极性决定置位/清零。
/// 返回 (寄存器名, 字段名, 写入值)，模拟 HAL 对 PAC 的最终写操作。
fn gate_action(p: &hc32_metapac::metadata::Peripheral) -> (&'static str, &'static str, bool) {
    let rcc = p
        .rcc
        .as_ref()
        .expect("USART 必须有 rcc 门控元数据");
    let en = rcc
        .enable
        .as_ref()
        .expect("USART 必须有 enable{register,field}");
    let polarity = rcc
        .polarity
        .expect("样本系列的门控极性必须已登记（RCC_META）");
    // high: 写 1 使能；low: 写 0 使能（LL FCG 复位值 0xFFFF_FFFF）
    let write_value = match polarity {
        "high" => true,
        "low" => false,
        other => panic!("未知极性 {other}，数据登记错误"),
    };
    (en.register, en.field, write_value)
}

/// HAL 侧 USART 输入频率：
/// kernel_clock 固定源频率 >> (2 × PSC)。
/// PSC ∈ {0,1,2,3} → 预分频 /1 /4 /16 /64（RM 表 17-1 F420 / RM 25.4.1.1 F460；
/// SDK usart.c `1U << (PSC * 2U)` 两代驱动同式）。
fn usart_kernel_hz(
    clocks: &std::collections::BTreeMap<&str, u32>,
    p: &hc32_metapac::metadata::Peripheral,
    psc: u32,
) -> u32 {
    assert!(psc <= 3, "BRR.PSC/PR.PSC 为 2 位域");
    let kc = p
        .kernel_clock
        .as_ref()
        .expect("USART 必须有 kernel_clock 元数据");
    match kc {
        hc32_metapac::metadata::PeripheralKernelClock::Fixed { name } => {
            let bus_hz = *clocks
                .get(name)
                .unwrap_or_else(|| panic!("时钟树节点 {name} 未在 HAL 状态中登记"));
            bus_hz >> (2 * psc)
        }
        hc32_metapac::metadata::PeripheralKernelClock::Selectable { .. } => {
            panic!("样本系列的 USART 必须是 fixed 源；selectable 属 ADC/DAC/TRNG")
        }
    }
}

#[test]
fn rcc_and_usart_clock() {
    let m = &hc32_metapac::metadata::METADATA;

    if m.name == "HC32F420JATB" {
        // ---- 样本 1：DDL F420 USART0 ----
        // 门控位：元数据必须给出 APB0EN.USART0（RM 表1-2 APB0 域 + 表3-5.13 b0；
        // 此前提取 bug 曾误记 AHB0EN，本断言即回归防线）。
        let p = m
            .peripherals
            .iter()
            .find(|p| p.name == "USART0")
            .expect("F420 必须有 USART0");
        assert_eq!(gate_action(p), ("APB0EN", "SysctrlPeripheralUsart0", true));
        assert_eq!(p.rcc.as_ref().unwrap().bus, Some("apb0"));

        // 输入频率：PCLK0=8MHz（HAL 读 CR0.CLKSRC/HCLKPRS/PCLK0PRS 的产物，
        // 此处以已知时钟态模拟），RM 图3-1 固定挂 PCLK0。
        let clocks = std::collections::BTreeMap::from([("pclk0", 8_000_000u32)]);
        assert_eq!(usart_kernel_hz(&clocks, p, 0), 8_000_000, "PSC=0: 8M/1");
        // PSC=1 回归：8MHz/4 = 2MHz，防 4^psc/2^psc 混淆。
        assert_eq!(usart_kernel_hz(&clocks, p, 1), 2_000_000, "PSC=1: 8M/4");
        assert_eq!(usart_kernel_hz(&clocks, p, 2), 500_000, "PSC=2: 8M/16");
        assert_eq!(usart_kernel_hz(&clocks, p, 3), 125_000, "PSC=3: 8M/64");
    } else if m.name == "HC32F460JCTA" {
        // ---- 样本 2：LL F460 USART1 ----
        // 门控位：FCG1 bit24，0=使能（RM 5.7.14 复位值 0xFFFF_FFFF）。
        let p = m
            .peripherals
            .iter()
            .find(|p| p.name == "USART1")
            .expect("F460 必须有 USART1");
        assert_eq!(gate_action(p), ("FCG1", "PWC_FCG1_USART1", false));
        assert_eq!(p.rcc.as_ref().unwrap().bus, Some("apb1"));

        // 输入频率：PCLK1=8MHz（HAL 读 CKSWR/SCFGR.PCLK1S）；
        // APB2 域的 USART3/4 时钟同为 PCLK1（RM 表1-1 两处标注），
        // 核对 bus=apb2 但 kernel=pclk1 不从 bus 推导。
        let clocks = std::collections::BTreeMap::from([("pclk1", 8_000_000u32)]);
        assert_eq!(usart_kernel_hz(&clocks, p, 0), 8_000_000, "PSC=0: 8M/1");
        // PSC=1 重复分频回归：与 F420 PCLK0=8MHz/PSC=1 同为 2MHz。
        assert_eq!(usart_kernel_hz(&clocks, p, 1), 2_000_000, "PSC=1: 8M/4");
        assert_eq!(usart_kernel_hz(&clocks, p, 2), 500_000, "PSC=2: 8M/16");
        assert_eq!(usart_kernel_hz(&clocks, p, 3), 125_000, "PSC=3: 8M/64");

        let p3 = m
            .peripherals
            .iter()
            .find(|p| p.name == "USART3")
            .expect("F460 必须有 USART3");
        assert_eq!(p3.rcc.as_ref().unwrap().bus, Some("apb2"), "USART3 属 APB2 域");
        assert_eq!(
            usart_kernel_hz(&clocks, p3, 1),
            2_000_000,
            "APB2 域 USART3 时钟仍取 PCLK1"
        );
    } else {
        panic!("本测试只对样本芯片运行，当前芯片 {m:?} 不在样本内");
    }
}
