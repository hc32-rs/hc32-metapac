include!("../metadata_0005.rs");
            pub static METADATA: Metadata = Metadata {
                name: "HC32F4A8SITB",
                family: "HC32F4A8",
                memory: crate::memory_select::MEMORY,
                peripherals: PERIPHERALS,
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };