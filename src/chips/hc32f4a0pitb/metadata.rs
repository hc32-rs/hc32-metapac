include!("../metadata_0004.rs");
            pub static METADATA: Metadata = Metadata {
                name: "HC32F4A0PITB",
                family: "HC32F4A0",
                memory: crate::memory_select::MEMORY,
                peripherals: PERIPHERALS,
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };