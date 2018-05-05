use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - System Status Register"]
    pub status: STATUS,
    #[doc = "0x08 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x0c - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x10 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x18 - I/O Routing Register"]
    pub route: ROUTE,
    _reserved0: [u8; 16usize],
    #[doc = "0x2c - Charger Detect Configuration Register"]
    pub cdconf: CDCONF,
    #[doc = "0x30 - Command Register"]
    pub cmd: CMD,
    _reserved1: [u8; 8usize],
    #[doc = "0x3c - USB LEM Control Register"]
    pub lemctrl: LEMCTRL,
    _reserved2: [u8; 909248usize],
    #[doc = "0xde000 - OTG Control and Status Register"]
    pub gotgctl: GOTGCTL,
    #[doc = "0xde004 - OTG Interrupt Register"]
    pub gotgint: GOTGINT,
    #[doc = "0xde008 - AHB Configuration Register"]
    pub gahbcfg: GAHBCFG,
    #[doc = "0xde00c - USB Configuration Register"]
    pub gusbcfg: GUSBCFG,
    #[doc = "0xde010 - Reset Register"]
    pub grstctl: GRSTCTL,
    #[doc = "0xde014 - Interrupt Register"]
    pub gintsts: GINTSTS,
    #[doc = "0xde018 - Interrupt Mask Register"]
    pub gintmsk: GINTMSK,
    #[doc = "0xde01c - Receive Status Debug Read Register"]
    pub grxstsr: GRXSTSR,
    #[doc = "0xde020 - Receive Status Read /Pop Register"]
    pub grxstsp: GRXSTSP,
    #[doc = "0xde024 - Receive FIFO Size Register"]
    pub grxfsiz: GRXFSIZ,
    #[doc = "0xde028 - Non-periodic Transmit FIFO Size Register"]
    pub gnptxfsiz: GNPTXFSIZ,
    #[doc = "0xde02c - Non-periodic Transmit FIFO/Queue Status Register"]
    pub gnptxsts: GNPTXSTS,
    _reserved3: [u8; 16usize],
    #[doc = "0xde040 - Synopsys ID Register"]
    pub gsnpsid: GSNPSID,
    _reserved4: [u8; 24usize],
    #[doc = "0xde05c - Global DFIFO Configuration Register"]
    pub gdfifocfg: GDFIFOCFG,
    _reserved5: [u8; 160usize],
    #[doc = "0xde100 - Host Periodic Transmit FIFO Size Register"]
    pub hptxfsiz: HPTXFSIZ,
    #[doc = "0xde104 - Device in Endpoint Transmit FIFO Size Register 1"]
    pub dieptxf1: DIEPTXF1,
    #[doc = "0xde108 - Device in Endpoint Transmit FIFO Size Register 2"]
    pub dieptxf2: DIEPTXF2,
    #[doc = "0xde10c - Device in Endpoint Transmit FIFO Size Register 3"]
    pub dieptxf3: DIEPTXF3,
    #[doc = "0xde110 - Device in Endpoint Transmit FIFO Size Register 4"]
    pub dieptxf4: DIEPTXF4,
    #[doc = "0xde114 - Device in Endpoint Transmit FIFO Size Register 5"]
    pub dieptxf5: DIEPTXF5,
    #[doc = "0xde118 - Device in Endpoint Transmit FIFO Size Register 6"]
    pub dieptxf6: DIEPTXF6,
    _reserved6: [u8; 740usize],
    #[doc = "0xde400 - Host Configuration Register"]
    pub hcfg: HCFG,
    #[doc = "0xde404 - Host Frame Interval Register"]
    pub hfir: HFIR,
    #[doc = "0xde408 - Host Frame Number/Frame Time Remaining Register"]
    pub hfnum: HFNUM,
    _reserved7: [u8; 4usize],
    #[doc = "0xde410 - Host Periodic Transmit FIFO/Queue Status Register"]
    pub hptxsts: HPTXSTS,
    #[doc = "0xde414 - Host All Channels Interrupt Register"]
    pub haint: HAINT,
    #[doc = "0xde418 - Host All Channels Interrupt Mask Register"]
    pub haintmsk: HAINTMSK,
    _reserved8: [u8; 36usize],
    #[doc = "0xde440 - Host Port Control and Status Register"]
    pub hprt: HPRT,
    _reserved9: [u8; 188usize],
    #[doc = "0xde500 - Host Channel 0 Characteristics Register"]
    pub hc0_char: HC0_CHAR,
    #[doc = "0xde504 - Host Channel 0 Split Control Register"]
    pub hc0_splt: HC0_SPLT,
    #[doc = "0xde508 - Host Channel 0 Interrupt Register"]
    pub hc0_int: HC0_INT,
    #[doc = "0xde50c - Host Channel 0 Interrupt Mask Register"]
    pub hc0_intmsk: HC0_INTMSK,
    #[doc = "0xde510 - Host Channel 0 Transfer Size Register"]
    pub hc0_tsiz: HC0_TSIZ,
    #[doc = "0xde514 - Host Channel 0 DMA Address Register"]
    pub hc0_dmaaddr: HC0_DMAADDR,
    _reserved10: [u8; 8usize],
    #[doc = "0xde520 - Host Channel 0 Characteristics Register"]
    pub hc1_char: HC1_CHAR,
    #[doc = "0xde524 - Host Channel 0 Split Control Register"]
    pub hc1_splt: HC1_SPLT,
    #[doc = "0xde528 - Host Channel 0 Interrupt Register"]
    pub hc1_int: HC1_INT,
    #[doc = "0xde52c - Host Channel 0 Interrupt Mask Register"]
    pub hc1_intmsk: HC1_INTMSK,
    #[doc = "0xde530 - Host Channel 0 Transfer Size Register"]
    pub hc1_tsiz: HC1_TSIZ,
    #[doc = "0xde534 - Host Channel 0 DMA Address Register"]
    pub hc1_dmaaddr: HC1_DMAADDR,
    _reserved11: [u8; 8usize],
    #[doc = "0xde540 - Host Channel 0 Characteristics Register"]
    pub hc2_char: HC2_CHAR,
    #[doc = "0xde544 - Host Channel 0 Split Control Register"]
    pub hc2_splt: HC2_SPLT,
    #[doc = "0xde548 - Host Channel 0 Interrupt Register"]
    pub hc2_int: HC2_INT,
    #[doc = "0xde54c - Host Channel 0 Interrupt Mask Register"]
    pub hc2_intmsk: HC2_INTMSK,
    #[doc = "0xde550 - Host Channel 0 Transfer Size Register"]
    pub hc2_tsiz: HC2_TSIZ,
    #[doc = "0xde554 - Host Channel 0 DMA Address Register"]
    pub hc2_dmaaddr: HC2_DMAADDR,
    _reserved12: [u8; 8usize],
    #[doc = "0xde560 - Host Channel 0 Characteristics Register"]
    pub hc3_char: HC3_CHAR,
    #[doc = "0xde564 - Host Channel 0 Split Control Register"]
    pub hc3_splt: HC3_SPLT,
    #[doc = "0xde568 - Host Channel 0 Interrupt Register"]
    pub hc3_int: HC3_INT,
    #[doc = "0xde56c - Host Channel 0 Interrupt Mask Register"]
    pub hc3_intmsk: HC3_INTMSK,
    #[doc = "0xde570 - Host Channel 0 Transfer Size Register"]
    pub hc3_tsiz: HC3_TSIZ,
    #[doc = "0xde574 - Host Channel 0 DMA Address Register"]
    pub hc3_dmaaddr: HC3_DMAADDR,
    _reserved13: [u8; 8usize],
    #[doc = "0xde580 - Host Channel 0 Characteristics Register"]
    pub hc4_char: HC4_CHAR,
    #[doc = "0xde584 - Host Channel 0 Split Control Register"]
    pub hc4_splt: HC4_SPLT,
    #[doc = "0xde588 - Host Channel 0 Interrupt Register"]
    pub hc4_int: HC4_INT,
    #[doc = "0xde58c - Host Channel 0 Interrupt Mask Register"]
    pub hc4_intmsk: HC4_INTMSK,
    #[doc = "0xde590 - Host Channel 0 Transfer Size Register"]
    pub hc4_tsiz: HC4_TSIZ,
    #[doc = "0xde594 - Host Channel 0 DMA Address Register"]
    pub hc4_dmaaddr: HC4_DMAADDR,
    _reserved14: [u8; 8usize],
    #[doc = "0xde5a0 - Host Channel 0 Characteristics Register"]
    pub hc5_char: HC5_CHAR,
    #[doc = "0xde5a4 - Host Channel 0 Split Control Register"]
    pub hc5_splt: HC5_SPLT,
    #[doc = "0xde5a8 - Host Channel 0 Interrupt Register"]
    pub hc5_int: HC5_INT,
    #[doc = "0xde5ac - Host Channel 0 Interrupt Mask Register"]
    pub hc5_intmsk: HC5_INTMSK,
    #[doc = "0xde5b0 - Host Channel 0 Transfer Size Register"]
    pub hc5_tsiz: HC5_TSIZ,
    #[doc = "0xde5b4 - Host Channel 0 DMA Address Register"]
    pub hc5_dmaaddr: HC5_DMAADDR,
    _reserved15: [u8; 8usize],
    #[doc = "0xde5c0 - Host Channel 0 Characteristics Register"]
    pub hc6_char: HC6_CHAR,
    #[doc = "0xde5c4 - Host Channel 0 Split Control Register"]
    pub hc6_splt: HC6_SPLT,
    #[doc = "0xde5c8 - Host Channel 0 Interrupt Register"]
    pub hc6_int: HC6_INT,
    #[doc = "0xde5cc - Host Channel 0 Interrupt Mask Register"]
    pub hc6_intmsk: HC6_INTMSK,
    #[doc = "0xde5d0 - Host Channel 0 Transfer Size Register"]
    pub hc6_tsiz: HC6_TSIZ,
    #[doc = "0xde5d4 - Host Channel 0 DMA Address Register"]
    pub hc6_dmaaddr: HC6_DMAADDR,
    _reserved16: [u8; 8usize],
    #[doc = "0xde5e0 - Host Channel 0 Characteristics Register"]
    pub hc7_char: HC7_CHAR,
    #[doc = "0xde5e4 - Host Channel 0 Split Control Register"]
    pub hc7_splt: HC7_SPLT,
    #[doc = "0xde5e8 - Host Channel 0 Interrupt Register"]
    pub hc7_int: HC7_INT,
    #[doc = "0xde5ec - Host Channel 0 Interrupt Mask Register"]
    pub hc7_intmsk: HC7_INTMSK,
    #[doc = "0xde5f0 - Host Channel 0 Transfer Size Register"]
    pub hc7_tsiz: HC7_TSIZ,
    #[doc = "0xde5f4 - Host Channel 0 DMA Address Register"]
    pub hc7_dmaaddr: HC7_DMAADDR,
    _reserved17: [u8; 8usize],
    #[doc = "0xde600 - Host Channel 0 Characteristics Register"]
    pub hc8_char: HC8_CHAR,
    #[doc = "0xde604 - Host Channel 0 Split Control Register"]
    pub hc8_splt: HC8_SPLT,
    #[doc = "0xde608 - Host Channel 0 Interrupt Register"]
    pub hc8_int: HC8_INT,
    #[doc = "0xde60c - Host Channel 0 Interrupt Mask Register"]
    pub hc8_intmsk: HC8_INTMSK,
    #[doc = "0xde610 - Host Channel 0 Transfer Size Register"]
    pub hc8_tsiz: HC8_TSIZ,
    #[doc = "0xde614 - Host Channel 0 DMA Address Register"]
    pub hc8_dmaaddr: HC8_DMAADDR,
    _reserved18: [u8; 8usize],
    #[doc = "0xde620 - Host Channel 0 Characteristics Register"]
    pub hc9_char: HC9_CHAR,
    #[doc = "0xde624 - Host Channel 0 Split Control Register"]
    pub hc9_splt: HC9_SPLT,
    #[doc = "0xde628 - Host Channel 0 Interrupt Register"]
    pub hc9_int: HC9_INT,
    #[doc = "0xde62c - Host Channel 0 Interrupt Mask Register"]
    pub hc9_intmsk: HC9_INTMSK,
    #[doc = "0xde630 - Host Channel 0 Transfer Size Register"]
    pub hc9_tsiz: HC9_TSIZ,
    #[doc = "0xde634 - Host Channel 0 DMA Address Register"]
    pub hc9_dmaaddr: HC9_DMAADDR,
    _reserved19: [u8; 8usize],
    #[doc = "0xde640 - Host Channel 0 Characteristics Register"]
    pub hc10_char: HC10_CHAR,
    #[doc = "0xde644 - Host Channel 0 Split Control Register"]
    pub hc10_splt: HC10_SPLT,
    #[doc = "0xde648 - Host Channel 0 Interrupt Register"]
    pub hc10_int: HC10_INT,
    #[doc = "0xde64c - Host Channel 0 Interrupt Mask Register"]
    pub hc10_intmsk: HC10_INTMSK,
    #[doc = "0xde650 - Host Channel 0 Transfer Size Register"]
    pub hc10_tsiz: HC10_TSIZ,
    #[doc = "0xde654 - Host Channel 0 DMA Address Register"]
    pub hc10_dmaaddr: HC10_DMAADDR,
    _reserved20: [u8; 8usize],
    #[doc = "0xde660 - Host Channel 0 Characteristics Register"]
    pub hc11_char: HC11_CHAR,
    #[doc = "0xde664 - Host Channel 0 Split Control Register"]
    pub hc11_splt: HC11_SPLT,
    #[doc = "0xde668 - Host Channel 0 Interrupt Register"]
    pub hc11_int: HC11_INT,
    #[doc = "0xde66c - Host Channel 0 Interrupt Mask Register"]
    pub hc11_intmsk: HC11_INTMSK,
    #[doc = "0xde670 - Host Channel 0 Transfer Size Register"]
    pub hc11_tsiz: HC11_TSIZ,
    #[doc = "0xde674 - Host Channel 0 DMA Address Register"]
    pub hc11_dmaaddr: HC11_DMAADDR,
    _reserved21: [u8; 8usize],
    #[doc = "0xde680 - Host Channel 0 Characteristics Register"]
    pub hc12_char: HC12_CHAR,
    #[doc = "0xde684 - Host Channel 0 Split Control Register"]
    pub hc12_splt: HC12_SPLT,
    #[doc = "0xde688 - Host Channel 0 Interrupt Register"]
    pub hc12_int: HC12_INT,
    #[doc = "0xde68c - Host Channel 0 Interrupt Mask Register"]
    pub hc12_intmsk: HC12_INTMSK,
    #[doc = "0xde690 - Host Channel 0 Transfer Size Register"]
    pub hc12_tsiz: HC12_TSIZ,
    #[doc = "0xde694 - Host Channel 0 DMA Address Register"]
    pub hc12_dmaaddr: HC12_DMAADDR,
    _reserved22: [u8; 8usize],
    #[doc = "0xde6a0 - Host Channel 0 Characteristics Register"]
    pub hc13_char: HC13_CHAR,
    #[doc = "0xde6a4 - Host Channel 0 Split Control Register"]
    pub hc13_splt: HC13_SPLT,
    #[doc = "0xde6a8 - Host Channel 0 Interrupt Register"]
    pub hc13_int: HC13_INT,
    #[doc = "0xde6ac - Host Channel 0 Interrupt Mask Register"]
    pub hc13_intmsk: HC13_INTMSK,
    #[doc = "0xde6b0 - Host Channel 0 Transfer Size Register"]
    pub hc13_tsiz: HC13_TSIZ,
    #[doc = "0xde6b4 - Host Channel 0 DMA Address Register"]
    pub hc13_dmaaddr: HC13_DMAADDR,
    _reserved23: [u8; 328usize],
    #[doc = "0xde800 - Device Configuration Register"]
    pub dcfg: DCFG,
    #[doc = "0xde804 - Device Control Register"]
    pub dctl: DCTL,
    #[doc = "0xde808 - Device Status Register"]
    pub dsts: DSTS,
    _reserved24: [u8; 4usize],
    #[doc = "0xde810 - Device in Endpoint Common Interrupt Mask Register"]
    pub diepmsk: DIEPMSK,
    #[doc = "0xde814 - Device OUT Endpoint Common Interrupt Mask Register"]
    pub doepmsk: DOEPMSK,
    #[doc = "0xde818 - Device All Endpoints Interrupt Register"]
    pub daint: DAINT,
    #[doc = "0xde81c - Device All Endpoints Interrupt Mask Register"]
    pub daintmsk: DAINTMSK,
    _reserved25: [u8; 8usize],
    #[doc = "0xde828 - Device VBUS Discharge Time Register"]
    pub dvbusdis: DVBUSDIS,
    #[doc = "0xde82c - Device VBUS Pulsing Time Register"]
    pub dvbuspulse: DVBUSPULSE,
    #[doc = "0xde830 - Device Threshold Control Register"]
    pub dthrctl: DTHRCTL,
    #[doc = "0xde834 - Device in Endpoint FIFO Empty Interrupt Mask Register"]
    pub diepempmsk: DIEPEMPMSK,
    _reserved26: [u8; 200usize],
    #[doc = "0xde900 - Device Control in Endpoint 0 Control Register"]
    pub diep0ctl: DIEP0CTL,
    _reserved27: [u8; 4usize],
    #[doc = "0xde908 - Device in Endpoint 0 Interrupt Register"]
    pub diep0int: DIEP0INT,
    _reserved28: [u8; 4usize],
    #[doc = "0xde910 - Device in Endpoint 0 Transfer Size Register"]
    pub diep0tsiz: DIEP0TSIZ,
    #[doc = "0xde914 - Device in Endpoint 0 DMA Address Register"]
    pub diep0dmaaddr: DIEP0DMAADDR,
    #[doc = "0xde918 - Device in Endpoint Transmit FIFO Status Register 0"]
    pub diep0txfsts: DIEP0TXFSTS,
    _reserved29: [u8; 4usize],
    #[doc = "0xde920 - Device Control in Endpoint 1 Control Register"]
    pub diep0_ctl: DIEP0_CTL,
    _reserved30: [u8; 4usize],
    #[doc = "0xde928 - Device in Endpoint 1 Interrupt Register"]
    pub diep0_int: DIEP0_INT,
    _reserved31: [u8; 4usize],
    #[doc = "0xde930 - Device in Endpoint 1 Transfer Size Register"]
    pub diep0_tsiz: DIEP0_TSIZ,
    #[doc = "0xde934 - Device in Endpoint 1 DMA Address Register"]
    pub diep0_dmaaddr: DIEP0_DMAADDR,
    #[doc = "0xde938 - Device in Endpoint Transmit FIFO Status Register 1"]
    pub diep0_dtxfsts: DIEP0_DTXFSTS,
    _reserved32: [u8; 4usize],
    #[doc = "0xde940 - Device Control in Endpoint 1 Control Register"]
    pub diep1_ctl: DIEP1_CTL,
    _reserved33: [u8; 4usize],
    #[doc = "0xde948 - Device in Endpoint 1 Interrupt Register"]
    pub diep1_int: DIEP1_INT,
    _reserved34: [u8; 4usize],
    #[doc = "0xde950 - Device in Endpoint 1 Transfer Size Register"]
    pub diep1_tsiz: DIEP1_TSIZ,
    #[doc = "0xde954 - Device in Endpoint 1 DMA Address Register"]
    pub diep1_dmaaddr: DIEP1_DMAADDR,
    #[doc = "0xde958 - Device in Endpoint Transmit FIFO Status Register 1"]
    pub diep1_dtxfsts: DIEP1_DTXFSTS,
    _reserved35: [u8; 4usize],
    #[doc = "0xde960 - Device Control in Endpoint 1 Control Register"]
    pub diep2_ctl: DIEP2_CTL,
    _reserved36: [u8; 4usize],
    #[doc = "0xde968 - Device in Endpoint 1 Interrupt Register"]
    pub diep2_int: DIEP2_INT,
    _reserved37: [u8; 4usize],
    #[doc = "0xde970 - Device in Endpoint 1 Transfer Size Register"]
    pub diep2_tsiz: DIEP2_TSIZ,
    #[doc = "0xde974 - Device in Endpoint 1 DMA Address Register"]
    pub diep2_dmaaddr: DIEP2_DMAADDR,
    #[doc = "0xde978 - Device in Endpoint Transmit FIFO Status Register 1"]
    pub diep2_dtxfsts: DIEP2_DTXFSTS,
    _reserved38: [u8; 4usize],
    #[doc = "0xde980 - Device Control in Endpoint 1 Control Register"]
    pub diep3_ctl: DIEP3_CTL,
    _reserved39: [u8; 4usize],
    #[doc = "0xde988 - Device in Endpoint 1 Interrupt Register"]
    pub diep3_int: DIEP3_INT,
    _reserved40: [u8; 4usize],
    #[doc = "0xde990 - Device in Endpoint 1 Transfer Size Register"]
    pub diep3_tsiz: DIEP3_TSIZ,
    #[doc = "0xde994 - Device in Endpoint 1 DMA Address Register"]
    pub diep3_dmaaddr: DIEP3_DMAADDR,
    #[doc = "0xde998 - Device in Endpoint Transmit FIFO Status Register 1"]
    pub diep3_dtxfsts: DIEP3_DTXFSTS,
    _reserved41: [u8; 4usize],
    #[doc = "0xde9a0 - Device Control in Endpoint 1 Control Register"]
    pub diep4_ctl: DIEP4_CTL,
    _reserved42: [u8; 4usize],
    #[doc = "0xde9a8 - Device in Endpoint 1 Interrupt Register"]
    pub diep4_int: DIEP4_INT,
    _reserved43: [u8; 4usize],
    #[doc = "0xde9b0 - Device in Endpoint 1 Transfer Size Register"]
    pub diep4_tsiz: DIEP4_TSIZ,
    #[doc = "0xde9b4 - Device in Endpoint 1 DMA Address Register"]
    pub diep4_dmaaddr: DIEP4_DMAADDR,
    #[doc = "0xde9b8 - Device in Endpoint Transmit FIFO Status Register 1"]
    pub diep4_dtxfsts: DIEP4_DTXFSTS,
    _reserved44: [u8; 4usize],
    #[doc = "0xde9c0 - Device Control in Endpoint 1 Control Register"]
    pub diep5_ctl: DIEP5_CTL,
    _reserved45: [u8; 4usize],
    #[doc = "0xde9c8 - Device in Endpoint 1 Interrupt Register"]
    pub diep5_int: DIEP5_INT,
    _reserved46: [u8; 4usize],
    #[doc = "0xde9d0 - Device in Endpoint 1 Transfer Size Register"]
    pub diep5_tsiz: DIEP5_TSIZ,
    #[doc = "0xde9d4 - Device in Endpoint 1 DMA Address Register"]
    pub diep5_dmaaddr: DIEP5_DMAADDR,
    #[doc = "0xde9d8 - Device in Endpoint Transmit FIFO Status Register 1"]
    pub diep5_dtxfsts: DIEP5_DTXFSTS,
    _reserved47: [u8; 292usize],
    #[doc = "0xdeb00 - Device Control OUT Endpoint 0 Control Register"]
    pub doep0ctl: DOEP0CTL,
    _reserved48: [u8; 4usize],
    #[doc = "0xdeb08 - Device OUT Endpoint 0 Interrupt Register"]
    pub doep0int: DOEP0INT,
    _reserved49: [u8; 4usize],
    #[doc = "0xdeb10 - Device OUT Endpoint 0 Transfer Size Register"]
    pub doep0tsiz: DOEP0TSIZ,
    #[doc = "0xdeb14 - Device OUT Endpoint 0 DMA Address Register"]
    pub doep0dmaaddr: DOEP0DMAADDR,
    _reserved50: [u8; 8usize],
    #[doc = "0xdeb20 - Device Control OUT Endpoint 1 Control Register"]
    pub doep0_ctl: DOEP0_CTL,
    _reserved51: [u8; 4usize],
    #[doc = "0xdeb28 - Device OUT Endpoint 1 Interrupt Register"]
    pub doep0_int: DOEP0_INT,
    _reserved52: [u8; 4usize],
    #[doc = "0xdeb30 - Device OUT Endpoint 1 Transfer Size Register"]
    pub doep0_tsiz: DOEP0_TSIZ,
    #[doc = "0xdeb34 - Device OUT Endpoint 1 DMA Address Register"]
    pub doep0_dmaaddr: DOEP0_DMAADDR,
    _reserved53: [u8; 8usize],
    #[doc = "0xdeb40 - Device Control OUT Endpoint 1 Control Register"]
    pub doep1_ctl: DOEP1_CTL,
    _reserved54: [u8; 4usize],
    #[doc = "0xdeb48 - Device OUT Endpoint 1 Interrupt Register"]
    pub doep1_int: DOEP1_INT,
    _reserved55: [u8; 4usize],
    #[doc = "0xdeb50 - Device OUT Endpoint 1 Transfer Size Register"]
    pub doep1_tsiz: DOEP1_TSIZ,
    #[doc = "0xdeb54 - Device OUT Endpoint 1 DMA Address Register"]
    pub doep1_dmaaddr: DOEP1_DMAADDR,
    _reserved56: [u8; 8usize],
    #[doc = "0xdeb60 - Device Control OUT Endpoint 1 Control Register"]
    pub doep2_ctl: DOEP2_CTL,
    _reserved57: [u8; 4usize],
    #[doc = "0xdeb68 - Device OUT Endpoint 1 Interrupt Register"]
    pub doep2_int: DOEP2_INT,
    _reserved58: [u8; 4usize],
    #[doc = "0xdeb70 - Device OUT Endpoint 1 Transfer Size Register"]
    pub doep2_tsiz: DOEP2_TSIZ,
    #[doc = "0xdeb74 - Device OUT Endpoint 1 DMA Address Register"]
    pub doep2_dmaaddr: DOEP2_DMAADDR,
    _reserved59: [u8; 8usize],
    #[doc = "0xdeb80 - Device Control OUT Endpoint 1 Control Register"]
    pub doep3_ctl: DOEP3_CTL,
    _reserved60: [u8; 4usize],
    #[doc = "0xdeb88 - Device OUT Endpoint 1 Interrupt Register"]
    pub doep3_int: DOEP3_INT,
    _reserved61: [u8; 4usize],
    #[doc = "0xdeb90 - Device OUT Endpoint 1 Transfer Size Register"]
    pub doep3_tsiz: DOEP3_TSIZ,
    #[doc = "0xdeb94 - Device OUT Endpoint 1 DMA Address Register"]
    pub doep3_dmaaddr: DOEP3_DMAADDR,
    _reserved62: [u8; 8usize],
    #[doc = "0xdeba0 - Device Control OUT Endpoint 1 Control Register"]
    pub doep4_ctl: DOEP4_CTL,
    _reserved63: [u8; 4usize],
    #[doc = "0xdeba8 - Device OUT Endpoint 1 Interrupt Register"]
    pub doep4_int: DOEP4_INT,
    _reserved64: [u8; 4usize],
    #[doc = "0xdebb0 - Device OUT Endpoint 1 Transfer Size Register"]
    pub doep4_tsiz: DOEP4_TSIZ,
    #[doc = "0xdebb4 - Device OUT Endpoint 1 DMA Address Register"]
    pub doep4_dmaaddr: DOEP4_DMAADDR,
    _reserved65: [u8; 8usize],
    #[doc = "0xdebc0 - Device Control OUT Endpoint 1 Control Register"]
    pub doep5_ctl: DOEP5_CTL,
    _reserved66: [u8; 4usize],
    #[doc = "0xdebc8 - Device OUT Endpoint 1 Interrupt Register"]
    pub doep5_int: DOEP5_INT,
    _reserved67: [u8; 4usize],
    #[doc = "0xdebd0 - Device OUT Endpoint 1 Transfer Size Register"]
    pub doep5_tsiz: DOEP5_TSIZ,
    #[doc = "0xdebd4 - Device OUT Endpoint 1 DMA Address Register"]
    pub doep5_dmaaddr: DOEP5_DMAADDR,
    _reserved68: [u8; 552usize],
    #[doc = "0xdee00 - Power and Clock Gating Control Register"]
    pub pcgcctl: PCGCCTL,
}
#[doc = "System Control Register"]
pub struct CTRL {
    register: VolatileCell<u32>,
}
#[doc = "System Control Register"]
pub mod ctrl;
#[doc = "System Status Register"]
pub struct STATUS {
    register: VolatileCell<u32>,
}
#[doc = "System Status Register"]
pub mod status;
#[doc = "Interrupt Flag Register"]
pub struct IF {
    register: VolatileCell<u32>,
}
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "Interrupt Flag Set Register"]
pub struct IFS {
    register: VolatileCell<u32>,
}
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "Interrupt Flag Clear Register"]
pub struct IFC {
    register: VolatileCell<u32>,
}
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "Interrupt Enable Register"]
pub struct IEN {
    register: VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "I/O Routing Register"]
pub struct ROUTE {
    register: VolatileCell<u32>,
}
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "Charger Detect Configuration Register"]
pub struct CDCONF {
    register: VolatileCell<u32>,
}
#[doc = "Charger Detect Configuration Register"]
pub mod cdconf;
#[doc = "Command Register"]
pub struct CMD {
    register: VolatileCell<u32>,
}
#[doc = "Command Register"]
pub mod cmd;
#[doc = "USB LEM Control Register"]
pub struct LEMCTRL {
    register: VolatileCell<u32>,
}
#[doc = "USB LEM Control Register"]
pub mod lemctrl;
#[doc = "OTG Control and Status Register"]
pub struct GOTGCTL {
    register: VolatileCell<u32>,
}
#[doc = "OTG Control and Status Register"]
pub mod gotgctl;
#[doc = "OTG Interrupt Register"]
pub struct GOTGINT {
    register: VolatileCell<u32>,
}
#[doc = "OTG Interrupt Register"]
pub mod gotgint;
#[doc = "AHB Configuration Register"]
pub struct GAHBCFG {
    register: VolatileCell<u32>,
}
#[doc = "AHB Configuration Register"]
pub mod gahbcfg;
#[doc = "USB Configuration Register"]
pub struct GUSBCFG {
    register: VolatileCell<u32>,
}
#[doc = "USB Configuration Register"]
pub mod gusbcfg;
#[doc = "Reset Register"]
pub struct GRSTCTL {
    register: VolatileCell<u32>,
}
#[doc = "Reset Register"]
pub mod grstctl;
#[doc = "Interrupt Register"]
pub struct GINTSTS {
    register: VolatileCell<u32>,
}
#[doc = "Interrupt Register"]
pub mod gintsts;
#[doc = "Interrupt Mask Register"]
pub struct GINTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Interrupt Mask Register"]
pub mod gintmsk;
#[doc = "Receive Status Debug Read Register"]
pub struct GRXSTSR {
    register: VolatileCell<u32>,
}
#[doc = "Receive Status Debug Read Register"]
pub mod grxstsr;
#[doc = "Receive Status Read /Pop Register"]
pub struct GRXSTSP {
    register: VolatileCell<u32>,
}
#[doc = "Receive Status Read /Pop Register"]
pub mod grxstsp;
#[doc = "Receive FIFO Size Register"]
pub struct GRXFSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Receive FIFO Size Register"]
pub mod grxfsiz;
#[doc = "Non-periodic Transmit FIFO Size Register"]
pub struct GNPTXFSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Non-periodic Transmit FIFO Size Register"]
pub mod gnptxfsiz;
#[doc = "Non-periodic Transmit FIFO/Queue Status Register"]
pub struct GNPTXSTS {
    register: VolatileCell<u32>,
}
#[doc = "Non-periodic Transmit FIFO/Queue Status Register"]
pub mod gnptxsts;
#[doc = "Synopsys ID Register"]
pub struct GSNPSID {
    register: VolatileCell<u32>,
}
#[doc = "Synopsys ID Register"]
pub mod gsnpsid;
#[doc = "Global DFIFO Configuration Register"]
pub struct GDFIFOCFG {
    register: VolatileCell<u32>,
}
#[doc = "Global DFIFO Configuration Register"]
pub mod gdfifocfg;
#[doc = "Host Periodic Transmit FIFO Size Register"]
pub struct HPTXFSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Host Periodic Transmit FIFO Size Register"]
pub mod hptxfsiz;
#[doc = "Device in Endpoint Transmit FIFO Size Register 1"]
pub struct DIEPTXF1 {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint Transmit FIFO Size Register 1"]
pub mod dieptxf1;
#[doc = "Device in Endpoint Transmit FIFO Size Register 2"]
pub struct DIEPTXF2 {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint Transmit FIFO Size Register 2"]
pub mod dieptxf2;
#[doc = "Device in Endpoint Transmit FIFO Size Register 3"]
pub struct DIEPTXF3 {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint Transmit FIFO Size Register 3"]
pub mod dieptxf3;
#[doc = "Device in Endpoint Transmit FIFO Size Register 4"]
pub struct DIEPTXF4 {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint Transmit FIFO Size Register 4"]
pub mod dieptxf4;
#[doc = "Device in Endpoint Transmit FIFO Size Register 5"]
pub struct DIEPTXF5 {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint Transmit FIFO Size Register 5"]
pub mod dieptxf5;
#[doc = "Device in Endpoint Transmit FIFO Size Register 6"]
pub struct DIEPTXF6 {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint Transmit FIFO Size Register 6"]
pub mod dieptxf6;
#[doc = "Host Configuration Register"]
pub struct HCFG {
    register: VolatileCell<u32>,
}
#[doc = "Host Configuration Register"]
pub mod hcfg;
#[doc = "Host Frame Interval Register"]
pub struct HFIR {
    register: VolatileCell<u32>,
}
#[doc = "Host Frame Interval Register"]
pub mod hfir;
#[doc = "Host Frame Number/Frame Time Remaining Register"]
pub struct HFNUM {
    register: VolatileCell<u32>,
}
#[doc = "Host Frame Number/Frame Time Remaining Register"]
pub mod hfnum;
#[doc = "Host Periodic Transmit FIFO/Queue Status Register"]
pub struct HPTXSTS {
    register: VolatileCell<u32>,
}
#[doc = "Host Periodic Transmit FIFO/Queue Status Register"]
pub mod hptxsts;
#[doc = "Host All Channels Interrupt Register"]
pub struct HAINT {
    register: VolatileCell<u32>,
}
#[doc = "Host All Channels Interrupt Register"]
pub mod haint;
#[doc = "Host All Channels Interrupt Mask Register"]
pub struct HAINTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Host All Channels Interrupt Mask Register"]
pub mod haintmsk;
#[doc = "Host Port Control and Status Register"]
pub struct HPRT {
    register: VolatileCell<u32>,
}
#[doc = "Host Port Control and Status Register"]
pub mod hprt;
#[doc = "Host Channel 0 Characteristics Register"]
pub struct HC0_CHAR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Characteristics Register"]
pub mod hc0_char;
#[doc = "Host Channel 0 Split Control Register"]
pub struct HC0_SPLT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Split Control Register"]
pub mod hc0_splt;
#[doc = "Host Channel 0 Interrupt Register"]
pub struct HC0_INT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Interrupt Register"]
pub mod hc0_int;
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub struct HC0_INTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub mod hc0_intmsk;
#[doc = "Host Channel 0 Transfer Size Register"]
pub struct HC0_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Transfer Size Register"]
pub mod hc0_tsiz;
#[doc = "Host Channel 0 DMA Address Register"]
pub struct HC0_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 DMA Address Register"]
pub mod hc0_dmaaddr;
#[doc = "Host Channel 0 Characteristics Register"]
pub struct HC1_CHAR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Characteristics Register"]
pub mod hc1_char;
#[doc = "Host Channel 0 Split Control Register"]
pub struct HC1_SPLT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Split Control Register"]
pub mod hc1_splt;
#[doc = "Host Channel 0 Interrupt Register"]
pub struct HC1_INT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Interrupt Register"]
pub mod hc1_int;
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub struct HC1_INTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub mod hc1_intmsk;
#[doc = "Host Channel 0 Transfer Size Register"]
pub struct HC1_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Transfer Size Register"]
pub mod hc1_tsiz;
#[doc = "Host Channel 0 DMA Address Register"]
pub struct HC1_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 DMA Address Register"]
pub mod hc1_dmaaddr;
#[doc = "Host Channel 0 Characteristics Register"]
pub struct HC2_CHAR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Characteristics Register"]
pub mod hc2_char;
#[doc = "Host Channel 0 Split Control Register"]
pub struct HC2_SPLT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Split Control Register"]
pub mod hc2_splt;
#[doc = "Host Channel 0 Interrupt Register"]
pub struct HC2_INT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Interrupt Register"]
pub mod hc2_int;
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub struct HC2_INTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub mod hc2_intmsk;
#[doc = "Host Channel 0 Transfer Size Register"]
pub struct HC2_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Transfer Size Register"]
pub mod hc2_tsiz;
#[doc = "Host Channel 0 DMA Address Register"]
pub struct HC2_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 DMA Address Register"]
pub mod hc2_dmaaddr;
#[doc = "Host Channel 0 Characteristics Register"]
pub struct HC3_CHAR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Characteristics Register"]
pub mod hc3_char;
#[doc = "Host Channel 0 Split Control Register"]
pub struct HC3_SPLT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Split Control Register"]
pub mod hc3_splt;
#[doc = "Host Channel 0 Interrupt Register"]
pub struct HC3_INT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Interrupt Register"]
pub mod hc3_int;
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub struct HC3_INTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub mod hc3_intmsk;
#[doc = "Host Channel 0 Transfer Size Register"]
pub struct HC3_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Transfer Size Register"]
pub mod hc3_tsiz;
#[doc = "Host Channel 0 DMA Address Register"]
pub struct HC3_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 DMA Address Register"]
pub mod hc3_dmaaddr;
#[doc = "Host Channel 0 Characteristics Register"]
pub struct HC4_CHAR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Characteristics Register"]
pub mod hc4_char;
#[doc = "Host Channel 0 Split Control Register"]
pub struct HC4_SPLT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Split Control Register"]
pub mod hc4_splt;
#[doc = "Host Channel 0 Interrupt Register"]
pub struct HC4_INT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Interrupt Register"]
pub mod hc4_int;
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub struct HC4_INTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub mod hc4_intmsk;
#[doc = "Host Channel 0 Transfer Size Register"]
pub struct HC4_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Transfer Size Register"]
pub mod hc4_tsiz;
#[doc = "Host Channel 0 DMA Address Register"]
pub struct HC4_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 DMA Address Register"]
pub mod hc4_dmaaddr;
#[doc = "Host Channel 0 Characteristics Register"]
pub struct HC5_CHAR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Characteristics Register"]
pub mod hc5_char;
#[doc = "Host Channel 0 Split Control Register"]
pub struct HC5_SPLT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Split Control Register"]
pub mod hc5_splt;
#[doc = "Host Channel 0 Interrupt Register"]
pub struct HC5_INT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Interrupt Register"]
pub mod hc5_int;
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub struct HC5_INTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub mod hc5_intmsk;
#[doc = "Host Channel 0 Transfer Size Register"]
pub struct HC5_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Transfer Size Register"]
pub mod hc5_tsiz;
#[doc = "Host Channel 0 DMA Address Register"]
pub struct HC5_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 DMA Address Register"]
pub mod hc5_dmaaddr;
#[doc = "Host Channel 0 Characteristics Register"]
pub struct HC6_CHAR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Characteristics Register"]
pub mod hc6_char;
#[doc = "Host Channel 0 Split Control Register"]
pub struct HC6_SPLT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Split Control Register"]
pub mod hc6_splt;
#[doc = "Host Channel 0 Interrupt Register"]
pub struct HC6_INT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Interrupt Register"]
pub mod hc6_int;
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub struct HC6_INTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub mod hc6_intmsk;
#[doc = "Host Channel 0 Transfer Size Register"]
pub struct HC6_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Transfer Size Register"]
pub mod hc6_tsiz;
#[doc = "Host Channel 0 DMA Address Register"]
pub struct HC6_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 DMA Address Register"]
pub mod hc6_dmaaddr;
#[doc = "Host Channel 0 Characteristics Register"]
pub struct HC7_CHAR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Characteristics Register"]
pub mod hc7_char;
#[doc = "Host Channel 0 Split Control Register"]
pub struct HC7_SPLT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Split Control Register"]
pub mod hc7_splt;
#[doc = "Host Channel 0 Interrupt Register"]
pub struct HC7_INT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Interrupt Register"]
pub mod hc7_int;
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub struct HC7_INTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub mod hc7_intmsk;
#[doc = "Host Channel 0 Transfer Size Register"]
pub struct HC7_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Transfer Size Register"]
pub mod hc7_tsiz;
#[doc = "Host Channel 0 DMA Address Register"]
pub struct HC7_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 DMA Address Register"]
pub mod hc7_dmaaddr;
#[doc = "Host Channel 0 Characteristics Register"]
pub struct HC8_CHAR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Characteristics Register"]
pub mod hc8_char;
#[doc = "Host Channel 0 Split Control Register"]
pub struct HC8_SPLT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Split Control Register"]
pub mod hc8_splt;
#[doc = "Host Channel 0 Interrupt Register"]
pub struct HC8_INT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Interrupt Register"]
pub mod hc8_int;
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub struct HC8_INTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub mod hc8_intmsk;
#[doc = "Host Channel 0 Transfer Size Register"]
pub struct HC8_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Transfer Size Register"]
pub mod hc8_tsiz;
#[doc = "Host Channel 0 DMA Address Register"]
pub struct HC8_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 DMA Address Register"]
pub mod hc8_dmaaddr;
#[doc = "Host Channel 0 Characteristics Register"]
pub struct HC9_CHAR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Characteristics Register"]
pub mod hc9_char;
#[doc = "Host Channel 0 Split Control Register"]
pub struct HC9_SPLT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Split Control Register"]
pub mod hc9_splt;
#[doc = "Host Channel 0 Interrupt Register"]
pub struct HC9_INT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Interrupt Register"]
pub mod hc9_int;
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub struct HC9_INTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub mod hc9_intmsk;
#[doc = "Host Channel 0 Transfer Size Register"]
pub struct HC9_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Transfer Size Register"]
pub mod hc9_tsiz;
#[doc = "Host Channel 0 DMA Address Register"]
pub struct HC9_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 DMA Address Register"]
pub mod hc9_dmaaddr;
#[doc = "Host Channel 0 Characteristics Register"]
pub struct HC10_CHAR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Characteristics Register"]
pub mod hc10_char;
#[doc = "Host Channel 0 Split Control Register"]
pub struct HC10_SPLT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Split Control Register"]
pub mod hc10_splt;
#[doc = "Host Channel 0 Interrupt Register"]
pub struct HC10_INT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Interrupt Register"]
pub mod hc10_int;
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub struct HC10_INTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub mod hc10_intmsk;
#[doc = "Host Channel 0 Transfer Size Register"]
pub struct HC10_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Transfer Size Register"]
pub mod hc10_tsiz;
#[doc = "Host Channel 0 DMA Address Register"]
pub struct HC10_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 DMA Address Register"]
pub mod hc10_dmaaddr;
#[doc = "Host Channel 0 Characteristics Register"]
pub struct HC11_CHAR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Characteristics Register"]
pub mod hc11_char;
#[doc = "Host Channel 0 Split Control Register"]
pub struct HC11_SPLT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Split Control Register"]
pub mod hc11_splt;
#[doc = "Host Channel 0 Interrupt Register"]
pub struct HC11_INT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Interrupt Register"]
pub mod hc11_int;
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub struct HC11_INTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub mod hc11_intmsk;
#[doc = "Host Channel 0 Transfer Size Register"]
pub struct HC11_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Transfer Size Register"]
pub mod hc11_tsiz;
#[doc = "Host Channel 0 DMA Address Register"]
pub struct HC11_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 DMA Address Register"]
pub mod hc11_dmaaddr;
#[doc = "Host Channel 0 Characteristics Register"]
pub struct HC12_CHAR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Characteristics Register"]
pub mod hc12_char;
#[doc = "Host Channel 0 Split Control Register"]
pub struct HC12_SPLT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Split Control Register"]
pub mod hc12_splt;
#[doc = "Host Channel 0 Interrupt Register"]
pub struct HC12_INT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Interrupt Register"]
pub mod hc12_int;
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub struct HC12_INTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub mod hc12_intmsk;
#[doc = "Host Channel 0 Transfer Size Register"]
pub struct HC12_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Transfer Size Register"]
pub mod hc12_tsiz;
#[doc = "Host Channel 0 DMA Address Register"]
pub struct HC12_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 DMA Address Register"]
pub mod hc12_dmaaddr;
#[doc = "Host Channel 0 Characteristics Register"]
pub struct HC13_CHAR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Characteristics Register"]
pub mod hc13_char;
#[doc = "Host Channel 0 Split Control Register"]
pub struct HC13_SPLT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Split Control Register"]
pub mod hc13_splt;
#[doc = "Host Channel 0 Interrupt Register"]
pub struct HC13_INT {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Interrupt Register"]
pub mod hc13_int;
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub struct HC13_INTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub mod hc13_intmsk;
#[doc = "Host Channel 0 Transfer Size Register"]
pub struct HC13_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 Transfer Size Register"]
pub mod hc13_tsiz;
#[doc = "Host Channel 0 DMA Address Register"]
pub struct HC13_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Host Channel 0 DMA Address Register"]
pub mod hc13_dmaaddr;
#[doc = "Device Configuration Register"]
pub struct DCFG {
    register: VolatileCell<u32>,
}
#[doc = "Device Configuration Register"]
pub mod dcfg;
#[doc = "Device Control Register"]
pub struct DCTL {
    register: VolatileCell<u32>,
}
#[doc = "Device Control Register"]
pub mod dctl;
#[doc = "Device Status Register"]
pub struct DSTS {
    register: VolatileCell<u32>,
}
#[doc = "Device Status Register"]
pub mod dsts;
#[doc = "Device in Endpoint Common Interrupt Mask Register"]
pub struct DIEPMSK {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint Common Interrupt Mask Register"]
pub mod diepmsk;
#[doc = "Device OUT Endpoint Common Interrupt Mask Register"]
pub struct DOEPMSK {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint Common Interrupt Mask Register"]
pub mod doepmsk;
#[doc = "Device All Endpoints Interrupt Register"]
pub struct DAINT {
    register: VolatileCell<u32>,
}
#[doc = "Device All Endpoints Interrupt Register"]
pub mod daint;
#[doc = "Device All Endpoints Interrupt Mask Register"]
pub struct DAINTMSK {
    register: VolatileCell<u32>,
}
#[doc = "Device All Endpoints Interrupt Mask Register"]
pub mod daintmsk;
#[doc = "Device VBUS Discharge Time Register"]
pub struct DVBUSDIS {
    register: VolatileCell<u32>,
}
#[doc = "Device VBUS Discharge Time Register"]
pub mod dvbusdis;
#[doc = "Device VBUS Pulsing Time Register"]
pub struct DVBUSPULSE {
    register: VolatileCell<u32>,
}
#[doc = "Device VBUS Pulsing Time Register"]
pub mod dvbuspulse;
#[doc = "Device Threshold Control Register"]
pub struct DTHRCTL {
    register: VolatileCell<u32>,
}
#[doc = "Device Threshold Control Register"]
pub mod dthrctl;
#[doc = "Device in Endpoint FIFO Empty Interrupt Mask Register"]
pub struct DIEPEMPMSK {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint FIFO Empty Interrupt Mask Register"]
pub mod diepempmsk;
#[doc = "Device Control in Endpoint 0 Control Register"]
pub struct DIEP0CTL {
    register: VolatileCell<u32>,
}
#[doc = "Device Control in Endpoint 0 Control Register"]
pub mod diep0ctl;
#[doc = "Device in Endpoint 0 Interrupt Register"]
pub struct DIEP0INT {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint 0 Interrupt Register"]
pub mod diep0int;
#[doc = "Device in Endpoint 0 Transfer Size Register"]
pub struct DIEP0TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint 0 Transfer Size Register"]
pub mod diep0tsiz;
#[doc = "Device in Endpoint 0 DMA Address Register"]
pub struct DIEP0DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint 0 DMA Address Register"]
pub mod diep0dmaaddr;
#[doc = "Device in Endpoint Transmit FIFO Status Register 0"]
pub struct DIEP0TXFSTS {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint Transmit FIFO Status Register 0"]
pub mod diep0txfsts;
#[doc = "Device Control in Endpoint 1 Control Register"]
pub struct DIEP0_CTL {
    register: VolatileCell<u32>,
}
#[doc = "Device Control in Endpoint 1 Control Register"]
pub mod diep0_ctl;
#[doc = "Device in Endpoint 1 Interrupt Register"]
pub struct DIEP0_INT {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint 1 Interrupt Register"]
pub mod diep0_int;
#[doc = "Device in Endpoint 1 Transfer Size Register"]
pub struct DIEP0_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint 1 Transfer Size Register"]
pub mod diep0_tsiz;
#[doc = "Device in Endpoint 1 DMA Address Register"]
pub struct DIEP0_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint 1 DMA Address Register"]
pub mod diep0_dmaaddr;
#[doc = "Device in Endpoint Transmit FIFO Status Register 1"]
pub struct DIEP0_DTXFSTS {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint Transmit FIFO Status Register 1"]
pub mod diep0_dtxfsts;
#[doc = "Device Control in Endpoint 1 Control Register"]
pub struct DIEP1_CTL {
    register: VolatileCell<u32>,
}
#[doc = "Device Control in Endpoint 1 Control Register"]
pub mod diep1_ctl;
#[doc = "Device in Endpoint 1 Interrupt Register"]
pub struct DIEP1_INT {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint 1 Interrupt Register"]
pub mod diep1_int;
#[doc = "Device in Endpoint 1 Transfer Size Register"]
pub struct DIEP1_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint 1 Transfer Size Register"]
pub mod diep1_tsiz;
#[doc = "Device in Endpoint 1 DMA Address Register"]
pub struct DIEP1_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint 1 DMA Address Register"]
pub mod diep1_dmaaddr;
#[doc = "Device in Endpoint Transmit FIFO Status Register 1"]
pub struct DIEP1_DTXFSTS {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint Transmit FIFO Status Register 1"]
pub mod diep1_dtxfsts;
#[doc = "Device Control in Endpoint 1 Control Register"]
pub struct DIEP2_CTL {
    register: VolatileCell<u32>,
}
#[doc = "Device Control in Endpoint 1 Control Register"]
pub mod diep2_ctl;
#[doc = "Device in Endpoint 1 Interrupt Register"]
pub struct DIEP2_INT {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint 1 Interrupt Register"]
pub mod diep2_int;
#[doc = "Device in Endpoint 1 Transfer Size Register"]
pub struct DIEP2_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint 1 Transfer Size Register"]
pub mod diep2_tsiz;
#[doc = "Device in Endpoint 1 DMA Address Register"]
pub struct DIEP2_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint 1 DMA Address Register"]
pub mod diep2_dmaaddr;
#[doc = "Device in Endpoint Transmit FIFO Status Register 1"]
pub struct DIEP2_DTXFSTS {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint Transmit FIFO Status Register 1"]
pub mod diep2_dtxfsts;
#[doc = "Device Control in Endpoint 1 Control Register"]
pub struct DIEP3_CTL {
    register: VolatileCell<u32>,
}
#[doc = "Device Control in Endpoint 1 Control Register"]
pub mod diep3_ctl;
#[doc = "Device in Endpoint 1 Interrupt Register"]
pub struct DIEP3_INT {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint 1 Interrupt Register"]
pub mod diep3_int;
#[doc = "Device in Endpoint 1 Transfer Size Register"]
pub struct DIEP3_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint 1 Transfer Size Register"]
pub mod diep3_tsiz;
#[doc = "Device in Endpoint 1 DMA Address Register"]
pub struct DIEP3_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint 1 DMA Address Register"]
pub mod diep3_dmaaddr;
#[doc = "Device in Endpoint Transmit FIFO Status Register 1"]
pub struct DIEP3_DTXFSTS {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint Transmit FIFO Status Register 1"]
pub mod diep3_dtxfsts;
#[doc = "Device Control in Endpoint 1 Control Register"]
pub struct DIEP4_CTL {
    register: VolatileCell<u32>,
}
#[doc = "Device Control in Endpoint 1 Control Register"]
pub mod diep4_ctl;
#[doc = "Device in Endpoint 1 Interrupt Register"]
pub struct DIEP4_INT {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint 1 Interrupt Register"]
pub mod diep4_int;
#[doc = "Device in Endpoint 1 Transfer Size Register"]
pub struct DIEP4_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint 1 Transfer Size Register"]
pub mod diep4_tsiz;
#[doc = "Device in Endpoint 1 DMA Address Register"]
pub struct DIEP4_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint 1 DMA Address Register"]
pub mod diep4_dmaaddr;
#[doc = "Device in Endpoint Transmit FIFO Status Register 1"]
pub struct DIEP4_DTXFSTS {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint Transmit FIFO Status Register 1"]
pub mod diep4_dtxfsts;
#[doc = "Device Control in Endpoint 1 Control Register"]
pub struct DIEP5_CTL {
    register: VolatileCell<u32>,
}
#[doc = "Device Control in Endpoint 1 Control Register"]
pub mod diep5_ctl;
#[doc = "Device in Endpoint 1 Interrupt Register"]
pub struct DIEP5_INT {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint 1 Interrupt Register"]
pub mod diep5_int;
#[doc = "Device in Endpoint 1 Transfer Size Register"]
pub struct DIEP5_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint 1 Transfer Size Register"]
pub mod diep5_tsiz;
#[doc = "Device in Endpoint 1 DMA Address Register"]
pub struct DIEP5_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint 1 DMA Address Register"]
pub mod diep5_dmaaddr;
#[doc = "Device in Endpoint Transmit FIFO Status Register 1"]
pub struct DIEP5_DTXFSTS {
    register: VolatileCell<u32>,
}
#[doc = "Device in Endpoint Transmit FIFO Status Register 1"]
pub mod diep5_dtxfsts;
#[doc = "Device Control OUT Endpoint 0 Control Register"]
pub struct DOEP0CTL {
    register: VolatileCell<u32>,
}
#[doc = "Device Control OUT Endpoint 0 Control Register"]
pub mod doep0ctl;
#[doc = "Device OUT Endpoint 0 Interrupt Register"]
pub struct DOEP0INT {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint 0 Interrupt Register"]
pub mod doep0int;
#[doc = "Device OUT Endpoint 0 Transfer Size Register"]
pub struct DOEP0TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint 0 Transfer Size Register"]
pub mod doep0tsiz;
#[doc = "Device OUT Endpoint 0 DMA Address Register"]
pub struct DOEP0DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint 0 DMA Address Register"]
pub mod doep0dmaaddr;
#[doc = "Device Control OUT Endpoint 1 Control Register"]
pub struct DOEP0_CTL {
    register: VolatileCell<u32>,
}
#[doc = "Device Control OUT Endpoint 1 Control Register"]
pub mod doep0_ctl;
#[doc = "Device OUT Endpoint 1 Interrupt Register"]
pub struct DOEP0_INT {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint 1 Interrupt Register"]
pub mod doep0_int;
#[doc = "Device OUT Endpoint 1 Transfer Size Register"]
pub struct DOEP0_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint 1 Transfer Size Register"]
pub mod doep0_tsiz;
#[doc = "Device OUT Endpoint 1 DMA Address Register"]
pub struct DOEP0_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint 1 DMA Address Register"]
pub mod doep0_dmaaddr;
#[doc = "Device Control OUT Endpoint 1 Control Register"]
pub struct DOEP1_CTL {
    register: VolatileCell<u32>,
}
#[doc = "Device Control OUT Endpoint 1 Control Register"]
pub mod doep1_ctl;
#[doc = "Device OUT Endpoint 1 Interrupt Register"]
pub struct DOEP1_INT {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint 1 Interrupt Register"]
pub mod doep1_int;
#[doc = "Device OUT Endpoint 1 Transfer Size Register"]
pub struct DOEP1_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint 1 Transfer Size Register"]
pub mod doep1_tsiz;
#[doc = "Device OUT Endpoint 1 DMA Address Register"]
pub struct DOEP1_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint 1 DMA Address Register"]
pub mod doep1_dmaaddr;
#[doc = "Device Control OUT Endpoint 1 Control Register"]
pub struct DOEP2_CTL {
    register: VolatileCell<u32>,
}
#[doc = "Device Control OUT Endpoint 1 Control Register"]
pub mod doep2_ctl;
#[doc = "Device OUT Endpoint 1 Interrupt Register"]
pub struct DOEP2_INT {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint 1 Interrupt Register"]
pub mod doep2_int;
#[doc = "Device OUT Endpoint 1 Transfer Size Register"]
pub struct DOEP2_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint 1 Transfer Size Register"]
pub mod doep2_tsiz;
#[doc = "Device OUT Endpoint 1 DMA Address Register"]
pub struct DOEP2_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint 1 DMA Address Register"]
pub mod doep2_dmaaddr;
#[doc = "Device Control OUT Endpoint 1 Control Register"]
pub struct DOEP3_CTL {
    register: VolatileCell<u32>,
}
#[doc = "Device Control OUT Endpoint 1 Control Register"]
pub mod doep3_ctl;
#[doc = "Device OUT Endpoint 1 Interrupt Register"]
pub struct DOEP3_INT {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint 1 Interrupt Register"]
pub mod doep3_int;
#[doc = "Device OUT Endpoint 1 Transfer Size Register"]
pub struct DOEP3_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint 1 Transfer Size Register"]
pub mod doep3_tsiz;
#[doc = "Device OUT Endpoint 1 DMA Address Register"]
pub struct DOEP3_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint 1 DMA Address Register"]
pub mod doep3_dmaaddr;
#[doc = "Device Control OUT Endpoint 1 Control Register"]
pub struct DOEP4_CTL {
    register: VolatileCell<u32>,
}
#[doc = "Device Control OUT Endpoint 1 Control Register"]
pub mod doep4_ctl;
#[doc = "Device OUT Endpoint 1 Interrupt Register"]
pub struct DOEP4_INT {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint 1 Interrupt Register"]
pub mod doep4_int;
#[doc = "Device OUT Endpoint 1 Transfer Size Register"]
pub struct DOEP4_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint 1 Transfer Size Register"]
pub mod doep4_tsiz;
#[doc = "Device OUT Endpoint 1 DMA Address Register"]
pub struct DOEP4_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint 1 DMA Address Register"]
pub mod doep4_dmaaddr;
#[doc = "Device Control OUT Endpoint 1 Control Register"]
pub struct DOEP5_CTL {
    register: VolatileCell<u32>,
}
#[doc = "Device Control OUT Endpoint 1 Control Register"]
pub mod doep5_ctl;
#[doc = "Device OUT Endpoint 1 Interrupt Register"]
pub struct DOEP5_INT {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint 1 Interrupt Register"]
pub mod doep5_int;
#[doc = "Device OUT Endpoint 1 Transfer Size Register"]
pub struct DOEP5_TSIZ {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint 1 Transfer Size Register"]
pub mod doep5_tsiz;
#[doc = "Device OUT Endpoint 1 DMA Address Register"]
pub struct DOEP5_DMAADDR {
    register: VolatileCell<u32>,
}
#[doc = "Device OUT Endpoint 1 DMA Address Register"]
pub mod doep5_dmaaddr;
#[doc = "Power and Clock Gating Control Register"]
pub struct PCGCCTL {
    register: VolatileCell<u32>,
}
#[doc = "Power and Clock Gating Control Register"]
pub mod pcgcctl;
