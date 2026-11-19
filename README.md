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

- `pac`（默认）：寄存器访问层，例如 F460 的 `hc32_metapac::USART1`。
- `metadata`：编译期元数据 `hc32_metapac::metadata::METADATA`，含外设、中断、DMA 和内存布局。
- `rt`：生成中断向量表和 `device.x` 支持，配合 `cortex-m-rt` 使用。
- `memory-x`：生成链接脚本 memory.x（FLASH/SRAM 布局）

芯片选择 feature 为完整型号小写，如 `hc32f003c4pa`、`hc32f460jeta`。

### 外设元数据覆盖

- `Peripheral.pins` 当前提供 F460 JCTA（LQFP48）和 PCTB（LQFP100）的 UART/SPI
  路由，已按型号封装过滤。其他型号的空列表表示当前没有录入路由。
- `rcc.polarity`、`rcc.bus` 和 `kernel_clock` 当前仅在 F420/F460 的已核对实例
  中提供。缺失值表示待确认。访问总线与功能时钟独立，例如 F460 USART3 属 APB2，
  内部功能时钟取 PCLK1。
- `usart_ddl_v1` 的 `CR1.TOE` 同时提供原有数值访问器
  `toe()/set_toe(u8)` 和类型化访问器 `toe_val()/set_toe_val(vals::Toe)`。

## License

MIT OR Apache-2.0
