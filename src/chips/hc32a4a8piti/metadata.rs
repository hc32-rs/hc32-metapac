include!("../metadata_0005.rs");
            pub static METADATA: Metadata = Metadata {
                name: "HC32A4A8PITI",
                family: "HC32A4A8",
                memory: crate::memory_select::MEMORY,
                peripherals: PERIPHERALS,
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };