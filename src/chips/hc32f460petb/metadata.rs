include!("../metadata_0003.rs");
            pub static METADATA: Metadata = Metadata {
                name: "HC32F460PETB",
                family: "HC32F460",
                memory: crate::memory_select::MEMORY,
                peripherals: PERIPHERALS,
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };