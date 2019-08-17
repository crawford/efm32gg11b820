#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Network control register"]
    pub networkctrl: NETWORKCTRL,
    #[doc = "0x04 - Network configuration register"]
    pub networkcfg: NETWORKCFG,
    #[doc = "0x08 - Network status register"]
    pub networkstatus: NETWORKSTATUS,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - DMA Configuration Register"]
    pub dmacfg: DMACFG,
    #[doc = "0x14 - Transmit status register"]
    pub txstatus: TXSTATUS,
    #[doc = "0x18 - Start address of the receive buffer queue"]
    pub rxqptr: RXQPTR,
    #[doc = "0x1c - Start address of the transmit buffer queue"]
    pub txqptr: TXQPTR,
    #[doc = "0x20 - Receive status register"]
    pub rxstatus: RXSTATUS,
    #[doc = "0x24 - Interrupt status register"]
    pub ifcr: IFCR,
    #[doc = "0x28 - Interrupt Enable Register"]
    pub iens: IENS,
    #[doc = "0x2c - Interrupt Disable Register"]
    pub ienc: IENC,
    #[doc = "0x30 - Interrupt mask register"]
    pub ienro: IENRO,
    #[doc = "0x34 - PHY management register"]
    pub phymngmnt: PHYMNGMNT,
    #[doc = "0x38 - Received Pause Quantum Register"]
    pub rxpausequant: RXPAUSEQUANT,
    #[doc = "0x3c - Transmit Pause Quantum Register"]
    pub txpausequant: TXPAUSEQUANT,
    #[doc = "0x40 - TX Partial Store and Forward"]
    pub pbuftxcutthru: PBUFTXCUTTHRU,
    #[doc = "0x44 - RX Partial Store and Forward"]
    pub pbufrxcutthru: PBUFRXCUTTHRU,
    #[doc = "0x48 - Maximum Jumbo Frame Size."]
    pub jumbomaxlen: JUMBOMAXLEN,
    _reserved18: [u8; 16usize],
    #[doc = "0x5c - Interrupt moderation register"]
    pub imod: IMOD,
    #[doc = "0x60 - System wake time"]
    pub syswaketime: SYSWAKETIME,
    _reserved20: [u8; 28usize],
    #[doc = "0x80 - Hash Register Bottom \\[31:0\\]"]
    pub hashbottom: HASHBOTTOM,
    #[doc = "0x84 - Hash Register Top \\[63:32\\]"]
    pub hashtop: HASHTOP,
    #[doc = "0x88 - Specific Address 1 Bottom"]
    pub specaddr1bottom: SPECADDR1BOTTOM,
    #[doc = "0x8c - Specific Address 1 Top"]
    pub specaddr1top: SPECADDR1TOP,
    #[doc = "0x90 - Specific Address 2 Bottom"]
    pub specaddr2bottom: SPECADDR2BOTTOM,
    #[doc = "0x94 - Specific Address 2 Top"]
    pub specaddr2top: SPECADDR2TOP,
    #[doc = "0x98 - Specific Address 3 Bottom"]
    pub specaddr3bottom: SPECADDR3BOTTOM,
    #[doc = "0x9c - Specific Address 3 Top"]
    pub specaddr3top: SPECADDR3TOP,
    #[doc = "0xa0 - Specific Address 4 Bottom"]
    pub specaddr4bottom: SPECADDR4BOTTOM,
    #[doc = "0xa4 - Specific Address 4 Top"]
    pub specaddr4top: SPECADDR4TOP,
    #[doc = "0xa8 - Type ID Match 1"]
    pub spectype1: SPECTYPE1,
    #[doc = "0xac - Type ID Match 2"]
    pub spectype2: SPECTYPE2,
    #[doc = "0xb0 - Type ID Match 3"]
    pub spectype3: SPECTYPE3,
    #[doc = "0xb4 - Type ID Match 4"]
    pub spectype4: SPECTYPE4,
    #[doc = "0xb8 - Wake on LAN Register"]
    pub wolreg: WOLREG,
    #[doc = "0xbc - IPG stretch register"]
    pub stretchratio: STRETCHRATIO,
    #[doc = "0xc0 - Stacked VLAN Register"]
    pub stackedvlan: STACKEDVLAN,
    #[doc = "0xc4 - Transmit PFC Pause Register"]
    pub txpfcpause: TXPFCPAUSE,
    #[doc = "0xc8 - Specific Address Mask 1 Bottom 31:0"]
    pub maskadd1bottom: MASKADD1BOTTOM,
    #[doc = "0xcc - Specific Address Mask 1 Top 47:32"]
    pub maskadd1top: MASKADD1TOP,
    _reserved40: [u8; 4usize],
    #[doc = "0xd4 - PTP RX unicast IP destination address"]
    pub rxptpunicast: RXPTPUNICAST,
    #[doc = "0xd8 - PTP TX unicast IP destination address"]
    pub txptpunicast: TXPTPUNICAST,
    #[doc = "0xdc - TSU timer comparison value nanoseconds"]
    pub tsunseccmp: TSUNSECCMP,
    #[doc = "0xe0 - TSU timer comparison value seconds \\[31:0\\]"]
    pub tsuseccmp: TSUSECCMP,
    #[doc = "0xe4 - TSU timer comparison value seconds \\[47:32\\]"]
    pub tsumsbseccmp: TSUMSBSECCMP,
    #[doc = "0xe8 - PTP Event Frame Transmitted Seconds Register 47:32"]
    pub tsuptptxmsbsec: TSUPTPTXMSBSEC,
    #[doc = "0xec - PTP Event Frame Received Seconds Register 47:32"]
    pub tsuptprxmsbsec: TSUPTPRXMSBSEC,
    #[doc = "0xf0 - PTP Peer Event Frame Transmitted Seconds Register 47:32"]
    pub tsupeertxmsbsec: TSUPEERTXMSBSEC,
    #[doc = "0xf4 - PTP Peer Event Frame Received Seconds Register 47:32"]
    pub tsupeerrxmsbsec: TSUPEERRXMSBSEC,
    _reserved49: [u8; 8usize],
    #[doc = "0x100 - Octets transmitted 31:0"]
    pub octetstxedbottom: OCTETSTXEDBOTTOM,
    #[doc = "0x104 - Octets Transmitted 47:32"]
    pub octetstxedtop: OCTETSTXEDTOP,
    #[doc = "0x108 - Frames Transmitted"]
    pub framestxedok: FRAMESTXEDOK,
    #[doc = "0x10c - Broadcast Frames Transmitted"]
    pub broadcasttxed: BROADCASTTXED,
    #[doc = "0x110 - Multicast Frames Transmitted"]
    pub multicasttxed: MULTICASTTXED,
    #[doc = "0x114 - Pause Frames Transmitted"]
    pub pframestxed: PFRAMESTXED,
    #[doc = "0x118 - 64 Byte Frames Transmitted"]
    pub framestxed64: FRAMESTXED64,
    #[doc = "0x11c - 65 to 127 Byte Frames Transmitted"]
    pub framestxed65: FRAMESTXED65,
    #[doc = "0x120 - 128 to 255 Byte Frames Transmitted"]
    pub framestxed128: FRAMESTXED128,
    #[doc = "0x124 - 256 to 511 Byte Frames Transmitted"]
    pub framestxed256: FRAMESTXED256,
    #[doc = "0x128 - 512 to 1023 Byte Frames Transmitted"]
    pub framestxed512: FRAMESTXED512,
    #[doc = "0x12c - 1024 to 1518 Byte Frames Transmitted"]
    pub framestxed1024: FRAMESTXED1024,
    #[doc = "0x130 - Greater Than 1518 Byte Frames Transmitted"]
    pub framestxed1519: FRAMESTXED1519,
    #[doc = "0x134 - Transmit Under Runs"]
    pub txunderruns: TXUNDERRUNS,
    #[doc = "0x138 - Single Collision Frames"]
    pub singlecols: SINGLECOLS,
    #[doc = "0x13c - Multiple Collision Frames"]
    pub multicols: MULTICOLS,
    #[doc = "0x140 - Excessive Collisions"]
    pub excesscols: EXCESSCOLS,
    #[doc = "0x144 - Late Collisions"]
    pub latecols: LATECOLS,
    #[doc = "0x148 - Deferred Transmission Frames"]
    pub deferredframes: DEFERREDFRAMES,
    #[doc = "0x14c - Carrier Sense Errors"]
    pub crserrs: CRSERRS,
    #[doc = "0x150 - Octets Received 31:0"]
    pub octetsrxedbottom: OCTETSRXEDBOTTOM,
    #[doc = "0x154 - Octets Received 47:32"]
    pub octetsrxedtop: OCTETSRXEDTOP,
    #[doc = "0x158 - Frames Received"]
    pub framesrxedok: FRAMESRXEDOK,
    #[doc = "0x15c - Broadcast Frames Received"]
    pub broadcastrxed: BROADCASTRXED,
    #[doc = "0x160 - Multicast Frames Received"]
    pub multicastrxed: MULTICASTRXED,
    #[doc = "0x164 - Pause Frames Received"]
    pub pframesrxed: PFRAMESRXED,
    #[doc = "0x168 - 64 Byte Frames Received"]
    pub framesrxed64: FRAMESRXED64,
    #[doc = "0x16c - 65 to 127 Byte Frames Received"]
    pub framesrxed65: FRAMESRXED65,
    #[doc = "0x170 - 128 to 255 Byte Frames Received"]
    pub framesrxed128: FRAMESRXED128,
    #[doc = "0x174 - 256 to 511 Byte Frames Received"]
    pub framesrxed256: FRAMESRXED256,
    #[doc = "0x178 - 512 to 1023 Byte Frames Received"]
    pub framesrxed512: FRAMESRXED512,
    #[doc = "0x17c - 1024 to 1518 Byte Frames Received"]
    pub framesrxed1024: FRAMESRXED1024,
    #[doc = "0x180 - 1519 to maximum Byte Frames Received"]
    pub framesrxed1519: FRAMESRXED1519,
    #[doc = "0x184 - Undersized Frames Received"]
    pub undersizeframes: UNDERSIZEFRAMES,
    #[doc = "0x188 - Oversize Frames Received"]
    pub excessiverxlen: EXCESSIVERXLEN,
    #[doc = "0x18c - Jabbers Received"]
    pub rxjabbers: RXJABBERS,
    #[doc = "0x190 - Frame Check Sequence Errors"]
    pub fcserrs: FCSERRS,
    #[doc = "0x194 - Length Field Frame Errors"]
    pub rxlenerrs: RXLENERRS,
    #[doc = "0x198 - Receive Symbol Errors"]
    pub rxsymbolerrs: RXSYMBOLERRS,
    #[doc = "0x19c - Alignment Errors"]
    pub alignerrs: ALIGNERRS,
    #[doc = "0x1a0 - Receive Resource Errors"]
    pub rxresourceerrs: RXRESOURCEERRS,
    #[doc = "0x1a4 - Receive Overruns"]
    pub rxoverruns: RXOVERRUNS,
    #[doc = "0x1a8 - IP Header Checksum Errors"]
    pub rxipckerrs: RXIPCKERRS,
    #[doc = "0x1ac - TCP Checksum Errors"]
    pub rxtcpckerrs: RXTCPCKERRS,
    #[doc = "0x1b0 - UDP Checksum Errors"]
    pub rxudpckerrs: RXUDPCKERRS,
    #[doc = "0x1b4 - Receive DMA Flushed Packets"]
    pub autoflushedpkts: AUTOFLUSHEDPKTS,
    _reserved95: [u8; 4usize],
    #[doc = "0x1bc - 1588 Timer Increment Register subscript nsec"]
    pub tsutimerincrsubnsec: TSUTIMERINCRSUBNSEC,
    #[doc = "0x1c0 - 1588 Timer Seconds Register 47:32"]
    pub tsutimermsbsec: TSUTIMERMSBSEC,
    _reserved97: [u8; 12usize],
    #[doc = "0x1d0 - 1588 Timer Seconds Register 31:0"]
    pub tsutimersec: TSUTIMERSEC,
    #[doc = "0x1d4 - 1588 Timer Nanoseconds Register"]
    pub tsutimernsec: TSUTIMERNSEC,
    #[doc = "0x1d8 - This register returns all zeroes when read."]
    pub tsutimeradjust: TSUTIMERADJUST,
    #[doc = "0x1dc - 1588 Timer Increment Register"]
    pub tsutimerincr: TSUTIMERINCR,
    #[doc = "0x1e0 - PTP Event Frame Transmitted Seconds Register 31:0"]
    pub tsuptptxsec: TSUPTPTXSEC,
    #[doc = "0x1e4 - PTP Event Frame Transmitted Nanoseconds Register"]
    pub tsuptptxnsec: TSUPTPTXNSEC,
    #[doc = "0x1e8 - PTP Event Frame Received Seconds Register 31:0"]
    pub tsuptprxsec: TSUPTPRXSEC,
    #[doc = "0x1ec - PTP Event Frame Received Nanoseconds Register"]
    pub tsuptprxnsec: TSUPTPRXNSEC,
    #[doc = "0x1f0 - PTP Peer Event Frame Transmitted Seconds Register 31:0"]
    pub tsupeertxsec: TSUPEERTXSEC,
    #[doc = "0x1f4 - PTP Peer Event Frame Transmitted Nanoseconds Register"]
    pub tsupeertxnsec: TSUPEERTXNSEC,
    #[doc = "0x1f8 - PTP Peer Event Frame Received Seconds Register 31:0"]
    pub tsupeerrxsec: TSUPEERRXSEC,
    #[doc = "0x1fc - PTP Peer Event Frame Received Nanoseconds Register"]
    pub tsupeerrxnsec: TSUPEERRXNSEC,
    _reserved109: [u8; 96usize],
    #[doc = "0x260 - Transmit Pause Quantum Register 1"]
    pub txpausequant1: TXPAUSEQUANT1,
    #[doc = "0x264 - Transmit Pause Quantum Register 2"]
    pub txpausequant2: TXPAUSEQUANT2,
    #[doc = "0x268 - Transmit Pause Quantum Register 3"]
    pub txpausequant3: TXPAUSEQUANT3,
    _reserved112: [u8; 4usize],
    #[doc = "0x270 - Received LPI transitions"]
    pub rxlpi: RXLPI,
    #[doc = "0x274 - Received LPI time"]
    pub rxlpitime: RXLPITIME,
    #[doc = "0x278 - Transmit LPI transitions"]
    pub txlpi: TXLPI,
    #[doc = "0x27c - Transmit LPI time"]
    pub txlpitime: TXLPITIME,
    _reserved116: [u8; 588usize],
    #[doc = "0x4cc - TX BD control register"]
    pub txbdctrl: TXBDCTRL,
    #[doc = "0x4d0 - RX BD control register"]
    pub rxbdctrl: RXBDCTRL,
    _reserved118: [u8; 1836usize],
    #[doc = "0xc00 - I/O Route Enable Register"]
    pub routepen: ROUTEPEN,
    #[doc = "0xc04 - I/O Route Location Register 0"]
    pub routeloc0: ROUTELOC0,
    _reserved120: [u8; 4usize],
    #[doc = "0xc0c - I/O Route Location Register 1"]
    pub routeloc1: ROUTELOC1,
    #[doc = "0xc10 - Ethernet control register"]
    pub ctrl: CTRL,
}
#[doc = "Network control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [networkctrl](networkctrl) module"]
pub type NETWORKCTRL = crate::Reg<u32, _NETWORKCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NETWORKCTRL;
#[doc = "`read()` method returns [networkctrl::R](networkctrl::R) reader structure"]
impl crate::Readable for NETWORKCTRL {}
#[doc = "`write(|w| ..)` method takes [networkctrl::W](networkctrl::W) writer structure"]
impl crate::Writable for NETWORKCTRL {}
#[doc = "Network control register"]
pub mod networkctrl;
#[doc = "Network configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [networkcfg](networkcfg) module"]
pub type NETWORKCFG = crate::Reg<u32, _NETWORKCFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NETWORKCFG;
#[doc = "`read()` method returns [networkcfg::R](networkcfg::R) reader structure"]
impl crate::Readable for NETWORKCFG {}
#[doc = "`write(|w| ..)` method takes [networkcfg::W](networkcfg::W) writer structure"]
impl crate::Writable for NETWORKCFG {}
#[doc = "Network configuration register"]
pub mod networkcfg;
#[doc = "Network status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [networkstatus](networkstatus) module"]
pub type NETWORKSTATUS = crate::Reg<u32, _NETWORKSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NETWORKSTATUS;
#[doc = "`read()` method returns [networkstatus::R](networkstatus::R) reader structure"]
impl crate::Readable for NETWORKSTATUS {}
#[doc = "Network status register"]
pub mod networkstatus;
#[doc = "DMA Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dmacfg](dmacfg) module"]
pub type DMACFG = crate::Reg<u32, _DMACFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DMACFG;
#[doc = "`read()` method returns [dmacfg::R](dmacfg::R) reader structure"]
impl crate::Readable for DMACFG {}
#[doc = "`write(|w| ..)` method takes [dmacfg::W](dmacfg::W) writer structure"]
impl crate::Writable for DMACFG {}
#[doc = "DMA Configuration Register"]
pub mod dmacfg;
#[doc = "Transmit status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txstatus](txstatus) module"]
pub type TXSTATUS = crate::Reg<u32, _TXSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXSTATUS;
#[doc = "`read()` method returns [txstatus::R](txstatus::R) reader structure"]
impl crate::Readable for TXSTATUS {}
#[doc = "`write(|w| ..)` method takes [txstatus::W](txstatus::W) writer structure"]
impl crate::Writable for TXSTATUS {}
#[doc = "Transmit status register"]
pub mod txstatus;
#[doc = "Start address of the receive buffer queue\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxqptr](rxqptr) module"]
pub type RXQPTR = crate::Reg<u32, _RXQPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXQPTR;
#[doc = "`read()` method returns [rxqptr::R](rxqptr::R) reader structure"]
impl crate::Readable for RXQPTR {}
#[doc = "`write(|w| ..)` method takes [rxqptr::W](rxqptr::W) writer structure"]
impl crate::Writable for RXQPTR {}
#[doc = "Start address of the receive buffer queue"]
pub mod rxqptr;
#[doc = "Start address of the transmit buffer queue\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txqptr](txqptr) module"]
pub type TXQPTR = crate::Reg<u32, _TXQPTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXQPTR;
#[doc = "`read()` method returns [txqptr::R](txqptr::R) reader structure"]
impl crate::Readable for TXQPTR {}
#[doc = "`write(|w| ..)` method takes [txqptr::W](txqptr::W) writer structure"]
impl crate::Writable for TXQPTR {}
#[doc = "Start address of the transmit buffer queue"]
pub mod txqptr;
#[doc = "Receive status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxstatus](rxstatus) module"]
pub type RXSTATUS = crate::Reg<u32, _RXSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXSTATUS;
#[doc = "`read()` method returns [rxstatus::R](rxstatus::R) reader structure"]
impl crate::Readable for RXSTATUS {}
#[doc = "`write(|w| ..)` method takes [rxstatus::W](rxstatus::W) writer structure"]
impl crate::Writable for RXSTATUS {}
#[doc = "Receive status register"]
pub mod rxstatus;
#[doc = "Interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ifcr](ifcr) module"]
pub type IFCR = crate::Reg<u32, _IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFCR;
#[doc = "`read()` method returns [ifcr::R](ifcr::R) reader structure"]
impl crate::Readable for IFCR {}
#[doc = "`write(|w| ..)` method takes [ifcr::W](ifcr::W) writer structure"]
impl crate::Writable for IFCR {}
#[doc = "Interrupt status register"]
pub mod ifcr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [iens](iens) module"]
pub type IENS = crate::Reg<u32, _IENS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IENS;
#[doc = "`write(|w| ..)` method takes [iens::W](iens::W) writer structure"]
impl crate::Writable for IENS {}
#[doc = "Interrupt Enable Register"]
pub mod iens;
#[doc = "Interrupt Disable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ienc](ienc) module"]
pub type IENC = crate::Reg<u32, _IENC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IENC;
#[doc = "`write(|w| ..)` method takes [ienc::W](ienc::W) writer structure"]
impl crate::Writable for IENC {}
#[doc = "Interrupt Disable Register"]
pub mod ienc;
#[doc = "Interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ienro](ienro) module"]
pub type IENRO = crate::Reg<u32, _IENRO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IENRO;
#[doc = "`read()` method returns [ienro::R](ienro::R) reader structure"]
impl crate::Readable for IENRO {}
#[doc = "`write(|w| ..)` method takes [ienro::W](ienro::W) writer structure"]
impl crate::Writable for IENRO {}
#[doc = "Interrupt mask register"]
pub mod ienro;
#[doc = "PHY management register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [phymngmnt](phymngmnt) module"]
pub type PHYMNGMNT = crate::Reg<u32, _PHYMNGMNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PHYMNGMNT;
#[doc = "`read()` method returns [phymngmnt::R](phymngmnt::R) reader structure"]
impl crate::Readable for PHYMNGMNT {}
#[doc = "`write(|w| ..)` method takes [phymngmnt::W](phymngmnt::W) writer structure"]
impl crate::Writable for PHYMNGMNT {}
#[doc = "PHY management register"]
pub mod phymngmnt;
#[doc = "Received Pause Quantum Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxpausequant](rxpausequant) module"]
pub type RXPAUSEQUANT = crate::Reg<u32, _RXPAUSEQUANT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXPAUSEQUANT;
#[doc = "`read()` method returns [rxpausequant::R](rxpausequant::R) reader structure"]
impl crate::Readable for RXPAUSEQUANT {}
#[doc = "Received Pause Quantum Register"]
pub mod rxpausequant;
#[doc = "Transmit Pause Quantum Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txpausequant](txpausequant) module"]
pub type TXPAUSEQUANT = crate::Reg<u32, _TXPAUSEQUANT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXPAUSEQUANT;
#[doc = "`read()` method returns [txpausequant::R](txpausequant::R) reader structure"]
impl crate::Readable for TXPAUSEQUANT {}
#[doc = "`write(|w| ..)` method takes [txpausequant::W](txpausequant::W) writer structure"]
impl crate::Writable for TXPAUSEQUANT {}
#[doc = "Transmit Pause Quantum Register"]
pub mod txpausequant;
#[doc = "TX Partial Store and Forward\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pbuftxcutthru](pbuftxcutthru) module"]
pub type PBUFTXCUTTHRU = crate::Reg<u32, _PBUFTXCUTTHRU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBUFTXCUTTHRU;
#[doc = "`read()` method returns [pbuftxcutthru::R](pbuftxcutthru::R) reader structure"]
impl crate::Readable for PBUFTXCUTTHRU {}
#[doc = "`write(|w| ..)` method takes [pbuftxcutthru::W](pbuftxcutthru::W) writer structure"]
impl crate::Writable for PBUFTXCUTTHRU {}
#[doc = "TX Partial Store and Forward"]
pub mod pbuftxcutthru;
#[doc = "RX Partial Store and Forward\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pbufrxcutthru](pbufrxcutthru) module"]
pub type PBUFRXCUTTHRU = crate::Reg<u32, _PBUFRXCUTTHRU>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PBUFRXCUTTHRU;
#[doc = "`read()` method returns [pbufrxcutthru::R](pbufrxcutthru::R) reader structure"]
impl crate::Readable for PBUFRXCUTTHRU {}
#[doc = "`write(|w| ..)` method takes [pbufrxcutthru::W](pbufrxcutthru::W) writer structure"]
impl crate::Writable for PBUFRXCUTTHRU {}
#[doc = "RX Partial Store and Forward"]
pub mod pbufrxcutthru;
#[doc = "Maximum Jumbo Frame Size.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [jumbomaxlen](jumbomaxlen) module"]
pub type JUMBOMAXLEN = crate::Reg<u32, _JUMBOMAXLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _JUMBOMAXLEN;
#[doc = "`read()` method returns [jumbomaxlen::R](jumbomaxlen::R) reader structure"]
impl crate::Readable for JUMBOMAXLEN {}
#[doc = "`write(|w| ..)` method takes [jumbomaxlen::W](jumbomaxlen::W) writer structure"]
impl crate::Writable for JUMBOMAXLEN {}
#[doc = "Maximum Jumbo Frame Size."]
pub mod jumbomaxlen;
#[doc = "Interrupt moderation register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [imod](imod) module"]
pub type IMOD = crate::Reg<u32, _IMOD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMOD;
#[doc = "`read()` method returns [imod::R](imod::R) reader structure"]
impl crate::Readable for IMOD {}
#[doc = "`write(|w| ..)` method takes [imod::W](imod::W) writer structure"]
impl crate::Writable for IMOD {}
#[doc = "Interrupt moderation register"]
pub mod imod;
#[doc = "System wake time\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syswaketime](syswaketime) module"]
pub type SYSWAKETIME = crate::Reg<u32, _SYSWAKETIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSWAKETIME;
#[doc = "`read()` method returns [syswaketime::R](syswaketime::R) reader structure"]
impl crate::Readable for SYSWAKETIME {}
#[doc = "`write(|w| ..)` method takes [syswaketime::W](syswaketime::W) writer structure"]
impl crate::Writable for SYSWAKETIME {}
#[doc = "System wake time"]
pub mod syswaketime;
#[doc = "Hash Register Bottom \\[31:0\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hashbottom](hashbottom) module"]
pub type HASHBOTTOM = crate::Reg<u32, _HASHBOTTOM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHBOTTOM;
#[doc = "`read()` method returns [hashbottom::R](hashbottom::R) reader structure"]
impl crate::Readable for HASHBOTTOM {}
#[doc = "`write(|w| ..)` method takes [hashbottom::W](hashbottom::W) writer structure"]
impl crate::Writable for HASHBOTTOM {}
#[doc = "Hash Register Bottom \\[31:0\\]"]
pub mod hashbottom;
#[doc = "Hash Register Top \\[63:32\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hashtop](hashtop) module"]
pub type HASHTOP = crate::Reg<u32, _HASHTOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HASHTOP;
#[doc = "`read()` method returns [hashtop::R](hashtop::R) reader structure"]
impl crate::Readable for HASHTOP {}
#[doc = "`write(|w| ..)` method takes [hashtop::W](hashtop::W) writer structure"]
impl crate::Writable for HASHTOP {}
#[doc = "Hash Register Top \\[63:32\\]"]
pub mod hashtop;
#[doc = "Specific Address 1 Bottom\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [specaddr1bottom](specaddr1bottom) module"]
pub type SPECADDR1BOTTOM = crate::Reg<u32, _SPECADDR1BOTTOM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPECADDR1BOTTOM;
#[doc = "`read()` method returns [specaddr1bottom::R](specaddr1bottom::R) reader structure"]
impl crate::Readable for SPECADDR1BOTTOM {}
#[doc = "`write(|w| ..)` method takes [specaddr1bottom::W](specaddr1bottom::W) writer structure"]
impl crate::Writable for SPECADDR1BOTTOM {}
#[doc = "Specific Address 1 Bottom"]
pub mod specaddr1bottom;
#[doc = "Specific Address 1 Top\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [specaddr1top](specaddr1top) module"]
pub type SPECADDR1TOP = crate::Reg<u32, _SPECADDR1TOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPECADDR1TOP;
#[doc = "`read()` method returns [specaddr1top::R](specaddr1top::R) reader structure"]
impl crate::Readable for SPECADDR1TOP {}
#[doc = "`write(|w| ..)` method takes [specaddr1top::W](specaddr1top::W) writer structure"]
impl crate::Writable for SPECADDR1TOP {}
#[doc = "Specific Address 1 Top"]
pub mod specaddr1top;
#[doc = "Specific Address 2 Bottom\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [specaddr2bottom](specaddr2bottom) module"]
pub type SPECADDR2BOTTOM = crate::Reg<u32, _SPECADDR2BOTTOM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPECADDR2BOTTOM;
#[doc = "`read()` method returns [specaddr2bottom::R](specaddr2bottom::R) reader structure"]
impl crate::Readable for SPECADDR2BOTTOM {}
#[doc = "`write(|w| ..)` method takes [specaddr2bottom::W](specaddr2bottom::W) writer structure"]
impl crate::Writable for SPECADDR2BOTTOM {}
#[doc = "Specific Address 2 Bottom"]
pub mod specaddr2bottom;
#[doc = "Specific Address 2 Top\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [specaddr2top](specaddr2top) module"]
pub type SPECADDR2TOP = crate::Reg<u32, _SPECADDR2TOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPECADDR2TOP;
#[doc = "`read()` method returns [specaddr2top::R](specaddr2top::R) reader structure"]
impl crate::Readable for SPECADDR2TOP {}
#[doc = "`write(|w| ..)` method takes [specaddr2top::W](specaddr2top::W) writer structure"]
impl crate::Writable for SPECADDR2TOP {}
#[doc = "Specific Address 2 Top"]
pub mod specaddr2top;
#[doc = "Specific Address 3 Bottom\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [specaddr3bottom](specaddr3bottom) module"]
pub type SPECADDR3BOTTOM = crate::Reg<u32, _SPECADDR3BOTTOM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPECADDR3BOTTOM;
#[doc = "`read()` method returns [specaddr3bottom::R](specaddr3bottom::R) reader structure"]
impl crate::Readable for SPECADDR3BOTTOM {}
#[doc = "`write(|w| ..)` method takes [specaddr3bottom::W](specaddr3bottom::W) writer structure"]
impl crate::Writable for SPECADDR3BOTTOM {}
#[doc = "Specific Address 3 Bottom"]
pub mod specaddr3bottom;
#[doc = "Specific Address 3 Top\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [specaddr3top](specaddr3top) module"]
pub type SPECADDR3TOP = crate::Reg<u32, _SPECADDR3TOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPECADDR3TOP;
#[doc = "`read()` method returns [specaddr3top::R](specaddr3top::R) reader structure"]
impl crate::Readable for SPECADDR3TOP {}
#[doc = "`write(|w| ..)` method takes [specaddr3top::W](specaddr3top::W) writer structure"]
impl crate::Writable for SPECADDR3TOP {}
#[doc = "Specific Address 3 Top"]
pub mod specaddr3top;
#[doc = "Specific Address 4 Bottom\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [specaddr4bottom](specaddr4bottom) module"]
pub type SPECADDR4BOTTOM = crate::Reg<u32, _SPECADDR4BOTTOM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPECADDR4BOTTOM;
#[doc = "`read()` method returns [specaddr4bottom::R](specaddr4bottom::R) reader structure"]
impl crate::Readable for SPECADDR4BOTTOM {}
#[doc = "`write(|w| ..)` method takes [specaddr4bottom::W](specaddr4bottom::W) writer structure"]
impl crate::Writable for SPECADDR4BOTTOM {}
#[doc = "Specific Address 4 Bottom"]
pub mod specaddr4bottom;
#[doc = "Specific Address 4 Top\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [specaddr4top](specaddr4top) module"]
pub type SPECADDR4TOP = crate::Reg<u32, _SPECADDR4TOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPECADDR4TOP;
#[doc = "`read()` method returns [specaddr4top::R](specaddr4top::R) reader structure"]
impl crate::Readable for SPECADDR4TOP {}
#[doc = "`write(|w| ..)` method takes [specaddr4top::W](specaddr4top::W) writer structure"]
impl crate::Writable for SPECADDR4TOP {}
#[doc = "Specific Address 4 Top"]
pub mod specaddr4top;
#[doc = "Type ID Match 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spectype1](spectype1) module"]
pub type SPECTYPE1 = crate::Reg<u32, _SPECTYPE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPECTYPE1;
#[doc = "`read()` method returns [spectype1::R](spectype1::R) reader structure"]
impl crate::Readable for SPECTYPE1 {}
#[doc = "`write(|w| ..)` method takes [spectype1::W](spectype1::W) writer structure"]
impl crate::Writable for SPECTYPE1 {}
#[doc = "Type ID Match 1"]
pub mod spectype1;
#[doc = "Type ID Match 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spectype2](spectype2) module"]
pub type SPECTYPE2 = crate::Reg<u32, _SPECTYPE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPECTYPE2;
#[doc = "`read()` method returns [spectype2::R](spectype2::R) reader structure"]
impl crate::Readable for SPECTYPE2 {}
#[doc = "`write(|w| ..)` method takes [spectype2::W](spectype2::W) writer structure"]
impl crate::Writable for SPECTYPE2 {}
#[doc = "Type ID Match 2"]
pub mod spectype2;
#[doc = "Type ID Match 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spectype3](spectype3) module"]
pub type SPECTYPE3 = crate::Reg<u32, _SPECTYPE3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPECTYPE3;
#[doc = "`read()` method returns [spectype3::R](spectype3::R) reader structure"]
impl crate::Readable for SPECTYPE3 {}
#[doc = "`write(|w| ..)` method takes [spectype3::W](spectype3::W) writer structure"]
impl crate::Writable for SPECTYPE3 {}
#[doc = "Type ID Match 3"]
pub mod spectype3;
#[doc = "Type ID Match 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [spectype4](spectype4) module"]
pub type SPECTYPE4 = crate::Reg<u32, _SPECTYPE4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPECTYPE4;
#[doc = "`read()` method returns [spectype4::R](spectype4::R) reader structure"]
impl crate::Readable for SPECTYPE4 {}
#[doc = "`write(|w| ..)` method takes [spectype4::W](spectype4::W) writer structure"]
impl crate::Writable for SPECTYPE4 {}
#[doc = "Type ID Match 4"]
pub mod spectype4;
#[doc = "Wake on LAN Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wolreg](wolreg) module"]
pub type WOLREG = crate::Reg<u32, _WOLREG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WOLREG;
#[doc = "`read()` method returns [wolreg::R](wolreg::R) reader structure"]
impl crate::Readable for WOLREG {}
#[doc = "`write(|w| ..)` method takes [wolreg::W](wolreg::W) writer structure"]
impl crate::Writable for WOLREG {}
#[doc = "Wake on LAN Register"]
pub mod wolreg;
#[doc = "IPG stretch register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stretchratio](stretchratio) module"]
pub type STRETCHRATIO = crate::Reg<u32, _STRETCHRATIO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STRETCHRATIO;
#[doc = "`read()` method returns [stretchratio::R](stretchratio::R) reader structure"]
impl crate::Readable for STRETCHRATIO {}
#[doc = "`write(|w| ..)` method takes [stretchratio::W](stretchratio::W) writer structure"]
impl crate::Writable for STRETCHRATIO {}
#[doc = "IPG stretch register"]
pub mod stretchratio;
#[doc = "Stacked VLAN Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [stackedvlan](stackedvlan) module"]
pub type STACKEDVLAN = crate::Reg<u32, _STACKEDVLAN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STACKEDVLAN;
#[doc = "`read()` method returns [stackedvlan::R](stackedvlan::R) reader structure"]
impl crate::Readable for STACKEDVLAN {}
#[doc = "`write(|w| ..)` method takes [stackedvlan::W](stackedvlan::W) writer structure"]
impl crate::Writable for STACKEDVLAN {}
#[doc = "Stacked VLAN Register"]
pub mod stackedvlan;
#[doc = "Transmit PFC Pause Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txpfcpause](txpfcpause) module"]
pub type TXPFCPAUSE = crate::Reg<u32, _TXPFCPAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXPFCPAUSE;
#[doc = "`read()` method returns [txpfcpause::R](txpfcpause::R) reader structure"]
impl crate::Readable for TXPFCPAUSE {}
#[doc = "`write(|w| ..)` method takes [txpfcpause::W](txpfcpause::W) writer structure"]
impl crate::Writable for TXPFCPAUSE {}
#[doc = "Transmit PFC Pause Register"]
pub mod txpfcpause;
#[doc = "Specific Address Mask 1 Bottom 31:0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [maskadd1bottom](maskadd1bottom) module"]
pub type MASKADD1BOTTOM = crate::Reg<u32, _MASKADD1BOTTOM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASKADD1BOTTOM;
#[doc = "`read()` method returns [maskadd1bottom::R](maskadd1bottom::R) reader structure"]
impl crate::Readable for MASKADD1BOTTOM {}
#[doc = "`write(|w| ..)` method takes [maskadd1bottom::W](maskadd1bottom::W) writer structure"]
impl crate::Writable for MASKADD1BOTTOM {}
#[doc = "Specific Address Mask 1 Bottom 31:0"]
pub mod maskadd1bottom;
#[doc = "Specific Address Mask 1 Top 47:32\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [maskadd1top](maskadd1top) module"]
pub type MASKADD1TOP = crate::Reg<u32, _MASKADD1TOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASKADD1TOP;
#[doc = "`read()` method returns [maskadd1top::R](maskadd1top::R) reader structure"]
impl crate::Readable for MASKADD1TOP {}
#[doc = "`write(|w| ..)` method takes [maskadd1top::W](maskadd1top::W) writer structure"]
impl crate::Writable for MASKADD1TOP {}
#[doc = "Specific Address Mask 1 Top 47:32"]
pub mod maskadd1top;
#[doc = "PTP RX unicast IP destination address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxptpunicast](rxptpunicast) module"]
pub type RXPTPUNICAST = crate::Reg<u32, _RXPTPUNICAST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXPTPUNICAST;
#[doc = "`read()` method returns [rxptpunicast::R](rxptpunicast::R) reader structure"]
impl crate::Readable for RXPTPUNICAST {}
#[doc = "`write(|w| ..)` method takes [rxptpunicast::W](rxptpunicast::W) writer structure"]
impl crate::Writable for RXPTPUNICAST {}
#[doc = "PTP RX unicast IP destination address"]
pub mod rxptpunicast;
#[doc = "PTP TX unicast IP destination address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txptpunicast](txptpunicast) module"]
pub type TXPTPUNICAST = crate::Reg<u32, _TXPTPUNICAST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXPTPUNICAST;
#[doc = "`read()` method returns [txptpunicast::R](txptpunicast::R) reader structure"]
impl crate::Readable for TXPTPUNICAST {}
#[doc = "`write(|w| ..)` method takes [txptpunicast::W](txptpunicast::W) writer structure"]
impl crate::Writable for TXPTPUNICAST {}
#[doc = "PTP TX unicast IP destination address"]
pub mod txptpunicast;
#[doc = "TSU timer comparison value nanoseconds\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tsunseccmp](tsunseccmp) module"]
pub type TSUNSECCMP = crate::Reg<u32, _TSUNSECCMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSUNSECCMP;
#[doc = "`read()` method returns [tsunseccmp::R](tsunseccmp::R) reader structure"]
impl crate::Readable for TSUNSECCMP {}
#[doc = "`write(|w| ..)` method takes [tsunseccmp::W](tsunseccmp::W) writer structure"]
impl crate::Writable for TSUNSECCMP {}
#[doc = "TSU timer comparison value nanoseconds"]
pub mod tsunseccmp;
#[doc = "TSU timer comparison value seconds \\[31:0\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tsuseccmp](tsuseccmp) module"]
pub type TSUSECCMP = crate::Reg<u32, _TSUSECCMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSUSECCMP;
#[doc = "`read()` method returns [tsuseccmp::R](tsuseccmp::R) reader structure"]
impl crate::Readable for TSUSECCMP {}
#[doc = "`write(|w| ..)` method takes [tsuseccmp::W](tsuseccmp::W) writer structure"]
impl crate::Writable for TSUSECCMP {}
#[doc = "TSU timer comparison value seconds \\[31:0\\]"]
pub mod tsuseccmp;
#[doc = "TSU timer comparison value seconds \\[47:32\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tsumsbseccmp](tsumsbseccmp) module"]
pub type TSUMSBSECCMP = crate::Reg<u32, _TSUMSBSECCMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSUMSBSECCMP;
#[doc = "`read()` method returns [tsumsbseccmp::R](tsumsbseccmp::R) reader structure"]
impl crate::Readable for TSUMSBSECCMP {}
#[doc = "`write(|w| ..)` method takes [tsumsbseccmp::W](tsumsbseccmp::W) writer structure"]
impl crate::Writable for TSUMSBSECCMP {}
#[doc = "TSU timer comparison value seconds \\[47:32\\]"]
pub mod tsumsbseccmp;
#[doc = "PTP Event Frame Transmitted Seconds Register 47:32\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tsuptptxmsbsec](tsuptptxmsbsec) module"]
pub type TSUPTPTXMSBSEC = crate::Reg<u32, _TSUPTPTXMSBSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSUPTPTXMSBSEC;
#[doc = "`read()` method returns [tsuptptxmsbsec::R](tsuptptxmsbsec::R) reader structure"]
impl crate::Readable for TSUPTPTXMSBSEC {}
#[doc = "PTP Event Frame Transmitted Seconds Register 47:32"]
pub mod tsuptptxmsbsec;
#[doc = "PTP Event Frame Received Seconds Register 47:32\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tsuptprxmsbsec](tsuptprxmsbsec) module"]
pub type TSUPTPRXMSBSEC = crate::Reg<u32, _TSUPTPRXMSBSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSUPTPRXMSBSEC;
#[doc = "`read()` method returns [tsuptprxmsbsec::R](tsuptprxmsbsec::R) reader structure"]
impl crate::Readable for TSUPTPRXMSBSEC {}
#[doc = "PTP Event Frame Received Seconds Register 47:32"]
pub mod tsuptprxmsbsec;
#[doc = "PTP Peer Event Frame Transmitted Seconds Register 47:32\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tsupeertxmsbsec](tsupeertxmsbsec) module"]
pub type TSUPEERTXMSBSEC = crate::Reg<u32, _TSUPEERTXMSBSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSUPEERTXMSBSEC;
#[doc = "`read()` method returns [tsupeertxmsbsec::R](tsupeertxmsbsec::R) reader structure"]
impl crate::Readable for TSUPEERTXMSBSEC {}
#[doc = "PTP Peer Event Frame Transmitted Seconds Register 47:32"]
pub mod tsupeertxmsbsec;
#[doc = "PTP Peer Event Frame Received Seconds Register 47:32\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tsupeerrxmsbsec](tsupeerrxmsbsec) module"]
pub type TSUPEERRXMSBSEC = crate::Reg<u32, _TSUPEERRXMSBSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSUPEERRXMSBSEC;
#[doc = "`read()` method returns [tsupeerrxmsbsec::R](tsupeerrxmsbsec::R) reader structure"]
impl crate::Readable for TSUPEERRXMSBSEC {}
#[doc = "PTP Peer Event Frame Received Seconds Register 47:32"]
pub mod tsupeerrxmsbsec;
#[doc = "Octets transmitted 31:0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [octetstxedbottom](octetstxedbottom) module"]
pub type OCTETSTXEDBOTTOM = crate::Reg<u32, _OCTETSTXEDBOTTOM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OCTETSTXEDBOTTOM;
#[doc = "`read()` method returns [octetstxedbottom::R](octetstxedbottom::R) reader structure"]
impl crate::Readable for OCTETSTXEDBOTTOM {}
#[doc = "`write(|w| ..)` method takes [octetstxedbottom::W](octetstxedbottom::W) writer structure"]
impl crate::Writable for OCTETSTXEDBOTTOM {}
#[doc = "Octets transmitted 31:0"]
pub mod octetstxedbottom;
#[doc = "Octets Transmitted 47:32\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [octetstxedtop](octetstxedtop) module"]
pub type OCTETSTXEDTOP = crate::Reg<u32, _OCTETSTXEDTOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OCTETSTXEDTOP;
#[doc = "`read()` method returns [octetstxedtop::R](octetstxedtop::R) reader structure"]
impl crate::Readable for OCTETSTXEDTOP {}
#[doc = "`write(|w| ..)` method takes [octetstxedtop::W](octetstxedtop::W) writer structure"]
impl crate::Writable for OCTETSTXEDTOP {}
#[doc = "Octets Transmitted 47:32"]
pub mod octetstxedtop;
#[doc = "Frames Transmitted\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [framestxedok](framestxedok) module"]
pub type FRAMESTXEDOK = crate::Reg<u32, _FRAMESTXEDOK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRAMESTXEDOK;
#[doc = "`read()` method returns [framestxedok::R](framestxedok::R) reader structure"]
impl crate::Readable for FRAMESTXEDOK {}
#[doc = "`write(|w| ..)` method takes [framestxedok::W](framestxedok::W) writer structure"]
impl crate::Writable for FRAMESTXEDOK {}
#[doc = "Frames Transmitted"]
pub mod framestxedok;
#[doc = "Broadcast Frames Transmitted\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [broadcasttxed](broadcasttxed) module"]
pub type BROADCASTTXED = crate::Reg<u32, _BROADCASTTXED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BROADCASTTXED;
#[doc = "`read()` method returns [broadcasttxed::R](broadcasttxed::R) reader structure"]
impl crate::Readable for BROADCASTTXED {}
#[doc = "`write(|w| ..)` method takes [broadcasttxed::W](broadcasttxed::W) writer structure"]
impl crate::Writable for BROADCASTTXED {}
#[doc = "Broadcast Frames Transmitted"]
pub mod broadcasttxed;
#[doc = "Multicast Frames Transmitted\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [multicasttxed](multicasttxed) module"]
pub type MULTICASTTXED = crate::Reg<u32, _MULTICASTTXED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MULTICASTTXED;
#[doc = "`read()` method returns [multicasttxed::R](multicasttxed::R) reader structure"]
impl crate::Readable for MULTICASTTXED {}
#[doc = "`write(|w| ..)` method takes [multicasttxed::W](multicasttxed::W) writer structure"]
impl crate::Writable for MULTICASTTXED {}
#[doc = "Multicast Frames Transmitted"]
pub mod multicasttxed;
#[doc = "Pause Frames Transmitted\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pframestxed](pframestxed) module"]
pub type PFRAMESTXED = crate::Reg<u32, _PFRAMESTXED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PFRAMESTXED;
#[doc = "`read()` method returns [pframestxed::R](pframestxed::R) reader structure"]
impl crate::Readable for PFRAMESTXED {}
#[doc = "`write(|w| ..)` method takes [pframestxed::W](pframestxed::W) writer structure"]
impl crate::Writable for PFRAMESTXED {}
#[doc = "Pause Frames Transmitted"]
pub mod pframestxed;
#[doc = "64 Byte Frames Transmitted\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [framestxed64](framestxed64) module"]
pub type FRAMESTXED64 = crate::Reg<u32, _FRAMESTXED64>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRAMESTXED64;
#[doc = "`read()` method returns [framestxed64::R](framestxed64::R) reader structure"]
impl crate::Readable for FRAMESTXED64 {}
#[doc = "`write(|w| ..)` method takes [framestxed64::W](framestxed64::W) writer structure"]
impl crate::Writable for FRAMESTXED64 {}
#[doc = "64 Byte Frames Transmitted"]
pub mod framestxed64;
#[doc = "65 to 127 Byte Frames Transmitted\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [framestxed65](framestxed65) module"]
pub type FRAMESTXED65 = crate::Reg<u32, _FRAMESTXED65>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRAMESTXED65;
#[doc = "`read()` method returns [framestxed65::R](framestxed65::R) reader structure"]
impl crate::Readable for FRAMESTXED65 {}
#[doc = "`write(|w| ..)` method takes [framestxed65::W](framestxed65::W) writer structure"]
impl crate::Writable for FRAMESTXED65 {}
#[doc = "65 to 127 Byte Frames Transmitted"]
pub mod framestxed65;
#[doc = "128 to 255 Byte Frames Transmitted\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [framestxed128](framestxed128) module"]
pub type FRAMESTXED128 = crate::Reg<u32, _FRAMESTXED128>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRAMESTXED128;
#[doc = "`read()` method returns [framestxed128::R](framestxed128::R) reader structure"]
impl crate::Readable for FRAMESTXED128 {}
#[doc = "`write(|w| ..)` method takes [framestxed128::W](framestxed128::W) writer structure"]
impl crate::Writable for FRAMESTXED128 {}
#[doc = "128 to 255 Byte Frames Transmitted"]
pub mod framestxed128;
#[doc = "256 to 511 Byte Frames Transmitted\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [framestxed256](framestxed256) module"]
pub type FRAMESTXED256 = crate::Reg<u32, _FRAMESTXED256>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRAMESTXED256;
#[doc = "`read()` method returns [framestxed256::R](framestxed256::R) reader structure"]
impl crate::Readable for FRAMESTXED256 {}
#[doc = "`write(|w| ..)` method takes [framestxed256::W](framestxed256::W) writer structure"]
impl crate::Writable for FRAMESTXED256 {}
#[doc = "256 to 511 Byte Frames Transmitted"]
pub mod framestxed256;
#[doc = "512 to 1023 Byte Frames Transmitted\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [framestxed512](framestxed512) module"]
pub type FRAMESTXED512 = crate::Reg<u32, _FRAMESTXED512>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRAMESTXED512;
#[doc = "`read()` method returns [framestxed512::R](framestxed512::R) reader structure"]
impl crate::Readable for FRAMESTXED512 {}
#[doc = "`write(|w| ..)` method takes [framestxed512::W](framestxed512::W) writer structure"]
impl crate::Writable for FRAMESTXED512 {}
#[doc = "512 to 1023 Byte Frames Transmitted"]
pub mod framestxed512;
#[doc = "1024 to 1518 Byte Frames Transmitted\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [framestxed1024](framestxed1024) module"]
pub type FRAMESTXED1024 = crate::Reg<u32, _FRAMESTXED1024>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRAMESTXED1024;
#[doc = "`read()` method returns [framestxed1024::R](framestxed1024::R) reader structure"]
impl crate::Readable for FRAMESTXED1024 {}
#[doc = "`write(|w| ..)` method takes [framestxed1024::W](framestxed1024::W) writer structure"]
impl crate::Writable for FRAMESTXED1024 {}
#[doc = "1024 to 1518 Byte Frames Transmitted"]
pub mod framestxed1024;
#[doc = "Greater Than 1518 Byte Frames Transmitted\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [framestxed1519](framestxed1519) module"]
pub type FRAMESTXED1519 = crate::Reg<u32, _FRAMESTXED1519>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRAMESTXED1519;
#[doc = "`read()` method returns [framestxed1519::R](framestxed1519::R) reader structure"]
impl crate::Readable for FRAMESTXED1519 {}
#[doc = "`write(|w| ..)` method takes [framestxed1519::W](framestxed1519::W) writer structure"]
impl crate::Writable for FRAMESTXED1519 {}
#[doc = "Greater Than 1518 Byte Frames Transmitted"]
pub mod framestxed1519;
#[doc = "Transmit Under Runs\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txunderruns](txunderruns) module"]
pub type TXUNDERRUNS = crate::Reg<u32, _TXUNDERRUNS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXUNDERRUNS;
#[doc = "`read()` method returns [txunderruns::R](txunderruns::R) reader structure"]
impl crate::Readable for TXUNDERRUNS {}
#[doc = "`write(|w| ..)` method takes [txunderruns::W](txunderruns::W) writer structure"]
impl crate::Writable for TXUNDERRUNS {}
#[doc = "Transmit Under Runs"]
pub mod txunderruns;
#[doc = "Single Collision Frames\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [singlecols](singlecols) module"]
pub type SINGLECOLS = crate::Reg<u32, _SINGLECOLS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SINGLECOLS;
#[doc = "`read()` method returns [singlecols::R](singlecols::R) reader structure"]
impl crate::Readable for SINGLECOLS {}
#[doc = "`write(|w| ..)` method takes [singlecols::W](singlecols::W) writer structure"]
impl crate::Writable for SINGLECOLS {}
#[doc = "Single Collision Frames"]
pub mod singlecols;
#[doc = "Multiple Collision Frames\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [multicols](multicols) module"]
pub type MULTICOLS = crate::Reg<u32, _MULTICOLS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MULTICOLS;
#[doc = "`read()` method returns [multicols::R](multicols::R) reader structure"]
impl crate::Readable for MULTICOLS {}
#[doc = "`write(|w| ..)` method takes [multicols::W](multicols::W) writer structure"]
impl crate::Writable for MULTICOLS {}
#[doc = "Multiple Collision Frames"]
pub mod multicols;
#[doc = "Excessive Collisions\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [excesscols](excesscols) module"]
pub type EXCESSCOLS = crate::Reg<u32, _EXCESSCOLS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXCESSCOLS;
#[doc = "`read()` method returns [excesscols::R](excesscols::R) reader structure"]
impl crate::Readable for EXCESSCOLS {}
#[doc = "`write(|w| ..)` method takes [excesscols::W](excesscols::W) writer structure"]
impl crate::Writable for EXCESSCOLS {}
#[doc = "Excessive Collisions"]
pub mod excesscols;
#[doc = "Late Collisions\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [latecols](latecols) module"]
pub type LATECOLS = crate::Reg<u32, _LATECOLS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LATECOLS;
#[doc = "`read()` method returns [latecols::R](latecols::R) reader structure"]
impl crate::Readable for LATECOLS {}
#[doc = "`write(|w| ..)` method takes [latecols::W](latecols::W) writer structure"]
impl crate::Writable for LATECOLS {}
#[doc = "Late Collisions"]
pub mod latecols;
#[doc = "Deferred Transmission Frames\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [deferredframes](deferredframes) module"]
pub type DEFERREDFRAMES = crate::Reg<u32, _DEFERREDFRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DEFERREDFRAMES;
#[doc = "`read()` method returns [deferredframes::R](deferredframes::R) reader structure"]
impl crate::Readable for DEFERREDFRAMES {}
#[doc = "`write(|w| ..)` method takes [deferredframes::W](deferredframes::W) writer structure"]
impl crate::Writable for DEFERREDFRAMES {}
#[doc = "Deferred Transmission Frames"]
pub mod deferredframes;
#[doc = "Carrier Sense Errors\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [crserrs](crserrs) module"]
pub type CRSERRS = crate::Reg<u32, _CRSERRS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRSERRS;
#[doc = "`read()` method returns [crserrs::R](crserrs::R) reader structure"]
impl crate::Readable for CRSERRS {}
#[doc = "`write(|w| ..)` method takes [crserrs::W](crserrs::W) writer structure"]
impl crate::Writable for CRSERRS {}
#[doc = "Carrier Sense Errors"]
pub mod crserrs;
#[doc = "Octets Received 31:0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [octetsrxedbottom](octetsrxedbottom) module"]
pub type OCTETSRXEDBOTTOM = crate::Reg<u32, _OCTETSRXEDBOTTOM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OCTETSRXEDBOTTOM;
#[doc = "`read()` method returns [octetsrxedbottom::R](octetsrxedbottom::R) reader structure"]
impl crate::Readable for OCTETSRXEDBOTTOM {}
#[doc = "`write(|w| ..)` method takes [octetsrxedbottom::W](octetsrxedbottom::W) writer structure"]
impl crate::Writable for OCTETSRXEDBOTTOM {}
#[doc = "Octets Received 31:0"]
pub mod octetsrxedbottom;
#[doc = "Octets Received 47:32\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [octetsrxedtop](octetsrxedtop) module"]
pub type OCTETSRXEDTOP = crate::Reg<u32, _OCTETSRXEDTOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OCTETSRXEDTOP;
#[doc = "`read()` method returns [octetsrxedtop::R](octetsrxedtop::R) reader structure"]
impl crate::Readable for OCTETSRXEDTOP {}
#[doc = "`write(|w| ..)` method takes [octetsrxedtop::W](octetsrxedtop::W) writer structure"]
impl crate::Writable for OCTETSRXEDTOP {}
#[doc = "Octets Received 47:32"]
pub mod octetsrxedtop;
#[doc = "Frames Received\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [framesrxedok](framesrxedok) module"]
pub type FRAMESRXEDOK = crate::Reg<u32, _FRAMESRXEDOK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRAMESRXEDOK;
#[doc = "`read()` method returns [framesrxedok::R](framesrxedok::R) reader structure"]
impl crate::Readable for FRAMESRXEDOK {}
#[doc = "`write(|w| ..)` method takes [framesrxedok::W](framesrxedok::W) writer structure"]
impl crate::Writable for FRAMESRXEDOK {}
#[doc = "Frames Received"]
pub mod framesrxedok;
#[doc = "Broadcast Frames Received\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [broadcastrxed](broadcastrxed) module"]
pub type BROADCASTRXED = crate::Reg<u32, _BROADCASTRXED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BROADCASTRXED;
#[doc = "`read()` method returns [broadcastrxed::R](broadcastrxed::R) reader structure"]
impl crate::Readable for BROADCASTRXED {}
#[doc = "`write(|w| ..)` method takes [broadcastrxed::W](broadcastrxed::W) writer structure"]
impl crate::Writable for BROADCASTRXED {}
#[doc = "Broadcast Frames Received"]
pub mod broadcastrxed;
#[doc = "Multicast Frames Received\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [multicastrxed](multicastrxed) module"]
pub type MULTICASTRXED = crate::Reg<u32, _MULTICASTRXED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MULTICASTRXED;
#[doc = "`read()` method returns [multicastrxed::R](multicastrxed::R) reader structure"]
impl crate::Readable for MULTICASTRXED {}
#[doc = "`write(|w| ..)` method takes [multicastrxed::W](multicastrxed::W) writer structure"]
impl crate::Writable for MULTICASTRXED {}
#[doc = "Multicast Frames Received"]
pub mod multicastrxed;
#[doc = "Pause Frames Received\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [pframesrxed](pframesrxed) module"]
pub type PFRAMESRXED = crate::Reg<u32, _PFRAMESRXED>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PFRAMESRXED;
#[doc = "`read()` method returns [pframesrxed::R](pframesrxed::R) reader structure"]
impl crate::Readable for PFRAMESRXED {}
#[doc = "`write(|w| ..)` method takes [pframesrxed::W](pframesrxed::W) writer structure"]
impl crate::Writable for PFRAMESRXED {}
#[doc = "Pause Frames Received"]
pub mod pframesrxed;
#[doc = "64 Byte Frames Received\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [framesrxed64](framesrxed64) module"]
pub type FRAMESRXED64 = crate::Reg<u32, _FRAMESRXED64>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRAMESRXED64;
#[doc = "`read()` method returns [framesrxed64::R](framesrxed64::R) reader structure"]
impl crate::Readable for FRAMESRXED64 {}
#[doc = "`write(|w| ..)` method takes [framesrxed64::W](framesrxed64::W) writer structure"]
impl crate::Writable for FRAMESRXED64 {}
#[doc = "64 Byte Frames Received"]
pub mod framesrxed64;
#[doc = "65 to 127 Byte Frames Received\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [framesrxed65](framesrxed65) module"]
pub type FRAMESRXED65 = crate::Reg<u32, _FRAMESRXED65>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRAMESRXED65;
#[doc = "`read()` method returns [framesrxed65::R](framesrxed65::R) reader structure"]
impl crate::Readable for FRAMESRXED65 {}
#[doc = "`write(|w| ..)` method takes [framesrxed65::W](framesrxed65::W) writer structure"]
impl crate::Writable for FRAMESRXED65 {}
#[doc = "65 to 127 Byte Frames Received"]
pub mod framesrxed65;
#[doc = "128 to 255 Byte Frames Received\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [framesrxed128](framesrxed128) module"]
pub type FRAMESRXED128 = crate::Reg<u32, _FRAMESRXED128>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRAMESRXED128;
#[doc = "`read()` method returns [framesrxed128::R](framesrxed128::R) reader structure"]
impl crate::Readable for FRAMESRXED128 {}
#[doc = "`write(|w| ..)` method takes [framesrxed128::W](framesrxed128::W) writer structure"]
impl crate::Writable for FRAMESRXED128 {}
#[doc = "128 to 255 Byte Frames Received"]
pub mod framesrxed128;
#[doc = "256 to 511 Byte Frames Received\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [framesrxed256](framesrxed256) module"]
pub type FRAMESRXED256 = crate::Reg<u32, _FRAMESRXED256>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRAMESRXED256;
#[doc = "`read()` method returns [framesrxed256::R](framesrxed256::R) reader structure"]
impl crate::Readable for FRAMESRXED256 {}
#[doc = "`write(|w| ..)` method takes [framesrxed256::W](framesrxed256::W) writer structure"]
impl crate::Writable for FRAMESRXED256 {}
#[doc = "256 to 511 Byte Frames Received"]
pub mod framesrxed256;
#[doc = "512 to 1023 Byte Frames Received\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [framesrxed512](framesrxed512) module"]
pub type FRAMESRXED512 = crate::Reg<u32, _FRAMESRXED512>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRAMESRXED512;
#[doc = "`read()` method returns [framesrxed512::R](framesrxed512::R) reader structure"]
impl crate::Readable for FRAMESRXED512 {}
#[doc = "`write(|w| ..)` method takes [framesrxed512::W](framesrxed512::W) writer structure"]
impl crate::Writable for FRAMESRXED512 {}
#[doc = "512 to 1023 Byte Frames Received"]
pub mod framesrxed512;
#[doc = "1024 to 1518 Byte Frames Received\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [framesrxed1024](framesrxed1024) module"]
pub type FRAMESRXED1024 = crate::Reg<u32, _FRAMESRXED1024>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRAMESRXED1024;
#[doc = "`read()` method returns [framesrxed1024::R](framesrxed1024::R) reader structure"]
impl crate::Readable for FRAMESRXED1024 {}
#[doc = "`write(|w| ..)` method takes [framesrxed1024::W](framesrxed1024::W) writer structure"]
impl crate::Writable for FRAMESRXED1024 {}
#[doc = "1024 to 1518 Byte Frames Received"]
pub mod framesrxed1024;
#[doc = "1519 to maximum Byte Frames Received\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [framesrxed1519](framesrxed1519) module"]
pub type FRAMESRXED1519 = crate::Reg<u32, _FRAMESRXED1519>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRAMESRXED1519;
#[doc = "`read()` method returns [framesrxed1519::R](framesrxed1519::R) reader structure"]
impl crate::Readable for FRAMESRXED1519 {}
#[doc = "`write(|w| ..)` method takes [framesrxed1519::W](framesrxed1519::W) writer structure"]
impl crate::Writable for FRAMESRXED1519 {}
#[doc = "1519 to maximum Byte Frames Received"]
pub mod framesrxed1519;
#[doc = "Undersized Frames Received\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [undersizeframes](undersizeframes) module"]
pub type UNDERSIZEFRAMES = crate::Reg<u32, _UNDERSIZEFRAMES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _UNDERSIZEFRAMES;
#[doc = "`read()` method returns [undersizeframes::R](undersizeframes::R) reader structure"]
impl crate::Readable for UNDERSIZEFRAMES {}
#[doc = "`write(|w| ..)` method takes [undersizeframes::W](undersizeframes::W) writer structure"]
impl crate::Writable for UNDERSIZEFRAMES {}
#[doc = "Undersized Frames Received"]
pub mod undersizeframes;
#[doc = "Oversize Frames Received\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [excessiverxlen](excessiverxlen) module"]
pub type EXCESSIVERXLEN = crate::Reg<u32, _EXCESSIVERXLEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EXCESSIVERXLEN;
#[doc = "`read()` method returns [excessiverxlen::R](excessiverxlen::R) reader structure"]
impl crate::Readable for EXCESSIVERXLEN {}
#[doc = "`write(|w| ..)` method takes [excessiverxlen::W](excessiverxlen::W) writer structure"]
impl crate::Writable for EXCESSIVERXLEN {}
#[doc = "Oversize Frames Received"]
pub mod excessiverxlen;
#[doc = "Jabbers Received\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxjabbers](rxjabbers) module"]
pub type RXJABBERS = crate::Reg<u32, _RXJABBERS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXJABBERS;
#[doc = "`read()` method returns [rxjabbers::R](rxjabbers::R) reader structure"]
impl crate::Readable for RXJABBERS {}
#[doc = "`write(|w| ..)` method takes [rxjabbers::W](rxjabbers::W) writer structure"]
impl crate::Writable for RXJABBERS {}
#[doc = "Jabbers Received"]
pub mod rxjabbers;
#[doc = "Frame Check Sequence Errors\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fcserrs](fcserrs) module"]
pub type FCSERRS = crate::Reg<u32, _FCSERRS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FCSERRS;
#[doc = "`read()` method returns [fcserrs::R](fcserrs::R) reader structure"]
impl crate::Readable for FCSERRS {}
#[doc = "`write(|w| ..)` method takes [fcserrs::W](fcserrs::W) writer structure"]
impl crate::Writable for FCSERRS {}
#[doc = "Frame Check Sequence Errors"]
pub mod fcserrs;
#[doc = "Length Field Frame Errors\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxlenerrs](rxlenerrs) module"]
pub type RXLENERRS = crate::Reg<u32, _RXLENERRS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXLENERRS;
#[doc = "`read()` method returns [rxlenerrs::R](rxlenerrs::R) reader structure"]
impl crate::Readable for RXLENERRS {}
#[doc = "`write(|w| ..)` method takes [rxlenerrs::W](rxlenerrs::W) writer structure"]
impl crate::Writable for RXLENERRS {}
#[doc = "Length Field Frame Errors"]
pub mod rxlenerrs;
#[doc = "Receive Symbol Errors\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxsymbolerrs](rxsymbolerrs) module"]
pub type RXSYMBOLERRS = crate::Reg<u32, _RXSYMBOLERRS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXSYMBOLERRS;
#[doc = "`read()` method returns [rxsymbolerrs::R](rxsymbolerrs::R) reader structure"]
impl crate::Readable for RXSYMBOLERRS {}
#[doc = "`write(|w| ..)` method takes [rxsymbolerrs::W](rxsymbolerrs::W) writer structure"]
impl crate::Writable for RXSYMBOLERRS {}
#[doc = "Receive Symbol Errors"]
pub mod rxsymbolerrs;
#[doc = "Alignment Errors\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [alignerrs](alignerrs) module"]
pub type ALIGNERRS = crate::Reg<u32, _ALIGNERRS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALIGNERRS;
#[doc = "`read()` method returns [alignerrs::R](alignerrs::R) reader structure"]
impl crate::Readable for ALIGNERRS {}
#[doc = "`write(|w| ..)` method takes [alignerrs::W](alignerrs::W) writer structure"]
impl crate::Writable for ALIGNERRS {}
#[doc = "Alignment Errors"]
pub mod alignerrs;
#[doc = "Receive Resource Errors\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxresourceerrs](rxresourceerrs) module"]
pub type RXRESOURCEERRS = crate::Reg<u32, _RXRESOURCEERRS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXRESOURCEERRS;
#[doc = "`read()` method returns [rxresourceerrs::R](rxresourceerrs::R) reader structure"]
impl crate::Readable for RXRESOURCEERRS {}
#[doc = "`write(|w| ..)` method takes [rxresourceerrs::W](rxresourceerrs::W) writer structure"]
impl crate::Writable for RXRESOURCEERRS {}
#[doc = "Receive Resource Errors"]
pub mod rxresourceerrs;
#[doc = "Receive Overruns\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxoverruns](rxoverruns) module"]
pub type RXOVERRUNS = crate::Reg<u32, _RXOVERRUNS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXOVERRUNS;
#[doc = "`read()` method returns [rxoverruns::R](rxoverruns::R) reader structure"]
impl crate::Readable for RXOVERRUNS {}
#[doc = "`write(|w| ..)` method takes [rxoverruns::W](rxoverruns::W) writer structure"]
impl crate::Writable for RXOVERRUNS {}
#[doc = "Receive Overruns"]
pub mod rxoverruns;
#[doc = "IP Header Checksum Errors\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxipckerrs](rxipckerrs) module"]
pub type RXIPCKERRS = crate::Reg<u32, _RXIPCKERRS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXIPCKERRS;
#[doc = "`read()` method returns [rxipckerrs::R](rxipckerrs::R) reader structure"]
impl crate::Readable for RXIPCKERRS {}
#[doc = "`write(|w| ..)` method takes [rxipckerrs::W](rxipckerrs::W) writer structure"]
impl crate::Writable for RXIPCKERRS {}
#[doc = "IP Header Checksum Errors"]
pub mod rxipckerrs;
#[doc = "TCP Checksum Errors\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxtcpckerrs](rxtcpckerrs) module"]
pub type RXTCPCKERRS = crate::Reg<u32, _RXTCPCKERRS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXTCPCKERRS;
#[doc = "`read()` method returns [rxtcpckerrs::R](rxtcpckerrs::R) reader structure"]
impl crate::Readable for RXTCPCKERRS {}
#[doc = "`write(|w| ..)` method takes [rxtcpckerrs::W](rxtcpckerrs::W) writer structure"]
impl crate::Writable for RXTCPCKERRS {}
#[doc = "TCP Checksum Errors"]
pub mod rxtcpckerrs;
#[doc = "UDP Checksum Errors\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxudpckerrs](rxudpckerrs) module"]
pub type RXUDPCKERRS = crate::Reg<u32, _RXUDPCKERRS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXUDPCKERRS;
#[doc = "`read()` method returns [rxudpckerrs::R](rxudpckerrs::R) reader structure"]
impl crate::Readable for RXUDPCKERRS {}
#[doc = "`write(|w| ..)` method takes [rxudpckerrs::W](rxudpckerrs::W) writer structure"]
impl crate::Writable for RXUDPCKERRS {}
#[doc = "UDP Checksum Errors"]
pub mod rxudpckerrs;
#[doc = "Receive DMA Flushed Packets\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [autoflushedpkts](autoflushedpkts) module"]
pub type AUTOFLUSHEDPKTS = crate::Reg<u32, _AUTOFLUSHEDPKTS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AUTOFLUSHEDPKTS;
#[doc = "`read()` method returns [autoflushedpkts::R](autoflushedpkts::R) reader structure"]
impl crate::Readable for AUTOFLUSHEDPKTS {}
#[doc = "`write(|w| ..)` method takes [autoflushedpkts::W](autoflushedpkts::W) writer structure"]
impl crate::Writable for AUTOFLUSHEDPKTS {}
#[doc = "Receive DMA Flushed Packets"]
pub mod autoflushedpkts;
#[doc = "1588 Timer Increment Register subscript nsec\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tsutimerincrsubnsec](tsutimerincrsubnsec) module"]
pub type TSUTIMERINCRSUBNSEC = crate::Reg<u32, _TSUTIMERINCRSUBNSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSUTIMERINCRSUBNSEC;
#[doc = "`read()` method returns [tsutimerincrsubnsec::R](tsutimerincrsubnsec::R) reader structure"]
impl crate::Readable for TSUTIMERINCRSUBNSEC {}
#[doc = "`write(|w| ..)` method takes [tsutimerincrsubnsec::W](tsutimerincrsubnsec::W) writer structure"]
impl crate::Writable for TSUTIMERINCRSUBNSEC {}
#[doc = "1588 Timer Increment Register subscript nsec"]
pub mod tsutimerincrsubnsec;
#[doc = "1588 Timer Seconds Register 47:32\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tsutimermsbsec](tsutimermsbsec) module"]
pub type TSUTIMERMSBSEC = crate::Reg<u32, _TSUTIMERMSBSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSUTIMERMSBSEC;
#[doc = "`read()` method returns [tsutimermsbsec::R](tsutimermsbsec::R) reader structure"]
impl crate::Readable for TSUTIMERMSBSEC {}
#[doc = "`write(|w| ..)` method takes [tsutimermsbsec::W](tsutimermsbsec::W) writer structure"]
impl crate::Writable for TSUTIMERMSBSEC {}
#[doc = "1588 Timer Seconds Register 47:32"]
pub mod tsutimermsbsec;
#[doc = "1588 Timer Seconds Register 31:0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tsutimersec](tsutimersec) module"]
pub type TSUTIMERSEC = crate::Reg<u32, _TSUTIMERSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSUTIMERSEC;
#[doc = "`read()` method returns [tsutimersec::R](tsutimersec::R) reader structure"]
impl crate::Readable for TSUTIMERSEC {}
#[doc = "`write(|w| ..)` method takes [tsutimersec::W](tsutimersec::W) writer structure"]
impl crate::Writable for TSUTIMERSEC {}
#[doc = "1588 Timer Seconds Register 31:0"]
pub mod tsutimersec;
#[doc = "1588 Timer Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tsutimernsec](tsutimernsec) module"]
pub type TSUTIMERNSEC = crate::Reg<u32, _TSUTIMERNSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSUTIMERNSEC;
#[doc = "`read()` method returns [tsutimernsec::R](tsutimernsec::R) reader structure"]
impl crate::Readable for TSUTIMERNSEC {}
#[doc = "`write(|w| ..)` method takes [tsutimernsec::W](tsutimernsec::W) writer structure"]
impl crate::Writable for TSUTIMERNSEC {}
#[doc = "1588 Timer Nanoseconds Register"]
pub mod tsutimernsec;
#[doc = "This register returns all zeroes when read.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tsutimeradjust](tsutimeradjust) module"]
pub type TSUTIMERADJUST = crate::Reg<u32, _TSUTIMERADJUST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSUTIMERADJUST;
#[doc = "`read()` method returns [tsutimeradjust::R](tsutimeradjust::R) reader structure"]
impl crate::Readable for TSUTIMERADJUST {}
#[doc = "`write(|w| ..)` method takes [tsutimeradjust::W](tsutimeradjust::W) writer structure"]
impl crate::Writable for TSUTIMERADJUST {}
#[doc = "This register returns all zeroes when read."]
pub mod tsutimeradjust;
#[doc = "1588 Timer Increment Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tsutimerincr](tsutimerincr) module"]
pub type TSUTIMERINCR = crate::Reg<u32, _TSUTIMERINCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSUTIMERINCR;
#[doc = "`read()` method returns [tsutimerincr::R](tsutimerincr::R) reader structure"]
impl crate::Readable for TSUTIMERINCR {}
#[doc = "`write(|w| ..)` method takes [tsutimerincr::W](tsutimerincr::W) writer structure"]
impl crate::Writable for TSUTIMERINCR {}
#[doc = "1588 Timer Increment Register"]
pub mod tsutimerincr;
#[doc = "PTP Event Frame Transmitted Seconds Register 31:0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tsuptptxsec](tsuptptxsec) module"]
pub type TSUPTPTXSEC = crate::Reg<u32, _TSUPTPTXSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSUPTPTXSEC;
#[doc = "`read()` method returns [tsuptptxsec::R](tsuptptxsec::R) reader structure"]
impl crate::Readable for TSUPTPTXSEC {}
#[doc = "PTP Event Frame Transmitted Seconds Register 31:0"]
pub mod tsuptptxsec;
#[doc = "PTP Event Frame Transmitted Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tsuptptxnsec](tsuptptxnsec) module"]
pub type TSUPTPTXNSEC = crate::Reg<u32, _TSUPTPTXNSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSUPTPTXNSEC;
#[doc = "`read()` method returns [tsuptptxnsec::R](tsuptptxnsec::R) reader structure"]
impl crate::Readable for TSUPTPTXNSEC {}
#[doc = "PTP Event Frame Transmitted Nanoseconds Register"]
pub mod tsuptptxnsec;
#[doc = "PTP Event Frame Received Seconds Register 31:0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tsuptprxsec](tsuptprxsec) module"]
pub type TSUPTPRXSEC = crate::Reg<u32, _TSUPTPRXSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSUPTPRXSEC;
#[doc = "`read()` method returns [tsuptprxsec::R](tsuptprxsec::R) reader structure"]
impl crate::Readable for TSUPTPRXSEC {}
#[doc = "PTP Event Frame Received Seconds Register 31:0"]
pub mod tsuptprxsec;
#[doc = "PTP Event Frame Received Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tsuptprxnsec](tsuptprxnsec) module"]
pub type TSUPTPRXNSEC = crate::Reg<u32, _TSUPTPRXNSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSUPTPRXNSEC;
#[doc = "`read()` method returns [tsuptprxnsec::R](tsuptprxnsec::R) reader structure"]
impl crate::Readable for TSUPTPRXNSEC {}
#[doc = "PTP Event Frame Received Nanoseconds Register"]
pub mod tsuptprxnsec;
#[doc = "PTP Peer Event Frame Transmitted Seconds Register 31:0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tsupeertxsec](tsupeertxsec) module"]
pub type TSUPEERTXSEC = crate::Reg<u32, _TSUPEERTXSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSUPEERTXSEC;
#[doc = "`read()` method returns [tsupeertxsec::R](tsupeertxsec::R) reader structure"]
impl crate::Readable for TSUPEERTXSEC {}
#[doc = "PTP Peer Event Frame Transmitted Seconds Register 31:0"]
pub mod tsupeertxsec;
#[doc = "PTP Peer Event Frame Transmitted Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tsupeertxnsec](tsupeertxnsec) module"]
pub type TSUPEERTXNSEC = crate::Reg<u32, _TSUPEERTXNSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSUPEERTXNSEC;
#[doc = "`read()` method returns [tsupeertxnsec::R](tsupeertxnsec::R) reader structure"]
impl crate::Readable for TSUPEERTXNSEC {}
#[doc = "PTP Peer Event Frame Transmitted Nanoseconds Register"]
pub mod tsupeertxnsec;
#[doc = "PTP Peer Event Frame Received Seconds Register 31:0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tsupeerrxsec](tsupeerrxsec) module"]
pub type TSUPEERRXSEC = crate::Reg<u32, _TSUPEERRXSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSUPEERRXSEC;
#[doc = "`read()` method returns [tsupeerrxsec::R](tsupeerrxsec::R) reader structure"]
impl crate::Readable for TSUPEERRXSEC {}
#[doc = "PTP Peer Event Frame Received Seconds Register 31:0"]
pub mod tsupeerrxsec;
#[doc = "PTP Peer Event Frame Received Nanoseconds Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tsupeerrxnsec](tsupeerrxnsec) module"]
pub type TSUPEERRXNSEC = crate::Reg<u32, _TSUPEERRXNSEC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TSUPEERRXNSEC;
#[doc = "`read()` method returns [tsupeerrxnsec::R](tsupeerrxnsec::R) reader structure"]
impl crate::Readable for TSUPEERRXNSEC {}
#[doc = "PTP Peer Event Frame Received Nanoseconds Register"]
pub mod tsupeerrxnsec;
#[doc = "Transmit Pause Quantum Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txpausequant1](txpausequant1) module"]
pub type TXPAUSEQUANT1 = crate::Reg<u32, _TXPAUSEQUANT1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXPAUSEQUANT1;
#[doc = "`read()` method returns [txpausequant1::R](txpausequant1::R) reader structure"]
impl crate::Readable for TXPAUSEQUANT1 {}
#[doc = "`write(|w| ..)` method takes [txpausequant1::W](txpausequant1::W) writer structure"]
impl crate::Writable for TXPAUSEQUANT1 {}
#[doc = "Transmit Pause Quantum Register 1"]
pub mod txpausequant1;
#[doc = "Transmit Pause Quantum Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txpausequant2](txpausequant2) module"]
pub type TXPAUSEQUANT2 = crate::Reg<u32, _TXPAUSEQUANT2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXPAUSEQUANT2;
#[doc = "`read()` method returns [txpausequant2::R](txpausequant2::R) reader structure"]
impl crate::Readable for TXPAUSEQUANT2 {}
#[doc = "`write(|w| ..)` method takes [txpausequant2::W](txpausequant2::W) writer structure"]
impl crate::Writable for TXPAUSEQUANT2 {}
#[doc = "Transmit Pause Quantum Register 2"]
pub mod txpausequant2;
#[doc = "Transmit Pause Quantum Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txpausequant3](txpausequant3) module"]
pub type TXPAUSEQUANT3 = crate::Reg<u32, _TXPAUSEQUANT3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXPAUSEQUANT3;
#[doc = "`read()` method returns [txpausequant3::R](txpausequant3::R) reader structure"]
impl crate::Readable for TXPAUSEQUANT3 {}
#[doc = "`write(|w| ..)` method takes [txpausequant3::W](txpausequant3::W) writer structure"]
impl crate::Writable for TXPAUSEQUANT3 {}
#[doc = "Transmit Pause Quantum Register 3"]
pub mod txpausequant3;
#[doc = "Received LPI transitions\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxlpi](rxlpi) module"]
pub type RXLPI = crate::Reg<u32, _RXLPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXLPI;
#[doc = "`read()` method returns [rxlpi::R](rxlpi::R) reader structure"]
impl crate::Readable for RXLPI {}
#[doc = "`write(|w| ..)` method takes [rxlpi::W](rxlpi::W) writer structure"]
impl crate::Writable for RXLPI {}
#[doc = "Received LPI transitions"]
pub mod rxlpi;
#[doc = "Received LPI time\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxlpitime](rxlpitime) module"]
pub type RXLPITIME = crate::Reg<u32, _RXLPITIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXLPITIME;
#[doc = "`read()` method returns [rxlpitime::R](rxlpitime::R) reader structure"]
impl crate::Readable for RXLPITIME {}
#[doc = "`write(|w| ..)` method takes [rxlpitime::W](rxlpitime::W) writer structure"]
impl crate::Writable for RXLPITIME {}
#[doc = "Received LPI time"]
pub mod rxlpitime;
#[doc = "Transmit LPI transitions\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txlpi](txlpi) module"]
pub type TXLPI = crate::Reg<u32, _TXLPI>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXLPI;
#[doc = "`read()` method returns [txlpi::R](txlpi::R) reader structure"]
impl crate::Readable for TXLPI {}
#[doc = "`write(|w| ..)` method takes [txlpi::W](txlpi::W) writer structure"]
impl crate::Writable for TXLPI {}
#[doc = "Transmit LPI transitions"]
pub mod txlpi;
#[doc = "Transmit LPI time\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txlpitime](txlpitime) module"]
pub type TXLPITIME = crate::Reg<u32, _TXLPITIME>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXLPITIME;
#[doc = "`read()` method returns [txlpitime::R](txlpitime::R) reader structure"]
impl crate::Readable for TXLPITIME {}
#[doc = "`write(|w| ..)` method takes [txlpitime::W](txlpitime::W) writer structure"]
impl crate::Writable for TXLPITIME {}
#[doc = "Transmit LPI time"]
pub mod txlpitime;
#[doc = "TX BD control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [txbdctrl](txbdctrl) module"]
pub type TXBDCTRL = crate::Reg<u32, _TXBDCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TXBDCTRL;
#[doc = "`read()` method returns [txbdctrl::R](txbdctrl::R) reader structure"]
impl crate::Readable for TXBDCTRL {}
#[doc = "`write(|w| ..)` method takes [txbdctrl::W](txbdctrl::W) writer structure"]
impl crate::Writable for TXBDCTRL {}
#[doc = "TX BD control register"]
pub mod txbdctrl;
#[doc = "RX BD control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rxbdctrl](rxbdctrl) module"]
pub type RXBDCTRL = crate::Reg<u32, _RXBDCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RXBDCTRL;
#[doc = "`read()` method returns [rxbdctrl::R](rxbdctrl::R) reader structure"]
impl crate::Readable for RXBDCTRL {}
#[doc = "`write(|w| ..)` method takes [rxbdctrl::W](rxbdctrl::W) writer structure"]
impl crate::Writable for RXBDCTRL {}
#[doc = "RX BD control register"]
pub mod rxbdctrl;
#[doc = "I/O Route Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [routepen](routepen) module"]
pub type ROUTEPEN = crate::Reg<u32, _ROUTEPEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROUTEPEN;
#[doc = "`read()` method returns [routepen::R](routepen::R) reader structure"]
impl crate::Readable for ROUTEPEN {}
#[doc = "`write(|w| ..)` method takes [routepen::W](routepen::W) writer structure"]
impl crate::Writable for ROUTEPEN {}
#[doc = "I/O Route Enable Register"]
pub mod routepen;
#[doc = "I/O Route Location Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [routeloc0](routeloc0) module"]
pub type ROUTELOC0 = crate::Reg<u32, _ROUTELOC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROUTELOC0;
#[doc = "`read()` method returns [routeloc0::R](routeloc0::R) reader structure"]
impl crate::Readable for ROUTELOC0 {}
#[doc = "`write(|w| ..)` method takes [routeloc0::W](routeloc0::W) writer structure"]
impl crate::Writable for ROUTELOC0 {}
#[doc = "I/O Route Location Register 0"]
pub mod routeloc0;
#[doc = "I/O Route Location Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [routeloc1](routeloc1) module"]
pub type ROUTELOC1 = crate::Reg<u32, _ROUTELOC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROUTELOC1;
#[doc = "`read()` method returns [routeloc1::R](routeloc1::R) reader structure"]
impl crate::Readable for ROUTELOC1 {}
#[doc = "`write(|w| ..)` method takes [routeloc1::W](routeloc1::W) writer structure"]
impl crate::Writable for ROUTELOC1 {}
#[doc = "I/O Route Location Register 1"]
pub mod routeloc1;
#[doc = "Ethernet control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Ethernet control register"]
pub mod ctrl;
