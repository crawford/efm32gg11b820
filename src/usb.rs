#[doc = r"Register block"]
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
    _reserved7: [u8; 16usize],
    #[doc = "0x2c - Charger Detect Configuration Register"]
    pub cdconf: CDCONF,
    #[doc = "0x30 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x34 - Data TRIM 1 Values for USB DP and DM"]
    pub dattrim1: DATTRIM1,
    _reserved10: [u8; 4usize],
    #[doc = "0x3c - USB LEM Control Register"]
    pub lemctrl: LEMCTRL,
    _reserved11: [u8; 909248usize],
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
    _reserved23: [u8; 16usize],
    #[doc = "0xde040 - Synopsys ID Register"]
    pub gsnpsid: GSNPSID,
    _reserved24: [u8; 24usize],
    #[doc = "0xde05c - Global DFIFO Configuration Register"]
    pub gdfifocfg: GDFIFOCFG,
    _reserved25: [u8; 160usize],
    #[doc = "0xde100 - Host Periodic Transmit FIFO Size Register"]
    pub hptxfsiz: HPTXFSIZ,
    #[doc = "0xde104 - Device IN Endpoint Transmit FIFO Size Register 1"]
    pub dieptxf1: DIEPTXF1,
    #[doc = "0xde108 - Device IN Endpoint Transmit FIFO Size Register 2"]
    pub dieptxf2: DIEPTXF2,
    #[doc = "0xde10c - Device IN Endpoint Transmit FIFO Size Register 3"]
    pub dieptxf3: DIEPTXF3,
    #[doc = "0xde110 - Device IN Endpoint Transmit FIFO Size Register 4"]
    pub dieptxf4: DIEPTXF4,
    #[doc = "0xde114 - Device IN Endpoint Transmit FIFO Size Register 5"]
    pub dieptxf5: DIEPTXF5,
    #[doc = "0xde118 - Device IN Endpoint Transmit FIFO Size Register 6"]
    pub dieptxf6: DIEPTXF6,
    _reserved32: [u8; 740usize],
    #[doc = "0xde400 - Host Configuration Register"]
    pub hcfg: HCFG,
    #[doc = "0xde404 - Host Frame Interval Register"]
    pub hfir: HFIR,
    #[doc = "0xde408 - Host Frame Number/Frame Time Remaining Register"]
    pub hfnum: HFNUM,
    _reserved35: [u8; 4usize],
    #[doc = "0xde410 - Host Periodic Transmit FIFO/Queue Status Register"]
    pub hptxsts: HPTXSTS,
    #[doc = "0xde414 - Host All Channels Interrupt Register"]
    pub haint: HAINT,
    #[doc = "0xde418 - Host All Channels Interrupt Mask Register"]
    pub haintmsk: HAINTMSK,
    _reserved38: [u8; 36usize],
    #[doc = "0xde440 - Host Port Control and Status Register"]
    pub hprt: HPRT,
    _reserved39: [u8; 188usize],
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
    _reserved45: [u8; 8usize],
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
    _reserved51: [u8; 8usize],
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
    _reserved57: [u8; 8usize],
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
    _reserved63: [u8; 8usize],
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
    _reserved69: [u8; 8usize],
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
    _reserved75: [u8; 8usize],
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
    _reserved81: [u8; 8usize],
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
    _reserved87: [u8; 8usize],
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
    _reserved93: [u8; 8usize],
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
    _reserved99: [u8; 8usize],
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
    _reserved105: [u8; 8usize],
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
    _reserved111: [u8; 8usize],
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
    _reserved117: [u8; 8usize],
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
    _reserved123: [u8; 328usize],
    #[doc = "0xde800 - Device Configuration Register"]
    pub dcfg: DCFG,
    #[doc = "0xde804 - Device Control Register"]
    pub dctl: DCTL,
    #[doc = "0xde808 - Device Status Register"]
    pub dsts: DSTS,
    _reserved126: [u8; 4usize],
    #[doc = "0xde810 - Device IN Endpoint Common Interrupt Mask Register"]
    pub diepmsk: DIEPMSK,
    #[doc = "0xde814 - Device OUT Endpoint Common Interrupt Mask Register"]
    pub doepmsk: DOEPMSK,
    #[doc = "0xde818 - Device All Endpoints Interrupt Register"]
    pub daint: DAINT,
    #[doc = "0xde81c - Device All Endpoints Interrupt Mask Register"]
    pub daintmsk: DAINTMSK,
    _reserved130: [u8; 8usize],
    #[doc = "0xde828 - Device VBUS Discharge Time Register"]
    pub dvbusdis: DVBUSDIS,
    #[doc = "0xde82c - Device VBUS Pulsing Time Register"]
    pub dvbuspulse: DVBUSPULSE,
    #[doc = "0xde830 - Device Threshold Control Register"]
    pub dthrctl: DTHRCTL,
    #[doc = "0xde834 - Device IN Endpoint FIFO Empty Interrupt Mask Register"]
    pub diepempmsk: DIEPEMPMSK,
    _reserved134: [u8; 200usize],
    #[doc = "0xde900 - Device Control IN Endpoint 0 Control Register"]
    pub diep0ctl: DIEP0CTL,
    _reserved135: [u8; 4usize],
    #[doc = "0xde908 - Device IN Endpoint 0 Interrupt Register"]
    pub diep0int: DIEP0INT,
    _reserved136: [u8; 4usize],
    #[doc = "0xde910 - Device IN Endpoint 0 Transfer Size Register"]
    pub diep0tsiz: DIEP0TSIZ,
    #[doc = "0xde914 - Device IN Endpoint 0 DMA Address Register"]
    pub diep0dmaaddr: DIEP0DMAADDR,
    #[doc = "0xde918 - Device IN Endpoint Transmit FIFO Status Register 0"]
    pub diep0txfsts: DIEP0TXFSTS,
    _reserved139: [u8; 4usize],
    #[doc = "0xde920 - Device Control IN Endpoint 1 Control Register"]
    pub diep0_ctl: DIEP0_CTL,
    _reserved140: [u8; 4usize],
    #[doc = "0xde928 - Device IN Endpoint 1 Interrupt Register"]
    pub diep0_int: DIEP0_INT,
    _reserved141: [u8; 4usize],
    #[doc = "0xde930 - Device IN Endpoint 1 Transfer Size Register"]
    pub diep0_tsiz: DIEP0_TSIZ,
    #[doc = "0xde934 - Device IN Endpoint 1 DMA Address Register"]
    pub diep0_dmaaddr: DIEP0_DMAADDR,
    #[doc = "0xde938 - Device IN Endpoint Transmit FIFO Status Register 1"]
    pub diep0_dtxfsts: DIEP0_DTXFSTS,
    _reserved144: [u8; 4usize],
    #[doc = "0xde940 - Device Control IN Endpoint 1 Control Register"]
    pub diep1_ctl: DIEP1_CTL,
    _reserved145: [u8; 4usize],
    #[doc = "0xde948 - Device IN Endpoint 1 Interrupt Register"]
    pub diep1_int: DIEP1_INT,
    _reserved146: [u8; 4usize],
    #[doc = "0xde950 - Device IN Endpoint 1 Transfer Size Register"]
    pub diep1_tsiz: DIEP1_TSIZ,
    #[doc = "0xde954 - Device IN Endpoint 1 DMA Address Register"]
    pub diep1_dmaaddr: DIEP1_DMAADDR,
    #[doc = "0xde958 - Device IN Endpoint Transmit FIFO Status Register 1"]
    pub diep1_dtxfsts: DIEP1_DTXFSTS,
    _reserved149: [u8; 4usize],
    #[doc = "0xde960 - Device Control IN Endpoint 1 Control Register"]
    pub diep2_ctl: DIEP2_CTL,
    _reserved150: [u8; 4usize],
    #[doc = "0xde968 - Device IN Endpoint 1 Interrupt Register"]
    pub diep2_int: DIEP2_INT,
    _reserved151: [u8; 4usize],
    #[doc = "0xde970 - Device IN Endpoint 1 Transfer Size Register"]
    pub diep2_tsiz: DIEP2_TSIZ,
    #[doc = "0xde974 - Device IN Endpoint 1 DMA Address Register"]
    pub diep2_dmaaddr: DIEP2_DMAADDR,
    #[doc = "0xde978 - Device IN Endpoint Transmit FIFO Status Register 1"]
    pub diep2_dtxfsts: DIEP2_DTXFSTS,
    _reserved154: [u8; 4usize],
    #[doc = "0xde980 - Device Control IN Endpoint 1 Control Register"]
    pub diep3_ctl: DIEP3_CTL,
    _reserved155: [u8; 4usize],
    #[doc = "0xde988 - Device IN Endpoint 1 Interrupt Register"]
    pub diep3_int: DIEP3_INT,
    _reserved156: [u8; 4usize],
    #[doc = "0xde990 - Device IN Endpoint 1 Transfer Size Register"]
    pub diep3_tsiz: DIEP3_TSIZ,
    #[doc = "0xde994 - Device IN Endpoint 1 DMA Address Register"]
    pub diep3_dmaaddr: DIEP3_DMAADDR,
    #[doc = "0xde998 - Device IN Endpoint Transmit FIFO Status Register 1"]
    pub diep3_dtxfsts: DIEP3_DTXFSTS,
    _reserved159: [u8; 4usize],
    #[doc = "0xde9a0 - Device Control IN Endpoint 1 Control Register"]
    pub diep4_ctl: DIEP4_CTL,
    _reserved160: [u8; 4usize],
    #[doc = "0xde9a8 - Device IN Endpoint 1 Interrupt Register"]
    pub diep4_int: DIEP4_INT,
    _reserved161: [u8; 4usize],
    #[doc = "0xde9b0 - Device IN Endpoint 1 Transfer Size Register"]
    pub diep4_tsiz: DIEP4_TSIZ,
    #[doc = "0xde9b4 - Device IN Endpoint 1 DMA Address Register"]
    pub diep4_dmaaddr: DIEP4_DMAADDR,
    #[doc = "0xde9b8 - Device IN Endpoint Transmit FIFO Status Register 1"]
    pub diep4_dtxfsts: DIEP4_DTXFSTS,
    _reserved164: [u8; 4usize],
    #[doc = "0xde9c0 - Device Control IN Endpoint 1 Control Register"]
    pub diep5_ctl: DIEP5_CTL,
    _reserved165: [u8; 4usize],
    #[doc = "0xde9c8 - Device IN Endpoint 1 Interrupt Register"]
    pub diep5_int: DIEP5_INT,
    _reserved166: [u8; 4usize],
    #[doc = "0xde9d0 - Device IN Endpoint 1 Transfer Size Register"]
    pub diep5_tsiz: DIEP5_TSIZ,
    #[doc = "0xde9d4 - Device IN Endpoint 1 DMA Address Register"]
    pub diep5_dmaaddr: DIEP5_DMAADDR,
    #[doc = "0xde9d8 - Device IN Endpoint Transmit FIFO Status Register 1"]
    pub diep5_dtxfsts: DIEP5_DTXFSTS,
    _reserved169: [u8; 292usize],
    #[doc = "0xdeb00 - Device Control OUT Endpoint 0 Control Register"]
    pub doep0ctl: DOEP0CTL,
    _reserved170: [u8; 4usize],
    #[doc = "0xdeb08 - Device OUT Endpoint 0 Interrupt Register"]
    pub doep0int: DOEP0INT,
    _reserved171: [u8; 4usize],
    #[doc = "0xdeb10 - Device OUT Endpoint 0 Transfer Size Register"]
    pub doep0tsiz: DOEP0TSIZ,
    #[doc = "0xdeb14 - Device OUT Endpoint 0 DMA Address Register"]
    pub doep0dmaaddr: DOEP0DMAADDR,
    _reserved173: [u8; 8usize],
    #[doc = "0xdeb20 - Device Control OUT Endpoint 1 Control Register"]
    pub doep0_ctl: DOEP0_CTL,
    _reserved174: [u8; 4usize],
    #[doc = "0xdeb28 - Device OUT Endpoint 1 Interrupt Register"]
    pub doep0_int: DOEP0_INT,
    _reserved175: [u8; 4usize],
    #[doc = "0xdeb30 - Device OUT Endpoint 1 Transfer Size Register"]
    pub doep0_tsiz: DOEP0_TSIZ,
    #[doc = "0xdeb34 - Device OUT Endpoint 1 DMA Address Register"]
    pub doep0_dmaaddr: DOEP0_DMAADDR,
    _reserved177: [u8; 8usize],
    #[doc = "0xdeb40 - Device Control OUT Endpoint 1 Control Register"]
    pub doep1_ctl: DOEP1_CTL,
    _reserved178: [u8; 4usize],
    #[doc = "0xdeb48 - Device OUT Endpoint 1 Interrupt Register"]
    pub doep1_int: DOEP1_INT,
    _reserved179: [u8; 4usize],
    #[doc = "0xdeb50 - Device OUT Endpoint 1 Transfer Size Register"]
    pub doep1_tsiz: DOEP1_TSIZ,
    #[doc = "0xdeb54 - Device OUT Endpoint 1 DMA Address Register"]
    pub doep1_dmaaddr: DOEP1_DMAADDR,
    _reserved181: [u8; 8usize],
    #[doc = "0xdeb60 - Device Control OUT Endpoint 1 Control Register"]
    pub doep2_ctl: DOEP2_CTL,
    _reserved182: [u8; 4usize],
    #[doc = "0xdeb68 - Device OUT Endpoint 1 Interrupt Register"]
    pub doep2_int: DOEP2_INT,
    _reserved183: [u8; 4usize],
    #[doc = "0xdeb70 - Device OUT Endpoint 1 Transfer Size Register"]
    pub doep2_tsiz: DOEP2_TSIZ,
    #[doc = "0xdeb74 - Device OUT Endpoint 1 DMA Address Register"]
    pub doep2_dmaaddr: DOEP2_DMAADDR,
    _reserved185: [u8; 8usize],
    #[doc = "0xdeb80 - Device Control OUT Endpoint 1 Control Register"]
    pub doep3_ctl: DOEP3_CTL,
    _reserved186: [u8; 4usize],
    #[doc = "0xdeb88 - Device OUT Endpoint 1 Interrupt Register"]
    pub doep3_int: DOEP3_INT,
    _reserved187: [u8; 4usize],
    #[doc = "0xdeb90 - Device OUT Endpoint 1 Transfer Size Register"]
    pub doep3_tsiz: DOEP3_TSIZ,
    #[doc = "0xdeb94 - Device OUT Endpoint 1 DMA Address Register"]
    pub doep3_dmaaddr: DOEP3_DMAADDR,
    _reserved189: [u8; 8usize],
    #[doc = "0xdeba0 - Device Control OUT Endpoint 1 Control Register"]
    pub doep4_ctl: DOEP4_CTL,
    _reserved190: [u8; 4usize],
    #[doc = "0xdeba8 - Device OUT Endpoint 1 Interrupt Register"]
    pub doep4_int: DOEP4_INT,
    _reserved191: [u8; 4usize],
    #[doc = "0xdebb0 - Device OUT Endpoint 1 Transfer Size Register"]
    pub doep4_tsiz: DOEP4_TSIZ,
    #[doc = "0xdebb4 - Device OUT Endpoint 1 DMA Address Register"]
    pub doep4_dmaaddr: DOEP4_DMAADDR,
    _reserved193: [u8; 8usize],
    #[doc = "0xdebc0 - Device Control OUT Endpoint 1 Control Register"]
    pub doep5_ctl: DOEP5_CTL,
    _reserved194: [u8; 4usize],
    #[doc = "0xdebc8 - Device OUT Endpoint 1 Interrupt Register"]
    pub doep5_int: DOEP5_INT,
    _reserved195: [u8; 4usize],
    #[doc = "0xdebd0 - Device OUT Endpoint 1 Transfer Size Register"]
    pub doep5_tsiz: DOEP5_TSIZ,
    #[doc = "0xdebd4 - Device OUT Endpoint 1 DMA Address Register"]
    pub doep5_dmaaddr: DOEP5_DMAADDR,
    _reserved197: [u8; 552usize],
    #[doc = "0xdee00 - Power and Clock Gating Control Register"]
    pub pcgcctl: PCGCCTL,
}
#[doc = "System Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "System Control Register"]
pub mod ctrl;
#[doc = "System Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "System Status Register"]
pub mod status;
#[doc = "Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](if_) module"]
pub type IF = crate::Reg<u32, _IF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF;
#[doc = "`read()` method returns [if_::R](if_::R) reader structure"]
impl crate::Readable for IF {}
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "Interrupt Flag Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifs](ifs) module"]
pub type IFS = crate::Reg<u32, _IFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFS;
#[doc = "`write(|w| ..)` method takes [ifs::W](ifs::W) writer structure"]
impl crate::Writable for IFS {}
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "Interrupt Flag Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifc](ifc) module"]
pub type IFC = crate::Reg<u32, _IFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFC;
#[doc = "`write(|w| ..)` method takes [ifc::W](ifc::W) writer structure"]
impl crate::Writable for IFC {}
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](ien) module"]
pub type IEN = crate::Reg<u32, _IEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEN;
#[doc = "`read()` method returns [ien::R](ien::R) reader structure"]
impl crate::Readable for IEN {}
#[doc = "`write(|w| ..)` method takes [ien::W](ien::W) writer structure"]
impl crate::Writable for IEN {}
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "I/O Routing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [route](route) module"]
pub type ROUTE = crate::Reg<u32, _ROUTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROUTE;
#[doc = "`read()` method returns [route::R](route::R) reader structure"]
impl crate::Readable for ROUTE {}
#[doc = "`write(|w| ..)` method takes [route::W](route::W) writer structure"]
impl crate::Writable for ROUTE {}
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "Charger Detect Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdconf](cdconf) module"]
pub type CDCONF = crate::Reg<u32, _CDCONF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CDCONF;
#[doc = "`read()` method returns [cdconf::R](cdconf::R) reader structure"]
impl crate::Readable for CDCONF {}
#[doc = "`write(|w| ..)` method takes [cdconf::W](cdconf::W) writer structure"]
impl crate::Writable for CDCONF {}
#[doc = "Charger Detect Configuration Register"]
pub mod cdconf;
#[doc = "Command Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "Command Register"]
pub mod cmd;
#[doc = "Data TRIM 1 Values for USB DP and DM\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dattrim1](dattrim1) module"]
pub type DATTRIM1 = crate::Reg<u32, _DATTRIM1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATTRIM1;
#[doc = "`read()` method returns [dattrim1::R](dattrim1::R) reader structure"]
impl crate::Readable for DATTRIM1 {}
#[doc = "`write(|w| ..)` method takes [dattrim1::W](dattrim1::W) writer structure"]
impl crate::Writable for DATTRIM1 {}
#[doc = "Data TRIM 1 Values for USB DP and DM"]
pub mod dattrim1;
#[doc = "USB LEM Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lemctrl](lemctrl) module"]
pub type LEMCTRL = crate::Reg<u32, _LEMCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LEMCTRL;
#[doc = "`read()` method returns [lemctrl::R](lemctrl::R) reader structure"]
impl crate::Readable for LEMCTRL {}
#[doc = "`write(|w| ..)` method takes [lemctrl::W](lemctrl::W) writer structure"]
impl crate::Writable for LEMCTRL {}
#[doc = "USB LEM Control Register"]
pub mod lemctrl;
#[doc = "OTG Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gotgctl](gotgctl) module"]
pub type GOTGCTL = crate::Reg<u32, _GOTGCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GOTGCTL;
#[doc = "`read()` method returns [gotgctl::R](gotgctl::R) reader structure"]
impl crate::Readable for GOTGCTL {}
#[doc = "`write(|w| ..)` method takes [gotgctl::W](gotgctl::W) writer structure"]
impl crate::Writable for GOTGCTL {}
#[doc = "OTG Control and Status Register"]
pub mod gotgctl;
#[doc = "OTG Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gotgint](gotgint) module"]
pub type GOTGINT = crate::Reg<u32, _GOTGINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GOTGINT;
#[doc = "`read()` method returns [gotgint::R](gotgint::R) reader structure"]
impl crate::Readable for GOTGINT {}
#[doc = "`write(|w| ..)` method takes [gotgint::W](gotgint::W) writer structure"]
impl crate::Writable for GOTGINT {}
#[doc = "OTG Interrupt Register"]
pub mod gotgint;
#[doc = "AHB Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gahbcfg](gahbcfg) module"]
pub type GAHBCFG = crate::Reg<u32, _GAHBCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GAHBCFG;
#[doc = "`read()` method returns [gahbcfg::R](gahbcfg::R) reader structure"]
impl crate::Readable for GAHBCFG {}
#[doc = "`write(|w| ..)` method takes [gahbcfg::W](gahbcfg::W) writer structure"]
impl crate::Writable for GAHBCFG {}
#[doc = "AHB Configuration Register"]
pub mod gahbcfg;
#[doc = "USB Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gusbcfg](gusbcfg) module"]
pub type GUSBCFG = crate::Reg<u32, _GUSBCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GUSBCFG;
#[doc = "`read()` method returns [gusbcfg::R](gusbcfg::R) reader structure"]
impl crate::Readable for GUSBCFG {}
#[doc = "`write(|w| ..)` method takes [gusbcfg::W](gusbcfg::W) writer structure"]
impl crate::Writable for GUSBCFG {}
#[doc = "USB Configuration Register"]
pub mod gusbcfg;
#[doc = "Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grstctl](grstctl) module"]
pub type GRSTCTL = crate::Reg<u32, _GRSTCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRSTCTL;
#[doc = "`read()` method returns [grstctl::R](grstctl::R) reader structure"]
impl crate::Readable for GRSTCTL {}
#[doc = "`write(|w| ..)` method takes [grstctl::W](grstctl::W) writer structure"]
impl crate::Writable for GRSTCTL {}
#[doc = "Reset Register"]
pub mod grstctl;
#[doc = "Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gintsts](gintsts) module"]
pub type GINTSTS = crate::Reg<u32, _GINTSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GINTSTS;
#[doc = "`read()` method returns [gintsts::R](gintsts::R) reader structure"]
impl crate::Readable for GINTSTS {}
#[doc = "`write(|w| ..)` method takes [gintsts::W](gintsts::W) writer structure"]
impl crate::Writable for GINTSTS {}
#[doc = "Interrupt Register"]
pub mod gintsts;
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gintmsk](gintmsk) module"]
pub type GINTMSK = crate::Reg<u32, _GINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GINTMSK;
#[doc = "`read()` method returns [gintmsk::R](gintmsk::R) reader structure"]
impl crate::Readable for GINTMSK {}
#[doc = "`write(|w| ..)` method takes [gintmsk::W](gintmsk::W) writer structure"]
impl crate::Writable for GINTMSK {}
#[doc = "Interrupt Mask Register"]
pub mod gintmsk;
#[doc = "Receive Status Debug Read Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grxstsr](grxstsr) module"]
pub type GRXSTSR = crate::Reg<u32, _GRXSTSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRXSTSR;
#[doc = "`read()` method returns [grxstsr::R](grxstsr::R) reader structure"]
impl crate::Readable for GRXSTSR {}
#[doc = "Receive Status Debug Read Register"]
pub mod grxstsr;
#[doc = "Receive Status Read /Pop Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grxstsp](grxstsp) module"]
pub type GRXSTSP = crate::Reg<u32, _GRXSTSP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRXSTSP;
#[doc = "`read()` method returns [grxstsp::R](grxstsp::R) reader structure"]
impl crate::Readable for GRXSTSP {}
#[doc = "Receive Status Read /Pop Register"]
pub mod grxstsp;
#[doc = "Receive FIFO Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [grxfsiz](grxfsiz) module"]
pub type GRXFSIZ = crate::Reg<u32, _GRXFSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GRXFSIZ;
#[doc = "`read()` method returns [grxfsiz::R](grxfsiz::R) reader structure"]
impl crate::Readable for GRXFSIZ {}
#[doc = "`write(|w| ..)` method takes [grxfsiz::W](grxfsiz::W) writer structure"]
impl crate::Writable for GRXFSIZ {}
#[doc = "Receive FIFO Size Register"]
pub mod grxfsiz;
#[doc = "Non-periodic Transmit FIFO Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gnptxfsiz](gnptxfsiz) module"]
pub type GNPTXFSIZ = crate::Reg<u32, _GNPTXFSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GNPTXFSIZ;
#[doc = "`read()` method returns [gnptxfsiz::R](gnptxfsiz::R) reader structure"]
impl crate::Readable for GNPTXFSIZ {}
#[doc = "`write(|w| ..)` method takes [gnptxfsiz::W](gnptxfsiz::W) writer structure"]
impl crate::Writable for GNPTXFSIZ {}
#[doc = "Non-periodic Transmit FIFO Size Register"]
pub mod gnptxfsiz;
#[doc = "Non-periodic Transmit FIFO/Queue Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gnptxsts](gnptxsts) module"]
pub type GNPTXSTS = crate::Reg<u32, _GNPTXSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GNPTXSTS;
#[doc = "`read()` method returns [gnptxsts::R](gnptxsts::R) reader structure"]
impl crate::Readable for GNPTXSTS {}
#[doc = "Non-periodic Transmit FIFO/Queue Status Register"]
pub mod gnptxsts;
#[doc = "Synopsys ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gsnpsid](gsnpsid) module"]
pub type GSNPSID = crate::Reg<u32, _GSNPSID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GSNPSID;
#[doc = "`read()` method returns [gsnpsid::R](gsnpsid::R) reader structure"]
impl crate::Readable for GSNPSID {}
#[doc = "Synopsys ID Register"]
pub mod gsnpsid;
#[doc = "Global DFIFO Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gdfifocfg](gdfifocfg) module"]
pub type GDFIFOCFG = crate::Reg<u32, _GDFIFOCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GDFIFOCFG;
#[doc = "`read()` method returns [gdfifocfg::R](gdfifocfg::R) reader structure"]
impl crate::Readable for GDFIFOCFG {}
#[doc = "`write(|w| ..)` method takes [gdfifocfg::W](gdfifocfg::W) writer structure"]
impl crate::Writable for GDFIFOCFG {}
#[doc = "Global DFIFO Configuration Register"]
pub mod gdfifocfg;
#[doc = "Host Periodic Transmit FIFO Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hptxfsiz](hptxfsiz) module"]
pub type HPTXFSIZ = crate::Reg<u32, _HPTXFSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPTXFSIZ;
#[doc = "`read()` method returns [hptxfsiz::R](hptxfsiz::R) reader structure"]
impl crate::Readable for HPTXFSIZ {}
#[doc = "`write(|w| ..)` method takes [hptxfsiz::W](hptxfsiz::W) writer structure"]
impl crate::Writable for HPTXFSIZ {}
#[doc = "Host Periodic Transmit FIFO Size Register"]
pub mod hptxfsiz;
#[doc = "Device IN Endpoint Transmit FIFO Size Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptxf1](dieptxf1) module"]
pub type DIEPTXF1 = crate::Reg<u32, _DIEPTXF1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTXF1;
#[doc = "`read()` method returns [dieptxf1::R](dieptxf1::R) reader structure"]
impl crate::Readable for DIEPTXF1 {}
#[doc = "`write(|w| ..)` method takes [dieptxf1::W](dieptxf1::W) writer structure"]
impl crate::Writable for DIEPTXF1 {}
#[doc = "Device IN Endpoint Transmit FIFO Size Register 1"]
pub mod dieptxf1;
#[doc = "Device IN Endpoint Transmit FIFO Size Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptxf2](dieptxf2) module"]
pub type DIEPTXF2 = crate::Reg<u32, _DIEPTXF2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTXF2;
#[doc = "`read()` method returns [dieptxf2::R](dieptxf2::R) reader structure"]
impl crate::Readable for DIEPTXF2 {}
#[doc = "`write(|w| ..)` method takes [dieptxf2::W](dieptxf2::W) writer structure"]
impl crate::Writable for DIEPTXF2 {}
#[doc = "Device IN Endpoint Transmit FIFO Size Register 2"]
pub mod dieptxf2;
#[doc = "Device IN Endpoint Transmit FIFO Size Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptxf3](dieptxf3) module"]
pub type DIEPTXF3 = crate::Reg<u32, _DIEPTXF3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTXF3;
#[doc = "`read()` method returns [dieptxf3::R](dieptxf3::R) reader structure"]
impl crate::Readable for DIEPTXF3 {}
#[doc = "`write(|w| ..)` method takes [dieptxf3::W](dieptxf3::W) writer structure"]
impl crate::Writable for DIEPTXF3 {}
#[doc = "Device IN Endpoint Transmit FIFO Size Register 3"]
pub mod dieptxf3;
#[doc = "Device IN Endpoint Transmit FIFO Size Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptxf4](dieptxf4) module"]
pub type DIEPTXF4 = crate::Reg<u32, _DIEPTXF4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTXF4;
#[doc = "`read()` method returns [dieptxf4::R](dieptxf4::R) reader structure"]
impl crate::Readable for DIEPTXF4 {}
#[doc = "`write(|w| ..)` method takes [dieptxf4::W](dieptxf4::W) writer structure"]
impl crate::Writable for DIEPTXF4 {}
#[doc = "Device IN Endpoint Transmit FIFO Size Register 4"]
pub mod dieptxf4;
#[doc = "Device IN Endpoint Transmit FIFO Size Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptxf5](dieptxf5) module"]
pub type DIEPTXF5 = crate::Reg<u32, _DIEPTXF5>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTXF5;
#[doc = "`read()` method returns [dieptxf5::R](dieptxf5::R) reader structure"]
impl crate::Readable for DIEPTXF5 {}
#[doc = "`write(|w| ..)` method takes [dieptxf5::W](dieptxf5::W) writer structure"]
impl crate::Writable for DIEPTXF5 {}
#[doc = "Device IN Endpoint Transmit FIFO Size Register 5"]
pub mod dieptxf5;
#[doc = "Device IN Endpoint Transmit FIFO Size Register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dieptxf6](dieptxf6) module"]
pub type DIEPTXF6 = crate::Reg<u32, _DIEPTXF6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPTXF6;
#[doc = "`read()` method returns [dieptxf6::R](dieptxf6::R) reader structure"]
impl crate::Readable for DIEPTXF6 {}
#[doc = "`write(|w| ..)` method takes [dieptxf6::W](dieptxf6::W) writer structure"]
impl crate::Writable for DIEPTXF6 {}
#[doc = "Device IN Endpoint Transmit FIFO Size Register 6"]
pub mod dieptxf6;
#[doc = "Host Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcfg](hcfg) module"]
pub type HCFG = crate::Reg<u32, _HCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HCFG;
#[doc = "`read()` method returns [hcfg::R](hcfg::R) reader structure"]
impl crate::Readable for HCFG {}
#[doc = "`write(|w| ..)` method takes [hcfg::W](hcfg::W) writer structure"]
impl crate::Writable for HCFG {}
#[doc = "Host Configuration Register"]
pub mod hcfg;
#[doc = "Host Frame Interval Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfir](hfir) module"]
pub type HFIR = crate::Reg<u32, _HFIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFIR;
#[doc = "`read()` method returns [hfir::R](hfir::R) reader structure"]
impl crate::Readable for HFIR {}
#[doc = "`write(|w| ..)` method takes [hfir::W](hfir::W) writer structure"]
impl crate::Writable for HFIR {}
#[doc = "Host Frame Interval Register"]
pub mod hfir;
#[doc = "Host Frame Number/Frame Time Remaining Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfnum](hfnum) module"]
pub type HFNUM = crate::Reg<u32, _HFNUM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HFNUM;
#[doc = "`read()` method returns [hfnum::R](hfnum::R) reader structure"]
impl crate::Readable for HFNUM {}
#[doc = "Host Frame Number/Frame Time Remaining Register"]
pub mod hfnum;
#[doc = "Host Periodic Transmit FIFO/Queue Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hptxsts](hptxsts) module"]
pub type HPTXSTS = crate::Reg<u32, _HPTXSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPTXSTS;
#[doc = "`read()` method returns [hptxsts::R](hptxsts::R) reader structure"]
impl crate::Readable for HPTXSTS {}
#[doc = "Host Periodic Transmit FIFO/Queue Status Register"]
pub mod hptxsts;
#[doc = "Host All Channels Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [haint](haint) module"]
pub type HAINT = crate::Reg<u32, _HAINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HAINT;
#[doc = "`read()` method returns [haint::R](haint::R) reader structure"]
impl crate::Readable for HAINT {}
#[doc = "Host All Channels Interrupt Register"]
pub mod haint;
#[doc = "Host All Channels Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [haintmsk](haintmsk) module"]
pub type HAINTMSK = crate::Reg<u32, _HAINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HAINTMSK;
#[doc = "`read()` method returns [haintmsk::R](haintmsk::R) reader structure"]
impl crate::Readable for HAINTMSK {}
#[doc = "`write(|w| ..)` method takes [haintmsk::W](haintmsk::W) writer structure"]
impl crate::Writable for HAINTMSK {}
#[doc = "Host All Channels Interrupt Mask Register"]
pub mod haintmsk;
#[doc = "Host Port Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hprt](hprt) module"]
pub type HPRT = crate::Reg<u32, _HPRT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HPRT;
#[doc = "`read()` method returns [hprt::R](hprt::R) reader structure"]
impl crate::Readable for HPRT {}
#[doc = "`write(|w| ..)` method takes [hprt::W](hprt::W) writer structure"]
impl crate::Writable for HPRT {}
#[doc = "Host Port Control and Status Register"]
pub mod hprt;
#[doc = "Host Channel 0 Characteristics Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc0_char](hc0_char) module"]
pub type HC0_CHAR = crate::Reg<u32, _HC0_CHAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC0_CHAR;
#[doc = "`read()` method returns [hc0_char::R](hc0_char::R) reader structure"]
impl crate::Readable for HC0_CHAR {}
#[doc = "`write(|w| ..)` method takes [hc0_char::W](hc0_char::W) writer structure"]
impl crate::Writable for HC0_CHAR {}
#[doc = "Host Channel 0 Characteristics Register"]
pub mod hc0_char;
#[doc = "Host Channel 0 Split Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc0_splt](hc0_splt) module"]
pub type HC0_SPLT = crate::Reg<u32, _HC0_SPLT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC0_SPLT;
#[doc = "`read()` method returns [hc0_splt::R](hc0_splt::R) reader structure"]
impl crate::Readable for HC0_SPLT {}
#[doc = "`write(|w| ..)` method takes [hc0_splt::W](hc0_splt::W) writer structure"]
impl crate::Writable for HC0_SPLT {}
#[doc = "Host Channel 0 Split Control Register"]
pub mod hc0_splt;
#[doc = "Host Channel 0 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc0_int](hc0_int) module"]
pub type HC0_INT = crate::Reg<u32, _HC0_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC0_INT;
#[doc = "`read()` method returns [hc0_int::R](hc0_int::R) reader structure"]
impl crate::Readable for HC0_INT {}
#[doc = "`write(|w| ..)` method takes [hc0_int::W](hc0_int::W) writer structure"]
impl crate::Writable for HC0_INT {}
#[doc = "Host Channel 0 Interrupt Register"]
pub mod hc0_int;
#[doc = "Host Channel 0 Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc0_intmsk](hc0_intmsk) module"]
pub type HC0_INTMSK = crate::Reg<u32, _HC0_INTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC0_INTMSK;
#[doc = "`read()` method returns [hc0_intmsk::R](hc0_intmsk::R) reader structure"]
impl crate::Readable for HC0_INTMSK {}
#[doc = "`write(|w| ..)` method takes [hc0_intmsk::W](hc0_intmsk::W) writer structure"]
impl crate::Writable for HC0_INTMSK {}
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub mod hc0_intmsk;
#[doc = "Host Channel 0 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc0_tsiz](hc0_tsiz) module"]
pub type HC0_TSIZ = crate::Reg<u32, _HC0_TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC0_TSIZ;
#[doc = "`read()` method returns [hc0_tsiz::R](hc0_tsiz::R) reader structure"]
impl crate::Readable for HC0_TSIZ {}
#[doc = "`write(|w| ..)` method takes [hc0_tsiz::W](hc0_tsiz::W) writer structure"]
impl crate::Writable for HC0_TSIZ {}
#[doc = "Host Channel 0 Transfer Size Register"]
pub mod hc0_tsiz;
#[doc = "Host Channel 0 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc0_dmaaddr](hc0_dmaaddr) module"]
pub type HC0_DMAADDR = crate::Reg<u32, _HC0_DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC0_DMAADDR;
#[doc = "`read()` method returns [hc0_dmaaddr::R](hc0_dmaaddr::R) reader structure"]
impl crate::Readable for HC0_DMAADDR {}
#[doc = "`write(|w| ..)` method takes [hc0_dmaaddr::W](hc0_dmaaddr::W) writer structure"]
impl crate::Writable for HC0_DMAADDR {}
#[doc = "Host Channel 0 DMA Address Register"]
pub mod hc0_dmaaddr;
#[doc = "Host Channel 0 Characteristics Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc1_char](hc1_char) module"]
pub type HC1_CHAR = crate::Reg<u32, _HC1_CHAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC1_CHAR;
#[doc = "`read()` method returns [hc1_char::R](hc1_char::R) reader structure"]
impl crate::Readable for HC1_CHAR {}
#[doc = "`write(|w| ..)` method takes [hc1_char::W](hc1_char::W) writer structure"]
impl crate::Writable for HC1_CHAR {}
#[doc = "Host Channel 0 Characteristics Register"]
pub mod hc1_char;
#[doc = "Host Channel 0 Split Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc1_splt](hc1_splt) module"]
pub type HC1_SPLT = crate::Reg<u32, _HC1_SPLT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC1_SPLT;
#[doc = "`read()` method returns [hc1_splt::R](hc1_splt::R) reader structure"]
impl crate::Readable for HC1_SPLT {}
#[doc = "`write(|w| ..)` method takes [hc1_splt::W](hc1_splt::W) writer structure"]
impl crate::Writable for HC1_SPLT {}
#[doc = "Host Channel 0 Split Control Register"]
pub mod hc1_splt;
#[doc = "Host Channel 0 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc1_int](hc1_int) module"]
pub type HC1_INT = crate::Reg<u32, _HC1_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC1_INT;
#[doc = "`read()` method returns [hc1_int::R](hc1_int::R) reader structure"]
impl crate::Readable for HC1_INT {}
#[doc = "`write(|w| ..)` method takes [hc1_int::W](hc1_int::W) writer structure"]
impl crate::Writable for HC1_INT {}
#[doc = "Host Channel 0 Interrupt Register"]
pub mod hc1_int;
#[doc = "Host Channel 0 Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc1_intmsk](hc1_intmsk) module"]
pub type HC1_INTMSK = crate::Reg<u32, _HC1_INTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC1_INTMSK;
#[doc = "`read()` method returns [hc1_intmsk::R](hc1_intmsk::R) reader structure"]
impl crate::Readable for HC1_INTMSK {}
#[doc = "`write(|w| ..)` method takes [hc1_intmsk::W](hc1_intmsk::W) writer structure"]
impl crate::Writable for HC1_INTMSK {}
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub mod hc1_intmsk;
#[doc = "Host Channel 0 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc1_tsiz](hc1_tsiz) module"]
pub type HC1_TSIZ = crate::Reg<u32, _HC1_TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC1_TSIZ;
#[doc = "`read()` method returns [hc1_tsiz::R](hc1_tsiz::R) reader structure"]
impl crate::Readable for HC1_TSIZ {}
#[doc = "`write(|w| ..)` method takes [hc1_tsiz::W](hc1_tsiz::W) writer structure"]
impl crate::Writable for HC1_TSIZ {}
#[doc = "Host Channel 0 Transfer Size Register"]
pub mod hc1_tsiz;
#[doc = "Host Channel 0 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc1_dmaaddr](hc1_dmaaddr) module"]
pub type HC1_DMAADDR = crate::Reg<u32, _HC1_DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC1_DMAADDR;
#[doc = "`read()` method returns [hc1_dmaaddr::R](hc1_dmaaddr::R) reader structure"]
impl crate::Readable for HC1_DMAADDR {}
#[doc = "`write(|w| ..)` method takes [hc1_dmaaddr::W](hc1_dmaaddr::W) writer structure"]
impl crate::Writable for HC1_DMAADDR {}
#[doc = "Host Channel 0 DMA Address Register"]
pub mod hc1_dmaaddr;
#[doc = "Host Channel 0 Characteristics Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc2_char](hc2_char) module"]
pub type HC2_CHAR = crate::Reg<u32, _HC2_CHAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC2_CHAR;
#[doc = "`read()` method returns [hc2_char::R](hc2_char::R) reader structure"]
impl crate::Readable for HC2_CHAR {}
#[doc = "`write(|w| ..)` method takes [hc2_char::W](hc2_char::W) writer structure"]
impl crate::Writable for HC2_CHAR {}
#[doc = "Host Channel 0 Characteristics Register"]
pub mod hc2_char;
#[doc = "Host Channel 0 Split Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc2_splt](hc2_splt) module"]
pub type HC2_SPLT = crate::Reg<u32, _HC2_SPLT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC2_SPLT;
#[doc = "`read()` method returns [hc2_splt::R](hc2_splt::R) reader structure"]
impl crate::Readable for HC2_SPLT {}
#[doc = "`write(|w| ..)` method takes [hc2_splt::W](hc2_splt::W) writer structure"]
impl crate::Writable for HC2_SPLT {}
#[doc = "Host Channel 0 Split Control Register"]
pub mod hc2_splt;
#[doc = "Host Channel 0 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc2_int](hc2_int) module"]
pub type HC2_INT = crate::Reg<u32, _HC2_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC2_INT;
#[doc = "`read()` method returns [hc2_int::R](hc2_int::R) reader structure"]
impl crate::Readable for HC2_INT {}
#[doc = "`write(|w| ..)` method takes [hc2_int::W](hc2_int::W) writer structure"]
impl crate::Writable for HC2_INT {}
#[doc = "Host Channel 0 Interrupt Register"]
pub mod hc2_int;
#[doc = "Host Channel 0 Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc2_intmsk](hc2_intmsk) module"]
pub type HC2_INTMSK = crate::Reg<u32, _HC2_INTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC2_INTMSK;
#[doc = "`read()` method returns [hc2_intmsk::R](hc2_intmsk::R) reader structure"]
impl crate::Readable for HC2_INTMSK {}
#[doc = "`write(|w| ..)` method takes [hc2_intmsk::W](hc2_intmsk::W) writer structure"]
impl crate::Writable for HC2_INTMSK {}
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub mod hc2_intmsk;
#[doc = "Host Channel 0 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc2_tsiz](hc2_tsiz) module"]
pub type HC2_TSIZ = crate::Reg<u32, _HC2_TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC2_TSIZ;
#[doc = "`read()` method returns [hc2_tsiz::R](hc2_tsiz::R) reader structure"]
impl crate::Readable for HC2_TSIZ {}
#[doc = "`write(|w| ..)` method takes [hc2_tsiz::W](hc2_tsiz::W) writer structure"]
impl crate::Writable for HC2_TSIZ {}
#[doc = "Host Channel 0 Transfer Size Register"]
pub mod hc2_tsiz;
#[doc = "Host Channel 0 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc2_dmaaddr](hc2_dmaaddr) module"]
pub type HC2_DMAADDR = crate::Reg<u32, _HC2_DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC2_DMAADDR;
#[doc = "`read()` method returns [hc2_dmaaddr::R](hc2_dmaaddr::R) reader structure"]
impl crate::Readable for HC2_DMAADDR {}
#[doc = "`write(|w| ..)` method takes [hc2_dmaaddr::W](hc2_dmaaddr::W) writer structure"]
impl crate::Writable for HC2_DMAADDR {}
#[doc = "Host Channel 0 DMA Address Register"]
pub mod hc2_dmaaddr;
#[doc = "Host Channel 0 Characteristics Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc3_char](hc3_char) module"]
pub type HC3_CHAR = crate::Reg<u32, _HC3_CHAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC3_CHAR;
#[doc = "`read()` method returns [hc3_char::R](hc3_char::R) reader structure"]
impl crate::Readable for HC3_CHAR {}
#[doc = "`write(|w| ..)` method takes [hc3_char::W](hc3_char::W) writer structure"]
impl crate::Writable for HC3_CHAR {}
#[doc = "Host Channel 0 Characteristics Register"]
pub mod hc3_char;
#[doc = "Host Channel 0 Split Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc3_splt](hc3_splt) module"]
pub type HC3_SPLT = crate::Reg<u32, _HC3_SPLT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC3_SPLT;
#[doc = "`read()` method returns [hc3_splt::R](hc3_splt::R) reader structure"]
impl crate::Readable for HC3_SPLT {}
#[doc = "`write(|w| ..)` method takes [hc3_splt::W](hc3_splt::W) writer structure"]
impl crate::Writable for HC3_SPLT {}
#[doc = "Host Channel 0 Split Control Register"]
pub mod hc3_splt;
#[doc = "Host Channel 0 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc3_int](hc3_int) module"]
pub type HC3_INT = crate::Reg<u32, _HC3_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC3_INT;
#[doc = "`read()` method returns [hc3_int::R](hc3_int::R) reader structure"]
impl crate::Readable for HC3_INT {}
#[doc = "`write(|w| ..)` method takes [hc3_int::W](hc3_int::W) writer structure"]
impl crate::Writable for HC3_INT {}
#[doc = "Host Channel 0 Interrupt Register"]
pub mod hc3_int;
#[doc = "Host Channel 0 Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc3_intmsk](hc3_intmsk) module"]
pub type HC3_INTMSK = crate::Reg<u32, _HC3_INTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC3_INTMSK;
#[doc = "`read()` method returns [hc3_intmsk::R](hc3_intmsk::R) reader structure"]
impl crate::Readable for HC3_INTMSK {}
#[doc = "`write(|w| ..)` method takes [hc3_intmsk::W](hc3_intmsk::W) writer structure"]
impl crate::Writable for HC3_INTMSK {}
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub mod hc3_intmsk;
#[doc = "Host Channel 0 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc3_tsiz](hc3_tsiz) module"]
pub type HC3_TSIZ = crate::Reg<u32, _HC3_TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC3_TSIZ;
#[doc = "`read()` method returns [hc3_tsiz::R](hc3_tsiz::R) reader structure"]
impl crate::Readable for HC3_TSIZ {}
#[doc = "`write(|w| ..)` method takes [hc3_tsiz::W](hc3_tsiz::W) writer structure"]
impl crate::Writable for HC3_TSIZ {}
#[doc = "Host Channel 0 Transfer Size Register"]
pub mod hc3_tsiz;
#[doc = "Host Channel 0 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc3_dmaaddr](hc3_dmaaddr) module"]
pub type HC3_DMAADDR = crate::Reg<u32, _HC3_DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC3_DMAADDR;
#[doc = "`read()` method returns [hc3_dmaaddr::R](hc3_dmaaddr::R) reader structure"]
impl crate::Readable for HC3_DMAADDR {}
#[doc = "`write(|w| ..)` method takes [hc3_dmaaddr::W](hc3_dmaaddr::W) writer structure"]
impl crate::Writable for HC3_DMAADDR {}
#[doc = "Host Channel 0 DMA Address Register"]
pub mod hc3_dmaaddr;
#[doc = "Host Channel 0 Characteristics Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc4_char](hc4_char) module"]
pub type HC4_CHAR = crate::Reg<u32, _HC4_CHAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC4_CHAR;
#[doc = "`read()` method returns [hc4_char::R](hc4_char::R) reader structure"]
impl crate::Readable for HC4_CHAR {}
#[doc = "`write(|w| ..)` method takes [hc4_char::W](hc4_char::W) writer structure"]
impl crate::Writable for HC4_CHAR {}
#[doc = "Host Channel 0 Characteristics Register"]
pub mod hc4_char;
#[doc = "Host Channel 0 Split Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc4_splt](hc4_splt) module"]
pub type HC4_SPLT = crate::Reg<u32, _HC4_SPLT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC4_SPLT;
#[doc = "`read()` method returns [hc4_splt::R](hc4_splt::R) reader structure"]
impl crate::Readable for HC4_SPLT {}
#[doc = "`write(|w| ..)` method takes [hc4_splt::W](hc4_splt::W) writer structure"]
impl crate::Writable for HC4_SPLT {}
#[doc = "Host Channel 0 Split Control Register"]
pub mod hc4_splt;
#[doc = "Host Channel 0 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc4_int](hc4_int) module"]
pub type HC4_INT = crate::Reg<u32, _HC4_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC4_INT;
#[doc = "`read()` method returns [hc4_int::R](hc4_int::R) reader structure"]
impl crate::Readable for HC4_INT {}
#[doc = "`write(|w| ..)` method takes [hc4_int::W](hc4_int::W) writer structure"]
impl crate::Writable for HC4_INT {}
#[doc = "Host Channel 0 Interrupt Register"]
pub mod hc4_int;
#[doc = "Host Channel 0 Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc4_intmsk](hc4_intmsk) module"]
pub type HC4_INTMSK = crate::Reg<u32, _HC4_INTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC4_INTMSK;
#[doc = "`read()` method returns [hc4_intmsk::R](hc4_intmsk::R) reader structure"]
impl crate::Readable for HC4_INTMSK {}
#[doc = "`write(|w| ..)` method takes [hc4_intmsk::W](hc4_intmsk::W) writer structure"]
impl crate::Writable for HC4_INTMSK {}
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub mod hc4_intmsk;
#[doc = "Host Channel 0 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc4_tsiz](hc4_tsiz) module"]
pub type HC4_TSIZ = crate::Reg<u32, _HC4_TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC4_TSIZ;
#[doc = "`read()` method returns [hc4_tsiz::R](hc4_tsiz::R) reader structure"]
impl crate::Readable for HC4_TSIZ {}
#[doc = "`write(|w| ..)` method takes [hc4_tsiz::W](hc4_tsiz::W) writer structure"]
impl crate::Writable for HC4_TSIZ {}
#[doc = "Host Channel 0 Transfer Size Register"]
pub mod hc4_tsiz;
#[doc = "Host Channel 0 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc4_dmaaddr](hc4_dmaaddr) module"]
pub type HC4_DMAADDR = crate::Reg<u32, _HC4_DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC4_DMAADDR;
#[doc = "`read()` method returns [hc4_dmaaddr::R](hc4_dmaaddr::R) reader structure"]
impl crate::Readable for HC4_DMAADDR {}
#[doc = "`write(|w| ..)` method takes [hc4_dmaaddr::W](hc4_dmaaddr::W) writer structure"]
impl crate::Writable for HC4_DMAADDR {}
#[doc = "Host Channel 0 DMA Address Register"]
pub mod hc4_dmaaddr;
#[doc = "Host Channel 0 Characteristics Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc5_char](hc5_char) module"]
pub type HC5_CHAR = crate::Reg<u32, _HC5_CHAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC5_CHAR;
#[doc = "`read()` method returns [hc5_char::R](hc5_char::R) reader structure"]
impl crate::Readable for HC5_CHAR {}
#[doc = "`write(|w| ..)` method takes [hc5_char::W](hc5_char::W) writer structure"]
impl crate::Writable for HC5_CHAR {}
#[doc = "Host Channel 0 Characteristics Register"]
pub mod hc5_char;
#[doc = "Host Channel 0 Split Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc5_splt](hc5_splt) module"]
pub type HC5_SPLT = crate::Reg<u32, _HC5_SPLT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC5_SPLT;
#[doc = "`read()` method returns [hc5_splt::R](hc5_splt::R) reader structure"]
impl crate::Readable for HC5_SPLT {}
#[doc = "`write(|w| ..)` method takes [hc5_splt::W](hc5_splt::W) writer structure"]
impl crate::Writable for HC5_SPLT {}
#[doc = "Host Channel 0 Split Control Register"]
pub mod hc5_splt;
#[doc = "Host Channel 0 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc5_int](hc5_int) module"]
pub type HC5_INT = crate::Reg<u32, _HC5_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC5_INT;
#[doc = "`read()` method returns [hc5_int::R](hc5_int::R) reader structure"]
impl crate::Readable for HC5_INT {}
#[doc = "`write(|w| ..)` method takes [hc5_int::W](hc5_int::W) writer structure"]
impl crate::Writable for HC5_INT {}
#[doc = "Host Channel 0 Interrupt Register"]
pub mod hc5_int;
#[doc = "Host Channel 0 Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc5_intmsk](hc5_intmsk) module"]
pub type HC5_INTMSK = crate::Reg<u32, _HC5_INTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC5_INTMSK;
#[doc = "`read()` method returns [hc5_intmsk::R](hc5_intmsk::R) reader structure"]
impl crate::Readable for HC5_INTMSK {}
#[doc = "`write(|w| ..)` method takes [hc5_intmsk::W](hc5_intmsk::W) writer structure"]
impl crate::Writable for HC5_INTMSK {}
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub mod hc5_intmsk;
#[doc = "Host Channel 0 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc5_tsiz](hc5_tsiz) module"]
pub type HC5_TSIZ = crate::Reg<u32, _HC5_TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC5_TSIZ;
#[doc = "`read()` method returns [hc5_tsiz::R](hc5_tsiz::R) reader structure"]
impl crate::Readable for HC5_TSIZ {}
#[doc = "`write(|w| ..)` method takes [hc5_tsiz::W](hc5_tsiz::W) writer structure"]
impl crate::Writable for HC5_TSIZ {}
#[doc = "Host Channel 0 Transfer Size Register"]
pub mod hc5_tsiz;
#[doc = "Host Channel 0 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc5_dmaaddr](hc5_dmaaddr) module"]
pub type HC5_DMAADDR = crate::Reg<u32, _HC5_DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC5_DMAADDR;
#[doc = "`read()` method returns [hc5_dmaaddr::R](hc5_dmaaddr::R) reader structure"]
impl crate::Readable for HC5_DMAADDR {}
#[doc = "`write(|w| ..)` method takes [hc5_dmaaddr::W](hc5_dmaaddr::W) writer structure"]
impl crate::Writable for HC5_DMAADDR {}
#[doc = "Host Channel 0 DMA Address Register"]
pub mod hc5_dmaaddr;
#[doc = "Host Channel 0 Characteristics Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc6_char](hc6_char) module"]
pub type HC6_CHAR = crate::Reg<u32, _HC6_CHAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC6_CHAR;
#[doc = "`read()` method returns [hc6_char::R](hc6_char::R) reader structure"]
impl crate::Readable for HC6_CHAR {}
#[doc = "`write(|w| ..)` method takes [hc6_char::W](hc6_char::W) writer structure"]
impl crate::Writable for HC6_CHAR {}
#[doc = "Host Channel 0 Characteristics Register"]
pub mod hc6_char;
#[doc = "Host Channel 0 Split Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc6_splt](hc6_splt) module"]
pub type HC6_SPLT = crate::Reg<u32, _HC6_SPLT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC6_SPLT;
#[doc = "`read()` method returns [hc6_splt::R](hc6_splt::R) reader structure"]
impl crate::Readable for HC6_SPLT {}
#[doc = "`write(|w| ..)` method takes [hc6_splt::W](hc6_splt::W) writer structure"]
impl crate::Writable for HC6_SPLT {}
#[doc = "Host Channel 0 Split Control Register"]
pub mod hc6_splt;
#[doc = "Host Channel 0 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc6_int](hc6_int) module"]
pub type HC6_INT = crate::Reg<u32, _HC6_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC6_INT;
#[doc = "`read()` method returns [hc6_int::R](hc6_int::R) reader structure"]
impl crate::Readable for HC6_INT {}
#[doc = "`write(|w| ..)` method takes [hc6_int::W](hc6_int::W) writer structure"]
impl crate::Writable for HC6_INT {}
#[doc = "Host Channel 0 Interrupt Register"]
pub mod hc6_int;
#[doc = "Host Channel 0 Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc6_intmsk](hc6_intmsk) module"]
pub type HC6_INTMSK = crate::Reg<u32, _HC6_INTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC6_INTMSK;
#[doc = "`read()` method returns [hc6_intmsk::R](hc6_intmsk::R) reader structure"]
impl crate::Readable for HC6_INTMSK {}
#[doc = "`write(|w| ..)` method takes [hc6_intmsk::W](hc6_intmsk::W) writer structure"]
impl crate::Writable for HC6_INTMSK {}
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub mod hc6_intmsk;
#[doc = "Host Channel 0 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc6_tsiz](hc6_tsiz) module"]
pub type HC6_TSIZ = crate::Reg<u32, _HC6_TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC6_TSIZ;
#[doc = "`read()` method returns [hc6_tsiz::R](hc6_tsiz::R) reader structure"]
impl crate::Readable for HC6_TSIZ {}
#[doc = "`write(|w| ..)` method takes [hc6_tsiz::W](hc6_tsiz::W) writer structure"]
impl crate::Writable for HC6_TSIZ {}
#[doc = "Host Channel 0 Transfer Size Register"]
pub mod hc6_tsiz;
#[doc = "Host Channel 0 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc6_dmaaddr](hc6_dmaaddr) module"]
pub type HC6_DMAADDR = crate::Reg<u32, _HC6_DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC6_DMAADDR;
#[doc = "`read()` method returns [hc6_dmaaddr::R](hc6_dmaaddr::R) reader structure"]
impl crate::Readable for HC6_DMAADDR {}
#[doc = "`write(|w| ..)` method takes [hc6_dmaaddr::W](hc6_dmaaddr::W) writer structure"]
impl crate::Writable for HC6_DMAADDR {}
#[doc = "Host Channel 0 DMA Address Register"]
pub mod hc6_dmaaddr;
#[doc = "Host Channel 0 Characteristics Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc7_char](hc7_char) module"]
pub type HC7_CHAR = crate::Reg<u32, _HC7_CHAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC7_CHAR;
#[doc = "`read()` method returns [hc7_char::R](hc7_char::R) reader structure"]
impl crate::Readable for HC7_CHAR {}
#[doc = "`write(|w| ..)` method takes [hc7_char::W](hc7_char::W) writer structure"]
impl crate::Writable for HC7_CHAR {}
#[doc = "Host Channel 0 Characteristics Register"]
pub mod hc7_char;
#[doc = "Host Channel 0 Split Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc7_splt](hc7_splt) module"]
pub type HC7_SPLT = crate::Reg<u32, _HC7_SPLT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC7_SPLT;
#[doc = "`read()` method returns [hc7_splt::R](hc7_splt::R) reader structure"]
impl crate::Readable for HC7_SPLT {}
#[doc = "`write(|w| ..)` method takes [hc7_splt::W](hc7_splt::W) writer structure"]
impl crate::Writable for HC7_SPLT {}
#[doc = "Host Channel 0 Split Control Register"]
pub mod hc7_splt;
#[doc = "Host Channel 0 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc7_int](hc7_int) module"]
pub type HC7_INT = crate::Reg<u32, _HC7_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC7_INT;
#[doc = "`read()` method returns [hc7_int::R](hc7_int::R) reader structure"]
impl crate::Readable for HC7_INT {}
#[doc = "`write(|w| ..)` method takes [hc7_int::W](hc7_int::W) writer structure"]
impl crate::Writable for HC7_INT {}
#[doc = "Host Channel 0 Interrupt Register"]
pub mod hc7_int;
#[doc = "Host Channel 0 Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc7_intmsk](hc7_intmsk) module"]
pub type HC7_INTMSK = crate::Reg<u32, _HC7_INTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC7_INTMSK;
#[doc = "`read()` method returns [hc7_intmsk::R](hc7_intmsk::R) reader structure"]
impl crate::Readable for HC7_INTMSK {}
#[doc = "`write(|w| ..)` method takes [hc7_intmsk::W](hc7_intmsk::W) writer structure"]
impl crate::Writable for HC7_INTMSK {}
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub mod hc7_intmsk;
#[doc = "Host Channel 0 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc7_tsiz](hc7_tsiz) module"]
pub type HC7_TSIZ = crate::Reg<u32, _HC7_TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC7_TSIZ;
#[doc = "`read()` method returns [hc7_tsiz::R](hc7_tsiz::R) reader structure"]
impl crate::Readable for HC7_TSIZ {}
#[doc = "`write(|w| ..)` method takes [hc7_tsiz::W](hc7_tsiz::W) writer structure"]
impl crate::Writable for HC7_TSIZ {}
#[doc = "Host Channel 0 Transfer Size Register"]
pub mod hc7_tsiz;
#[doc = "Host Channel 0 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc7_dmaaddr](hc7_dmaaddr) module"]
pub type HC7_DMAADDR = crate::Reg<u32, _HC7_DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC7_DMAADDR;
#[doc = "`read()` method returns [hc7_dmaaddr::R](hc7_dmaaddr::R) reader structure"]
impl crate::Readable for HC7_DMAADDR {}
#[doc = "`write(|w| ..)` method takes [hc7_dmaaddr::W](hc7_dmaaddr::W) writer structure"]
impl crate::Writable for HC7_DMAADDR {}
#[doc = "Host Channel 0 DMA Address Register"]
pub mod hc7_dmaaddr;
#[doc = "Host Channel 0 Characteristics Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc8_char](hc8_char) module"]
pub type HC8_CHAR = crate::Reg<u32, _HC8_CHAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC8_CHAR;
#[doc = "`read()` method returns [hc8_char::R](hc8_char::R) reader structure"]
impl crate::Readable for HC8_CHAR {}
#[doc = "`write(|w| ..)` method takes [hc8_char::W](hc8_char::W) writer structure"]
impl crate::Writable for HC8_CHAR {}
#[doc = "Host Channel 0 Characteristics Register"]
pub mod hc8_char;
#[doc = "Host Channel 0 Split Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc8_splt](hc8_splt) module"]
pub type HC8_SPLT = crate::Reg<u32, _HC8_SPLT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC8_SPLT;
#[doc = "`read()` method returns [hc8_splt::R](hc8_splt::R) reader structure"]
impl crate::Readable for HC8_SPLT {}
#[doc = "`write(|w| ..)` method takes [hc8_splt::W](hc8_splt::W) writer structure"]
impl crate::Writable for HC8_SPLT {}
#[doc = "Host Channel 0 Split Control Register"]
pub mod hc8_splt;
#[doc = "Host Channel 0 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc8_int](hc8_int) module"]
pub type HC8_INT = crate::Reg<u32, _HC8_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC8_INT;
#[doc = "`read()` method returns [hc8_int::R](hc8_int::R) reader structure"]
impl crate::Readable for HC8_INT {}
#[doc = "`write(|w| ..)` method takes [hc8_int::W](hc8_int::W) writer structure"]
impl crate::Writable for HC8_INT {}
#[doc = "Host Channel 0 Interrupt Register"]
pub mod hc8_int;
#[doc = "Host Channel 0 Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc8_intmsk](hc8_intmsk) module"]
pub type HC8_INTMSK = crate::Reg<u32, _HC8_INTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC8_INTMSK;
#[doc = "`read()` method returns [hc8_intmsk::R](hc8_intmsk::R) reader structure"]
impl crate::Readable for HC8_INTMSK {}
#[doc = "`write(|w| ..)` method takes [hc8_intmsk::W](hc8_intmsk::W) writer structure"]
impl crate::Writable for HC8_INTMSK {}
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub mod hc8_intmsk;
#[doc = "Host Channel 0 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc8_tsiz](hc8_tsiz) module"]
pub type HC8_TSIZ = crate::Reg<u32, _HC8_TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC8_TSIZ;
#[doc = "`read()` method returns [hc8_tsiz::R](hc8_tsiz::R) reader structure"]
impl crate::Readable for HC8_TSIZ {}
#[doc = "`write(|w| ..)` method takes [hc8_tsiz::W](hc8_tsiz::W) writer structure"]
impl crate::Writable for HC8_TSIZ {}
#[doc = "Host Channel 0 Transfer Size Register"]
pub mod hc8_tsiz;
#[doc = "Host Channel 0 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc8_dmaaddr](hc8_dmaaddr) module"]
pub type HC8_DMAADDR = crate::Reg<u32, _HC8_DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC8_DMAADDR;
#[doc = "`read()` method returns [hc8_dmaaddr::R](hc8_dmaaddr::R) reader structure"]
impl crate::Readable for HC8_DMAADDR {}
#[doc = "`write(|w| ..)` method takes [hc8_dmaaddr::W](hc8_dmaaddr::W) writer structure"]
impl crate::Writable for HC8_DMAADDR {}
#[doc = "Host Channel 0 DMA Address Register"]
pub mod hc8_dmaaddr;
#[doc = "Host Channel 0 Characteristics Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc9_char](hc9_char) module"]
pub type HC9_CHAR = crate::Reg<u32, _HC9_CHAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC9_CHAR;
#[doc = "`read()` method returns [hc9_char::R](hc9_char::R) reader structure"]
impl crate::Readable for HC9_CHAR {}
#[doc = "`write(|w| ..)` method takes [hc9_char::W](hc9_char::W) writer structure"]
impl crate::Writable for HC9_CHAR {}
#[doc = "Host Channel 0 Characteristics Register"]
pub mod hc9_char;
#[doc = "Host Channel 0 Split Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc9_splt](hc9_splt) module"]
pub type HC9_SPLT = crate::Reg<u32, _HC9_SPLT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC9_SPLT;
#[doc = "`read()` method returns [hc9_splt::R](hc9_splt::R) reader structure"]
impl crate::Readable for HC9_SPLT {}
#[doc = "`write(|w| ..)` method takes [hc9_splt::W](hc9_splt::W) writer structure"]
impl crate::Writable for HC9_SPLT {}
#[doc = "Host Channel 0 Split Control Register"]
pub mod hc9_splt;
#[doc = "Host Channel 0 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc9_int](hc9_int) module"]
pub type HC9_INT = crate::Reg<u32, _HC9_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC9_INT;
#[doc = "`read()` method returns [hc9_int::R](hc9_int::R) reader structure"]
impl crate::Readable for HC9_INT {}
#[doc = "`write(|w| ..)` method takes [hc9_int::W](hc9_int::W) writer structure"]
impl crate::Writable for HC9_INT {}
#[doc = "Host Channel 0 Interrupt Register"]
pub mod hc9_int;
#[doc = "Host Channel 0 Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc9_intmsk](hc9_intmsk) module"]
pub type HC9_INTMSK = crate::Reg<u32, _HC9_INTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC9_INTMSK;
#[doc = "`read()` method returns [hc9_intmsk::R](hc9_intmsk::R) reader structure"]
impl crate::Readable for HC9_INTMSK {}
#[doc = "`write(|w| ..)` method takes [hc9_intmsk::W](hc9_intmsk::W) writer structure"]
impl crate::Writable for HC9_INTMSK {}
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub mod hc9_intmsk;
#[doc = "Host Channel 0 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc9_tsiz](hc9_tsiz) module"]
pub type HC9_TSIZ = crate::Reg<u32, _HC9_TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC9_TSIZ;
#[doc = "`read()` method returns [hc9_tsiz::R](hc9_tsiz::R) reader structure"]
impl crate::Readable for HC9_TSIZ {}
#[doc = "`write(|w| ..)` method takes [hc9_tsiz::W](hc9_tsiz::W) writer structure"]
impl crate::Writable for HC9_TSIZ {}
#[doc = "Host Channel 0 Transfer Size Register"]
pub mod hc9_tsiz;
#[doc = "Host Channel 0 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc9_dmaaddr](hc9_dmaaddr) module"]
pub type HC9_DMAADDR = crate::Reg<u32, _HC9_DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC9_DMAADDR;
#[doc = "`read()` method returns [hc9_dmaaddr::R](hc9_dmaaddr::R) reader structure"]
impl crate::Readable for HC9_DMAADDR {}
#[doc = "`write(|w| ..)` method takes [hc9_dmaaddr::W](hc9_dmaaddr::W) writer structure"]
impl crate::Writable for HC9_DMAADDR {}
#[doc = "Host Channel 0 DMA Address Register"]
pub mod hc9_dmaaddr;
#[doc = "Host Channel 0 Characteristics Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc10_char](hc10_char) module"]
pub type HC10_CHAR = crate::Reg<u32, _HC10_CHAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC10_CHAR;
#[doc = "`read()` method returns [hc10_char::R](hc10_char::R) reader structure"]
impl crate::Readable for HC10_CHAR {}
#[doc = "`write(|w| ..)` method takes [hc10_char::W](hc10_char::W) writer structure"]
impl crate::Writable for HC10_CHAR {}
#[doc = "Host Channel 0 Characteristics Register"]
pub mod hc10_char;
#[doc = "Host Channel 0 Split Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc10_splt](hc10_splt) module"]
pub type HC10_SPLT = crate::Reg<u32, _HC10_SPLT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC10_SPLT;
#[doc = "`read()` method returns [hc10_splt::R](hc10_splt::R) reader structure"]
impl crate::Readable for HC10_SPLT {}
#[doc = "`write(|w| ..)` method takes [hc10_splt::W](hc10_splt::W) writer structure"]
impl crate::Writable for HC10_SPLT {}
#[doc = "Host Channel 0 Split Control Register"]
pub mod hc10_splt;
#[doc = "Host Channel 0 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc10_int](hc10_int) module"]
pub type HC10_INT = crate::Reg<u32, _HC10_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC10_INT;
#[doc = "`read()` method returns [hc10_int::R](hc10_int::R) reader structure"]
impl crate::Readable for HC10_INT {}
#[doc = "`write(|w| ..)` method takes [hc10_int::W](hc10_int::W) writer structure"]
impl crate::Writable for HC10_INT {}
#[doc = "Host Channel 0 Interrupt Register"]
pub mod hc10_int;
#[doc = "Host Channel 0 Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc10_intmsk](hc10_intmsk) module"]
pub type HC10_INTMSK = crate::Reg<u32, _HC10_INTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC10_INTMSK;
#[doc = "`read()` method returns [hc10_intmsk::R](hc10_intmsk::R) reader structure"]
impl crate::Readable for HC10_INTMSK {}
#[doc = "`write(|w| ..)` method takes [hc10_intmsk::W](hc10_intmsk::W) writer structure"]
impl crate::Writable for HC10_INTMSK {}
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub mod hc10_intmsk;
#[doc = "Host Channel 0 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc10_tsiz](hc10_tsiz) module"]
pub type HC10_TSIZ = crate::Reg<u32, _HC10_TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC10_TSIZ;
#[doc = "`read()` method returns [hc10_tsiz::R](hc10_tsiz::R) reader structure"]
impl crate::Readable for HC10_TSIZ {}
#[doc = "`write(|w| ..)` method takes [hc10_tsiz::W](hc10_tsiz::W) writer structure"]
impl crate::Writable for HC10_TSIZ {}
#[doc = "Host Channel 0 Transfer Size Register"]
pub mod hc10_tsiz;
#[doc = "Host Channel 0 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc10_dmaaddr](hc10_dmaaddr) module"]
pub type HC10_DMAADDR = crate::Reg<u32, _HC10_DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC10_DMAADDR;
#[doc = "`read()` method returns [hc10_dmaaddr::R](hc10_dmaaddr::R) reader structure"]
impl crate::Readable for HC10_DMAADDR {}
#[doc = "`write(|w| ..)` method takes [hc10_dmaaddr::W](hc10_dmaaddr::W) writer structure"]
impl crate::Writable for HC10_DMAADDR {}
#[doc = "Host Channel 0 DMA Address Register"]
pub mod hc10_dmaaddr;
#[doc = "Host Channel 0 Characteristics Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc11_char](hc11_char) module"]
pub type HC11_CHAR = crate::Reg<u32, _HC11_CHAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC11_CHAR;
#[doc = "`read()` method returns [hc11_char::R](hc11_char::R) reader structure"]
impl crate::Readable for HC11_CHAR {}
#[doc = "`write(|w| ..)` method takes [hc11_char::W](hc11_char::W) writer structure"]
impl crate::Writable for HC11_CHAR {}
#[doc = "Host Channel 0 Characteristics Register"]
pub mod hc11_char;
#[doc = "Host Channel 0 Split Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc11_splt](hc11_splt) module"]
pub type HC11_SPLT = crate::Reg<u32, _HC11_SPLT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC11_SPLT;
#[doc = "`read()` method returns [hc11_splt::R](hc11_splt::R) reader structure"]
impl crate::Readable for HC11_SPLT {}
#[doc = "`write(|w| ..)` method takes [hc11_splt::W](hc11_splt::W) writer structure"]
impl crate::Writable for HC11_SPLT {}
#[doc = "Host Channel 0 Split Control Register"]
pub mod hc11_splt;
#[doc = "Host Channel 0 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc11_int](hc11_int) module"]
pub type HC11_INT = crate::Reg<u32, _HC11_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC11_INT;
#[doc = "`read()` method returns [hc11_int::R](hc11_int::R) reader structure"]
impl crate::Readable for HC11_INT {}
#[doc = "`write(|w| ..)` method takes [hc11_int::W](hc11_int::W) writer structure"]
impl crate::Writable for HC11_INT {}
#[doc = "Host Channel 0 Interrupt Register"]
pub mod hc11_int;
#[doc = "Host Channel 0 Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc11_intmsk](hc11_intmsk) module"]
pub type HC11_INTMSK = crate::Reg<u32, _HC11_INTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC11_INTMSK;
#[doc = "`read()` method returns [hc11_intmsk::R](hc11_intmsk::R) reader structure"]
impl crate::Readable for HC11_INTMSK {}
#[doc = "`write(|w| ..)` method takes [hc11_intmsk::W](hc11_intmsk::W) writer structure"]
impl crate::Writable for HC11_INTMSK {}
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub mod hc11_intmsk;
#[doc = "Host Channel 0 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc11_tsiz](hc11_tsiz) module"]
pub type HC11_TSIZ = crate::Reg<u32, _HC11_TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC11_TSIZ;
#[doc = "`read()` method returns [hc11_tsiz::R](hc11_tsiz::R) reader structure"]
impl crate::Readable for HC11_TSIZ {}
#[doc = "`write(|w| ..)` method takes [hc11_tsiz::W](hc11_tsiz::W) writer structure"]
impl crate::Writable for HC11_TSIZ {}
#[doc = "Host Channel 0 Transfer Size Register"]
pub mod hc11_tsiz;
#[doc = "Host Channel 0 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc11_dmaaddr](hc11_dmaaddr) module"]
pub type HC11_DMAADDR = crate::Reg<u32, _HC11_DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC11_DMAADDR;
#[doc = "`read()` method returns [hc11_dmaaddr::R](hc11_dmaaddr::R) reader structure"]
impl crate::Readable for HC11_DMAADDR {}
#[doc = "`write(|w| ..)` method takes [hc11_dmaaddr::W](hc11_dmaaddr::W) writer structure"]
impl crate::Writable for HC11_DMAADDR {}
#[doc = "Host Channel 0 DMA Address Register"]
pub mod hc11_dmaaddr;
#[doc = "Host Channel 0 Characteristics Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc12_char](hc12_char) module"]
pub type HC12_CHAR = crate::Reg<u32, _HC12_CHAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC12_CHAR;
#[doc = "`read()` method returns [hc12_char::R](hc12_char::R) reader structure"]
impl crate::Readable for HC12_CHAR {}
#[doc = "`write(|w| ..)` method takes [hc12_char::W](hc12_char::W) writer structure"]
impl crate::Writable for HC12_CHAR {}
#[doc = "Host Channel 0 Characteristics Register"]
pub mod hc12_char;
#[doc = "Host Channel 0 Split Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc12_splt](hc12_splt) module"]
pub type HC12_SPLT = crate::Reg<u32, _HC12_SPLT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC12_SPLT;
#[doc = "`read()` method returns [hc12_splt::R](hc12_splt::R) reader structure"]
impl crate::Readable for HC12_SPLT {}
#[doc = "`write(|w| ..)` method takes [hc12_splt::W](hc12_splt::W) writer structure"]
impl crate::Writable for HC12_SPLT {}
#[doc = "Host Channel 0 Split Control Register"]
pub mod hc12_splt;
#[doc = "Host Channel 0 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc12_int](hc12_int) module"]
pub type HC12_INT = crate::Reg<u32, _HC12_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC12_INT;
#[doc = "`read()` method returns [hc12_int::R](hc12_int::R) reader structure"]
impl crate::Readable for HC12_INT {}
#[doc = "`write(|w| ..)` method takes [hc12_int::W](hc12_int::W) writer structure"]
impl crate::Writable for HC12_INT {}
#[doc = "Host Channel 0 Interrupt Register"]
pub mod hc12_int;
#[doc = "Host Channel 0 Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc12_intmsk](hc12_intmsk) module"]
pub type HC12_INTMSK = crate::Reg<u32, _HC12_INTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC12_INTMSK;
#[doc = "`read()` method returns [hc12_intmsk::R](hc12_intmsk::R) reader structure"]
impl crate::Readable for HC12_INTMSK {}
#[doc = "`write(|w| ..)` method takes [hc12_intmsk::W](hc12_intmsk::W) writer structure"]
impl crate::Writable for HC12_INTMSK {}
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub mod hc12_intmsk;
#[doc = "Host Channel 0 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc12_tsiz](hc12_tsiz) module"]
pub type HC12_TSIZ = crate::Reg<u32, _HC12_TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC12_TSIZ;
#[doc = "`read()` method returns [hc12_tsiz::R](hc12_tsiz::R) reader structure"]
impl crate::Readable for HC12_TSIZ {}
#[doc = "`write(|w| ..)` method takes [hc12_tsiz::W](hc12_tsiz::W) writer structure"]
impl crate::Writable for HC12_TSIZ {}
#[doc = "Host Channel 0 Transfer Size Register"]
pub mod hc12_tsiz;
#[doc = "Host Channel 0 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc12_dmaaddr](hc12_dmaaddr) module"]
pub type HC12_DMAADDR = crate::Reg<u32, _HC12_DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC12_DMAADDR;
#[doc = "`read()` method returns [hc12_dmaaddr::R](hc12_dmaaddr::R) reader structure"]
impl crate::Readable for HC12_DMAADDR {}
#[doc = "`write(|w| ..)` method takes [hc12_dmaaddr::W](hc12_dmaaddr::W) writer structure"]
impl crate::Writable for HC12_DMAADDR {}
#[doc = "Host Channel 0 DMA Address Register"]
pub mod hc12_dmaaddr;
#[doc = "Host Channel 0 Characteristics Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc13_char](hc13_char) module"]
pub type HC13_CHAR = crate::Reg<u32, _HC13_CHAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC13_CHAR;
#[doc = "`read()` method returns [hc13_char::R](hc13_char::R) reader structure"]
impl crate::Readable for HC13_CHAR {}
#[doc = "`write(|w| ..)` method takes [hc13_char::W](hc13_char::W) writer structure"]
impl crate::Writable for HC13_CHAR {}
#[doc = "Host Channel 0 Characteristics Register"]
pub mod hc13_char;
#[doc = "Host Channel 0 Split Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc13_splt](hc13_splt) module"]
pub type HC13_SPLT = crate::Reg<u32, _HC13_SPLT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC13_SPLT;
#[doc = "`read()` method returns [hc13_splt::R](hc13_splt::R) reader structure"]
impl crate::Readable for HC13_SPLT {}
#[doc = "`write(|w| ..)` method takes [hc13_splt::W](hc13_splt::W) writer structure"]
impl crate::Writable for HC13_SPLT {}
#[doc = "Host Channel 0 Split Control Register"]
pub mod hc13_splt;
#[doc = "Host Channel 0 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc13_int](hc13_int) module"]
pub type HC13_INT = crate::Reg<u32, _HC13_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC13_INT;
#[doc = "`read()` method returns [hc13_int::R](hc13_int::R) reader structure"]
impl crate::Readable for HC13_INT {}
#[doc = "`write(|w| ..)` method takes [hc13_int::W](hc13_int::W) writer structure"]
impl crate::Writable for HC13_INT {}
#[doc = "Host Channel 0 Interrupt Register"]
pub mod hc13_int;
#[doc = "Host Channel 0 Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc13_intmsk](hc13_intmsk) module"]
pub type HC13_INTMSK = crate::Reg<u32, _HC13_INTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC13_INTMSK;
#[doc = "`read()` method returns [hc13_intmsk::R](hc13_intmsk::R) reader structure"]
impl crate::Readable for HC13_INTMSK {}
#[doc = "`write(|w| ..)` method takes [hc13_intmsk::W](hc13_intmsk::W) writer structure"]
impl crate::Writable for HC13_INTMSK {}
#[doc = "Host Channel 0 Interrupt Mask Register"]
pub mod hc13_intmsk;
#[doc = "Host Channel 0 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc13_tsiz](hc13_tsiz) module"]
pub type HC13_TSIZ = crate::Reg<u32, _HC13_TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC13_TSIZ;
#[doc = "`read()` method returns [hc13_tsiz::R](hc13_tsiz::R) reader structure"]
impl crate::Readable for HC13_TSIZ {}
#[doc = "`write(|w| ..)` method takes [hc13_tsiz::W](hc13_tsiz::W) writer structure"]
impl crate::Writable for HC13_TSIZ {}
#[doc = "Host Channel 0 Transfer Size Register"]
pub mod hc13_tsiz;
#[doc = "Host Channel 0 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hc13_dmaaddr](hc13_dmaaddr) module"]
pub type HC13_DMAADDR = crate::Reg<u32, _HC13_DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HC13_DMAADDR;
#[doc = "`read()` method returns [hc13_dmaaddr::R](hc13_dmaaddr::R) reader structure"]
impl crate::Readable for HC13_DMAADDR {}
#[doc = "`write(|w| ..)` method takes [hc13_dmaaddr::W](hc13_dmaaddr::W) writer structure"]
impl crate::Writable for HC13_DMAADDR {}
#[doc = "Host Channel 0 DMA Address Register"]
pub mod hc13_dmaaddr;
#[doc = "Device Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcfg](dcfg) module"]
pub type DCFG = crate::Reg<u32, _DCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCFG;
#[doc = "`read()` method returns [dcfg::R](dcfg::R) reader structure"]
impl crate::Readable for DCFG {}
#[doc = "`write(|w| ..)` method takes [dcfg::W](dcfg::W) writer structure"]
impl crate::Writable for DCFG {}
#[doc = "Device Configuration Register"]
pub mod dcfg;
#[doc = "Device Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dctl](dctl) module"]
pub type DCTL = crate::Reg<u32, _DCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCTL;
#[doc = "`read()` method returns [dctl::R](dctl::R) reader structure"]
impl crate::Readable for DCTL {}
#[doc = "`write(|w| ..)` method takes [dctl::W](dctl::W) writer structure"]
impl crate::Writable for DCTL {}
#[doc = "Device Control Register"]
pub mod dctl;
#[doc = "Device Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsts](dsts) module"]
pub type DSTS = crate::Reg<u32, _DSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSTS;
#[doc = "`read()` method returns [dsts::R](dsts::R) reader structure"]
impl crate::Readable for DSTS {}
#[doc = "Device Status Register"]
pub mod dsts;
#[doc = "Device IN Endpoint Common Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepmsk](diepmsk) module"]
pub type DIEPMSK = crate::Reg<u32, _DIEPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPMSK;
#[doc = "`read()` method returns [diepmsk::R](diepmsk::R) reader structure"]
impl crate::Readable for DIEPMSK {}
#[doc = "`write(|w| ..)` method takes [diepmsk::W](diepmsk::W) writer structure"]
impl crate::Writable for DIEPMSK {}
#[doc = "Device IN Endpoint Common Interrupt Mask Register"]
pub mod diepmsk;
#[doc = "Device OUT Endpoint Common Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepmsk](doepmsk) module"]
pub type DOEPMSK = crate::Reg<u32, _DOEPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEPMSK;
#[doc = "`read()` method returns [doepmsk::R](doepmsk::R) reader structure"]
impl crate::Readable for DOEPMSK {}
#[doc = "`write(|w| ..)` method takes [doepmsk::W](doepmsk::W) writer structure"]
impl crate::Writable for DOEPMSK {}
#[doc = "Device OUT Endpoint Common Interrupt Mask Register"]
pub mod doepmsk;
#[doc = "Device All Endpoints Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daint](daint) module"]
pub type DAINT = crate::Reg<u32, _DAINT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAINT;
#[doc = "`read()` method returns [daint::R](daint::R) reader structure"]
impl crate::Readable for DAINT {}
#[doc = "Device All Endpoints Interrupt Register"]
pub mod daint;
#[doc = "Device All Endpoints Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daintmsk](daintmsk) module"]
pub type DAINTMSK = crate::Reg<u32, _DAINTMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DAINTMSK;
#[doc = "`read()` method returns [daintmsk::R](daintmsk::R) reader structure"]
impl crate::Readable for DAINTMSK {}
#[doc = "`write(|w| ..)` method takes [daintmsk::W](daintmsk::W) writer structure"]
impl crate::Writable for DAINTMSK {}
#[doc = "Device All Endpoints Interrupt Mask Register"]
pub mod daintmsk;
#[doc = "Device VBUS Discharge Time Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvbusdis](dvbusdis) module"]
pub type DVBUSDIS = crate::Reg<u32, _DVBUSDIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DVBUSDIS;
#[doc = "`read()` method returns [dvbusdis::R](dvbusdis::R) reader structure"]
impl crate::Readable for DVBUSDIS {}
#[doc = "`write(|w| ..)` method takes [dvbusdis::W](dvbusdis::W) writer structure"]
impl crate::Writable for DVBUSDIS {}
#[doc = "Device VBUS Discharge Time Register"]
pub mod dvbusdis;
#[doc = "Device VBUS Pulsing Time Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvbuspulse](dvbuspulse) module"]
pub type DVBUSPULSE = crate::Reg<u32, _DVBUSPULSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DVBUSPULSE;
#[doc = "`read()` method returns [dvbuspulse::R](dvbuspulse::R) reader structure"]
impl crate::Readable for DVBUSPULSE {}
#[doc = "`write(|w| ..)` method takes [dvbuspulse::W](dvbuspulse::W) writer structure"]
impl crate::Writable for DVBUSPULSE {}
#[doc = "Device VBUS Pulsing Time Register"]
pub mod dvbuspulse;
#[doc = "Device Threshold Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dthrctl](dthrctl) module"]
pub type DTHRCTL = crate::Reg<u32, _DTHRCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTHRCTL;
#[doc = "`read()` method returns [dthrctl::R](dthrctl::R) reader structure"]
impl crate::Readable for DTHRCTL {}
#[doc = "`write(|w| ..)` method takes [dthrctl::W](dthrctl::W) writer structure"]
impl crate::Writable for DTHRCTL {}
#[doc = "Device Threshold Control Register"]
pub mod dthrctl;
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diepempmsk](diepempmsk) module"]
pub type DIEPEMPMSK = crate::Reg<u32, _DIEPEMPMSK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEPEMPMSK;
#[doc = "`read()` method returns [diepempmsk::R](diepempmsk::R) reader structure"]
impl crate::Readable for DIEPEMPMSK {}
#[doc = "`write(|w| ..)` method takes [diepempmsk::W](diepempmsk::W) writer structure"]
impl crate::Writable for DIEPEMPMSK {}
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register"]
pub mod diepempmsk;
#[doc = "Device Control IN Endpoint 0 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep0ctl](diep0ctl) module"]
pub type DIEP0CTL = crate::Reg<u32, _DIEP0CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP0CTL;
#[doc = "`read()` method returns [diep0ctl::R](diep0ctl::R) reader structure"]
impl crate::Readable for DIEP0CTL {}
#[doc = "`write(|w| ..)` method takes [diep0ctl::W](diep0ctl::W) writer structure"]
impl crate::Writable for DIEP0CTL {}
#[doc = "Device Control IN Endpoint 0 Control Register"]
pub mod diep0ctl;
#[doc = "Device IN Endpoint 0 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep0int](diep0int) module"]
pub type DIEP0INT = crate::Reg<u32, _DIEP0INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP0INT;
#[doc = "`read()` method returns [diep0int::R](diep0int::R) reader structure"]
impl crate::Readable for DIEP0INT {}
#[doc = "`write(|w| ..)` method takes [diep0int::W](diep0int::W) writer structure"]
impl crate::Writable for DIEP0INT {}
#[doc = "Device IN Endpoint 0 Interrupt Register"]
pub mod diep0int;
#[doc = "Device IN Endpoint 0 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep0tsiz](diep0tsiz) module"]
pub type DIEP0TSIZ = crate::Reg<u32, _DIEP0TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP0TSIZ;
#[doc = "`read()` method returns [diep0tsiz::R](diep0tsiz::R) reader structure"]
impl crate::Readable for DIEP0TSIZ {}
#[doc = "`write(|w| ..)` method takes [diep0tsiz::W](diep0tsiz::W) writer structure"]
impl crate::Writable for DIEP0TSIZ {}
#[doc = "Device IN Endpoint 0 Transfer Size Register"]
pub mod diep0tsiz;
#[doc = "Device IN Endpoint 0 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep0dmaaddr](diep0dmaaddr) module"]
pub type DIEP0DMAADDR = crate::Reg<u32, _DIEP0DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP0DMAADDR;
#[doc = "`read()` method returns [diep0dmaaddr::R](diep0dmaaddr::R) reader structure"]
impl crate::Readable for DIEP0DMAADDR {}
#[doc = "`write(|w| ..)` method takes [diep0dmaaddr::W](diep0dmaaddr::W) writer structure"]
impl crate::Writable for DIEP0DMAADDR {}
#[doc = "Device IN Endpoint 0 DMA Address Register"]
pub mod diep0dmaaddr;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep0txfsts](diep0txfsts) module"]
pub type DIEP0TXFSTS = crate::Reg<u32, _DIEP0TXFSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP0TXFSTS;
#[doc = "`read()` method returns [diep0txfsts::R](diep0txfsts::R) reader structure"]
impl crate::Readable for DIEP0TXFSTS {}
#[doc = "Device IN Endpoint Transmit FIFO Status Register 0"]
pub mod diep0txfsts;
#[doc = "Device Control IN Endpoint 1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep0_ctl](diep0_ctl) module"]
pub type DIEP0_CTL = crate::Reg<u32, _DIEP0_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP0_CTL;
#[doc = "`read()` method returns [diep0_ctl::R](diep0_ctl::R) reader structure"]
impl crate::Readable for DIEP0_CTL {}
#[doc = "`write(|w| ..)` method takes [diep0_ctl::W](diep0_ctl::W) writer structure"]
impl crate::Writable for DIEP0_CTL {}
#[doc = "Device Control IN Endpoint 1 Control Register"]
pub mod diep0_ctl;
#[doc = "Device IN Endpoint 1 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep0_int](diep0_int) module"]
pub type DIEP0_INT = crate::Reg<u32, _DIEP0_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP0_INT;
#[doc = "`read()` method returns [diep0_int::R](diep0_int::R) reader structure"]
impl crate::Readable for DIEP0_INT {}
#[doc = "`write(|w| ..)` method takes [diep0_int::W](diep0_int::W) writer structure"]
impl crate::Writable for DIEP0_INT {}
#[doc = "Device IN Endpoint 1 Interrupt Register"]
pub mod diep0_int;
#[doc = "Device IN Endpoint 1 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep0_tsiz](diep0_tsiz) module"]
pub type DIEP0_TSIZ = crate::Reg<u32, _DIEP0_TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP0_TSIZ;
#[doc = "`read()` method returns [diep0_tsiz::R](diep0_tsiz::R) reader structure"]
impl crate::Readable for DIEP0_TSIZ {}
#[doc = "`write(|w| ..)` method takes [diep0_tsiz::W](diep0_tsiz::W) writer structure"]
impl crate::Writable for DIEP0_TSIZ {}
#[doc = "Device IN Endpoint 1 Transfer Size Register"]
pub mod diep0_tsiz;
#[doc = "Device IN Endpoint 1 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep0_dmaaddr](diep0_dmaaddr) module"]
pub type DIEP0_DMAADDR = crate::Reg<u32, _DIEP0_DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP0_DMAADDR;
#[doc = "`read()` method returns [diep0_dmaaddr::R](diep0_dmaaddr::R) reader structure"]
impl crate::Readable for DIEP0_DMAADDR {}
#[doc = "`write(|w| ..)` method takes [diep0_dmaaddr::W](diep0_dmaaddr::W) writer structure"]
impl crate::Writable for DIEP0_DMAADDR {}
#[doc = "Device IN Endpoint 1 DMA Address Register"]
pub mod diep0_dmaaddr;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep0_dtxfsts](diep0_dtxfsts) module"]
pub type DIEP0_DTXFSTS = crate::Reg<u32, _DIEP0_DTXFSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP0_DTXFSTS;
#[doc = "`read()` method returns [diep0_dtxfsts::R](diep0_dtxfsts::R) reader structure"]
impl crate::Readable for DIEP0_DTXFSTS {}
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1"]
pub mod diep0_dtxfsts;
#[doc = "Device Control IN Endpoint 1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep1_ctl](diep1_ctl) module"]
pub type DIEP1_CTL = crate::Reg<u32, _DIEP1_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP1_CTL;
#[doc = "`read()` method returns [diep1_ctl::R](diep1_ctl::R) reader structure"]
impl crate::Readable for DIEP1_CTL {}
#[doc = "`write(|w| ..)` method takes [diep1_ctl::W](diep1_ctl::W) writer structure"]
impl crate::Writable for DIEP1_CTL {}
#[doc = "Device Control IN Endpoint 1 Control Register"]
pub mod diep1_ctl;
#[doc = "Device IN Endpoint 1 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep1_int](diep1_int) module"]
pub type DIEP1_INT = crate::Reg<u32, _DIEP1_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP1_INT;
#[doc = "`read()` method returns [diep1_int::R](diep1_int::R) reader structure"]
impl crate::Readable for DIEP1_INT {}
#[doc = "`write(|w| ..)` method takes [diep1_int::W](diep1_int::W) writer structure"]
impl crate::Writable for DIEP1_INT {}
#[doc = "Device IN Endpoint 1 Interrupt Register"]
pub mod diep1_int;
#[doc = "Device IN Endpoint 1 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep1_tsiz](diep1_tsiz) module"]
pub type DIEP1_TSIZ = crate::Reg<u32, _DIEP1_TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP1_TSIZ;
#[doc = "`read()` method returns [diep1_tsiz::R](diep1_tsiz::R) reader structure"]
impl crate::Readable for DIEP1_TSIZ {}
#[doc = "`write(|w| ..)` method takes [diep1_tsiz::W](diep1_tsiz::W) writer structure"]
impl crate::Writable for DIEP1_TSIZ {}
#[doc = "Device IN Endpoint 1 Transfer Size Register"]
pub mod diep1_tsiz;
#[doc = "Device IN Endpoint 1 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep1_dmaaddr](diep1_dmaaddr) module"]
pub type DIEP1_DMAADDR = crate::Reg<u32, _DIEP1_DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP1_DMAADDR;
#[doc = "`read()` method returns [diep1_dmaaddr::R](diep1_dmaaddr::R) reader structure"]
impl crate::Readable for DIEP1_DMAADDR {}
#[doc = "`write(|w| ..)` method takes [diep1_dmaaddr::W](diep1_dmaaddr::W) writer structure"]
impl crate::Writable for DIEP1_DMAADDR {}
#[doc = "Device IN Endpoint 1 DMA Address Register"]
pub mod diep1_dmaaddr;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep1_dtxfsts](diep1_dtxfsts) module"]
pub type DIEP1_DTXFSTS = crate::Reg<u32, _DIEP1_DTXFSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP1_DTXFSTS;
#[doc = "`read()` method returns [diep1_dtxfsts::R](diep1_dtxfsts::R) reader structure"]
impl crate::Readable for DIEP1_DTXFSTS {}
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1"]
pub mod diep1_dtxfsts;
#[doc = "Device Control IN Endpoint 1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep2_ctl](diep2_ctl) module"]
pub type DIEP2_CTL = crate::Reg<u32, _DIEP2_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP2_CTL;
#[doc = "`read()` method returns [diep2_ctl::R](diep2_ctl::R) reader structure"]
impl crate::Readable for DIEP2_CTL {}
#[doc = "`write(|w| ..)` method takes [diep2_ctl::W](diep2_ctl::W) writer structure"]
impl crate::Writable for DIEP2_CTL {}
#[doc = "Device Control IN Endpoint 1 Control Register"]
pub mod diep2_ctl;
#[doc = "Device IN Endpoint 1 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep2_int](diep2_int) module"]
pub type DIEP2_INT = crate::Reg<u32, _DIEP2_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP2_INT;
#[doc = "`read()` method returns [diep2_int::R](diep2_int::R) reader structure"]
impl crate::Readable for DIEP2_INT {}
#[doc = "`write(|w| ..)` method takes [diep2_int::W](diep2_int::W) writer structure"]
impl crate::Writable for DIEP2_INT {}
#[doc = "Device IN Endpoint 1 Interrupt Register"]
pub mod diep2_int;
#[doc = "Device IN Endpoint 1 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep2_tsiz](diep2_tsiz) module"]
pub type DIEP2_TSIZ = crate::Reg<u32, _DIEP2_TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP2_TSIZ;
#[doc = "`read()` method returns [diep2_tsiz::R](diep2_tsiz::R) reader structure"]
impl crate::Readable for DIEP2_TSIZ {}
#[doc = "`write(|w| ..)` method takes [diep2_tsiz::W](diep2_tsiz::W) writer structure"]
impl crate::Writable for DIEP2_TSIZ {}
#[doc = "Device IN Endpoint 1 Transfer Size Register"]
pub mod diep2_tsiz;
#[doc = "Device IN Endpoint 1 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep2_dmaaddr](diep2_dmaaddr) module"]
pub type DIEP2_DMAADDR = crate::Reg<u32, _DIEP2_DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP2_DMAADDR;
#[doc = "`read()` method returns [diep2_dmaaddr::R](diep2_dmaaddr::R) reader structure"]
impl crate::Readable for DIEP2_DMAADDR {}
#[doc = "`write(|w| ..)` method takes [diep2_dmaaddr::W](diep2_dmaaddr::W) writer structure"]
impl crate::Writable for DIEP2_DMAADDR {}
#[doc = "Device IN Endpoint 1 DMA Address Register"]
pub mod diep2_dmaaddr;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep2_dtxfsts](diep2_dtxfsts) module"]
pub type DIEP2_DTXFSTS = crate::Reg<u32, _DIEP2_DTXFSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP2_DTXFSTS;
#[doc = "`read()` method returns [diep2_dtxfsts::R](diep2_dtxfsts::R) reader structure"]
impl crate::Readable for DIEP2_DTXFSTS {}
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1"]
pub mod diep2_dtxfsts;
#[doc = "Device Control IN Endpoint 1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep3_ctl](diep3_ctl) module"]
pub type DIEP3_CTL = crate::Reg<u32, _DIEP3_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP3_CTL;
#[doc = "`read()` method returns [diep3_ctl::R](diep3_ctl::R) reader structure"]
impl crate::Readable for DIEP3_CTL {}
#[doc = "`write(|w| ..)` method takes [diep3_ctl::W](diep3_ctl::W) writer structure"]
impl crate::Writable for DIEP3_CTL {}
#[doc = "Device Control IN Endpoint 1 Control Register"]
pub mod diep3_ctl;
#[doc = "Device IN Endpoint 1 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep3_int](diep3_int) module"]
pub type DIEP3_INT = crate::Reg<u32, _DIEP3_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP3_INT;
#[doc = "`read()` method returns [diep3_int::R](diep3_int::R) reader structure"]
impl crate::Readable for DIEP3_INT {}
#[doc = "`write(|w| ..)` method takes [diep3_int::W](diep3_int::W) writer structure"]
impl crate::Writable for DIEP3_INT {}
#[doc = "Device IN Endpoint 1 Interrupt Register"]
pub mod diep3_int;
#[doc = "Device IN Endpoint 1 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep3_tsiz](diep3_tsiz) module"]
pub type DIEP3_TSIZ = crate::Reg<u32, _DIEP3_TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP3_TSIZ;
#[doc = "`read()` method returns [diep3_tsiz::R](diep3_tsiz::R) reader structure"]
impl crate::Readable for DIEP3_TSIZ {}
#[doc = "`write(|w| ..)` method takes [diep3_tsiz::W](diep3_tsiz::W) writer structure"]
impl crate::Writable for DIEP3_TSIZ {}
#[doc = "Device IN Endpoint 1 Transfer Size Register"]
pub mod diep3_tsiz;
#[doc = "Device IN Endpoint 1 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep3_dmaaddr](diep3_dmaaddr) module"]
pub type DIEP3_DMAADDR = crate::Reg<u32, _DIEP3_DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP3_DMAADDR;
#[doc = "`read()` method returns [diep3_dmaaddr::R](diep3_dmaaddr::R) reader structure"]
impl crate::Readable for DIEP3_DMAADDR {}
#[doc = "`write(|w| ..)` method takes [diep3_dmaaddr::W](diep3_dmaaddr::W) writer structure"]
impl crate::Writable for DIEP3_DMAADDR {}
#[doc = "Device IN Endpoint 1 DMA Address Register"]
pub mod diep3_dmaaddr;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep3_dtxfsts](diep3_dtxfsts) module"]
pub type DIEP3_DTXFSTS = crate::Reg<u32, _DIEP3_DTXFSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP3_DTXFSTS;
#[doc = "`read()` method returns [diep3_dtxfsts::R](diep3_dtxfsts::R) reader structure"]
impl crate::Readable for DIEP3_DTXFSTS {}
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1"]
pub mod diep3_dtxfsts;
#[doc = "Device Control IN Endpoint 1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep4_ctl](diep4_ctl) module"]
pub type DIEP4_CTL = crate::Reg<u32, _DIEP4_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP4_CTL;
#[doc = "`read()` method returns [diep4_ctl::R](diep4_ctl::R) reader structure"]
impl crate::Readable for DIEP4_CTL {}
#[doc = "`write(|w| ..)` method takes [diep4_ctl::W](diep4_ctl::W) writer structure"]
impl crate::Writable for DIEP4_CTL {}
#[doc = "Device Control IN Endpoint 1 Control Register"]
pub mod diep4_ctl;
#[doc = "Device IN Endpoint 1 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep4_int](diep4_int) module"]
pub type DIEP4_INT = crate::Reg<u32, _DIEP4_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP4_INT;
#[doc = "`read()` method returns [diep4_int::R](diep4_int::R) reader structure"]
impl crate::Readable for DIEP4_INT {}
#[doc = "`write(|w| ..)` method takes [diep4_int::W](diep4_int::W) writer structure"]
impl crate::Writable for DIEP4_INT {}
#[doc = "Device IN Endpoint 1 Interrupt Register"]
pub mod diep4_int;
#[doc = "Device IN Endpoint 1 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep4_tsiz](diep4_tsiz) module"]
pub type DIEP4_TSIZ = crate::Reg<u32, _DIEP4_TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP4_TSIZ;
#[doc = "`read()` method returns [diep4_tsiz::R](diep4_tsiz::R) reader structure"]
impl crate::Readable for DIEP4_TSIZ {}
#[doc = "`write(|w| ..)` method takes [diep4_tsiz::W](diep4_tsiz::W) writer structure"]
impl crate::Writable for DIEP4_TSIZ {}
#[doc = "Device IN Endpoint 1 Transfer Size Register"]
pub mod diep4_tsiz;
#[doc = "Device IN Endpoint 1 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep4_dmaaddr](diep4_dmaaddr) module"]
pub type DIEP4_DMAADDR = crate::Reg<u32, _DIEP4_DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP4_DMAADDR;
#[doc = "`read()` method returns [diep4_dmaaddr::R](diep4_dmaaddr::R) reader structure"]
impl crate::Readable for DIEP4_DMAADDR {}
#[doc = "`write(|w| ..)` method takes [diep4_dmaaddr::W](diep4_dmaaddr::W) writer structure"]
impl crate::Writable for DIEP4_DMAADDR {}
#[doc = "Device IN Endpoint 1 DMA Address Register"]
pub mod diep4_dmaaddr;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep4_dtxfsts](diep4_dtxfsts) module"]
pub type DIEP4_DTXFSTS = crate::Reg<u32, _DIEP4_DTXFSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP4_DTXFSTS;
#[doc = "`read()` method returns [diep4_dtxfsts::R](diep4_dtxfsts::R) reader structure"]
impl crate::Readable for DIEP4_DTXFSTS {}
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1"]
pub mod diep4_dtxfsts;
#[doc = "Device Control IN Endpoint 1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep5_ctl](diep5_ctl) module"]
pub type DIEP5_CTL = crate::Reg<u32, _DIEP5_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP5_CTL;
#[doc = "`read()` method returns [diep5_ctl::R](diep5_ctl::R) reader structure"]
impl crate::Readable for DIEP5_CTL {}
#[doc = "`write(|w| ..)` method takes [diep5_ctl::W](diep5_ctl::W) writer structure"]
impl crate::Writable for DIEP5_CTL {}
#[doc = "Device Control IN Endpoint 1 Control Register"]
pub mod diep5_ctl;
#[doc = "Device IN Endpoint 1 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep5_int](diep5_int) module"]
pub type DIEP5_INT = crate::Reg<u32, _DIEP5_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP5_INT;
#[doc = "`read()` method returns [diep5_int::R](diep5_int::R) reader structure"]
impl crate::Readable for DIEP5_INT {}
#[doc = "`write(|w| ..)` method takes [diep5_int::W](diep5_int::W) writer structure"]
impl crate::Writable for DIEP5_INT {}
#[doc = "Device IN Endpoint 1 Interrupt Register"]
pub mod diep5_int;
#[doc = "Device IN Endpoint 1 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep5_tsiz](diep5_tsiz) module"]
pub type DIEP5_TSIZ = crate::Reg<u32, _DIEP5_TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP5_TSIZ;
#[doc = "`read()` method returns [diep5_tsiz::R](diep5_tsiz::R) reader structure"]
impl crate::Readable for DIEP5_TSIZ {}
#[doc = "`write(|w| ..)` method takes [diep5_tsiz::W](diep5_tsiz::W) writer structure"]
impl crate::Writable for DIEP5_TSIZ {}
#[doc = "Device IN Endpoint 1 Transfer Size Register"]
pub mod diep5_tsiz;
#[doc = "Device IN Endpoint 1 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep5_dmaaddr](diep5_dmaaddr) module"]
pub type DIEP5_DMAADDR = crate::Reg<u32, _DIEP5_DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP5_DMAADDR;
#[doc = "`read()` method returns [diep5_dmaaddr::R](diep5_dmaaddr::R) reader structure"]
impl crate::Readable for DIEP5_DMAADDR {}
#[doc = "`write(|w| ..)` method takes [diep5_dmaaddr::W](diep5_dmaaddr::W) writer structure"]
impl crate::Writable for DIEP5_DMAADDR {}
#[doc = "Device IN Endpoint 1 DMA Address Register"]
pub mod diep5_dmaaddr;
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep5_dtxfsts](diep5_dtxfsts) module"]
pub type DIEP5_DTXFSTS = crate::Reg<u32, _DIEP5_DTXFSTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIEP5_DTXFSTS;
#[doc = "`read()` method returns [diep5_dtxfsts::R](diep5_dtxfsts::R) reader structure"]
impl crate::Readable for DIEP5_DTXFSTS {}
#[doc = "Device IN Endpoint Transmit FIFO Status Register 1"]
pub mod diep5_dtxfsts;
#[doc = "Device Control OUT Endpoint 0 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep0ctl](doep0ctl) module"]
pub type DOEP0CTL = crate::Reg<u32, _DOEP0CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP0CTL;
#[doc = "`read()` method returns [doep0ctl::R](doep0ctl::R) reader structure"]
impl crate::Readable for DOEP0CTL {}
#[doc = "`write(|w| ..)` method takes [doep0ctl::W](doep0ctl::W) writer structure"]
impl crate::Writable for DOEP0CTL {}
#[doc = "Device Control OUT Endpoint 0 Control Register"]
pub mod doep0ctl;
#[doc = "Device OUT Endpoint 0 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep0int](doep0int) module"]
pub type DOEP0INT = crate::Reg<u32, _DOEP0INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP0INT;
#[doc = "`read()` method returns [doep0int::R](doep0int::R) reader structure"]
impl crate::Readable for DOEP0INT {}
#[doc = "`write(|w| ..)` method takes [doep0int::W](doep0int::W) writer structure"]
impl crate::Writable for DOEP0INT {}
#[doc = "Device OUT Endpoint 0 Interrupt Register"]
pub mod doep0int;
#[doc = "Device OUT Endpoint 0 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep0tsiz](doep0tsiz) module"]
pub type DOEP0TSIZ = crate::Reg<u32, _DOEP0TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP0TSIZ;
#[doc = "`read()` method returns [doep0tsiz::R](doep0tsiz::R) reader structure"]
impl crate::Readable for DOEP0TSIZ {}
#[doc = "`write(|w| ..)` method takes [doep0tsiz::W](doep0tsiz::W) writer structure"]
impl crate::Writable for DOEP0TSIZ {}
#[doc = "Device OUT Endpoint 0 Transfer Size Register"]
pub mod doep0tsiz;
#[doc = "Device OUT Endpoint 0 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep0dmaaddr](doep0dmaaddr) module"]
pub type DOEP0DMAADDR = crate::Reg<u32, _DOEP0DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP0DMAADDR;
#[doc = "`read()` method returns [doep0dmaaddr::R](doep0dmaaddr::R) reader structure"]
impl crate::Readable for DOEP0DMAADDR {}
#[doc = "`write(|w| ..)` method takes [doep0dmaaddr::W](doep0dmaaddr::W) writer structure"]
impl crate::Writable for DOEP0DMAADDR {}
#[doc = "Device OUT Endpoint 0 DMA Address Register"]
pub mod doep0dmaaddr;
#[doc = "Device Control OUT Endpoint 1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep0_ctl](doep0_ctl) module"]
pub type DOEP0_CTL = crate::Reg<u32, _DOEP0_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP0_CTL;
#[doc = "`read()` method returns [doep0_ctl::R](doep0_ctl::R) reader structure"]
impl crate::Readable for DOEP0_CTL {}
#[doc = "`write(|w| ..)` method takes [doep0_ctl::W](doep0_ctl::W) writer structure"]
impl crate::Writable for DOEP0_CTL {}
#[doc = "Device Control OUT Endpoint 1 Control Register"]
pub mod doep0_ctl;
#[doc = "Device OUT Endpoint 1 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep0_int](doep0_int) module"]
pub type DOEP0_INT = crate::Reg<u32, _DOEP0_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP0_INT;
#[doc = "`read()` method returns [doep0_int::R](doep0_int::R) reader structure"]
impl crate::Readable for DOEP0_INT {}
#[doc = "`write(|w| ..)` method takes [doep0_int::W](doep0_int::W) writer structure"]
impl crate::Writable for DOEP0_INT {}
#[doc = "Device OUT Endpoint 1 Interrupt Register"]
pub mod doep0_int;
#[doc = "Device OUT Endpoint 1 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep0_tsiz](doep0_tsiz) module"]
pub type DOEP0_TSIZ = crate::Reg<u32, _DOEP0_TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP0_TSIZ;
#[doc = "`read()` method returns [doep0_tsiz::R](doep0_tsiz::R) reader structure"]
impl crate::Readable for DOEP0_TSIZ {}
#[doc = "`write(|w| ..)` method takes [doep0_tsiz::W](doep0_tsiz::W) writer structure"]
impl crate::Writable for DOEP0_TSIZ {}
#[doc = "Device OUT Endpoint 1 Transfer Size Register"]
pub mod doep0_tsiz;
#[doc = "Device OUT Endpoint 1 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep0_dmaaddr](doep0_dmaaddr) module"]
pub type DOEP0_DMAADDR = crate::Reg<u32, _DOEP0_DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP0_DMAADDR;
#[doc = "`read()` method returns [doep0_dmaaddr::R](doep0_dmaaddr::R) reader structure"]
impl crate::Readable for DOEP0_DMAADDR {}
#[doc = "`write(|w| ..)` method takes [doep0_dmaaddr::W](doep0_dmaaddr::W) writer structure"]
impl crate::Writable for DOEP0_DMAADDR {}
#[doc = "Device OUT Endpoint 1 DMA Address Register"]
pub mod doep0_dmaaddr;
#[doc = "Device Control OUT Endpoint 1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep1_ctl](doep1_ctl) module"]
pub type DOEP1_CTL = crate::Reg<u32, _DOEP1_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP1_CTL;
#[doc = "`read()` method returns [doep1_ctl::R](doep1_ctl::R) reader structure"]
impl crate::Readable for DOEP1_CTL {}
#[doc = "`write(|w| ..)` method takes [doep1_ctl::W](doep1_ctl::W) writer structure"]
impl crate::Writable for DOEP1_CTL {}
#[doc = "Device Control OUT Endpoint 1 Control Register"]
pub mod doep1_ctl;
#[doc = "Device OUT Endpoint 1 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep1_int](doep1_int) module"]
pub type DOEP1_INT = crate::Reg<u32, _DOEP1_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP1_INT;
#[doc = "`read()` method returns [doep1_int::R](doep1_int::R) reader structure"]
impl crate::Readable for DOEP1_INT {}
#[doc = "`write(|w| ..)` method takes [doep1_int::W](doep1_int::W) writer structure"]
impl crate::Writable for DOEP1_INT {}
#[doc = "Device OUT Endpoint 1 Interrupt Register"]
pub mod doep1_int;
#[doc = "Device OUT Endpoint 1 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep1_tsiz](doep1_tsiz) module"]
pub type DOEP1_TSIZ = crate::Reg<u32, _DOEP1_TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP1_TSIZ;
#[doc = "`read()` method returns [doep1_tsiz::R](doep1_tsiz::R) reader structure"]
impl crate::Readable for DOEP1_TSIZ {}
#[doc = "`write(|w| ..)` method takes [doep1_tsiz::W](doep1_tsiz::W) writer structure"]
impl crate::Writable for DOEP1_TSIZ {}
#[doc = "Device OUT Endpoint 1 Transfer Size Register"]
pub mod doep1_tsiz;
#[doc = "Device OUT Endpoint 1 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep1_dmaaddr](doep1_dmaaddr) module"]
pub type DOEP1_DMAADDR = crate::Reg<u32, _DOEP1_DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP1_DMAADDR;
#[doc = "`read()` method returns [doep1_dmaaddr::R](doep1_dmaaddr::R) reader structure"]
impl crate::Readable for DOEP1_DMAADDR {}
#[doc = "`write(|w| ..)` method takes [doep1_dmaaddr::W](doep1_dmaaddr::W) writer structure"]
impl crate::Writable for DOEP1_DMAADDR {}
#[doc = "Device OUT Endpoint 1 DMA Address Register"]
pub mod doep1_dmaaddr;
#[doc = "Device Control OUT Endpoint 1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep2_ctl](doep2_ctl) module"]
pub type DOEP2_CTL = crate::Reg<u32, _DOEP2_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP2_CTL;
#[doc = "`read()` method returns [doep2_ctl::R](doep2_ctl::R) reader structure"]
impl crate::Readable for DOEP2_CTL {}
#[doc = "`write(|w| ..)` method takes [doep2_ctl::W](doep2_ctl::W) writer structure"]
impl crate::Writable for DOEP2_CTL {}
#[doc = "Device Control OUT Endpoint 1 Control Register"]
pub mod doep2_ctl;
#[doc = "Device OUT Endpoint 1 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep2_int](doep2_int) module"]
pub type DOEP2_INT = crate::Reg<u32, _DOEP2_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP2_INT;
#[doc = "`read()` method returns [doep2_int::R](doep2_int::R) reader structure"]
impl crate::Readable for DOEP2_INT {}
#[doc = "`write(|w| ..)` method takes [doep2_int::W](doep2_int::W) writer structure"]
impl crate::Writable for DOEP2_INT {}
#[doc = "Device OUT Endpoint 1 Interrupt Register"]
pub mod doep2_int;
#[doc = "Device OUT Endpoint 1 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep2_tsiz](doep2_tsiz) module"]
pub type DOEP2_TSIZ = crate::Reg<u32, _DOEP2_TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP2_TSIZ;
#[doc = "`read()` method returns [doep2_tsiz::R](doep2_tsiz::R) reader structure"]
impl crate::Readable for DOEP2_TSIZ {}
#[doc = "`write(|w| ..)` method takes [doep2_tsiz::W](doep2_tsiz::W) writer structure"]
impl crate::Writable for DOEP2_TSIZ {}
#[doc = "Device OUT Endpoint 1 Transfer Size Register"]
pub mod doep2_tsiz;
#[doc = "Device OUT Endpoint 1 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep2_dmaaddr](doep2_dmaaddr) module"]
pub type DOEP2_DMAADDR = crate::Reg<u32, _DOEP2_DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP2_DMAADDR;
#[doc = "`read()` method returns [doep2_dmaaddr::R](doep2_dmaaddr::R) reader structure"]
impl crate::Readable for DOEP2_DMAADDR {}
#[doc = "`write(|w| ..)` method takes [doep2_dmaaddr::W](doep2_dmaaddr::W) writer structure"]
impl crate::Writable for DOEP2_DMAADDR {}
#[doc = "Device OUT Endpoint 1 DMA Address Register"]
pub mod doep2_dmaaddr;
#[doc = "Device Control OUT Endpoint 1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep3_ctl](doep3_ctl) module"]
pub type DOEP3_CTL = crate::Reg<u32, _DOEP3_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP3_CTL;
#[doc = "`read()` method returns [doep3_ctl::R](doep3_ctl::R) reader structure"]
impl crate::Readable for DOEP3_CTL {}
#[doc = "`write(|w| ..)` method takes [doep3_ctl::W](doep3_ctl::W) writer structure"]
impl crate::Writable for DOEP3_CTL {}
#[doc = "Device Control OUT Endpoint 1 Control Register"]
pub mod doep3_ctl;
#[doc = "Device OUT Endpoint 1 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep3_int](doep3_int) module"]
pub type DOEP3_INT = crate::Reg<u32, _DOEP3_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP3_INT;
#[doc = "`read()` method returns [doep3_int::R](doep3_int::R) reader structure"]
impl crate::Readable for DOEP3_INT {}
#[doc = "`write(|w| ..)` method takes [doep3_int::W](doep3_int::W) writer structure"]
impl crate::Writable for DOEP3_INT {}
#[doc = "Device OUT Endpoint 1 Interrupt Register"]
pub mod doep3_int;
#[doc = "Device OUT Endpoint 1 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep3_tsiz](doep3_tsiz) module"]
pub type DOEP3_TSIZ = crate::Reg<u32, _DOEP3_TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP3_TSIZ;
#[doc = "`read()` method returns [doep3_tsiz::R](doep3_tsiz::R) reader structure"]
impl crate::Readable for DOEP3_TSIZ {}
#[doc = "`write(|w| ..)` method takes [doep3_tsiz::W](doep3_tsiz::W) writer structure"]
impl crate::Writable for DOEP3_TSIZ {}
#[doc = "Device OUT Endpoint 1 Transfer Size Register"]
pub mod doep3_tsiz;
#[doc = "Device OUT Endpoint 1 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep3_dmaaddr](doep3_dmaaddr) module"]
pub type DOEP3_DMAADDR = crate::Reg<u32, _DOEP3_DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP3_DMAADDR;
#[doc = "`read()` method returns [doep3_dmaaddr::R](doep3_dmaaddr::R) reader structure"]
impl crate::Readable for DOEP3_DMAADDR {}
#[doc = "`write(|w| ..)` method takes [doep3_dmaaddr::W](doep3_dmaaddr::W) writer structure"]
impl crate::Writable for DOEP3_DMAADDR {}
#[doc = "Device OUT Endpoint 1 DMA Address Register"]
pub mod doep3_dmaaddr;
#[doc = "Device Control OUT Endpoint 1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep4_ctl](doep4_ctl) module"]
pub type DOEP4_CTL = crate::Reg<u32, _DOEP4_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP4_CTL;
#[doc = "`read()` method returns [doep4_ctl::R](doep4_ctl::R) reader structure"]
impl crate::Readable for DOEP4_CTL {}
#[doc = "`write(|w| ..)` method takes [doep4_ctl::W](doep4_ctl::W) writer structure"]
impl crate::Writable for DOEP4_CTL {}
#[doc = "Device Control OUT Endpoint 1 Control Register"]
pub mod doep4_ctl;
#[doc = "Device OUT Endpoint 1 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep4_int](doep4_int) module"]
pub type DOEP4_INT = crate::Reg<u32, _DOEP4_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP4_INT;
#[doc = "`read()` method returns [doep4_int::R](doep4_int::R) reader structure"]
impl crate::Readable for DOEP4_INT {}
#[doc = "`write(|w| ..)` method takes [doep4_int::W](doep4_int::W) writer structure"]
impl crate::Writable for DOEP4_INT {}
#[doc = "Device OUT Endpoint 1 Interrupt Register"]
pub mod doep4_int;
#[doc = "Device OUT Endpoint 1 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep4_tsiz](doep4_tsiz) module"]
pub type DOEP4_TSIZ = crate::Reg<u32, _DOEP4_TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP4_TSIZ;
#[doc = "`read()` method returns [doep4_tsiz::R](doep4_tsiz::R) reader structure"]
impl crate::Readable for DOEP4_TSIZ {}
#[doc = "`write(|w| ..)` method takes [doep4_tsiz::W](doep4_tsiz::W) writer structure"]
impl crate::Writable for DOEP4_TSIZ {}
#[doc = "Device OUT Endpoint 1 Transfer Size Register"]
pub mod doep4_tsiz;
#[doc = "Device OUT Endpoint 1 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep4_dmaaddr](doep4_dmaaddr) module"]
pub type DOEP4_DMAADDR = crate::Reg<u32, _DOEP4_DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP4_DMAADDR;
#[doc = "`read()` method returns [doep4_dmaaddr::R](doep4_dmaaddr::R) reader structure"]
impl crate::Readable for DOEP4_DMAADDR {}
#[doc = "`write(|w| ..)` method takes [doep4_dmaaddr::W](doep4_dmaaddr::W) writer structure"]
impl crate::Writable for DOEP4_DMAADDR {}
#[doc = "Device OUT Endpoint 1 DMA Address Register"]
pub mod doep4_dmaaddr;
#[doc = "Device Control OUT Endpoint 1 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep5_ctl](doep5_ctl) module"]
pub type DOEP5_CTL = crate::Reg<u32, _DOEP5_CTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP5_CTL;
#[doc = "`read()` method returns [doep5_ctl::R](doep5_ctl::R) reader structure"]
impl crate::Readable for DOEP5_CTL {}
#[doc = "`write(|w| ..)` method takes [doep5_ctl::W](doep5_ctl::W) writer structure"]
impl crate::Writable for DOEP5_CTL {}
#[doc = "Device Control OUT Endpoint 1 Control Register"]
pub mod doep5_ctl;
#[doc = "Device OUT Endpoint 1 Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep5_int](doep5_int) module"]
pub type DOEP5_INT = crate::Reg<u32, _DOEP5_INT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP5_INT;
#[doc = "`read()` method returns [doep5_int::R](doep5_int::R) reader structure"]
impl crate::Readable for DOEP5_INT {}
#[doc = "`write(|w| ..)` method takes [doep5_int::W](doep5_int::W) writer structure"]
impl crate::Writable for DOEP5_INT {}
#[doc = "Device OUT Endpoint 1 Interrupt Register"]
pub mod doep5_int;
#[doc = "Device OUT Endpoint 1 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep5_tsiz](doep5_tsiz) module"]
pub type DOEP5_TSIZ = crate::Reg<u32, _DOEP5_TSIZ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP5_TSIZ;
#[doc = "`read()` method returns [doep5_tsiz::R](doep5_tsiz::R) reader structure"]
impl crate::Readable for DOEP5_TSIZ {}
#[doc = "`write(|w| ..)` method takes [doep5_tsiz::W](doep5_tsiz::W) writer structure"]
impl crate::Writable for DOEP5_TSIZ {}
#[doc = "Device OUT Endpoint 1 Transfer Size Register"]
pub mod doep5_tsiz;
#[doc = "Device OUT Endpoint 1 DMA Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep5_dmaaddr](doep5_dmaaddr) module"]
pub type DOEP5_DMAADDR = crate::Reg<u32, _DOEP5_DMAADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DOEP5_DMAADDR;
#[doc = "`read()` method returns [doep5_dmaaddr::R](doep5_dmaaddr::R) reader structure"]
impl crate::Readable for DOEP5_DMAADDR {}
#[doc = "`write(|w| ..)` method takes [doep5_dmaaddr::W](doep5_dmaaddr::W) writer structure"]
impl crate::Writable for DOEP5_DMAADDR {}
#[doc = "Device OUT Endpoint 1 DMA Address Register"]
pub mod doep5_dmaaddr;
#[doc = "Power and Clock Gating Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcgcctl](pcgcctl) module"]
pub type PCGCCTL = crate::Reg<u32, _PCGCCTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PCGCCTL;
#[doc = "`read()` method returns [pcgcctl::R](pcgcctl::R) reader structure"]
impl crate::Readable for PCGCCTL {}
#[doc = "`write(|w| ..)` method takes [pcgcctl::W](pcgcctl::W) writer structure"]
impl crate::Writable for PCGCCTL {}
#[doc = "Power and Clock Gating Control Register"]
pub mod pcgcctl;
