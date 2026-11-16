include!("../metadata_0007.rs");
            pub static METADATA: Metadata = Metadata {
                name: "HC32F003C4PA",
                family: "HC32F003",
                memory: crate::memory_select::MEMORY,
                peripherals: PERIPHERALS,
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };