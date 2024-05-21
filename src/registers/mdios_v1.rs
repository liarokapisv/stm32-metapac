
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Mdios",
            extends: None,
            description: Some(
                "Management data input/output slave",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "MDIOS configuration register",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wrfr",
                    description: Some(
                        "MDIOS write flag register",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Wrfr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cwrfr",
                    description: Some(
                        "MDIOS clear write flag register",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cwrfr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "rdfr",
                    description: Some(
                        "MDIOS read flag register",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Rdfr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "crdfr",
                    description: Some(
                        "MDIOS clear read flag register",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Crdfr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "sr",
                    description: Some(
                        "MDIOS status register",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Sr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clrfr",
                    description: Some(
                        "MDIOS clear flag register",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Clrfr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dinr",
                    description: Some(
                        "MDIOS input data register %s",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 32,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x1c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Dinr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "doutr",
                    description: Some(
                        "MDIOS output data register %s",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 32,
                                stride: 4,
                            },
                        ),
                    ),
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Doutr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Clrfr",
            extends: None,
            description: Some(
                "MDIOS clear flag register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cperf",
                    description: Some(
                        "Clear the preamble error flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cserf",
                    description: Some(
                        "Clear the start error flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cterf",
                    description: Some(
                        "Clear the turnaround error flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "MDIOS configuration register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "Peripheral enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wrie",
                    description: Some(
                        "Register write interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rdie",
                    description: Some(
                        "Register Read Interrupt Enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eie",
                    description: Some(
                        "Error interrupt enable",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dpc",
                    description: Some(
                        "Disable Preamble Check",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "port_address",
                    description: Some(
                        "Slaves's address",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 5,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Crdfr",
            extends: None,
            description: Some(
                "MDIOS clear read flag register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "crdf",
                    description: Some(
                        "Clear the read flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cwrfr",
            extends: None,
            description: Some(
                "MDIOS clear write flag register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cwrf",
                    description: Some(
                        "Clear the write flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dinr",
            extends: None,
            description: Some(
                "MDIOS input data register %s",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "din",
                    description: Some(
                        "Input data received from MDIO Master during write frames",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Doutr",
            extends: None,
            description: Some(
                "MDIOS output data register %s",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dout",
                    description: Some(
                        "Output data sent to MDIO Master during read frames",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Rdfr",
            extends: None,
            description: Some(
                "MDIOS read flag register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rdf",
                    description: Some(
                        "Read flags for MDIO registers 0 to 31",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Sr",
            extends: None,
            description: Some(
                "MDIOS status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "perf",
                    description: Some(
                        "Preamble error flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "serf",
                    description: Some(
                        "Start error flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "terf",
                    description: Some(
                        "Turnaround error flag",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wrfr",
            extends: None,
            description: Some(
                "MDIOS write flag register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wrf",
                    description: Some(
                        "Write flags for MDIO registers 0 to 31",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                