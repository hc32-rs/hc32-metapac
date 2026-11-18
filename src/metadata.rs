pub mod ir {
    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct IR {
        pub blocks: &'static [Block],
        pub fieldsets: &'static [FieldSet],
        pub enums: &'static [Enum],
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct Block {
        pub name: &'static str,
        pub extends: Option<&'static str>,

        pub description: Option<&'static str>,
        pub items: &'static [BlockItem],
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct BlockItem {
        pub name: &'static str,
        pub description: Option<&'static str>,

        pub array: Option<Array>,
        pub byte_offset: u32,

        pub inner: BlockItemInner,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum BlockItemInner {
        Block(BlockItemBlock),
        Register(Register),
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct Register {
        pub access: Access,
        pub bit_size: u32,
        pub fieldset: Option<&'static str>,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct BlockItemBlock {
        pub block: &'static str,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum Access {
        ReadWrite,
        Read,
        Write,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct FieldSet {
        pub name: &'static str,
        pub extends: Option<&'static str>,

        pub description: Option<&'static str>,
        pub bit_size: u32,
        pub fields: &'static [Field],
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct Field {
        pub name: &'static str,
        pub description: Option<&'static str>,

        pub bit_offset: BitOffset,
        pub bit_size: u32,
        pub array: Option<Array>,
        pub enumm: Option<&'static str>,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum Array {
        Regular(RegularArray),
        Cursed(CursedArray),
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct RegularArray {
        pub len: u32,
        pub stride: u32,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct CursedArray {
        pub offsets: &'static [u32],
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum BitOffset {
        Regular(RegularBitOffset),
        Cursed(CursedBitOffset),
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct RegularBitOffset {
        pub offset: u32,
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct CursedBitOffset {
        pub ranges: &'static [core::ops::RangeInclusive<u32>],
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct Enum {
        pub name: &'static str,
        pub description: Option<&'static str>,
        pub bit_size: u32,
        pub variants: &'static [EnumVariant],
    }

    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct EnumVariant {
        pub name: &'static str,
        pub description: Option<&'static str>,
        pub value: u64,
    }
}

pub use crate::mem_layout::{Access, MemoryRegion, MemoryRegionKind, MemoryRole};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Metadata {
    pub name: &'static str,
    pub family: &'static str,
    pub memory: &'static [MemoryRegion],
    pub peripherals: &'static [Peripheral],
    pub interrupts: &'static [Interrupt],
    pub dma_channels: &'static [DmaChannel],
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Interrupt {
    pub name: &'static str,
    pub number: u32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Peripheral {
    pub name: &'static str,
    pub address: u64,
    pub registers: Option<PeripheralRegisters>,
    /// MODE 拆分定时器的 SVD 视图（同基址布局叠影；与 `registers` 互斥）。
    pub views: &'static [PeripheralView],
    pub rcc: Option<PeripheralRcc>,
    pub pins: &'static [PeripheralPin],
    pub dma_channels: &'static [PeripheralDmaChannel],
    pub interrupts: &'static [PeripheralInterrupt],
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralRegisters {
    pub kind: &'static str,
    pub version: &'static str,
    pub block: &'static str,
    pub ir: &'static ir::IR,
}

/// MODE 拆分定时器的一个 SVD 视图：工作模式由控制寄存器 MD 位运行时选择，
/// 地址/时钟门/中断与物理实例共用（见 Peripheral::name）。
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralView {
    /// SVD 视图原名（与 PAC 侧 `pub const` 同名）。
    pub name: &'static str,
    /// 工作模式档位：MODE0 / MODE1 / MODE23。
    pub mode: &'static str,
    pub registers: PeripheralRegisters,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralInterrupt {
    pub signal: &'static str,
    pub interrupt: &'static str,
}

/// HC32 时钟门控：PWC/FCG 使能寄存器位（如 PERI_CLKEN.SysctrlPeripheralAdcBgr）。
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralRcc {
    pub enable: Option<PeripheralRccRegister>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralRccRegister {
    pub register: &'static str,
    pub field: &'static str,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralPin {
    pub pin: &'static str,
    pub signal: &'static str,
    pub func: Option<u8>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DmaChannel {
    pub name: &'static str,
    pub dma: &'static str,
    pub channel: u32,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct PeripheralDmaChannel {
    pub signal: &'static str,
    pub dma: Option<&'static str>,
    pub channel: Option<&'static str>,
    pub request: Option<u16>,
}
