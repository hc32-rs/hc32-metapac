include!("../metadata_0000.rs");
            pub static METADATA: Metadata = Metadata {
                name: "HC32F052FATB",
                family: "HC32F052",
                memory: crate::memory_select::MEMORY,
                peripherals: PERIPHERALS,
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };