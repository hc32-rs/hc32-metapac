include!("../metadata_0002.rs");
            pub static METADATA: Metadata = Metadata {
                name: "HC32A448MCTI",
                family: "HC32A448",
                memory: crate::memory_select::MEMORY,
                peripherals: PERIPHERALS,
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };