use vcell::VolatileCell;
#[doc = r" Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - DMA Status Register"]
    pub status: STATUS,
    #[doc = "0x08 - DMA Synchronization Trigger Register (Single-Cycle RMW)"]
    pub sync: SYNC,
    _reserved0: [u8; 20usize],
    #[doc = "0x20 - DMA Channel Enable Register (Single-Cycle RMW)"]
    pub chen: CHEN,
    #[doc = "0x24 - DMA Channel Busy Register"]
    pub chbusy: CHBUSY,
    #[doc = "0x28 - DMA Channel Linking Done Register (Single-Cycle RMW)"]
    pub chdone: CHDONE,
    #[doc = "0x2c - DMA Channel Debug Halt Register"]
    pub dbghalt: DBGHALT,
    #[doc = "0x30 - DMA Channel Software Transfer Request Register"]
    pub swreq: SWREQ,
    #[doc = "0x34 - DMA Channel Request Disable Register"]
    pub reqdis: REQDIS,
    #[doc = "0x38 - DMA Channel Requests Pending Register"]
    pub reqpend: REQPEND,
    #[doc = "0x3c - DMA Channel Link Load Register"]
    pub linkload: LINKLOAD,
    #[doc = "0x40 - DMA Channel Request Clear Register"]
    pub reqclear: REQCLEAR,
    _reserved1: [u8; 28usize],
    #[doc = "0x60 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x64 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x68 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x6c - Interrupt Enable Register"]
    pub ien: IEN,
    _reserved2: [u8; 16usize],
    #[doc = "0x80 - Channel Peripheral Request Select Register"]
    pub ch0_reqsel: CH0_REQSEL,
    #[doc = "0x84 - Channel Configuration Register"]
    pub ch0_cfg: CH0_CFG,
    #[doc = "0x88 - Channel Loop Counter Register"]
    pub ch0_loop: CH0_LOOP,
    #[doc = "0x8c - Channel Descriptor Control Word Register"]
    pub ch0_ctrl: CH0_CTRL,
    #[doc = "0x90 - Channel Descriptor Source Data Address Register"]
    pub ch0_src: CH0_SRC,
    #[doc = "0x94 - Channel Descriptor Destination Data Address Register"]
    pub ch0_dst: CH0_DST,
    #[doc = "0x98 - Channel Descriptor Link Structure Address Register"]
    pub ch0_link: CH0_LINK,
    _reserved3: [u8; 20usize],
    #[doc = "0xb0 - Channel Peripheral Request Select Register"]
    pub ch1_reqsel: CH1_REQSEL,
    #[doc = "0xb4 - Channel Configuration Register"]
    pub ch1_cfg: CH1_CFG,
    #[doc = "0xb8 - Channel Loop Counter Register"]
    pub ch1_loop: CH1_LOOP,
    #[doc = "0xbc - Channel Descriptor Control Word Register"]
    pub ch1_ctrl: CH1_CTRL,
    #[doc = "0xc0 - Channel Descriptor Source Data Address Register"]
    pub ch1_src: CH1_SRC,
    #[doc = "0xc4 - Channel Descriptor Destination Data Address Register"]
    pub ch1_dst: CH1_DST,
    #[doc = "0xc8 - Channel Descriptor Link Structure Address Register"]
    pub ch1_link: CH1_LINK,
    _reserved4: [u8; 20usize],
    #[doc = "0xe0 - Channel Peripheral Request Select Register"]
    pub ch2_reqsel: CH2_REQSEL,
    #[doc = "0xe4 - Channel Configuration Register"]
    pub ch2_cfg: CH2_CFG,
    #[doc = "0xe8 - Channel Loop Counter Register"]
    pub ch2_loop: CH2_LOOP,
    #[doc = "0xec - Channel Descriptor Control Word Register"]
    pub ch2_ctrl: CH2_CTRL,
    #[doc = "0xf0 - Channel Descriptor Source Data Address Register"]
    pub ch2_src: CH2_SRC,
    #[doc = "0xf4 - Channel Descriptor Destination Data Address Register"]
    pub ch2_dst: CH2_DST,
    #[doc = "0xf8 - Channel Descriptor Link Structure Address Register"]
    pub ch2_link: CH2_LINK,
    _reserved5: [u8; 20usize],
    #[doc = "0x110 - Channel Peripheral Request Select Register"]
    pub ch3_reqsel: CH3_REQSEL,
    #[doc = "0x114 - Channel Configuration Register"]
    pub ch3_cfg: CH3_CFG,
    #[doc = "0x118 - Channel Loop Counter Register"]
    pub ch3_loop: CH3_LOOP,
    #[doc = "0x11c - Channel Descriptor Control Word Register"]
    pub ch3_ctrl: CH3_CTRL,
    #[doc = "0x120 - Channel Descriptor Source Data Address Register"]
    pub ch3_src: CH3_SRC,
    #[doc = "0x124 - Channel Descriptor Destination Data Address Register"]
    pub ch3_dst: CH3_DST,
    #[doc = "0x128 - Channel Descriptor Link Structure Address Register"]
    pub ch3_link: CH3_LINK,
    _reserved6: [u8; 20usize],
    #[doc = "0x140 - Channel Peripheral Request Select Register"]
    pub ch4_reqsel: CH4_REQSEL,
    #[doc = "0x144 - Channel Configuration Register"]
    pub ch4_cfg: CH4_CFG,
    #[doc = "0x148 - Channel Loop Counter Register"]
    pub ch4_loop: CH4_LOOP,
    #[doc = "0x14c - Channel Descriptor Control Word Register"]
    pub ch4_ctrl: CH4_CTRL,
    #[doc = "0x150 - Channel Descriptor Source Data Address Register"]
    pub ch4_src: CH4_SRC,
    #[doc = "0x154 - Channel Descriptor Destination Data Address Register"]
    pub ch4_dst: CH4_DST,
    #[doc = "0x158 - Channel Descriptor Link Structure Address Register"]
    pub ch4_link: CH4_LINK,
    _reserved7: [u8; 20usize],
    #[doc = "0x170 - Channel Peripheral Request Select Register"]
    pub ch5_reqsel: CH5_REQSEL,
    #[doc = "0x174 - Channel Configuration Register"]
    pub ch5_cfg: CH5_CFG,
    #[doc = "0x178 - Channel Loop Counter Register"]
    pub ch5_loop: CH5_LOOP,
    #[doc = "0x17c - Channel Descriptor Control Word Register"]
    pub ch5_ctrl: CH5_CTRL,
    #[doc = "0x180 - Channel Descriptor Source Data Address Register"]
    pub ch5_src: CH5_SRC,
    #[doc = "0x184 - Channel Descriptor Destination Data Address Register"]
    pub ch5_dst: CH5_DST,
    #[doc = "0x188 - Channel Descriptor Link Structure Address Register"]
    pub ch5_link: CH5_LINK,
    _reserved8: [u8; 20usize],
    #[doc = "0x1a0 - Channel Peripheral Request Select Register"]
    pub ch6_reqsel: CH6_REQSEL,
    #[doc = "0x1a4 - Channel Configuration Register"]
    pub ch6_cfg: CH6_CFG,
    #[doc = "0x1a8 - Channel Loop Counter Register"]
    pub ch6_loop: CH6_LOOP,
    #[doc = "0x1ac - Channel Descriptor Control Word Register"]
    pub ch6_ctrl: CH6_CTRL,
    #[doc = "0x1b0 - Channel Descriptor Source Data Address Register"]
    pub ch6_src: CH6_SRC,
    #[doc = "0x1b4 - Channel Descriptor Destination Data Address Register"]
    pub ch6_dst: CH6_DST,
    #[doc = "0x1b8 - Channel Descriptor Link Structure Address Register"]
    pub ch6_link: CH6_LINK,
    _reserved9: [u8; 20usize],
    #[doc = "0x1d0 - Channel Peripheral Request Select Register"]
    pub ch7_reqsel: CH7_REQSEL,
    #[doc = "0x1d4 - Channel Configuration Register"]
    pub ch7_cfg: CH7_CFG,
    #[doc = "0x1d8 - Channel Loop Counter Register"]
    pub ch7_loop: CH7_LOOP,
    #[doc = "0x1dc - Channel Descriptor Control Word Register"]
    pub ch7_ctrl: CH7_CTRL,
    #[doc = "0x1e0 - Channel Descriptor Source Data Address Register"]
    pub ch7_src: CH7_SRC,
    #[doc = "0x1e4 - Channel Descriptor Destination Data Address Register"]
    pub ch7_dst: CH7_DST,
    #[doc = "0x1e8 - Channel Descriptor Link Structure Address Register"]
    pub ch7_link: CH7_LINK,
    _reserved10: [u8; 20usize],
    #[doc = "0x200 - Channel Peripheral Request Select Register"]
    pub ch8_reqsel: CH8_REQSEL,
    #[doc = "0x204 - Channel Configuration Register"]
    pub ch8_cfg: CH8_CFG,
    #[doc = "0x208 - Channel Loop Counter Register"]
    pub ch8_loop: CH8_LOOP,
    #[doc = "0x20c - Channel Descriptor Control Word Register"]
    pub ch8_ctrl: CH8_CTRL,
    #[doc = "0x210 - Channel Descriptor Source Data Address Register"]
    pub ch8_src: CH8_SRC,
    #[doc = "0x214 - Channel Descriptor Destination Data Address Register"]
    pub ch8_dst: CH8_DST,
    #[doc = "0x218 - Channel Descriptor Link Structure Address Register"]
    pub ch8_link: CH8_LINK,
    _reserved11: [u8; 20usize],
    #[doc = "0x230 - Channel Peripheral Request Select Register"]
    pub ch9_reqsel: CH9_REQSEL,
    #[doc = "0x234 - Channel Configuration Register"]
    pub ch9_cfg: CH9_CFG,
    #[doc = "0x238 - Channel Loop Counter Register"]
    pub ch9_loop: CH9_LOOP,
    #[doc = "0x23c - Channel Descriptor Control Word Register"]
    pub ch9_ctrl: CH9_CTRL,
    #[doc = "0x240 - Channel Descriptor Source Data Address Register"]
    pub ch9_src: CH9_SRC,
    #[doc = "0x244 - Channel Descriptor Destination Data Address Register"]
    pub ch9_dst: CH9_DST,
    #[doc = "0x248 - Channel Descriptor Link Structure Address Register"]
    pub ch9_link: CH9_LINK,
    _reserved12: [u8; 20usize],
    #[doc = "0x260 - Channel Peripheral Request Select Register"]
    pub ch10_reqsel: CH10_REQSEL,
    #[doc = "0x264 - Channel Configuration Register"]
    pub ch10_cfg: CH10_CFG,
    #[doc = "0x268 - Channel Loop Counter Register"]
    pub ch10_loop: CH10_LOOP,
    #[doc = "0x26c - Channel Descriptor Control Word Register"]
    pub ch10_ctrl: CH10_CTRL,
    #[doc = "0x270 - Channel Descriptor Source Data Address Register"]
    pub ch10_src: CH10_SRC,
    #[doc = "0x274 - Channel Descriptor Destination Data Address Register"]
    pub ch10_dst: CH10_DST,
    #[doc = "0x278 - Channel Descriptor Link Structure Address Register"]
    pub ch10_link: CH10_LINK,
    _reserved13: [u8; 20usize],
    #[doc = "0x290 - Channel Peripheral Request Select Register"]
    pub ch11_reqsel: CH11_REQSEL,
    #[doc = "0x294 - Channel Configuration Register"]
    pub ch11_cfg: CH11_CFG,
    #[doc = "0x298 - Channel Loop Counter Register"]
    pub ch11_loop: CH11_LOOP,
    #[doc = "0x29c - Channel Descriptor Control Word Register"]
    pub ch11_ctrl: CH11_CTRL,
    #[doc = "0x2a0 - Channel Descriptor Source Data Address Register"]
    pub ch11_src: CH11_SRC,
    #[doc = "0x2a4 - Channel Descriptor Destination Data Address Register"]
    pub ch11_dst: CH11_DST,
    #[doc = "0x2a8 - Channel Descriptor Link Structure Address Register"]
    pub ch11_link: CH11_LINK,
    _reserved14: [u8; 20usize],
    #[doc = "0x2c0 - Channel Peripheral Request Select Register"]
    pub ch12_reqsel: CH12_REQSEL,
    #[doc = "0x2c4 - Channel Configuration Register"]
    pub ch12_cfg: CH12_CFG,
    #[doc = "0x2c8 - Channel Loop Counter Register"]
    pub ch12_loop: CH12_LOOP,
    #[doc = "0x2cc - Channel Descriptor Control Word Register"]
    pub ch12_ctrl: CH12_CTRL,
    #[doc = "0x2d0 - Channel Descriptor Source Data Address Register"]
    pub ch12_src: CH12_SRC,
    #[doc = "0x2d4 - Channel Descriptor Destination Data Address Register"]
    pub ch12_dst: CH12_DST,
    #[doc = "0x2d8 - Channel Descriptor Link Structure Address Register"]
    pub ch12_link: CH12_LINK,
    _reserved15: [u8; 20usize],
    #[doc = "0x2f0 - Channel Peripheral Request Select Register"]
    pub ch13_reqsel: CH13_REQSEL,
    #[doc = "0x2f4 - Channel Configuration Register"]
    pub ch13_cfg: CH13_CFG,
    #[doc = "0x2f8 - Channel Loop Counter Register"]
    pub ch13_loop: CH13_LOOP,
    #[doc = "0x2fc - Channel Descriptor Control Word Register"]
    pub ch13_ctrl: CH13_CTRL,
    #[doc = "0x300 - Channel Descriptor Source Data Address Register"]
    pub ch13_src: CH13_SRC,
    #[doc = "0x304 - Channel Descriptor Destination Data Address Register"]
    pub ch13_dst: CH13_DST,
    #[doc = "0x308 - Channel Descriptor Link Structure Address Register"]
    pub ch13_link: CH13_LINK,
    _reserved16: [u8; 20usize],
    #[doc = "0x320 - Channel Peripheral Request Select Register"]
    pub ch14_reqsel: CH14_REQSEL,
    #[doc = "0x324 - Channel Configuration Register"]
    pub ch14_cfg: CH14_CFG,
    #[doc = "0x328 - Channel Loop Counter Register"]
    pub ch14_loop: CH14_LOOP,
    #[doc = "0x32c - Channel Descriptor Control Word Register"]
    pub ch14_ctrl: CH14_CTRL,
    #[doc = "0x330 - Channel Descriptor Source Data Address Register"]
    pub ch14_src: CH14_SRC,
    #[doc = "0x334 - Channel Descriptor Destination Data Address Register"]
    pub ch14_dst: CH14_DST,
    #[doc = "0x338 - Channel Descriptor Link Structure Address Register"]
    pub ch14_link: CH14_LINK,
    _reserved17: [u8; 20usize],
    #[doc = "0x350 - Channel Peripheral Request Select Register"]
    pub ch15_reqsel: CH15_REQSEL,
    #[doc = "0x354 - Channel Configuration Register"]
    pub ch15_cfg: CH15_CFG,
    #[doc = "0x358 - Channel Loop Counter Register"]
    pub ch15_loop: CH15_LOOP,
    #[doc = "0x35c - Channel Descriptor Control Word Register"]
    pub ch15_ctrl: CH15_CTRL,
    #[doc = "0x360 - Channel Descriptor Source Data Address Register"]
    pub ch15_src: CH15_SRC,
    #[doc = "0x364 - Channel Descriptor Destination Data Address Register"]
    pub ch15_dst: CH15_DST,
    #[doc = "0x368 - Channel Descriptor Link Structure Address Register"]
    pub ch15_link: CH15_LINK,
    _reserved18: [u8; 20usize],
    #[doc = "0x380 - Channel Peripheral Request Select Register"]
    pub ch16_reqsel: CH16_REQSEL,
    #[doc = "0x384 - Channel Configuration Register"]
    pub ch16_cfg: CH16_CFG,
    #[doc = "0x388 - Channel Loop Counter Register"]
    pub ch16_loop: CH16_LOOP,
    #[doc = "0x38c - Channel Descriptor Control Word Register"]
    pub ch16_ctrl: CH16_CTRL,
    #[doc = "0x390 - Channel Descriptor Source Data Address Register"]
    pub ch16_src: CH16_SRC,
    #[doc = "0x394 - Channel Descriptor Destination Data Address Register"]
    pub ch16_dst: CH16_DST,
    #[doc = "0x398 - Channel Descriptor Link Structure Address Register"]
    pub ch16_link: CH16_LINK,
    _reserved19: [u8; 20usize],
    #[doc = "0x3b0 - Channel Peripheral Request Select Register"]
    pub ch17_reqsel: CH17_REQSEL,
    #[doc = "0x3b4 - Channel Configuration Register"]
    pub ch17_cfg: CH17_CFG,
    #[doc = "0x3b8 - Channel Loop Counter Register"]
    pub ch17_loop: CH17_LOOP,
    #[doc = "0x3bc - Channel Descriptor Control Word Register"]
    pub ch17_ctrl: CH17_CTRL,
    #[doc = "0x3c0 - Channel Descriptor Source Data Address Register"]
    pub ch17_src: CH17_SRC,
    #[doc = "0x3c4 - Channel Descriptor Destination Data Address Register"]
    pub ch17_dst: CH17_DST,
    #[doc = "0x3c8 - Channel Descriptor Link Structure Address Register"]
    pub ch17_link: CH17_LINK,
    _reserved20: [u8; 20usize],
    #[doc = "0x3e0 - Channel Peripheral Request Select Register"]
    pub ch18_reqsel: CH18_REQSEL,
    #[doc = "0x3e4 - Channel Configuration Register"]
    pub ch18_cfg: CH18_CFG,
    #[doc = "0x3e8 - Channel Loop Counter Register"]
    pub ch18_loop: CH18_LOOP,
    #[doc = "0x3ec - Channel Descriptor Control Word Register"]
    pub ch18_ctrl: CH18_CTRL,
    #[doc = "0x3f0 - Channel Descriptor Source Data Address Register"]
    pub ch18_src: CH18_SRC,
    #[doc = "0x3f4 - Channel Descriptor Destination Data Address Register"]
    pub ch18_dst: CH18_DST,
    #[doc = "0x3f8 - Channel Descriptor Link Structure Address Register"]
    pub ch18_link: CH18_LINK,
    _reserved21: [u8; 20usize],
    #[doc = "0x410 - Channel Peripheral Request Select Register"]
    pub ch19_reqsel: CH19_REQSEL,
    #[doc = "0x414 - Channel Configuration Register"]
    pub ch19_cfg: CH19_CFG,
    #[doc = "0x418 - Channel Loop Counter Register"]
    pub ch19_loop: CH19_LOOP,
    #[doc = "0x41c - Channel Descriptor Control Word Register"]
    pub ch19_ctrl: CH19_CTRL,
    #[doc = "0x420 - Channel Descriptor Source Data Address Register"]
    pub ch19_src: CH19_SRC,
    #[doc = "0x424 - Channel Descriptor Destination Data Address Register"]
    pub ch19_dst: CH19_DST,
    #[doc = "0x428 - Channel Descriptor Link Structure Address Register"]
    pub ch19_link: CH19_LINK,
    _reserved22: [u8; 20usize],
    #[doc = "0x440 - Channel Peripheral Request Select Register"]
    pub ch20_reqsel: CH20_REQSEL,
    #[doc = "0x444 - Channel Configuration Register"]
    pub ch20_cfg: CH20_CFG,
    #[doc = "0x448 - Channel Loop Counter Register"]
    pub ch20_loop: CH20_LOOP,
    #[doc = "0x44c - Channel Descriptor Control Word Register"]
    pub ch20_ctrl: CH20_CTRL,
    #[doc = "0x450 - Channel Descriptor Source Data Address Register"]
    pub ch20_src: CH20_SRC,
    #[doc = "0x454 - Channel Descriptor Destination Data Address Register"]
    pub ch20_dst: CH20_DST,
    #[doc = "0x458 - Channel Descriptor Link Structure Address Register"]
    pub ch20_link: CH20_LINK,
    _reserved23: [u8; 20usize],
    #[doc = "0x470 - Channel Peripheral Request Select Register"]
    pub ch21_reqsel: CH21_REQSEL,
    #[doc = "0x474 - Channel Configuration Register"]
    pub ch21_cfg: CH21_CFG,
    #[doc = "0x478 - Channel Loop Counter Register"]
    pub ch21_loop: CH21_LOOP,
    #[doc = "0x47c - Channel Descriptor Control Word Register"]
    pub ch21_ctrl: CH21_CTRL,
    #[doc = "0x480 - Channel Descriptor Source Data Address Register"]
    pub ch21_src: CH21_SRC,
    #[doc = "0x484 - Channel Descriptor Destination Data Address Register"]
    pub ch21_dst: CH21_DST,
    #[doc = "0x488 - Channel Descriptor Link Structure Address Register"]
    pub ch21_link: CH21_LINK,
    _reserved24: [u8; 20usize],
    #[doc = "0x4a0 - Channel Peripheral Request Select Register"]
    pub ch22_reqsel: CH22_REQSEL,
    #[doc = "0x4a4 - Channel Configuration Register"]
    pub ch22_cfg: CH22_CFG,
    #[doc = "0x4a8 - Channel Loop Counter Register"]
    pub ch22_loop: CH22_LOOP,
    #[doc = "0x4ac - Channel Descriptor Control Word Register"]
    pub ch22_ctrl: CH22_CTRL,
    #[doc = "0x4b0 - Channel Descriptor Source Data Address Register"]
    pub ch22_src: CH22_SRC,
    #[doc = "0x4b4 - Channel Descriptor Destination Data Address Register"]
    pub ch22_dst: CH22_DST,
    #[doc = "0x4b8 - Channel Descriptor Link Structure Address Register"]
    pub ch22_link: CH22_LINK,
    _reserved25: [u8; 20usize],
    #[doc = "0x4d0 - Channel Peripheral Request Select Register"]
    pub ch23_reqsel: CH23_REQSEL,
    #[doc = "0x4d4 - Channel Configuration Register"]
    pub ch23_cfg: CH23_CFG,
    #[doc = "0x4d8 - Channel Loop Counter Register"]
    pub ch23_loop: CH23_LOOP,
    #[doc = "0x4dc - Channel Descriptor Control Word Register"]
    pub ch23_ctrl: CH23_CTRL,
    #[doc = "0x4e0 - Channel Descriptor Source Data Address Register"]
    pub ch23_src: CH23_SRC,
    #[doc = "0x4e4 - Channel Descriptor Destination Data Address Register"]
    pub ch23_dst: CH23_DST,
    #[doc = "0x4e8 - Channel Descriptor Link Structure Address Register"]
    pub ch23_link: CH23_LINK,
}
#[doc = "DMA Control Register"]
pub struct CTRL {
    register: VolatileCell<u32>,
}
#[doc = "DMA Control Register"]
pub mod ctrl;
#[doc = "DMA Status Register"]
pub struct STATUS {
    register: VolatileCell<u32>,
}
#[doc = "DMA Status Register"]
pub mod status;
#[doc = "DMA Synchronization Trigger Register (Single-Cycle RMW)"]
pub struct SYNC {
    register: VolatileCell<u32>,
}
#[doc = "DMA Synchronization Trigger Register (Single-Cycle RMW)"]
pub mod sync;
#[doc = "DMA Channel Enable Register (Single-Cycle RMW)"]
pub struct CHEN {
    register: VolatileCell<u32>,
}
#[doc = "DMA Channel Enable Register (Single-Cycle RMW)"]
pub mod chen;
#[doc = "DMA Channel Busy Register"]
pub struct CHBUSY {
    register: VolatileCell<u32>,
}
#[doc = "DMA Channel Busy Register"]
pub mod chbusy;
#[doc = "DMA Channel Linking Done Register (Single-Cycle RMW)"]
pub struct CHDONE {
    register: VolatileCell<u32>,
}
#[doc = "DMA Channel Linking Done Register (Single-Cycle RMW)"]
pub mod chdone;
#[doc = "DMA Channel Debug Halt Register"]
pub struct DBGHALT {
    register: VolatileCell<u32>,
}
#[doc = "DMA Channel Debug Halt Register"]
pub mod dbghalt;
#[doc = "DMA Channel Software Transfer Request Register"]
pub struct SWREQ {
    register: VolatileCell<u32>,
}
#[doc = "DMA Channel Software Transfer Request Register"]
pub mod swreq;
#[doc = "DMA Channel Request Disable Register"]
pub struct REQDIS {
    register: VolatileCell<u32>,
}
#[doc = "DMA Channel Request Disable Register"]
pub mod reqdis;
#[doc = "DMA Channel Requests Pending Register"]
pub struct REQPEND {
    register: VolatileCell<u32>,
}
#[doc = "DMA Channel Requests Pending Register"]
pub mod reqpend;
#[doc = "DMA Channel Link Load Register"]
pub struct LINKLOAD {
    register: VolatileCell<u32>,
}
#[doc = "DMA Channel Link Load Register"]
pub mod linkload;
#[doc = "DMA Channel Request Clear Register"]
pub struct REQCLEAR {
    register: VolatileCell<u32>,
}
#[doc = "DMA Channel Request Clear Register"]
pub mod reqclear;
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
#[doc = "Channel Peripheral Request Select Register"]
pub struct CH0_REQSEL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch0_reqsel;
#[doc = "Channel Configuration Register"]
pub struct CH0_CFG {
    register: VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ch0_cfg;
#[doc = "Channel Loop Counter Register"]
pub struct CH0_LOOP {
    register: VolatileCell<u32>,
}
#[doc = "Channel Loop Counter Register"]
pub mod ch0_loop;
#[doc = "Channel Descriptor Control Word Register"]
pub struct CH0_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch0_ctrl;
#[doc = "Channel Descriptor Source Data Address Register"]
pub struct CH0_SRC {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch0_src;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub struct CH0_DST {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch0_dst;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub struct CH0_LINK {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch0_link;
#[doc = "Channel Peripheral Request Select Register"]
pub struct CH1_REQSEL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch1_reqsel;
#[doc = "Channel Configuration Register"]
pub struct CH1_CFG {
    register: VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ch1_cfg;
#[doc = "Channel Loop Counter Register"]
pub struct CH1_LOOP {
    register: VolatileCell<u32>,
}
#[doc = "Channel Loop Counter Register"]
pub mod ch1_loop;
#[doc = "Channel Descriptor Control Word Register"]
pub struct CH1_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch1_ctrl;
#[doc = "Channel Descriptor Source Data Address Register"]
pub struct CH1_SRC {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch1_src;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub struct CH1_DST {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch1_dst;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub struct CH1_LINK {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch1_link;
#[doc = "Channel Peripheral Request Select Register"]
pub struct CH2_REQSEL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch2_reqsel;
#[doc = "Channel Configuration Register"]
pub struct CH2_CFG {
    register: VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ch2_cfg;
#[doc = "Channel Loop Counter Register"]
pub struct CH2_LOOP {
    register: VolatileCell<u32>,
}
#[doc = "Channel Loop Counter Register"]
pub mod ch2_loop;
#[doc = "Channel Descriptor Control Word Register"]
pub struct CH2_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch2_ctrl;
#[doc = "Channel Descriptor Source Data Address Register"]
pub struct CH2_SRC {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch2_src;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub struct CH2_DST {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch2_dst;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub struct CH2_LINK {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch2_link;
#[doc = "Channel Peripheral Request Select Register"]
pub struct CH3_REQSEL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch3_reqsel;
#[doc = "Channel Configuration Register"]
pub struct CH3_CFG {
    register: VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ch3_cfg;
#[doc = "Channel Loop Counter Register"]
pub struct CH3_LOOP {
    register: VolatileCell<u32>,
}
#[doc = "Channel Loop Counter Register"]
pub mod ch3_loop;
#[doc = "Channel Descriptor Control Word Register"]
pub struct CH3_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch3_ctrl;
#[doc = "Channel Descriptor Source Data Address Register"]
pub struct CH3_SRC {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch3_src;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub struct CH3_DST {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch3_dst;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub struct CH3_LINK {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch3_link;
#[doc = "Channel Peripheral Request Select Register"]
pub struct CH4_REQSEL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch4_reqsel;
#[doc = "Channel Configuration Register"]
pub struct CH4_CFG {
    register: VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ch4_cfg;
#[doc = "Channel Loop Counter Register"]
pub struct CH4_LOOP {
    register: VolatileCell<u32>,
}
#[doc = "Channel Loop Counter Register"]
pub mod ch4_loop;
#[doc = "Channel Descriptor Control Word Register"]
pub struct CH4_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch4_ctrl;
#[doc = "Channel Descriptor Source Data Address Register"]
pub struct CH4_SRC {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch4_src;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub struct CH4_DST {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch4_dst;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub struct CH4_LINK {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch4_link;
#[doc = "Channel Peripheral Request Select Register"]
pub struct CH5_REQSEL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch5_reqsel;
#[doc = "Channel Configuration Register"]
pub struct CH5_CFG {
    register: VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ch5_cfg;
#[doc = "Channel Loop Counter Register"]
pub struct CH5_LOOP {
    register: VolatileCell<u32>,
}
#[doc = "Channel Loop Counter Register"]
pub mod ch5_loop;
#[doc = "Channel Descriptor Control Word Register"]
pub struct CH5_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch5_ctrl;
#[doc = "Channel Descriptor Source Data Address Register"]
pub struct CH5_SRC {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch5_src;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub struct CH5_DST {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch5_dst;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub struct CH5_LINK {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch5_link;
#[doc = "Channel Peripheral Request Select Register"]
pub struct CH6_REQSEL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch6_reqsel;
#[doc = "Channel Configuration Register"]
pub struct CH6_CFG {
    register: VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ch6_cfg;
#[doc = "Channel Loop Counter Register"]
pub struct CH6_LOOP {
    register: VolatileCell<u32>,
}
#[doc = "Channel Loop Counter Register"]
pub mod ch6_loop;
#[doc = "Channel Descriptor Control Word Register"]
pub struct CH6_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch6_ctrl;
#[doc = "Channel Descriptor Source Data Address Register"]
pub struct CH6_SRC {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch6_src;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub struct CH6_DST {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch6_dst;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub struct CH6_LINK {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch6_link;
#[doc = "Channel Peripheral Request Select Register"]
pub struct CH7_REQSEL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch7_reqsel;
#[doc = "Channel Configuration Register"]
pub struct CH7_CFG {
    register: VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ch7_cfg;
#[doc = "Channel Loop Counter Register"]
pub struct CH7_LOOP {
    register: VolatileCell<u32>,
}
#[doc = "Channel Loop Counter Register"]
pub mod ch7_loop;
#[doc = "Channel Descriptor Control Word Register"]
pub struct CH7_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch7_ctrl;
#[doc = "Channel Descriptor Source Data Address Register"]
pub struct CH7_SRC {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch7_src;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub struct CH7_DST {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch7_dst;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub struct CH7_LINK {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch7_link;
#[doc = "Channel Peripheral Request Select Register"]
pub struct CH8_REQSEL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch8_reqsel;
#[doc = "Channel Configuration Register"]
pub struct CH8_CFG {
    register: VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ch8_cfg;
#[doc = "Channel Loop Counter Register"]
pub struct CH8_LOOP {
    register: VolatileCell<u32>,
}
#[doc = "Channel Loop Counter Register"]
pub mod ch8_loop;
#[doc = "Channel Descriptor Control Word Register"]
pub struct CH8_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch8_ctrl;
#[doc = "Channel Descriptor Source Data Address Register"]
pub struct CH8_SRC {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch8_src;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub struct CH8_DST {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch8_dst;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub struct CH8_LINK {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch8_link;
#[doc = "Channel Peripheral Request Select Register"]
pub struct CH9_REQSEL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch9_reqsel;
#[doc = "Channel Configuration Register"]
pub struct CH9_CFG {
    register: VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ch9_cfg;
#[doc = "Channel Loop Counter Register"]
pub struct CH9_LOOP {
    register: VolatileCell<u32>,
}
#[doc = "Channel Loop Counter Register"]
pub mod ch9_loop;
#[doc = "Channel Descriptor Control Word Register"]
pub struct CH9_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch9_ctrl;
#[doc = "Channel Descriptor Source Data Address Register"]
pub struct CH9_SRC {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch9_src;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub struct CH9_DST {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch9_dst;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub struct CH9_LINK {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch9_link;
#[doc = "Channel Peripheral Request Select Register"]
pub struct CH10_REQSEL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch10_reqsel;
#[doc = "Channel Configuration Register"]
pub struct CH10_CFG {
    register: VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ch10_cfg;
#[doc = "Channel Loop Counter Register"]
pub struct CH10_LOOP {
    register: VolatileCell<u32>,
}
#[doc = "Channel Loop Counter Register"]
pub mod ch10_loop;
#[doc = "Channel Descriptor Control Word Register"]
pub struct CH10_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch10_ctrl;
#[doc = "Channel Descriptor Source Data Address Register"]
pub struct CH10_SRC {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch10_src;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub struct CH10_DST {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch10_dst;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub struct CH10_LINK {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch10_link;
#[doc = "Channel Peripheral Request Select Register"]
pub struct CH11_REQSEL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch11_reqsel;
#[doc = "Channel Configuration Register"]
pub struct CH11_CFG {
    register: VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ch11_cfg;
#[doc = "Channel Loop Counter Register"]
pub struct CH11_LOOP {
    register: VolatileCell<u32>,
}
#[doc = "Channel Loop Counter Register"]
pub mod ch11_loop;
#[doc = "Channel Descriptor Control Word Register"]
pub struct CH11_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch11_ctrl;
#[doc = "Channel Descriptor Source Data Address Register"]
pub struct CH11_SRC {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch11_src;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub struct CH11_DST {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch11_dst;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub struct CH11_LINK {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch11_link;
#[doc = "Channel Peripheral Request Select Register"]
pub struct CH12_REQSEL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch12_reqsel;
#[doc = "Channel Configuration Register"]
pub struct CH12_CFG {
    register: VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ch12_cfg;
#[doc = "Channel Loop Counter Register"]
pub struct CH12_LOOP {
    register: VolatileCell<u32>,
}
#[doc = "Channel Loop Counter Register"]
pub mod ch12_loop;
#[doc = "Channel Descriptor Control Word Register"]
pub struct CH12_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch12_ctrl;
#[doc = "Channel Descriptor Source Data Address Register"]
pub struct CH12_SRC {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch12_src;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub struct CH12_DST {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch12_dst;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub struct CH12_LINK {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch12_link;
#[doc = "Channel Peripheral Request Select Register"]
pub struct CH13_REQSEL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch13_reqsel;
#[doc = "Channel Configuration Register"]
pub struct CH13_CFG {
    register: VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ch13_cfg;
#[doc = "Channel Loop Counter Register"]
pub struct CH13_LOOP {
    register: VolatileCell<u32>,
}
#[doc = "Channel Loop Counter Register"]
pub mod ch13_loop;
#[doc = "Channel Descriptor Control Word Register"]
pub struct CH13_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch13_ctrl;
#[doc = "Channel Descriptor Source Data Address Register"]
pub struct CH13_SRC {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch13_src;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub struct CH13_DST {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch13_dst;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub struct CH13_LINK {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch13_link;
#[doc = "Channel Peripheral Request Select Register"]
pub struct CH14_REQSEL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch14_reqsel;
#[doc = "Channel Configuration Register"]
pub struct CH14_CFG {
    register: VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ch14_cfg;
#[doc = "Channel Loop Counter Register"]
pub struct CH14_LOOP {
    register: VolatileCell<u32>,
}
#[doc = "Channel Loop Counter Register"]
pub mod ch14_loop;
#[doc = "Channel Descriptor Control Word Register"]
pub struct CH14_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch14_ctrl;
#[doc = "Channel Descriptor Source Data Address Register"]
pub struct CH14_SRC {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch14_src;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub struct CH14_DST {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch14_dst;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub struct CH14_LINK {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch14_link;
#[doc = "Channel Peripheral Request Select Register"]
pub struct CH15_REQSEL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch15_reqsel;
#[doc = "Channel Configuration Register"]
pub struct CH15_CFG {
    register: VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ch15_cfg;
#[doc = "Channel Loop Counter Register"]
pub struct CH15_LOOP {
    register: VolatileCell<u32>,
}
#[doc = "Channel Loop Counter Register"]
pub mod ch15_loop;
#[doc = "Channel Descriptor Control Word Register"]
pub struct CH15_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch15_ctrl;
#[doc = "Channel Descriptor Source Data Address Register"]
pub struct CH15_SRC {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch15_src;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub struct CH15_DST {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch15_dst;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub struct CH15_LINK {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch15_link;
#[doc = "Channel Peripheral Request Select Register"]
pub struct CH16_REQSEL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch16_reqsel;
#[doc = "Channel Configuration Register"]
pub struct CH16_CFG {
    register: VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ch16_cfg;
#[doc = "Channel Loop Counter Register"]
pub struct CH16_LOOP {
    register: VolatileCell<u32>,
}
#[doc = "Channel Loop Counter Register"]
pub mod ch16_loop;
#[doc = "Channel Descriptor Control Word Register"]
pub struct CH16_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch16_ctrl;
#[doc = "Channel Descriptor Source Data Address Register"]
pub struct CH16_SRC {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch16_src;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub struct CH16_DST {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch16_dst;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub struct CH16_LINK {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch16_link;
#[doc = "Channel Peripheral Request Select Register"]
pub struct CH17_REQSEL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch17_reqsel;
#[doc = "Channel Configuration Register"]
pub struct CH17_CFG {
    register: VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ch17_cfg;
#[doc = "Channel Loop Counter Register"]
pub struct CH17_LOOP {
    register: VolatileCell<u32>,
}
#[doc = "Channel Loop Counter Register"]
pub mod ch17_loop;
#[doc = "Channel Descriptor Control Word Register"]
pub struct CH17_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch17_ctrl;
#[doc = "Channel Descriptor Source Data Address Register"]
pub struct CH17_SRC {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch17_src;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub struct CH17_DST {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch17_dst;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub struct CH17_LINK {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch17_link;
#[doc = "Channel Peripheral Request Select Register"]
pub struct CH18_REQSEL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch18_reqsel;
#[doc = "Channel Configuration Register"]
pub struct CH18_CFG {
    register: VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ch18_cfg;
#[doc = "Channel Loop Counter Register"]
pub struct CH18_LOOP {
    register: VolatileCell<u32>,
}
#[doc = "Channel Loop Counter Register"]
pub mod ch18_loop;
#[doc = "Channel Descriptor Control Word Register"]
pub struct CH18_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch18_ctrl;
#[doc = "Channel Descriptor Source Data Address Register"]
pub struct CH18_SRC {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch18_src;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub struct CH18_DST {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch18_dst;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub struct CH18_LINK {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch18_link;
#[doc = "Channel Peripheral Request Select Register"]
pub struct CH19_REQSEL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch19_reqsel;
#[doc = "Channel Configuration Register"]
pub struct CH19_CFG {
    register: VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ch19_cfg;
#[doc = "Channel Loop Counter Register"]
pub struct CH19_LOOP {
    register: VolatileCell<u32>,
}
#[doc = "Channel Loop Counter Register"]
pub mod ch19_loop;
#[doc = "Channel Descriptor Control Word Register"]
pub struct CH19_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch19_ctrl;
#[doc = "Channel Descriptor Source Data Address Register"]
pub struct CH19_SRC {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch19_src;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub struct CH19_DST {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch19_dst;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub struct CH19_LINK {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch19_link;
#[doc = "Channel Peripheral Request Select Register"]
pub struct CH20_REQSEL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch20_reqsel;
#[doc = "Channel Configuration Register"]
pub struct CH20_CFG {
    register: VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ch20_cfg;
#[doc = "Channel Loop Counter Register"]
pub struct CH20_LOOP {
    register: VolatileCell<u32>,
}
#[doc = "Channel Loop Counter Register"]
pub mod ch20_loop;
#[doc = "Channel Descriptor Control Word Register"]
pub struct CH20_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch20_ctrl;
#[doc = "Channel Descriptor Source Data Address Register"]
pub struct CH20_SRC {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch20_src;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub struct CH20_DST {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch20_dst;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub struct CH20_LINK {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch20_link;
#[doc = "Channel Peripheral Request Select Register"]
pub struct CH21_REQSEL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch21_reqsel;
#[doc = "Channel Configuration Register"]
pub struct CH21_CFG {
    register: VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ch21_cfg;
#[doc = "Channel Loop Counter Register"]
pub struct CH21_LOOP {
    register: VolatileCell<u32>,
}
#[doc = "Channel Loop Counter Register"]
pub mod ch21_loop;
#[doc = "Channel Descriptor Control Word Register"]
pub struct CH21_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch21_ctrl;
#[doc = "Channel Descriptor Source Data Address Register"]
pub struct CH21_SRC {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch21_src;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub struct CH21_DST {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch21_dst;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub struct CH21_LINK {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch21_link;
#[doc = "Channel Peripheral Request Select Register"]
pub struct CH22_REQSEL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch22_reqsel;
#[doc = "Channel Configuration Register"]
pub struct CH22_CFG {
    register: VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ch22_cfg;
#[doc = "Channel Loop Counter Register"]
pub struct CH22_LOOP {
    register: VolatileCell<u32>,
}
#[doc = "Channel Loop Counter Register"]
pub mod ch22_loop;
#[doc = "Channel Descriptor Control Word Register"]
pub struct CH22_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch22_ctrl;
#[doc = "Channel Descriptor Source Data Address Register"]
pub struct CH22_SRC {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch22_src;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub struct CH22_DST {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch22_dst;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub struct CH22_LINK {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch22_link;
#[doc = "Channel Peripheral Request Select Register"]
pub struct CH23_REQSEL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch23_reqsel;
#[doc = "Channel Configuration Register"]
pub struct CH23_CFG {
    register: VolatileCell<u32>,
}
#[doc = "Channel Configuration Register"]
pub mod ch23_cfg;
#[doc = "Channel Loop Counter Register"]
pub struct CH23_LOOP {
    register: VolatileCell<u32>,
}
#[doc = "Channel Loop Counter Register"]
pub mod ch23_loop;
#[doc = "Channel Descriptor Control Word Register"]
pub struct CH23_CTRL {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch23_ctrl;
#[doc = "Channel Descriptor Source Data Address Register"]
pub struct CH23_SRC {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch23_src;
#[doc = "Channel Descriptor Destination Data Address Register"]
pub struct CH23_DST {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch23_dst;
#[doc = "Channel Descriptor Link Structure Address Register"]
pub struct CH23_LINK {
    register: VolatileCell<u32>,
}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch23_link;
