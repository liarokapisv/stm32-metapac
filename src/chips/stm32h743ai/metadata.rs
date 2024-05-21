include!("../metadata_0423.rs");
            use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
            pub static METADATA: Metadata = Metadata {
                name: "STM32H743AI",
                family: "STM32H7",
                line: "STM32H743/753",
                memory: &[
    MemoryRegion {
        name: "BANK_1",
        kind: MemoryRegionKind::Flash,
        address: 0x8000000,
        size: 1048576,
        settings: Some(
            FlashSettings {
                erase_size: 131072,
                write_size: 32,
                erase_value: 255,
            },
        ),
    },
    MemoryRegion {
        name: "BANK_2",
        kind: MemoryRegionKind::Flash,
        address: 0x8100000,
        size: 1048576,
        settings: Some(
            FlashSettings {
                erase_size: 131072,
                write_size: 32,
                erase_value: 255,
            },
        ),
    },
    MemoryRegion {
        name: "SRAM",
        kind: MemoryRegionKind::Ram,
        address: 0x24000000,
        size: 524288,
        settings: None,
    },
],
                peripherals: PERIPHERALS,
                nvic_priority_bits: Some(4),
                interrupts: INTERRUPTS,
                dma_channels: DMA_CHANNELS,
            };