#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Network control register"]
    pub networkctrl: NETWORKCTRL,
    #[doc = "0x04 - Network configuration register"]
    pub networkcfg: NETWORKCFG,
    #[doc = "0x08 - Network status register"]
    pub networkstatus: NETWORKSTATUS,
    _reserved0: [u8; 4usize],
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
    _reserved1: [u8; 16usize],
    #[doc = "0x5c - Interrupt moderation register"]
    pub imod: IMOD,
    #[doc = "0x60 - System wake time"]
    pub syswaketime: SYSWAKETIME,
    _reserved2: [u8; 28usize],
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
    _reserved3: [u8; 4usize],
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
    _reserved4: [u8; 8usize],
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
    _reserved5: [u8; 4usize],
    #[doc = "0x1bc - 1588 Timer Increment Register subscript nsec"]
    pub tsutimerincrsubnsec: TSUTIMERINCRSUBNSEC,
    #[doc = "0x1c0 - 1588 Timer Seconds Register 47:32"]
    pub tsutimermsbsec: TSUTIMERMSBSEC,
    _reserved6: [u8; 12usize],
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
    _reserved7: [u8; 96usize],
    #[doc = "0x260 - Transmit Pause Quantum Register 1"]
    pub txpausequant1: TXPAUSEQUANT1,
    #[doc = "0x264 - Transmit Pause Quantum Register 2"]
    pub txpausequant2: TXPAUSEQUANT2,
    #[doc = "0x268 - Transmit Pause Quantum Register 3"]
    pub txpausequant3: TXPAUSEQUANT3,
    _reserved8: [u8; 4usize],
    #[doc = "0x270 - Received LPI transitions"]
    pub rxlpi: RXLPI,
    #[doc = "0x274 - Received LPI time"]
    pub rxlpitime: RXLPITIME,
    #[doc = "0x278 - Transmit LPI transitions"]
    pub txlpi: TXLPI,
    #[doc = "0x27c - Transmit LPI time"]
    pub txlpitime: TXLPITIME,
    _reserved9: [u8; 588usize],
    #[doc = "0x4cc - TX BD control register"]
    pub txbdctrl: TXBDCTRL,
    #[doc = "0x4d0 - RX BD control register"]
    pub rxbdctrl: RXBDCTRL,
    _reserved10: [u8; 1836usize],
    #[doc = "0xc00 - I/O Route Enable Register"]
    pub routepen: ROUTEPEN,
    #[doc = "0xc04 - I/O Route Location Register 0"]
    pub routeloc0: ROUTELOC0,
    _reserved11: [u8; 4usize],
    #[doc = "0xc0c - I/O Route Location Register 1"]
    pub routeloc1: ROUTELOC1,
    #[doc = "0xc10 - Ethernet control register"]
    pub ctrl: CTRL,
}
#[doc = "Network control register"]
pub struct NETWORKCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Network control register"]
pub mod networkctrl;
#[doc = "Network configuration register"]
pub struct NETWORKCFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Network configuration register"]
pub mod networkcfg;
#[doc = "Network status register"]
pub struct NETWORKSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Network status register"]
pub mod networkstatus;
#[doc = "DMA Configuration Register"]
pub struct DMACFG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "DMA Configuration Register"]
pub mod dmacfg;
#[doc = "Transmit status register"]
pub struct TXSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit status register"]
pub mod txstatus;
#[doc = "Start address of the receive buffer queue"]
pub struct RXQPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start address of the receive buffer queue"]
pub mod rxqptr;
#[doc = "Start address of the transmit buffer queue"]
pub struct TXQPTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Start address of the transmit buffer queue"]
pub mod txqptr;
#[doc = "Receive status register"]
pub struct RXSTATUS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive status register"]
pub mod rxstatus;
#[doc = "Interrupt status register"]
pub struct IFCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt status register"]
pub mod ifcr;
#[doc = "Interrupt Enable Register"]
pub struct IENS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Enable Register"]
pub mod iens;
#[doc = "Interrupt Disable Register"]
pub struct IENC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt Disable Register"]
pub mod ienc;
#[doc = "Interrupt mask register"]
pub struct IENRO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt mask register"]
pub mod ienro;
#[doc = "PHY management register"]
pub struct PHYMNGMNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PHY management register"]
pub mod phymngmnt;
#[doc = "Received Pause Quantum Register"]
pub struct RXPAUSEQUANT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Received Pause Quantum Register"]
pub mod rxpausequant;
#[doc = "Transmit Pause Quantum Register"]
pub struct TXPAUSEQUANT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Pause Quantum Register"]
pub mod txpausequant;
#[doc = "TX Partial Store and Forward"]
pub struct PBUFTXCUTTHRU {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX Partial Store and Forward"]
pub mod pbuftxcutthru;
#[doc = "RX Partial Store and Forward"]
pub struct PBUFRXCUTTHRU {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX Partial Store and Forward"]
pub mod pbufrxcutthru;
#[doc = "Maximum Jumbo Frame Size."]
pub struct JUMBOMAXLEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Maximum Jumbo Frame Size."]
pub mod jumbomaxlen;
#[doc = "Interrupt moderation register"]
pub struct IMOD {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Interrupt moderation register"]
pub mod imod;
#[doc = "System wake time"]
pub struct SYSWAKETIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "System wake time"]
pub mod syswaketime;
#[doc = "Hash Register Bottom \\[31:0\\]"]
pub struct HASHBOTTOM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hash Register Bottom \\[31:0\\]"]
pub mod hashbottom;
#[doc = "Hash Register Top \\[63:32\\]"]
pub struct HASHTOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Hash Register Top \\[63:32\\]"]
pub mod hashtop;
#[doc = "Specific Address 1 Bottom"]
pub struct SPECADDR1BOTTOM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Specific Address 1 Bottom"]
pub mod specaddr1bottom;
#[doc = "Specific Address 1 Top"]
pub struct SPECADDR1TOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Specific Address 1 Top"]
pub mod specaddr1top;
#[doc = "Specific Address 2 Bottom"]
pub struct SPECADDR2BOTTOM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Specific Address 2 Bottom"]
pub mod specaddr2bottom;
#[doc = "Specific Address 2 Top"]
pub struct SPECADDR2TOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Specific Address 2 Top"]
pub mod specaddr2top;
#[doc = "Specific Address 3 Bottom"]
pub struct SPECADDR3BOTTOM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Specific Address 3 Bottom"]
pub mod specaddr3bottom;
#[doc = "Specific Address 3 Top"]
pub struct SPECADDR3TOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Specific Address 3 Top"]
pub mod specaddr3top;
#[doc = "Specific Address 4 Bottom"]
pub struct SPECADDR4BOTTOM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Specific Address 4 Bottom"]
pub mod specaddr4bottom;
#[doc = "Specific Address 4 Top"]
pub struct SPECADDR4TOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Specific Address 4 Top"]
pub mod specaddr4top;
#[doc = "Type ID Match 1"]
pub struct SPECTYPE1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Type ID Match 1"]
pub mod spectype1;
#[doc = "Type ID Match 2"]
pub struct SPECTYPE2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Type ID Match 2"]
pub mod spectype2;
#[doc = "Type ID Match 3"]
pub struct SPECTYPE3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Type ID Match 3"]
pub mod spectype3;
#[doc = "Type ID Match 4"]
pub struct SPECTYPE4 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Type ID Match 4"]
pub mod spectype4;
#[doc = "Wake on LAN Register"]
pub struct WOLREG {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Wake on LAN Register"]
pub mod wolreg;
#[doc = "IPG stretch register"]
pub struct STRETCHRATIO {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IPG stretch register"]
pub mod stretchratio;
#[doc = "Stacked VLAN Register"]
pub struct STACKEDVLAN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Stacked VLAN Register"]
pub mod stackedvlan;
#[doc = "Transmit PFC Pause Register"]
pub struct TXPFCPAUSE {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit PFC Pause Register"]
pub mod txpfcpause;
#[doc = "Specific Address Mask 1 Bottom 31:0"]
pub struct MASKADD1BOTTOM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Specific Address Mask 1 Bottom 31:0"]
pub mod maskadd1bottom;
#[doc = "Specific Address Mask 1 Top 47:32"]
pub struct MASKADD1TOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Specific Address Mask 1 Top 47:32"]
pub mod maskadd1top;
#[doc = "PTP RX unicast IP destination address"]
pub struct RXPTPUNICAST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PTP RX unicast IP destination address"]
pub mod rxptpunicast;
#[doc = "PTP TX unicast IP destination address"]
pub struct TXPTPUNICAST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PTP TX unicast IP destination address"]
pub mod txptpunicast;
#[doc = "TSU timer comparison value nanoseconds"]
pub struct TSUNSECCMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSU timer comparison value nanoseconds"]
pub mod tsunseccmp;
#[doc = "TSU timer comparison value seconds \\[31:0\\]"]
pub struct TSUSECCMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSU timer comparison value seconds \\[31:0\\]"]
pub mod tsuseccmp;
#[doc = "TSU timer comparison value seconds \\[47:32\\]"]
pub struct TSUMSBSECCMP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TSU timer comparison value seconds \\[47:32\\]"]
pub mod tsumsbseccmp;
#[doc = "PTP Event Frame Transmitted Seconds Register 47:32"]
pub struct TSUPTPTXMSBSEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PTP Event Frame Transmitted Seconds Register 47:32"]
pub mod tsuptptxmsbsec;
#[doc = "PTP Event Frame Received Seconds Register 47:32"]
pub struct TSUPTPRXMSBSEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PTP Event Frame Received Seconds Register 47:32"]
pub mod tsuptprxmsbsec;
#[doc = "PTP Peer Event Frame Transmitted Seconds Register 47:32"]
pub struct TSUPEERTXMSBSEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PTP Peer Event Frame Transmitted Seconds Register 47:32"]
pub mod tsupeertxmsbsec;
#[doc = "PTP Peer Event Frame Received Seconds Register 47:32"]
pub struct TSUPEERRXMSBSEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PTP Peer Event Frame Received Seconds Register 47:32"]
pub mod tsupeerrxmsbsec;
#[doc = "Octets transmitted 31:0"]
pub struct OCTETSTXEDBOTTOM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Octets transmitted 31:0"]
pub mod octetstxedbottom;
#[doc = "Octets Transmitted 47:32"]
pub struct OCTETSTXEDTOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Octets Transmitted 47:32"]
pub mod octetstxedtop;
#[doc = "Frames Transmitted"]
pub struct FRAMESTXEDOK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frames Transmitted"]
pub mod framestxedok;
#[doc = "Broadcast Frames Transmitted"]
pub struct BROADCASTTXED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Broadcast Frames Transmitted"]
pub mod broadcasttxed;
#[doc = "Multicast Frames Transmitted"]
pub struct MULTICASTTXED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Multicast Frames Transmitted"]
pub mod multicasttxed;
#[doc = "Pause Frames Transmitted"]
pub struct PFRAMESTXED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pause Frames Transmitted"]
pub mod pframestxed;
#[doc = "64 Byte Frames Transmitted"]
pub struct FRAMESTXED64 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "64 Byte Frames Transmitted"]
pub mod framestxed64;
#[doc = "65 to 127 Byte Frames Transmitted"]
pub struct FRAMESTXED65 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "65 to 127 Byte Frames Transmitted"]
pub mod framestxed65;
#[doc = "128 to 255 Byte Frames Transmitted"]
pub struct FRAMESTXED128 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "128 to 255 Byte Frames Transmitted"]
pub mod framestxed128;
#[doc = "256 to 511 Byte Frames Transmitted"]
pub struct FRAMESTXED256 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "256 to 511 Byte Frames Transmitted"]
pub mod framestxed256;
#[doc = "512 to 1023 Byte Frames Transmitted"]
pub struct FRAMESTXED512 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "512 to 1023 Byte Frames Transmitted"]
pub mod framestxed512;
#[doc = "1024 to 1518 Byte Frames Transmitted"]
pub struct FRAMESTXED1024 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "1024 to 1518 Byte Frames Transmitted"]
pub mod framestxed1024;
#[doc = "Greater Than 1518 Byte Frames Transmitted"]
pub struct FRAMESTXED1519 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Greater Than 1518 Byte Frames Transmitted"]
pub mod framestxed1519;
#[doc = "Transmit Under Runs"]
pub struct TXUNDERRUNS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Under Runs"]
pub mod txunderruns;
#[doc = "Single Collision Frames"]
pub struct SINGLECOLS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Single Collision Frames"]
pub mod singlecols;
#[doc = "Multiple Collision Frames"]
pub struct MULTICOLS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Multiple Collision Frames"]
pub mod multicols;
#[doc = "Excessive Collisions"]
pub struct EXCESSCOLS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Excessive Collisions"]
pub mod excesscols;
#[doc = "Late Collisions"]
pub struct LATECOLS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Late Collisions"]
pub mod latecols;
#[doc = "Deferred Transmission Frames"]
pub struct DEFERREDFRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Deferred Transmission Frames"]
pub mod deferredframes;
#[doc = "Carrier Sense Errors"]
pub struct CRSERRS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Carrier Sense Errors"]
pub mod crserrs;
#[doc = "Octets Received 31:0"]
pub struct OCTETSRXEDBOTTOM {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Octets Received 31:0"]
pub mod octetsrxedbottom;
#[doc = "Octets Received 47:32"]
pub struct OCTETSRXEDTOP {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Octets Received 47:32"]
pub mod octetsrxedtop;
#[doc = "Frames Received"]
pub struct FRAMESRXEDOK {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frames Received"]
pub mod framesrxedok;
#[doc = "Broadcast Frames Received"]
pub struct BROADCASTRXED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Broadcast Frames Received"]
pub mod broadcastrxed;
#[doc = "Multicast Frames Received"]
pub struct MULTICASTRXED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Multicast Frames Received"]
pub mod multicastrxed;
#[doc = "Pause Frames Received"]
pub struct PFRAMESRXED {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Pause Frames Received"]
pub mod pframesrxed;
#[doc = "64 Byte Frames Received"]
pub struct FRAMESRXED64 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "64 Byte Frames Received"]
pub mod framesrxed64;
#[doc = "65 to 127 Byte Frames Received"]
pub struct FRAMESRXED65 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "65 to 127 Byte Frames Received"]
pub mod framesrxed65;
#[doc = "128 to 255 Byte Frames Received"]
pub struct FRAMESRXED128 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "128 to 255 Byte Frames Received"]
pub mod framesrxed128;
#[doc = "256 to 511 Byte Frames Received"]
pub struct FRAMESRXED256 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "256 to 511 Byte Frames Received"]
pub mod framesrxed256;
#[doc = "512 to 1023 Byte Frames Received"]
pub struct FRAMESRXED512 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "512 to 1023 Byte Frames Received"]
pub mod framesrxed512;
#[doc = "1024 to 1518 Byte Frames Received"]
pub struct FRAMESRXED1024 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "1024 to 1518 Byte Frames Received"]
pub mod framesrxed1024;
#[doc = "1519 to maximum Byte Frames Received"]
pub struct FRAMESRXED1519 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "1519 to maximum Byte Frames Received"]
pub mod framesrxed1519;
#[doc = "Undersized Frames Received"]
pub struct UNDERSIZEFRAMES {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Undersized Frames Received"]
pub mod undersizeframes;
#[doc = "Oversize Frames Received"]
pub struct EXCESSIVERXLEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Oversize Frames Received"]
pub mod excessiverxlen;
#[doc = "Jabbers Received"]
pub struct RXJABBERS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Jabbers Received"]
pub mod rxjabbers;
#[doc = "Frame Check Sequence Errors"]
pub struct FCSERRS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Frame Check Sequence Errors"]
pub mod fcserrs;
#[doc = "Length Field Frame Errors"]
pub struct RXLENERRS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Length Field Frame Errors"]
pub mod rxlenerrs;
#[doc = "Receive Symbol Errors"]
pub struct RXSYMBOLERRS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Symbol Errors"]
pub mod rxsymbolerrs;
#[doc = "Alignment Errors"]
pub struct ALIGNERRS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Alignment Errors"]
pub mod alignerrs;
#[doc = "Receive Resource Errors"]
pub struct RXRESOURCEERRS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Resource Errors"]
pub mod rxresourceerrs;
#[doc = "Receive Overruns"]
pub struct RXOVERRUNS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive Overruns"]
pub mod rxoverruns;
#[doc = "IP Header Checksum Errors"]
pub struct RXIPCKERRS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "IP Header Checksum Errors"]
pub mod rxipckerrs;
#[doc = "TCP Checksum Errors"]
pub struct RXTCPCKERRS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TCP Checksum Errors"]
pub mod rxtcpckerrs;
#[doc = "UDP Checksum Errors"]
pub struct RXUDPCKERRS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "UDP Checksum Errors"]
pub mod rxudpckerrs;
#[doc = "Receive DMA Flushed Packets"]
pub struct AUTOFLUSHEDPKTS {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Receive DMA Flushed Packets"]
pub mod autoflushedpkts;
#[doc = "1588 Timer Increment Register subscript nsec"]
pub struct TSUTIMERINCRSUBNSEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "1588 Timer Increment Register subscript nsec"]
pub mod tsutimerincrsubnsec;
#[doc = "1588 Timer Seconds Register 47:32"]
pub struct TSUTIMERMSBSEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "1588 Timer Seconds Register 47:32"]
pub mod tsutimermsbsec;
#[doc = "1588 Timer Seconds Register 31:0"]
pub struct TSUTIMERSEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "1588 Timer Seconds Register 31:0"]
pub mod tsutimersec;
#[doc = "1588 Timer Nanoseconds Register"]
pub struct TSUTIMERNSEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "1588 Timer Nanoseconds Register"]
pub mod tsutimernsec;
#[doc = "This register returns all zeroes when read."]
pub struct TSUTIMERADJUST {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "This register returns all zeroes when read."]
pub mod tsutimeradjust;
#[doc = "1588 Timer Increment Register"]
pub struct TSUTIMERINCR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "1588 Timer Increment Register"]
pub mod tsutimerincr;
#[doc = "PTP Event Frame Transmitted Seconds Register 31:0"]
pub struct TSUPTPTXSEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PTP Event Frame Transmitted Seconds Register 31:0"]
pub mod tsuptptxsec;
#[doc = "PTP Event Frame Transmitted Nanoseconds Register"]
pub struct TSUPTPTXNSEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PTP Event Frame Transmitted Nanoseconds Register"]
pub mod tsuptptxnsec;
#[doc = "PTP Event Frame Received Seconds Register 31:0"]
pub struct TSUPTPRXSEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PTP Event Frame Received Seconds Register 31:0"]
pub mod tsuptprxsec;
#[doc = "PTP Event Frame Received Nanoseconds Register"]
pub struct TSUPTPRXNSEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PTP Event Frame Received Nanoseconds Register"]
pub mod tsuptprxnsec;
#[doc = "PTP Peer Event Frame Transmitted Seconds Register 31:0"]
pub struct TSUPEERTXSEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PTP Peer Event Frame Transmitted Seconds Register 31:0"]
pub mod tsupeertxsec;
#[doc = "PTP Peer Event Frame Transmitted Nanoseconds Register"]
pub struct TSUPEERTXNSEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PTP Peer Event Frame Transmitted Nanoseconds Register"]
pub mod tsupeertxnsec;
#[doc = "PTP Peer Event Frame Received Seconds Register 31:0"]
pub struct TSUPEERRXSEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PTP Peer Event Frame Received Seconds Register 31:0"]
pub mod tsupeerrxsec;
#[doc = "PTP Peer Event Frame Received Nanoseconds Register"]
pub struct TSUPEERRXNSEC {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "PTP Peer Event Frame Received Nanoseconds Register"]
pub mod tsupeerrxnsec;
#[doc = "Transmit Pause Quantum Register 1"]
pub struct TXPAUSEQUANT1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Pause Quantum Register 1"]
pub mod txpausequant1;
#[doc = "Transmit Pause Quantum Register 2"]
pub struct TXPAUSEQUANT2 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Pause Quantum Register 2"]
pub mod txpausequant2;
#[doc = "Transmit Pause Quantum Register 3"]
pub struct TXPAUSEQUANT3 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit Pause Quantum Register 3"]
pub mod txpausequant3;
#[doc = "Received LPI transitions"]
pub struct RXLPI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Received LPI transitions"]
pub mod rxlpi;
#[doc = "Received LPI time"]
pub struct RXLPITIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Received LPI time"]
pub mod rxlpitime;
#[doc = "Transmit LPI transitions"]
pub struct TXLPI {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit LPI transitions"]
pub mod txlpi;
#[doc = "Transmit LPI time"]
pub struct TXLPITIME {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Transmit LPI time"]
pub mod txlpitime;
#[doc = "TX BD control register"]
pub struct TXBDCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "TX BD control register"]
pub mod txbdctrl;
#[doc = "RX BD control register"]
pub struct RXBDCTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "RX BD control register"]
pub mod rxbdctrl;
#[doc = "I/O Route Enable Register"]
pub struct ROUTEPEN {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O Route Enable Register"]
pub mod routepen;
#[doc = "I/O Route Location Register 0"]
pub struct ROUTELOC0 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O Route Location Register 0"]
pub mod routeloc0;
#[doc = "I/O Route Location Register 1"]
pub struct ROUTELOC1 {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "I/O Route Location Register 1"]
pub mod routeloc1;
#[doc = "Ethernet control register"]
pub struct CTRL {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Ethernet control register"]
pub mod ctrl;
