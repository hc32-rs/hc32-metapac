# hc32-metapac

**本仓库由 [hc32-data-tools](https://github.com/hc32-rs/hc32-data-tools) 自动生成，请勿手工修改；
所有 Issue/PR 请到 [hc32-data-tools](https://github.com/hc32-rs/hc32-data-tools) 或对应数据仓库提交。**

小华半导体（XHSC）HC32 全系列单片机的 Peripheral Access Crate（PAC）与元数据，
架构对标 [ch32-rs/ch32-data](https://github.com/ch32-rs/ch32-data) 生成的 `ch32-metapac`。

- 数据来源：[hc32-ddl-data](https://github.com/hc32-rs/hc32-ddl-data)（老 DDL 驱动系列）
  与 [hc32-ll-data](https://github.com/hc32-rs/hc32-ll-data)（LL 驱动系列）
- 提取/生成流水线：[hc32-data-tools](https://github.com/hc32-rs/hc32-data-tools)

## 用法

```toml
[dependencies]
hc32-metapac = { version = "0.1", features = ["hc32f460jeta"] }
```

- `pac`（默认）：寄存器访问层，每外设一个模块，`hc32_metapac::USART0.cr().write(|w| ..)`
- `metadata`：编译期元数据 `hc32_metapac::METADATA`（外设/中断/DMA/内存布局）
- `rt`：中断向量表（需配合 cortex-m-rt），`#[hc32_metapac::interrupt]`
- `memory-x`：生成链接脚本 memory.x（FLASH/SRAM 布局）

芯片选择 feature 为完整型号小写，如 `hc32f003c4pa`、`hc32f460jeta`。

## License

MIT OR Apache-2.0
