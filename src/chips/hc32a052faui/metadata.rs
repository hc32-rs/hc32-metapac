include!("../metadata_0000.rs");
            pub static METADATA: Metadata = Metadata {
                name: "HC32A052FAUI",
                family: "HC32A052",
                memory: crate::memory_select::MEMORY,
                peripherals: PERIPHERALS,
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };