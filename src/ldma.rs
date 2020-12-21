#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - DMA Status Register"]
    pub status: STATUS,
    #[doc = "0x08 - DMA Synchronization Trigger Register (Single-Cycle RMW)"]
    pub sync: SYNC,
    _reserved3: [u8; 20usize],
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
    _reserved12: [u8; 28usize],
    #[doc = "0x60 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x64 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x68 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x6c - Interrupt Enable Register"]
    pub ien: IEN,
    _reserved16: [u8; 16usize],
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
    _reserved23: [u8; 20usize],
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
    _reserved30: [u8; 20usize],
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
    _reserved37: [u8; 20usize],
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
    _reserved44: [u8; 20usize],
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
    _reserved51: [u8; 20usize],
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
    _reserved58: [u8; 20usize],
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
    _reserved65: [u8; 20usize],
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
    _reserved72: [u8; 20usize],
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
    _reserved79: [u8; 20usize],
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
    _reserved86: [u8; 20usize],
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
    _reserved93: [u8; 20usize],
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
    _reserved100: [u8; 20usize],
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
    _reserved107: [u8; 20usize],
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
    _reserved114: [u8; 20usize],
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
    _reserved121: [u8; 20usize],
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
    _reserved128: [u8; 20usize],
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
    _reserved135: [u8; 20usize],
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
    _reserved142: [u8; 20usize],
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
    _reserved149: [u8; 20usize],
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
    _reserved156: [u8; 20usize],
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
    _reserved163: [u8; 20usize],
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
    _reserved170: [u8; 20usize],
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
    _reserved177: [u8; 20usize],
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
#[doc = "DMA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "DMA Control Register"]
pub mod ctrl;
#[doc = "DMA Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "DMA Status Register"]
pub mod status;
#[doc = "DMA Synchronization Trigger Register (Single-Cycle RMW)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sync](sync) module"]
pub type SYNC = crate::Reg<u32, _SYNC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNC;
#[doc = "`read()` method returns [sync::R](sync::R) reader structure"]
impl crate::Readable for SYNC {}
#[doc = "`write(|w| ..)` method takes [sync::W](sync::W) writer structure"]
impl crate::Writable for SYNC {}
#[doc = "DMA Synchronization Trigger Register (Single-Cycle RMW)"]
pub mod sync;
#[doc = "DMA Channel Enable Register (Single-Cycle RMW)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chen](chen) module"]
pub type CHEN = crate::Reg<u32, _CHEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHEN;
#[doc = "`read()` method returns [chen::R](chen::R) reader structure"]
impl crate::Readable for CHEN {}
#[doc = "`write(|w| ..)` method takes [chen::W](chen::W) writer structure"]
impl crate::Writable for CHEN {}
#[doc = "DMA Channel Enable Register (Single-Cycle RMW)"]
pub mod chen;
#[doc = "DMA Channel Busy Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chbusy](chbusy) module"]
pub type CHBUSY = crate::Reg<u32, _CHBUSY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHBUSY;
#[doc = "`read()` method returns [chbusy::R](chbusy::R) reader structure"]
impl crate::Readable for CHBUSY {}
#[doc = "DMA Channel Busy Register"]
pub mod chbusy;
#[doc = "DMA Channel Linking Done Register (Single-Cycle RMW)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chdone](chdone) module"]
pub type CHDONE = crate::Reg<u32, _CHDONE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHDONE;
#[doc = "`read()` method returns [chdone::R](chdone::R) reader structure"]
impl crate::Readable for CHDONE {}
#[doc = "`write(|w| ..)` method takes [chdone::W](chdone::W) writer structure"]
impl crate::Writable for CHDONE {}
#[doc = "DMA Channel Linking Done Register (Single-Cycle RMW)"]
pub mod chdone;
#[doc = "DMA Channel Debug Halt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbghalt](dbghalt) module"]
pub type DBGHALT = crate::Reg<u32, _DBGHALT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DBGHALT;
#[doc = "`read()` method returns [dbghalt::R](dbghalt::R) reader structure"]
impl crate::Readable for DBGHALT {}
#[doc = "`write(|w| ..)` method takes [dbghalt::W](dbghalt::W) writer structure"]
impl crate::Writable for DBGHALT {}
#[doc = "DMA Channel Debug Halt Register"]
pub mod dbghalt;
#[doc = "DMA Channel Software Transfer Request Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swreq](swreq) module"]
pub type SWREQ = crate::Reg<u32, _SWREQ>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SWREQ;
#[doc = "`write(|w| ..)` method takes [swreq::W](swreq::W) writer structure"]
impl crate::Writable for SWREQ {}
#[doc = "DMA Channel Software Transfer Request Register"]
pub mod swreq;
#[doc = "DMA Channel Request Disable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reqdis](reqdis) module"]
pub type REQDIS = crate::Reg<u32, _REQDIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REQDIS;
#[doc = "`read()` method returns [reqdis::R](reqdis::R) reader structure"]
impl crate::Readable for REQDIS {}
#[doc = "`write(|w| ..)` method takes [reqdis::W](reqdis::W) writer structure"]
impl crate::Writable for REQDIS {}
#[doc = "DMA Channel Request Disable Register"]
pub mod reqdis;
#[doc = "DMA Channel Requests Pending Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reqpend](reqpend) module"]
pub type REQPEND = crate::Reg<u32, _REQPEND>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REQPEND;
#[doc = "`read()` method returns [reqpend::R](reqpend::R) reader structure"]
impl crate::Readable for REQPEND {}
#[doc = "DMA Channel Requests Pending Register"]
pub mod reqpend;
#[doc = "DMA Channel Link Load Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [linkload](linkload) module"]
pub type LINKLOAD = crate::Reg<u32, _LINKLOAD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LINKLOAD;
#[doc = "`write(|w| ..)` method takes [linkload::W](linkload::W) writer structure"]
impl crate::Writable for LINKLOAD {}
#[doc = "DMA Channel Link Load Register"]
pub mod linkload;
#[doc = "DMA Channel Request Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reqclear](reqclear) module"]
pub type REQCLEAR = crate::Reg<u32, _REQCLEAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REQCLEAR;
#[doc = "`write(|w| ..)` method takes [reqclear::W](reqclear::W) writer structure"]
impl crate::Writable for REQCLEAR {}
#[doc = "DMA Channel Request Clear Register"]
pub mod reqclear;
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
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_reqsel](ch0_reqsel) module"]
pub type CH0_REQSEL = crate::Reg<u32, _CH0_REQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_REQSEL;
#[doc = "`read()` method returns [ch0_reqsel::R](ch0_reqsel::R) reader structure"]
impl crate::Readable for CH0_REQSEL {}
#[doc = "`write(|w| ..)` method takes [ch0_reqsel::W](ch0_reqsel::W) writer structure"]
impl crate::Writable for CH0_REQSEL {}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch0_reqsel;
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_cfg](ch0_cfg) module"]
pub type CH0_CFG = crate::Reg<u32, _CH0_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_CFG;
#[doc = "`read()` method returns [ch0_cfg::R](ch0_cfg::R) reader structure"]
impl crate::Readable for CH0_CFG {}
#[doc = "`write(|w| ..)` method takes [ch0_cfg::W](ch0_cfg::W) writer structure"]
impl crate::Writable for CH0_CFG {}
#[doc = "Channel Configuration Register"]
pub mod ch0_cfg;
#[doc = "Channel Loop Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_loop](ch0_loop) module"]
pub type CH0_LOOP = crate::Reg<u32, _CH0_LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_LOOP;
#[doc = "`read()` method returns [ch0_loop::R](ch0_loop::R) reader structure"]
impl crate::Readable for CH0_LOOP {}
#[doc = "`write(|w| ..)` method takes [ch0_loop::W](ch0_loop::W) writer structure"]
impl crate::Writable for CH0_LOOP {}
#[doc = "Channel Loop Counter Register"]
pub mod ch0_loop;
#[doc = "Channel Descriptor Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_ctrl](ch0_ctrl) module"]
pub type CH0_CTRL = crate::Reg<u32, _CH0_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_CTRL;
#[doc = "`read()` method returns [ch0_ctrl::R](ch0_ctrl::R) reader structure"]
impl crate::Readable for CH0_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch0_ctrl::W](ch0_ctrl::W) writer structure"]
impl crate::Writable for CH0_CTRL {}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch0_ctrl;
#[doc = "Channel Descriptor Source Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_src](ch0_src) module"]
pub type CH0_SRC = crate::Reg<u32, _CH0_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_SRC;
#[doc = "`read()` method returns [ch0_src::R](ch0_src::R) reader structure"]
impl crate::Readable for CH0_SRC {}
#[doc = "`write(|w| ..)` method takes [ch0_src::W](ch0_src::W) writer structure"]
impl crate::Writable for CH0_SRC {}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch0_src;
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_dst](ch0_dst) module"]
pub type CH0_DST = crate::Reg<u32, _CH0_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_DST;
#[doc = "`read()` method returns [ch0_dst::R](ch0_dst::R) reader structure"]
impl crate::Readable for CH0_DST {}
#[doc = "`write(|w| ..)` method takes [ch0_dst::W](ch0_dst::W) writer structure"]
impl crate::Writable for CH0_DST {}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch0_dst;
#[doc = "Channel Descriptor Link Structure Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_link](ch0_link) module"]
pub type CH0_LINK = crate::Reg<u32, _CH0_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH0_LINK;
#[doc = "`read()` method returns [ch0_link::R](ch0_link::R) reader structure"]
impl crate::Readable for CH0_LINK {}
#[doc = "`write(|w| ..)` method takes [ch0_link::W](ch0_link::W) writer structure"]
impl crate::Writable for CH0_LINK {}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch0_link;
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_reqsel](ch1_reqsel) module"]
pub type CH1_REQSEL = crate::Reg<u32, _CH1_REQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_REQSEL;
#[doc = "`read()` method returns [ch1_reqsel::R](ch1_reqsel::R) reader structure"]
impl crate::Readable for CH1_REQSEL {}
#[doc = "`write(|w| ..)` method takes [ch1_reqsel::W](ch1_reqsel::W) writer structure"]
impl crate::Writable for CH1_REQSEL {}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch1_reqsel;
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_cfg](ch1_cfg) module"]
pub type CH1_CFG = crate::Reg<u32, _CH1_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_CFG;
#[doc = "`read()` method returns [ch1_cfg::R](ch1_cfg::R) reader structure"]
impl crate::Readable for CH1_CFG {}
#[doc = "`write(|w| ..)` method takes [ch1_cfg::W](ch1_cfg::W) writer structure"]
impl crate::Writable for CH1_CFG {}
#[doc = "Channel Configuration Register"]
pub mod ch1_cfg;
#[doc = "Channel Loop Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_loop](ch1_loop) module"]
pub type CH1_LOOP = crate::Reg<u32, _CH1_LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_LOOP;
#[doc = "`read()` method returns [ch1_loop::R](ch1_loop::R) reader structure"]
impl crate::Readable for CH1_LOOP {}
#[doc = "`write(|w| ..)` method takes [ch1_loop::W](ch1_loop::W) writer structure"]
impl crate::Writable for CH1_LOOP {}
#[doc = "Channel Loop Counter Register"]
pub mod ch1_loop;
#[doc = "Channel Descriptor Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_ctrl](ch1_ctrl) module"]
pub type CH1_CTRL = crate::Reg<u32, _CH1_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_CTRL;
#[doc = "`read()` method returns [ch1_ctrl::R](ch1_ctrl::R) reader structure"]
impl crate::Readable for CH1_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch1_ctrl::W](ch1_ctrl::W) writer structure"]
impl crate::Writable for CH1_CTRL {}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch1_ctrl;
#[doc = "Channel Descriptor Source Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_src](ch1_src) module"]
pub type CH1_SRC = crate::Reg<u32, _CH1_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_SRC;
#[doc = "`read()` method returns [ch1_src::R](ch1_src::R) reader structure"]
impl crate::Readable for CH1_SRC {}
#[doc = "`write(|w| ..)` method takes [ch1_src::W](ch1_src::W) writer structure"]
impl crate::Writable for CH1_SRC {}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch1_src;
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_dst](ch1_dst) module"]
pub type CH1_DST = crate::Reg<u32, _CH1_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_DST;
#[doc = "`read()` method returns [ch1_dst::R](ch1_dst::R) reader structure"]
impl crate::Readable for CH1_DST {}
#[doc = "`write(|w| ..)` method takes [ch1_dst::W](ch1_dst::W) writer structure"]
impl crate::Writable for CH1_DST {}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch1_dst;
#[doc = "Channel Descriptor Link Structure Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1_link](ch1_link) module"]
pub type CH1_LINK = crate::Reg<u32, _CH1_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH1_LINK;
#[doc = "`read()` method returns [ch1_link::R](ch1_link::R) reader structure"]
impl crate::Readable for CH1_LINK {}
#[doc = "`write(|w| ..)` method takes [ch1_link::W](ch1_link::W) writer structure"]
impl crate::Writable for CH1_LINK {}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch1_link;
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_reqsel](ch2_reqsel) module"]
pub type CH2_REQSEL = crate::Reg<u32, _CH2_REQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_REQSEL;
#[doc = "`read()` method returns [ch2_reqsel::R](ch2_reqsel::R) reader structure"]
impl crate::Readable for CH2_REQSEL {}
#[doc = "`write(|w| ..)` method takes [ch2_reqsel::W](ch2_reqsel::W) writer structure"]
impl crate::Writable for CH2_REQSEL {}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch2_reqsel;
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_cfg](ch2_cfg) module"]
pub type CH2_CFG = crate::Reg<u32, _CH2_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_CFG;
#[doc = "`read()` method returns [ch2_cfg::R](ch2_cfg::R) reader structure"]
impl crate::Readable for CH2_CFG {}
#[doc = "`write(|w| ..)` method takes [ch2_cfg::W](ch2_cfg::W) writer structure"]
impl crate::Writable for CH2_CFG {}
#[doc = "Channel Configuration Register"]
pub mod ch2_cfg;
#[doc = "Channel Loop Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_loop](ch2_loop) module"]
pub type CH2_LOOP = crate::Reg<u32, _CH2_LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_LOOP;
#[doc = "`read()` method returns [ch2_loop::R](ch2_loop::R) reader structure"]
impl crate::Readable for CH2_LOOP {}
#[doc = "`write(|w| ..)` method takes [ch2_loop::W](ch2_loop::W) writer structure"]
impl crate::Writable for CH2_LOOP {}
#[doc = "Channel Loop Counter Register"]
pub mod ch2_loop;
#[doc = "Channel Descriptor Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_ctrl](ch2_ctrl) module"]
pub type CH2_CTRL = crate::Reg<u32, _CH2_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_CTRL;
#[doc = "`read()` method returns [ch2_ctrl::R](ch2_ctrl::R) reader structure"]
impl crate::Readable for CH2_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch2_ctrl::W](ch2_ctrl::W) writer structure"]
impl crate::Writable for CH2_CTRL {}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch2_ctrl;
#[doc = "Channel Descriptor Source Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_src](ch2_src) module"]
pub type CH2_SRC = crate::Reg<u32, _CH2_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_SRC;
#[doc = "`read()` method returns [ch2_src::R](ch2_src::R) reader structure"]
impl crate::Readable for CH2_SRC {}
#[doc = "`write(|w| ..)` method takes [ch2_src::W](ch2_src::W) writer structure"]
impl crate::Writable for CH2_SRC {}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch2_src;
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_dst](ch2_dst) module"]
pub type CH2_DST = crate::Reg<u32, _CH2_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_DST;
#[doc = "`read()` method returns [ch2_dst::R](ch2_dst::R) reader structure"]
impl crate::Readable for CH2_DST {}
#[doc = "`write(|w| ..)` method takes [ch2_dst::W](ch2_dst::W) writer structure"]
impl crate::Writable for CH2_DST {}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch2_dst;
#[doc = "Channel Descriptor Link Structure Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_link](ch2_link) module"]
pub type CH2_LINK = crate::Reg<u32, _CH2_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH2_LINK;
#[doc = "`read()` method returns [ch2_link::R](ch2_link::R) reader structure"]
impl crate::Readable for CH2_LINK {}
#[doc = "`write(|w| ..)` method takes [ch2_link::W](ch2_link::W) writer structure"]
impl crate::Writable for CH2_LINK {}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch2_link;
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_reqsel](ch3_reqsel) module"]
pub type CH3_REQSEL = crate::Reg<u32, _CH3_REQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_REQSEL;
#[doc = "`read()` method returns [ch3_reqsel::R](ch3_reqsel::R) reader structure"]
impl crate::Readable for CH3_REQSEL {}
#[doc = "`write(|w| ..)` method takes [ch3_reqsel::W](ch3_reqsel::W) writer structure"]
impl crate::Writable for CH3_REQSEL {}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch3_reqsel;
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_cfg](ch3_cfg) module"]
pub type CH3_CFG = crate::Reg<u32, _CH3_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_CFG;
#[doc = "`read()` method returns [ch3_cfg::R](ch3_cfg::R) reader structure"]
impl crate::Readable for CH3_CFG {}
#[doc = "`write(|w| ..)` method takes [ch3_cfg::W](ch3_cfg::W) writer structure"]
impl crate::Writable for CH3_CFG {}
#[doc = "Channel Configuration Register"]
pub mod ch3_cfg;
#[doc = "Channel Loop Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_loop](ch3_loop) module"]
pub type CH3_LOOP = crate::Reg<u32, _CH3_LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_LOOP;
#[doc = "`read()` method returns [ch3_loop::R](ch3_loop::R) reader structure"]
impl crate::Readable for CH3_LOOP {}
#[doc = "`write(|w| ..)` method takes [ch3_loop::W](ch3_loop::W) writer structure"]
impl crate::Writable for CH3_LOOP {}
#[doc = "Channel Loop Counter Register"]
pub mod ch3_loop;
#[doc = "Channel Descriptor Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_ctrl](ch3_ctrl) module"]
pub type CH3_CTRL = crate::Reg<u32, _CH3_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_CTRL;
#[doc = "`read()` method returns [ch3_ctrl::R](ch3_ctrl::R) reader structure"]
impl crate::Readable for CH3_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch3_ctrl::W](ch3_ctrl::W) writer structure"]
impl crate::Writable for CH3_CTRL {}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch3_ctrl;
#[doc = "Channel Descriptor Source Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_src](ch3_src) module"]
pub type CH3_SRC = crate::Reg<u32, _CH3_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_SRC;
#[doc = "`read()` method returns [ch3_src::R](ch3_src::R) reader structure"]
impl crate::Readable for CH3_SRC {}
#[doc = "`write(|w| ..)` method takes [ch3_src::W](ch3_src::W) writer structure"]
impl crate::Writable for CH3_SRC {}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch3_src;
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_dst](ch3_dst) module"]
pub type CH3_DST = crate::Reg<u32, _CH3_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_DST;
#[doc = "`read()` method returns [ch3_dst::R](ch3_dst::R) reader structure"]
impl crate::Readable for CH3_DST {}
#[doc = "`write(|w| ..)` method takes [ch3_dst::W](ch3_dst::W) writer structure"]
impl crate::Writable for CH3_DST {}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch3_dst;
#[doc = "Channel Descriptor Link Structure Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch3_link](ch3_link) module"]
pub type CH3_LINK = crate::Reg<u32, _CH3_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH3_LINK;
#[doc = "`read()` method returns [ch3_link::R](ch3_link::R) reader structure"]
impl crate::Readable for CH3_LINK {}
#[doc = "`write(|w| ..)` method takes [ch3_link::W](ch3_link::W) writer structure"]
impl crate::Writable for CH3_LINK {}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch3_link;
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_reqsel](ch4_reqsel) module"]
pub type CH4_REQSEL = crate::Reg<u32, _CH4_REQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_REQSEL;
#[doc = "`read()` method returns [ch4_reqsel::R](ch4_reqsel::R) reader structure"]
impl crate::Readable for CH4_REQSEL {}
#[doc = "`write(|w| ..)` method takes [ch4_reqsel::W](ch4_reqsel::W) writer structure"]
impl crate::Writable for CH4_REQSEL {}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch4_reqsel;
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_cfg](ch4_cfg) module"]
pub type CH4_CFG = crate::Reg<u32, _CH4_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_CFG;
#[doc = "`read()` method returns [ch4_cfg::R](ch4_cfg::R) reader structure"]
impl crate::Readable for CH4_CFG {}
#[doc = "`write(|w| ..)` method takes [ch4_cfg::W](ch4_cfg::W) writer structure"]
impl crate::Writable for CH4_CFG {}
#[doc = "Channel Configuration Register"]
pub mod ch4_cfg;
#[doc = "Channel Loop Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_loop](ch4_loop) module"]
pub type CH4_LOOP = crate::Reg<u32, _CH4_LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_LOOP;
#[doc = "`read()` method returns [ch4_loop::R](ch4_loop::R) reader structure"]
impl crate::Readable for CH4_LOOP {}
#[doc = "`write(|w| ..)` method takes [ch4_loop::W](ch4_loop::W) writer structure"]
impl crate::Writable for CH4_LOOP {}
#[doc = "Channel Loop Counter Register"]
pub mod ch4_loop;
#[doc = "Channel Descriptor Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_ctrl](ch4_ctrl) module"]
pub type CH4_CTRL = crate::Reg<u32, _CH4_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_CTRL;
#[doc = "`read()` method returns [ch4_ctrl::R](ch4_ctrl::R) reader structure"]
impl crate::Readable for CH4_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch4_ctrl::W](ch4_ctrl::W) writer structure"]
impl crate::Writable for CH4_CTRL {}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch4_ctrl;
#[doc = "Channel Descriptor Source Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_src](ch4_src) module"]
pub type CH4_SRC = crate::Reg<u32, _CH4_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_SRC;
#[doc = "`read()` method returns [ch4_src::R](ch4_src::R) reader structure"]
impl crate::Readable for CH4_SRC {}
#[doc = "`write(|w| ..)` method takes [ch4_src::W](ch4_src::W) writer structure"]
impl crate::Writable for CH4_SRC {}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch4_src;
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_dst](ch4_dst) module"]
pub type CH4_DST = crate::Reg<u32, _CH4_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_DST;
#[doc = "`read()` method returns [ch4_dst::R](ch4_dst::R) reader structure"]
impl crate::Readable for CH4_DST {}
#[doc = "`write(|w| ..)` method takes [ch4_dst::W](ch4_dst::W) writer structure"]
impl crate::Writable for CH4_DST {}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch4_dst;
#[doc = "Channel Descriptor Link Structure Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch4_link](ch4_link) module"]
pub type CH4_LINK = crate::Reg<u32, _CH4_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH4_LINK;
#[doc = "`read()` method returns [ch4_link::R](ch4_link::R) reader structure"]
impl crate::Readable for CH4_LINK {}
#[doc = "`write(|w| ..)` method takes [ch4_link::W](ch4_link::W) writer structure"]
impl crate::Writable for CH4_LINK {}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch4_link;
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_reqsel](ch5_reqsel) module"]
pub type CH5_REQSEL = crate::Reg<u32, _CH5_REQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_REQSEL;
#[doc = "`read()` method returns [ch5_reqsel::R](ch5_reqsel::R) reader structure"]
impl crate::Readable for CH5_REQSEL {}
#[doc = "`write(|w| ..)` method takes [ch5_reqsel::W](ch5_reqsel::W) writer structure"]
impl crate::Writable for CH5_REQSEL {}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch5_reqsel;
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_cfg](ch5_cfg) module"]
pub type CH5_CFG = crate::Reg<u32, _CH5_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_CFG;
#[doc = "`read()` method returns [ch5_cfg::R](ch5_cfg::R) reader structure"]
impl crate::Readable for CH5_CFG {}
#[doc = "`write(|w| ..)` method takes [ch5_cfg::W](ch5_cfg::W) writer structure"]
impl crate::Writable for CH5_CFG {}
#[doc = "Channel Configuration Register"]
pub mod ch5_cfg;
#[doc = "Channel Loop Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_loop](ch5_loop) module"]
pub type CH5_LOOP = crate::Reg<u32, _CH5_LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_LOOP;
#[doc = "`read()` method returns [ch5_loop::R](ch5_loop::R) reader structure"]
impl crate::Readable for CH5_LOOP {}
#[doc = "`write(|w| ..)` method takes [ch5_loop::W](ch5_loop::W) writer structure"]
impl crate::Writable for CH5_LOOP {}
#[doc = "Channel Loop Counter Register"]
pub mod ch5_loop;
#[doc = "Channel Descriptor Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_ctrl](ch5_ctrl) module"]
pub type CH5_CTRL = crate::Reg<u32, _CH5_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_CTRL;
#[doc = "`read()` method returns [ch5_ctrl::R](ch5_ctrl::R) reader structure"]
impl crate::Readable for CH5_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch5_ctrl::W](ch5_ctrl::W) writer structure"]
impl crate::Writable for CH5_CTRL {}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch5_ctrl;
#[doc = "Channel Descriptor Source Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_src](ch5_src) module"]
pub type CH5_SRC = crate::Reg<u32, _CH5_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_SRC;
#[doc = "`read()` method returns [ch5_src::R](ch5_src::R) reader structure"]
impl crate::Readable for CH5_SRC {}
#[doc = "`write(|w| ..)` method takes [ch5_src::W](ch5_src::W) writer structure"]
impl crate::Writable for CH5_SRC {}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch5_src;
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_dst](ch5_dst) module"]
pub type CH5_DST = crate::Reg<u32, _CH5_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_DST;
#[doc = "`read()` method returns [ch5_dst::R](ch5_dst::R) reader structure"]
impl crate::Readable for CH5_DST {}
#[doc = "`write(|w| ..)` method takes [ch5_dst::W](ch5_dst::W) writer structure"]
impl crate::Writable for CH5_DST {}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch5_dst;
#[doc = "Channel Descriptor Link Structure Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch5_link](ch5_link) module"]
pub type CH5_LINK = crate::Reg<u32, _CH5_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH5_LINK;
#[doc = "`read()` method returns [ch5_link::R](ch5_link::R) reader structure"]
impl crate::Readable for CH5_LINK {}
#[doc = "`write(|w| ..)` method takes [ch5_link::W](ch5_link::W) writer structure"]
impl crate::Writable for CH5_LINK {}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch5_link;
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_reqsel](ch6_reqsel) module"]
pub type CH6_REQSEL = crate::Reg<u32, _CH6_REQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_REQSEL;
#[doc = "`read()` method returns [ch6_reqsel::R](ch6_reqsel::R) reader structure"]
impl crate::Readable for CH6_REQSEL {}
#[doc = "`write(|w| ..)` method takes [ch6_reqsel::W](ch6_reqsel::W) writer structure"]
impl crate::Writable for CH6_REQSEL {}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch6_reqsel;
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_cfg](ch6_cfg) module"]
pub type CH6_CFG = crate::Reg<u32, _CH6_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_CFG;
#[doc = "`read()` method returns [ch6_cfg::R](ch6_cfg::R) reader structure"]
impl crate::Readable for CH6_CFG {}
#[doc = "`write(|w| ..)` method takes [ch6_cfg::W](ch6_cfg::W) writer structure"]
impl crate::Writable for CH6_CFG {}
#[doc = "Channel Configuration Register"]
pub mod ch6_cfg;
#[doc = "Channel Loop Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_loop](ch6_loop) module"]
pub type CH6_LOOP = crate::Reg<u32, _CH6_LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_LOOP;
#[doc = "`read()` method returns [ch6_loop::R](ch6_loop::R) reader structure"]
impl crate::Readable for CH6_LOOP {}
#[doc = "`write(|w| ..)` method takes [ch6_loop::W](ch6_loop::W) writer structure"]
impl crate::Writable for CH6_LOOP {}
#[doc = "Channel Loop Counter Register"]
pub mod ch6_loop;
#[doc = "Channel Descriptor Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_ctrl](ch6_ctrl) module"]
pub type CH6_CTRL = crate::Reg<u32, _CH6_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_CTRL;
#[doc = "`read()` method returns [ch6_ctrl::R](ch6_ctrl::R) reader structure"]
impl crate::Readable for CH6_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch6_ctrl::W](ch6_ctrl::W) writer structure"]
impl crate::Writable for CH6_CTRL {}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch6_ctrl;
#[doc = "Channel Descriptor Source Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_src](ch6_src) module"]
pub type CH6_SRC = crate::Reg<u32, _CH6_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_SRC;
#[doc = "`read()` method returns [ch6_src::R](ch6_src::R) reader structure"]
impl crate::Readable for CH6_SRC {}
#[doc = "`write(|w| ..)` method takes [ch6_src::W](ch6_src::W) writer structure"]
impl crate::Writable for CH6_SRC {}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch6_src;
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_dst](ch6_dst) module"]
pub type CH6_DST = crate::Reg<u32, _CH6_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_DST;
#[doc = "`read()` method returns [ch6_dst::R](ch6_dst::R) reader structure"]
impl crate::Readable for CH6_DST {}
#[doc = "`write(|w| ..)` method takes [ch6_dst::W](ch6_dst::W) writer structure"]
impl crate::Writable for CH6_DST {}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch6_dst;
#[doc = "Channel Descriptor Link Structure Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch6_link](ch6_link) module"]
pub type CH6_LINK = crate::Reg<u32, _CH6_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH6_LINK;
#[doc = "`read()` method returns [ch6_link::R](ch6_link::R) reader structure"]
impl crate::Readable for CH6_LINK {}
#[doc = "`write(|w| ..)` method takes [ch6_link::W](ch6_link::W) writer structure"]
impl crate::Writable for CH6_LINK {}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch6_link;
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_reqsel](ch7_reqsel) module"]
pub type CH7_REQSEL = crate::Reg<u32, _CH7_REQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_REQSEL;
#[doc = "`read()` method returns [ch7_reqsel::R](ch7_reqsel::R) reader structure"]
impl crate::Readable for CH7_REQSEL {}
#[doc = "`write(|w| ..)` method takes [ch7_reqsel::W](ch7_reqsel::W) writer structure"]
impl crate::Writable for CH7_REQSEL {}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch7_reqsel;
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_cfg](ch7_cfg) module"]
pub type CH7_CFG = crate::Reg<u32, _CH7_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_CFG;
#[doc = "`read()` method returns [ch7_cfg::R](ch7_cfg::R) reader structure"]
impl crate::Readable for CH7_CFG {}
#[doc = "`write(|w| ..)` method takes [ch7_cfg::W](ch7_cfg::W) writer structure"]
impl crate::Writable for CH7_CFG {}
#[doc = "Channel Configuration Register"]
pub mod ch7_cfg;
#[doc = "Channel Loop Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_loop](ch7_loop) module"]
pub type CH7_LOOP = crate::Reg<u32, _CH7_LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_LOOP;
#[doc = "`read()` method returns [ch7_loop::R](ch7_loop::R) reader structure"]
impl crate::Readable for CH7_LOOP {}
#[doc = "`write(|w| ..)` method takes [ch7_loop::W](ch7_loop::W) writer structure"]
impl crate::Writable for CH7_LOOP {}
#[doc = "Channel Loop Counter Register"]
pub mod ch7_loop;
#[doc = "Channel Descriptor Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_ctrl](ch7_ctrl) module"]
pub type CH7_CTRL = crate::Reg<u32, _CH7_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_CTRL;
#[doc = "`read()` method returns [ch7_ctrl::R](ch7_ctrl::R) reader structure"]
impl crate::Readable for CH7_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch7_ctrl::W](ch7_ctrl::W) writer structure"]
impl crate::Writable for CH7_CTRL {}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch7_ctrl;
#[doc = "Channel Descriptor Source Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_src](ch7_src) module"]
pub type CH7_SRC = crate::Reg<u32, _CH7_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_SRC;
#[doc = "`read()` method returns [ch7_src::R](ch7_src::R) reader structure"]
impl crate::Readable for CH7_SRC {}
#[doc = "`write(|w| ..)` method takes [ch7_src::W](ch7_src::W) writer structure"]
impl crate::Writable for CH7_SRC {}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch7_src;
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_dst](ch7_dst) module"]
pub type CH7_DST = crate::Reg<u32, _CH7_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_DST;
#[doc = "`read()` method returns [ch7_dst::R](ch7_dst::R) reader structure"]
impl crate::Readable for CH7_DST {}
#[doc = "`write(|w| ..)` method takes [ch7_dst::W](ch7_dst::W) writer structure"]
impl crate::Writable for CH7_DST {}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch7_dst;
#[doc = "Channel Descriptor Link Structure Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch7_link](ch7_link) module"]
pub type CH7_LINK = crate::Reg<u32, _CH7_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH7_LINK;
#[doc = "`read()` method returns [ch7_link::R](ch7_link::R) reader structure"]
impl crate::Readable for CH7_LINK {}
#[doc = "`write(|w| ..)` method takes [ch7_link::W](ch7_link::W) writer structure"]
impl crate::Writable for CH7_LINK {}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch7_link;
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch8_reqsel](ch8_reqsel) module"]
pub type CH8_REQSEL = crate::Reg<u32, _CH8_REQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH8_REQSEL;
#[doc = "`read()` method returns [ch8_reqsel::R](ch8_reqsel::R) reader structure"]
impl crate::Readable for CH8_REQSEL {}
#[doc = "`write(|w| ..)` method takes [ch8_reqsel::W](ch8_reqsel::W) writer structure"]
impl crate::Writable for CH8_REQSEL {}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch8_reqsel;
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch8_cfg](ch8_cfg) module"]
pub type CH8_CFG = crate::Reg<u32, _CH8_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH8_CFG;
#[doc = "`read()` method returns [ch8_cfg::R](ch8_cfg::R) reader structure"]
impl crate::Readable for CH8_CFG {}
#[doc = "`write(|w| ..)` method takes [ch8_cfg::W](ch8_cfg::W) writer structure"]
impl crate::Writable for CH8_CFG {}
#[doc = "Channel Configuration Register"]
pub mod ch8_cfg;
#[doc = "Channel Loop Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch8_loop](ch8_loop) module"]
pub type CH8_LOOP = crate::Reg<u32, _CH8_LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH8_LOOP;
#[doc = "`read()` method returns [ch8_loop::R](ch8_loop::R) reader structure"]
impl crate::Readable for CH8_LOOP {}
#[doc = "`write(|w| ..)` method takes [ch8_loop::W](ch8_loop::W) writer structure"]
impl crate::Writable for CH8_LOOP {}
#[doc = "Channel Loop Counter Register"]
pub mod ch8_loop;
#[doc = "Channel Descriptor Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch8_ctrl](ch8_ctrl) module"]
pub type CH8_CTRL = crate::Reg<u32, _CH8_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH8_CTRL;
#[doc = "`read()` method returns [ch8_ctrl::R](ch8_ctrl::R) reader structure"]
impl crate::Readable for CH8_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch8_ctrl::W](ch8_ctrl::W) writer structure"]
impl crate::Writable for CH8_CTRL {}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch8_ctrl;
#[doc = "Channel Descriptor Source Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch8_src](ch8_src) module"]
pub type CH8_SRC = crate::Reg<u32, _CH8_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH8_SRC;
#[doc = "`read()` method returns [ch8_src::R](ch8_src::R) reader structure"]
impl crate::Readable for CH8_SRC {}
#[doc = "`write(|w| ..)` method takes [ch8_src::W](ch8_src::W) writer structure"]
impl crate::Writable for CH8_SRC {}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch8_src;
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch8_dst](ch8_dst) module"]
pub type CH8_DST = crate::Reg<u32, _CH8_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH8_DST;
#[doc = "`read()` method returns [ch8_dst::R](ch8_dst::R) reader structure"]
impl crate::Readable for CH8_DST {}
#[doc = "`write(|w| ..)` method takes [ch8_dst::W](ch8_dst::W) writer structure"]
impl crate::Writable for CH8_DST {}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch8_dst;
#[doc = "Channel Descriptor Link Structure Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch8_link](ch8_link) module"]
pub type CH8_LINK = crate::Reg<u32, _CH8_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH8_LINK;
#[doc = "`read()` method returns [ch8_link::R](ch8_link::R) reader structure"]
impl crate::Readable for CH8_LINK {}
#[doc = "`write(|w| ..)` method takes [ch8_link::W](ch8_link::W) writer structure"]
impl crate::Writable for CH8_LINK {}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch8_link;
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch9_reqsel](ch9_reqsel) module"]
pub type CH9_REQSEL = crate::Reg<u32, _CH9_REQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH9_REQSEL;
#[doc = "`read()` method returns [ch9_reqsel::R](ch9_reqsel::R) reader structure"]
impl crate::Readable for CH9_REQSEL {}
#[doc = "`write(|w| ..)` method takes [ch9_reqsel::W](ch9_reqsel::W) writer structure"]
impl crate::Writable for CH9_REQSEL {}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch9_reqsel;
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch9_cfg](ch9_cfg) module"]
pub type CH9_CFG = crate::Reg<u32, _CH9_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH9_CFG;
#[doc = "`read()` method returns [ch9_cfg::R](ch9_cfg::R) reader structure"]
impl crate::Readable for CH9_CFG {}
#[doc = "`write(|w| ..)` method takes [ch9_cfg::W](ch9_cfg::W) writer structure"]
impl crate::Writable for CH9_CFG {}
#[doc = "Channel Configuration Register"]
pub mod ch9_cfg;
#[doc = "Channel Loop Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch9_loop](ch9_loop) module"]
pub type CH9_LOOP = crate::Reg<u32, _CH9_LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH9_LOOP;
#[doc = "`read()` method returns [ch9_loop::R](ch9_loop::R) reader structure"]
impl crate::Readable for CH9_LOOP {}
#[doc = "`write(|w| ..)` method takes [ch9_loop::W](ch9_loop::W) writer structure"]
impl crate::Writable for CH9_LOOP {}
#[doc = "Channel Loop Counter Register"]
pub mod ch9_loop;
#[doc = "Channel Descriptor Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch9_ctrl](ch9_ctrl) module"]
pub type CH9_CTRL = crate::Reg<u32, _CH9_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH9_CTRL;
#[doc = "`read()` method returns [ch9_ctrl::R](ch9_ctrl::R) reader structure"]
impl crate::Readable for CH9_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch9_ctrl::W](ch9_ctrl::W) writer structure"]
impl crate::Writable for CH9_CTRL {}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch9_ctrl;
#[doc = "Channel Descriptor Source Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch9_src](ch9_src) module"]
pub type CH9_SRC = crate::Reg<u32, _CH9_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH9_SRC;
#[doc = "`read()` method returns [ch9_src::R](ch9_src::R) reader structure"]
impl crate::Readable for CH9_SRC {}
#[doc = "`write(|w| ..)` method takes [ch9_src::W](ch9_src::W) writer structure"]
impl crate::Writable for CH9_SRC {}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch9_src;
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch9_dst](ch9_dst) module"]
pub type CH9_DST = crate::Reg<u32, _CH9_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH9_DST;
#[doc = "`read()` method returns [ch9_dst::R](ch9_dst::R) reader structure"]
impl crate::Readable for CH9_DST {}
#[doc = "`write(|w| ..)` method takes [ch9_dst::W](ch9_dst::W) writer structure"]
impl crate::Writable for CH9_DST {}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch9_dst;
#[doc = "Channel Descriptor Link Structure Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch9_link](ch9_link) module"]
pub type CH9_LINK = crate::Reg<u32, _CH9_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH9_LINK;
#[doc = "`read()` method returns [ch9_link::R](ch9_link::R) reader structure"]
impl crate::Readable for CH9_LINK {}
#[doc = "`write(|w| ..)` method takes [ch9_link::W](ch9_link::W) writer structure"]
impl crate::Writable for CH9_LINK {}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch9_link;
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch10_reqsel](ch10_reqsel) module"]
pub type CH10_REQSEL = crate::Reg<u32, _CH10_REQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH10_REQSEL;
#[doc = "`read()` method returns [ch10_reqsel::R](ch10_reqsel::R) reader structure"]
impl crate::Readable for CH10_REQSEL {}
#[doc = "`write(|w| ..)` method takes [ch10_reqsel::W](ch10_reqsel::W) writer structure"]
impl crate::Writable for CH10_REQSEL {}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch10_reqsel;
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch10_cfg](ch10_cfg) module"]
pub type CH10_CFG = crate::Reg<u32, _CH10_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH10_CFG;
#[doc = "`read()` method returns [ch10_cfg::R](ch10_cfg::R) reader structure"]
impl crate::Readable for CH10_CFG {}
#[doc = "`write(|w| ..)` method takes [ch10_cfg::W](ch10_cfg::W) writer structure"]
impl crate::Writable for CH10_CFG {}
#[doc = "Channel Configuration Register"]
pub mod ch10_cfg;
#[doc = "Channel Loop Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch10_loop](ch10_loop) module"]
pub type CH10_LOOP = crate::Reg<u32, _CH10_LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH10_LOOP;
#[doc = "`read()` method returns [ch10_loop::R](ch10_loop::R) reader structure"]
impl crate::Readable for CH10_LOOP {}
#[doc = "`write(|w| ..)` method takes [ch10_loop::W](ch10_loop::W) writer structure"]
impl crate::Writable for CH10_LOOP {}
#[doc = "Channel Loop Counter Register"]
pub mod ch10_loop;
#[doc = "Channel Descriptor Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch10_ctrl](ch10_ctrl) module"]
pub type CH10_CTRL = crate::Reg<u32, _CH10_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH10_CTRL;
#[doc = "`read()` method returns [ch10_ctrl::R](ch10_ctrl::R) reader structure"]
impl crate::Readable for CH10_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch10_ctrl::W](ch10_ctrl::W) writer structure"]
impl crate::Writable for CH10_CTRL {}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch10_ctrl;
#[doc = "Channel Descriptor Source Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch10_src](ch10_src) module"]
pub type CH10_SRC = crate::Reg<u32, _CH10_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH10_SRC;
#[doc = "`read()` method returns [ch10_src::R](ch10_src::R) reader structure"]
impl crate::Readable for CH10_SRC {}
#[doc = "`write(|w| ..)` method takes [ch10_src::W](ch10_src::W) writer structure"]
impl crate::Writable for CH10_SRC {}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch10_src;
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch10_dst](ch10_dst) module"]
pub type CH10_DST = crate::Reg<u32, _CH10_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH10_DST;
#[doc = "`read()` method returns [ch10_dst::R](ch10_dst::R) reader structure"]
impl crate::Readable for CH10_DST {}
#[doc = "`write(|w| ..)` method takes [ch10_dst::W](ch10_dst::W) writer structure"]
impl crate::Writable for CH10_DST {}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch10_dst;
#[doc = "Channel Descriptor Link Structure Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch10_link](ch10_link) module"]
pub type CH10_LINK = crate::Reg<u32, _CH10_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH10_LINK;
#[doc = "`read()` method returns [ch10_link::R](ch10_link::R) reader structure"]
impl crate::Readable for CH10_LINK {}
#[doc = "`write(|w| ..)` method takes [ch10_link::W](ch10_link::W) writer structure"]
impl crate::Writable for CH10_LINK {}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch10_link;
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch11_reqsel](ch11_reqsel) module"]
pub type CH11_REQSEL = crate::Reg<u32, _CH11_REQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH11_REQSEL;
#[doc = "`read()` method returns [ch11_reqsel::R](ch11_reqsel::R) reader structure"]
impl crate::Readable for CH11_REQSEL {}
#[doc = "`write(|w| ..)` method takes [ch11_reqsel::W](ch11_reqsel::W) writer structure"]
impl crate::Writable for CH11_REQSEL {}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch11_reqsel;
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch11_cfg](ch11_cfg) module"]
pub type CH11_CFG = crate::Reg<u32, _CH11_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH11_CFG;
#[doc = "`read()` method returns [ch11_cfg::R](ch11_cfg::R) reader structure"]
impl crate::Readable for CH11_CFG {}
#[doc = "`write(|w| ..)` method takes [ch11_cfg::W](ch11_cfg::W) writer structure"]
impl crate::Writable for CH11_CFG {}
#[doc = "Channel Configuration Register"]
pub mod ch11_cfg;
#[doc = "Channel Loop Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch11_loop](ch11_loop) module"]
pub type CH11_LOOP = crate::Reg<u32, _CH11_LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH11_LOOP;
#[doc = "`read()` method returns [ch11_loop::R](ch11_loop::R) reader structure"]
impl crate::Readable for CH11_LOOP {}
#[doc = "`write(|w| ..)` method takes [ch11_loop::W](ch11_loop::W) writer structure"]
impl crate::Writable for CH11_LOOP {}
#[doc = "Channel Loop Counter Register"]
pub mod ch11_loop;
#[doc = "Channel Descriptor Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch11_ctrl](ch11_ctrl) module"]
pub type CH11_CTRL = crate::Reg<u32, _CH11_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH11_CTRL;
#[doc = "`read()` method returns [ch11_ctrl::R](ch11_ctrl::R) reader structure"]
impl crate::Readable for CH11_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch11_ctrl::W](ch11_ctrl::W) writer structure"]
impl crate::Writable for CH11_CTRL {}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch11_ctrl;
#[doc = "Channel Descriptor Source Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch11_src](ch11_src) module"]
pub type CH11_SRC = crate::Reg<u32, _CH11_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH11_SRC;
#[doc = "`read()` method returns [ch11_src::R](ch11_src::R) reader structure"]
impl crate::Readable for CH11_SRC {}
#[doc = "`write(|w| ..)` method takes [ch11_src::W](ch11_src::W) writer structure"]
impl crate::Writable for CH11_SRC {}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch11_src;
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch11_dst](ch11_dst) module"]
pub type CH11_DST = crate::Reg<u32, _CH11_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH11_DST;
#[doc = "`read()` method returns [ch11_dst::R](ch11_dst::R) reader structure"]
impl crate::Readable for CH11_DST {}
#[doc = "`write(|w| ..)` method takes [ch11_dst::W](ch11_dst::W) writer structure"]
impl crate::Writable for CH11_DST {}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch11_dst;
#[doc = "Channel Descriptor Link Structure Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch11_link](ch11_link) module"]
pub type CH11_LINK = crate::Reg<u32, _CH11_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH11_LINK;
#[doc = "`read()` method returns [ch11_link::R](ch11_link::R) reader structure"]
impl crate::Readable for CH11_LINK {}
#[doc = "`write(|w| ..)` method takes [ch11_link::W](ch11_link::W) writer structure"]
impl crate::Writable for CH11_LINK {}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch11_link;
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch12_reqsel](ch12_reqsel) module"]
pub type CH12_REQSEL = crate::Reg<u32, _CH12_REQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH12_REQSEL;
#[doc = "`read()` method returns [ch12_reqsel::R](ch12_reqsel::R) reader structure"]
impl crate::Readable for CH12_REQSEL {}
#[doc = "`write(|w| ..)` method takes [ch12_reqsel::W](ch12_reqsel::W) writer structure"]
impl crate::Writable for CH12_REQSEL {}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch12_reqsel;
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch12_cfg](ch12_cfg) module"]
pub type CH12_CFG = crate::Reg<u32, _CH12_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH12_CFG;
#[doc = "`read()` method returns [ch12_cfg::R](ch12_cfg::R) reader structure"]
impl crate::Readable for CH12_CFG {}
#[doc = "`write(|w| ..)` method takes [ch12_cfg::W](ch12_cfg::W) writer structure"]
impl crate::Writable for CH12_CFG {}
#[doc = "Channel Configuration Register"]
pub mod ch12_cfg;
#[doc = "Channel Loop Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch12_loop](ch12_loop) module"]
pub type CH12_LOOP = crate::Reg<u32, _CH12_LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH12_LOOP;
#[doc = "`read()` method returns [ch12_loop::R](ch12_loop::R) reader structure"]
impl crate::Readable for CH12_LOOP {}
#[doc = "`write(|w| ..)` method takes [ch12_loop::W](ch12_loop::W) writer structure"]
impl crate::Writable for CH12_LOOP {}
#[doc = "Channel Loop Counter Register"]
pub mod ch12_loop;
#[doc = "Channel Descriptor Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch12_ctrl](ch12_ctrl) module"]
pub type CH12_CTRL = crate::Reg<u32, _CH12_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH12_CTRL;
#[doc = "`read()` method returns [ch12_ctrl::R](ch12_ctrl::R) reader structure"]
impl crate::Readable for CH12_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch12_ctrl::W](ch12_ctrl::W) writer structure"]
impl crate::Writable for CH12_CTRL {}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch12_ctrl;
#[doc = "Channel Descriptor Source Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch12_src](ch12_src) module"]
pub type CH12_SRC = crate::Reg<u32, _CH12_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH12_SRC;
#[doc = "`read()` method returns [ch12_src::R](ch12_src::R) reader structure"]
impl crate::Readable for CH12_SRC {}
#[doc = "`write(|w| ..)` method takes [ch12_src::W](ch12_src::W) writer structure"]
impl crate::Writable for CH12_SRC {}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch12_src;
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch12_dst](ch12_dst) module"]
pub type CH12_DST = crate::Reg<u32, _CH12_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH12_DST;
#[doc = "`read()` method returns [ch12_dst::R](ch12_dst::R) reader structure"]
impl crate::Readable for CH12_DST {}
#[doc = "`write(|w| ..)` method takes [ch12_dst::W](ch12_dst::W) writer structure"]
impl crate::Writable for CH12_DST {}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch12_dst;
#[doc = "Channel Descriptor Link Structure Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch12_link](ch12_link) module"]
pub type CH12_LINK = crate::Reg<u32, _CH12_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH12_LINK;
#[doc = "`read()` method returns [ch12_link::R](ch12_link::R) reader structure"]
impl crate::Readable for CH12_LINK {}
#[doc = "`write(|w| ..)` method takes [ch12_link::W](ch12_link::W) writer structure"]
impl crate::Writable for CH12_LINK {}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch12_link;
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch13_reqsel](ch13_reqsel) module"]
pub type CH13_REQSEL = crate::Reg<u32, _CH13_REQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH13_REQSEL;
#[doc = "`read()` method returns [ch13_reqsel::R](ch13_reqsel::R) reader structure"]
impl crate::Readable for CH13_REQSEL {}
#[doc = "`write(|w| ..)` method takes [ch13_reqsel::W](ch13_reqsel::W) writer structure"]
impl crate::Writable for CH13_REQSEL {}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch13_reqsel;
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch13_cfg](ch13_cfg) module"]
pub type CH13_CFG = crate::Reg<u32, _CH13_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH13_CFG;
#[doc = "`read()` method returns [ch13_cfg::R](ch13_cfg::R) reader structure"]
impl crate::Readable for CH13_CFG {}
#[doc = "`write(|w| ..)` method takes [ch13_cfg::W](ch13_cfg::W) writer structure"]
impl crate::Writable for CH13_CFG {}
#[doc = "Channel Configuration Register"]
pub mod ch13_cfg;
#[doc = "Channel Loop Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch13_loop](ch13_loop) module"]
pub type CH13_LOOP = crate::Reg<u32, _CH13_LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH13_LOOP;
#[doc = "`read()` method returns [ch13_loop::R](ch13_loop::R) reader structure"]
impl crate::Readable for CH13_LOOP {}
#[doc = "`write(|w| ..)` method takes [ch13_loop::W](ch13_loop::W) writer structure"]
impl crate::Writable for CH13_LOOP {}
#[doc = "Channel Loop Counter Register"]
pub mod ch13_loop;
#[doc = "Channel Descriptor Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch13_ctrl](ch13_ctrl) module"]
pub type CH13_CTRL = crate::Reg<u32, _CH13_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH13_CTRL;
#[doc = "`read()` method returns [ch13_ctrl::R](ch13_ctrl::R) reader structure"]
impl crate::Readable for CH13_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch13_ctrl::W](ch13_ctrl::W) writer structure"]
impl crate::Writable for CH13_CTRL {}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch13_ctrl;
#[doc = "Channel Descriptor Source Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch13_src](ch13_src) module"]
pub type CH13_SRC = crate::Reg<u32, _CH13_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH13_SRC;
#[doc = "`read()` method returns [ch13_src::R](ch13_src::R) reader structure"]
impl crate::Readable for CH13_SRC {}
#[doc = "`write(|w| ..)` method takes [ch13_src::W](ch13_src::W) writer structure"]
impl crate::Writable for CH13_SRC {}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch13_src;
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch13_dst](ch13_dst) module"]
pub type CH13_DST = crate::Reg<u32, _CH13_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH13_DST;
#[doc = "`read()` method returns [ch13_dst::R](ch13_dst::R) reader structure"]
impl crate::Readable for CH13_DST {}
#[doc = "`write(|w| ..)` method takes [ch13_dst::W](ch13_dst::W) writer structure"]
impl crate::Writable for CH13_DST {}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch13_dst;
#[doc = "Channel Descriptor Link Structure Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch13_link](ch13_link) module"]
pub type CH13_LINK = crate::Reg<u32, _CH13_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH13_LINK;
#[doc = "`read()` method returns [ch13_link::R](ch13_link::R) reader structure"]
impl crate::Readable for CH13_LINK {}
#[doc = "`write(|w| ..)` method takes [ch13_link::W](ch13_link::W) writer structure"]
impl crate::Writable for CH13_LINK {}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch13_link;
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch14_reqsel](ch14_reqsel) module"]
pub type CH14_REQSEL = crate::Reg<u32, _CH14_REQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH14_REQSEL;
#[doc = "`read()` method returns [ch14_reqsel::R](ch14_reqsel::R) reader structure"]
impl crate::Readable for CH14_REQSEL {}
#[doc = "`write(|w| ..)` method takes [ch14_reqsel::W](ch14_reqsel::W) writer structure"]
impl crate::Writable for CH14_REQSEL {}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch14_reqsel;
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch14_cfg](ch14_cfg) module"]
pub type CH14_CFG = crate::Reg<u32, _CH14_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH14_CFG;
#[doc = "`read()` method returns [ch14_cfg::R](ch14_cfg::R) reader structure"]
impl crate::Readable for CH14_CFG {}
#[doc = "`write(|w| ..)` method takes [ch14_cfg::W](ch14_cfg::W) writer structure"]
impl crate::Writable for CH14_CFG {}
#[doc = "Channel Configuration Register"]
pub mod ch14_cfg;
#[doc = "Channel Loop Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch14_loop](ch14_loop) module"]
pub type CH14_LOOP = crate::Reg<u32, _CH14_LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH14_LOOP;
#[doc = "`read()` method returns [ch14_loop::R](ch14_loop::R) reader structure"]
impl crate::Readable for CH14_LOOP {}
#[doc = "`write(|w| ..)` method takes [ch14_loop::W](ch14_loop::W) writer structure"]
impl crate::Writable for CH14_LOOP {}
#[doc = "Channel Loop Counter Register"]
pub mod ch14_loop;
#[doc = "Channel Descriptor Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch14_ctrl](ch14_ctrl) module"]
pub type CH14_CTRL = crate::Reg<u32, _CH14_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH14_CTRL;
#[doc = "`read()` method returns [ch14_ctrl::R](ch14_ctrl::R) reader structure"]
impl crate::Readable for CH14_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch14_ctrl::W](ch14_ctrl::W) writer structure"]
impl crate::Writable for CH14_CTRL {}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch14_ctrl;
#[doc = "Channel Descriptor Source Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch14_src](ch14_src) module"]
pub type CH14_SRC = crate::Reg<u32, _CH14_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH14_SRC;
#[doc = "`read()` method returns [ch14_src::R](ch14_src::R) reader structure"]
impl crate::Readable for CH14_SRC {}
#[doc = "`write(|w| ..)` method takes [ch14_src::W](ch14_src::W) writer structure"]
impl crate::Writable for CH14_SRC {}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch14_src;
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch14_dst](ch14_dst) module"]
pub type CH14_DST = crate::Reg<u32, _CH14_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH14_DST;
#[doc = "`read()` method returns [ch14_dst::R](ch14_dst::R) reader structure"]
impl crate::Readable for CH14_DST {}
#[doc = "`write(|w| ..)` method takes [ch14_dst::W](ch14_dst::W) writer structure"]
impl crate::Writable for CH14_DST {}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch14_dst;
#[doc = "Channel Descriptor Link Structure Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch14_link](ch14_link) module"]
pub type CH14_LINK = crate::Reg<u32, _CH14_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH14_LINK;
#[doc = "`read()` method returns [ch14_link::R](ch14_link::R) reader structure"]
impl crate::Readable for CH14_LINK {}
#[doc = "`write(|w| ..)` method takes [ch14_link::W](ch14_link::W) writer structure"]
impl crate::Writable for CH14_LINK {}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch14_link;
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch15_reqsel](ch15_reqsel) module"]
pub type CH15_REQSEL = crate::Reg<u32, _CH15_REQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH15_REQSEL;
#[doc = "`read()` method returns [ch15_reqsel::R](ch15_reqsel::R) reader structure"]
impl crate::Readable for CH15_REQSEL {}
#[doc = "`write(|w| ..)` method takes [ch15_reqsel::W](ch15_reqsel::W) writer structure"]
impl crate::Writable for CH15_REQSEL {}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch15_reqsel;
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch15_cfg](ch15_cfg) module"]
pub type CH15_CFG = crate::Reg<u32, _CH15_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH15_CFG;
#[doc = "`read()` method returns [ch15_cfg::R](ch15_cfg::R) reader structure"]
impl crate::Readable for CH15_CFG {}
#[doc = "`write(|w| ..)` method takes [ch15_cfg::W](ch15_cfg::W) writer structure"]
impl crate::Writable for CH15_CFG {}
#[doc = "Channel Configuration Register"]
pub mod ch15_cfg;
#[doc = "Channel Loop Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch15_loop](ch15_loop) module"]
pub type CH15_LOOP = crate::Reg<u32, _CH15_LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH15_LOOP;
#[doc = "`read()` method returns [ch15_loop::R](ch15_loop::R) reader structure"]
impl crate::Readable for CH15_LOOP {}
#[doc = "`write(|w| ..)` method takes [ch15_loop::W](ch15_loop::W) writer structure"]
impl crate::Writable for CH15_LOOP {}
#[doc = "Channel Loop Counter Register"]
pub mod ch15_loop;
#[doc = "Channel Descriptor Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch15_ctrl](ch15_ctrl) module"]
pub type CH15_CTRL = crate::Reg<u32, _CH15_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH15_CTRL;
#[doc = "`read()` method returns [ch15_ctrl::R](ch15_ctrl::R) reader structure"]
impl crate::Readable for CH15_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch15_ctrl::W](ch15_ctrl::W) writer structure"]
impl crate::Writable for CH15_CTRL {}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch15_ctrl;
#[doc = "Channel Descriptor Source Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch15_src](ch15_src) module"]
pub type CH15_SRC = crate::Reg<u32, _CH15_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH15_SRC;
#[doc = "`read()` method returns [ch15_src::R](ch15_src::R) reader structure"]
impl crate::Readable for CH15_SRC {}
#[doc = "`write(|w| ..)` method takes [ch15_src::W](ch15_src::W) writer structure"]
impl crate::Writable for CH15_SRC {}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch15_src;
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch15_dst](ch15_dst) module"]
pub type CH15_DST = crate::Reg<u32, _CH15_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH15_DST;
#[doc = "`read()` method returns [ch15_dst::R](ch15_dst::R) reader structure"]
impl crate::Readable for CH15_DST {}
#[doc = "`write(|w| ..)` method takes [ch15_dst::W](ch15_dst::W) writer structure"]
impl crate::Writable for CH15_DST {}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch15_dst;
#[doc = "Channel Descriptor Link Structure Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch15_link](ch15_link) module"]
pub type CH15_LINK = crate::Reg<u32, _CH15_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH15_LINK;
#[doc = "`read()` method returns [ch15_link::R](ch15_link::R) reader structure"]
impl crate::Readable for CH15_LINK {}
#[doc = "`write(|w| ..)` method takes [ch15_link::W](ch15_link::W) writer structure"]
impl crate::Writable for CH15_LINK {}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch15_link;
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch16_reqsel](ch16_reqsel) module"]
pub type CH16_REQSEL = crate::Reg<u32, _CH16_REQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH16_REQSEL;
#[doc = "`read()` method returns [ch16_reqsel::R](ch16_reqsel::R) reader structure"]
impl crate::Readable for CH16_REQSEL {}
#[doc = "`write(|w| ..)` method takes [ch16_reqsel::W](ch16_reqsel::W) writer structure"]
impl crate::Writable for CH16_REQSEL {}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch16_reqsel;
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch16_cfg](ch16_cfg) module"]
pub type CH16_CFG = crate::Reg<u32, _CH16_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH16_CFG;
#[doc = "`read()` method returns [ch16_cfg::R](ch16_cfg::R) reader structure"]
impl crate::Readable for CH16_CFG {}
#[doc = "`write(|w| ..)` method takes [ch16_cfg::W](ch16_cfg::W) writer structure"]
impl crate::Writable for CH16_CFG {}
#[doc = "Channel Configuration Register"]
pub mod ch16_cfg;
#[doc = "Channel Loop Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch16_loop](ch16_loop) module"]
pub type CH16_LOOP = crate::Reg<u32, _CH16_LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH16_LOOP;
#[doc = "`read()` method returns [ch16_loop::R](ch16_loop::R) reader structure"]
impl crate::Readable for CH16_LOOP {}
#[doc = "`write(|w| ..)` method takes [ch16_loop::W](ch16_loop::W) writer structure"]
impl crate::Writable for CH16_LOOP {}
#[doc = "Channel Loop Counter Register"]
pub mod ch16_loop;
#[doc = "Channel Descriptor Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch16_ctrl](ch16_ctrl) module"]
pub type CH16_CTRL = crate::Reg<u32, _CH16_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH16_CTRL;
#[doc = "`read()` method returns [ch16_ctrl::R](ch16_ctrl::R) reader structure"]
impl crate::Readable for CH16_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch16_ctrl::W](ch16_ctrl::W) writer structure"]
impl crate::Writable for CH16_CTRL {}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch16_ctrl;
#[doc = "Channel Descriptor Source Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch16_src](ch16_src) module"]
pub type CH16_SRC = crate::Reg<u32, _CH16_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH16_SRC;
#[doc = "`read()` method returns [ch16_src::R](ch16_src::R) reader structure"]
impl crate::Readable for CH16_SRC {}
#[doc = "`write(|w| ..)` method takes [ch16_src::W](ch16_src::W) writer structure"]
impl crate::Writable for CH16_SRC {}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch16_src;
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch16_dst](ch16_dst) module"]
pub type CH16_DST = crate::Reg<u32, _CH16_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH16_DST;
#[doc = "`read()` method returns [ch16_dst::R](ch16_dst::R) reader structure"]
impl crate::Readable for CH16_DST {}
#[doc = "`write(|w| ..)` method takes [ch16_dst::W](ch16_dst::W) writer structure"]
impl crate::Writable for CH16_DST {}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch16_dst;
#[doc = "Channel Descriptor Link Structure Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch16_link](ch16_link) module"]
pub type CH16_LINK = crate::Reg<u32, _CH16_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH16_LINK;
#[doc = "`read()` method returns [ch16_link::R](ch16_link::R) reader structure"]
impl crate::Readable for CH16_LINK {}
#[doc = "`write(|w| ..)` method takes [ch16_link::W](ch16_link::W) writer structure"]
impl crate::Writable for CH16_LINK {}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch16_link;
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch17_reqsel](ch17_reqsel) module"]
pub type CH17_REQSEL = crate::Reg<u32, _CH17_REQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH17_REQSEL;
#[doc = "`read()` method returns [ch17_reqsel::R](ch17_reqsel::R) reader structure"]
impl crate::Readable for CH17_REQSEL {}
#[doc = "`write(|w| ..)` method takes [ch17_reqsel::W](ch17_reqsel::W) writer structure"]
impl crate::Writable for CH17_REQSEL {}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch17_reqsel;
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch17_cfg](ch17_cfg) module"]
pub type CH17_CFG = crate::Reg<u32, _CH17_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH17_CFG;
#[doc = "`read()` method returns [ch17_cfg::R](ch17_cfg::R) reader structure"]
impl crate::Readable for CH17_CFG {}
#[doc = "`write(|w| ..)` method takes [ch17_cfg::W](ch17_cfg::W) writer structure"]
impl crate::Writable for CH17_CFG {}
#[doc = "Channel Configuration Register"]
pub mod ch17_cfg;
#[doc = "Channel Loop Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch17_loop](ch17_loop) module"]
pub type CH17_LOOP = crate::Reg<u32, _CH17_LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH17_LOOP;
#[doc = "`read()` method returns [ch17_loop::R](ch17_loop::R) reader structure"]
impl crate::Readable for CH17_LOOP {}
#[doc = "`write(|w| ..)` method takes [ch17_loop::W](ch17_loop::W) writer structure"]
impl crate::Writable for CH17_LOOP {}
#[doc = "Channel Loop Counter Register"]
pub mod ch17_loop;
#[doc = "Channel Descriptor Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch17_ctrl](ch17_ctrl) module"]
pub type CH17_CTRL = crate::Reg<u32, _CH17_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH17_CTRL;
#[doc = "`read()` method returns [ch17_ctrl::R](ch17_ctrl::R) reader structure"]
impl crate::Readable for CH17_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch17_ctrl::W](ch17_ctrl::W) writer structure"]
impl crate::Writable for CH17_CTRL {}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch17_ctrl;
#[doc = "Channel Descriptor Source Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch17_src](ch17_src) module"]
pub type CH17_SRC = crate::Reg<u32, _CH17_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH17_SRC;
#[doc = "`read()` method returns [ch17_src::R](ch17_src::R) reader structure"]
impl crate::Readable for CH17_SRC {}
#[doc = "`write(|w| ..)` method takes [ch17_src::W](ch17_src::W) writer structure"]
impl crate::Writable for CH17_SRC {}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch17_src;
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch17_dst](ch17_dst) module"]
pub type CH17_DST = crate::Reg<u32, _CH17_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH17_DST;
#[doc = "`read()` method returns [ch17_dst::R](ch17_dst::R) reader structure"]
impl crate::Readable for CH17_DST {}
#[doc = "`write(|w| ..)` method takes [ch17_dst::W](ch17_dst::W) writer structure"]
impl crate::Writable for CH17_DST {}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch17_dst;
#[doc = "Channel Descriptor Link Structure Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch17_link](ch17_link) module"]
pub type CH17_LINK = crate::Reg<u32, _CH17_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH17_LINK;
#[doc = "`read()` method returns [ch17_link::R](ch17_link::R) reader structure"]
impl crate::Readable for CH17_LINK {}
#[doc = "`write(|w| ..)` method takes [ch17_link::W](ch17_link::W) writer structure"]
impl crate::Writable for CH17_LINK {}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch17_link;
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch18_reqsel](ch18_reqsel) module"]
pub type CH18_REQSEL = crate::Reg<u32, _CH18_REQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH18_REQSEL;
#[doc = "`read()` method returns [ch18_reqsel::R](ch18_reqsel::R) reader structure"]
impl crate::Readable for CH18_REQSEL {}
#[doc = "`write(|w| ..)` method takes [ch18_reqsel::W](ch18_reqsel::W) writer structure"]
impl crate::Writable for CH18_REQSEL {}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch18_reqsel;
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch18_cfg](ch18_cfg) module"]
pub type CH18_CFG = crate::Reg<u32, _CH18_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH18_CFG;
#[doc = "`read()` method returns [ch18_cfg::R](ch18_cfg::R) reader structure"]
impl crate::Readable for CH18_CFG {}
#[doc = "`write(|w| ..)` method takes [ch18_cfg::W](ch18_cfg::W) writer structure"]
impl crate::Writable for CH18_CFG {}
#[doc = "Channel Configuration Register"]
pub mod ch18_cfg;
#[doc = "Channel Loop Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch18_loop](ch18_loop) module"]
pub type CH18_LOOP = crate::Reg<u32, _CH18_LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH18_LOOP;
#[doc = "`read()` method returns [ch18_loop::R](ch18_loop::R) reader structure"]
impl crate::Readable for CH18_LOOP {}
#[doc = "`write(|w| ..)` method takes [ch18_loop::W](ch18_loop::W) writer structure"]
impl crate::Writable for CH18_LOOP {}
#[doc = "Channel Loop Counter Register"]
pub mod ch18_loop;
#[doc = "Channel Descriptor Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch18_ctrl](ch18_ctrl) module"]
pub type CH18_CTRL = crate::Reg<u32, _CH18_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH18_CTRL;
#[doc = "`read()` method returns [ch18_ctrl::R](ch18_ctrl::R) reader structure"]
impl crate::Readable for CH18_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch18_ctrl::W](ch18_ctrl::W) writer structure"]
impl crate::Writable for CH18_CTRL {}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch18_ctrl;
#[doc = "Channel Descriptor Source Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch18_src](ch18_src) module"]
pub type CH18_SRC = crate::Reg<u32, _CH18_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH18_SRC;
#[doc = "`read()` method returns [ch18_src::R](ch18_src::R) reader structure"]
impl crate::Readable for CH18_SRC {}
#[doc = "`write(|w| ..)` method takes [ch18_src::W](ch18_src::W) writer structure"]
impl crate::Writable for CH18_SRC {}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch18_src;
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch18_dst](ch18_dst) module"]
pub type CH18_DST = crate::Reg<u32, _CH18_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH18_DST;
#[doc = "`read()` method returns [ch18_dst::R](ch18_dst::R) reader structure"]
impl crate::Readable for CH18_DST {}
#[doc = "`write(|w| ..)` method takes [ch18_dst::W](ch18_dst::W) writer structure"]
impl crate::Writable for CH18_DST {}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch18_dst;
#[doc = "Channel Descriptor Link Structure Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch18_link](ch18_link) module"]
pub type CH18_LINK = crate::Reg<u32, _CH18_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH18_LINK;
#[doc = "`read()` method returns [ch18_link::R](ch18_link::R) reader structure"]
impl crate::Readable for CH18_LINK {}
#[doc = "`write(|w| ..)` method takes [ch18_link::W](ch18_link::W) writer structure"]
impl crate::Writable for CH18_LINK {}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch18_link;
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch19_reqsel](ch19_reqsel) module"]
pub type CH19_REQSEL = crate::Reg<u32, _CH19_REQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH19_REQSEL;
#[doc = "`read()` method returns [ch19_reqsel::R](ch19_reqsel::R) reader structure"]
impl crate::Readable for CH19_REQSEL {}
#[doc = "`write(|w| ..)` method takes [ch19_reqsel::W](ch19_reqsel::W) writer structure"]
impl crate::Writable for CH19_REQSEL {}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch19_reqsel;
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch19_cfg](ch19_cfg) module"]
pub type CH19_CFG = crate::Reg<u32, _CH19_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH19_CFG;
#[doc = "`read()` method returns [ch19_cfg::R](ch19_cfg::R) reader structure"]
impl crate::Readable for CH19_CFG {}
#[doc = "`write(|w| ..)` method takes [ch19_cfg::W](ch19_cfg::W) writer structure"]
impl crate::Writable for CH19_CFG {}
#[doc = "Channel Configuration Register"]
pub mod ch19_cfg;
#[doc = "Channel Loop Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch19_loop](ch19_loop) module"]
pub type CH19_LOOP = crate::Reg<u32, _CH19_LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH19_LOOP;
#[doc = "`read()` method returns [ch19_loop::R](ch19_loop::R) reader structure"]
impl crate::Readable for CH19_LOOP {}
#[doc = "`write(|w| ..)` method takes [ch19_loop::W](ch19_loop::W) writer structure"]
impl crate::Writable for CH19_LOOP {}
#[doc = "Channel Loop Counter Register"]
pub mod ch19_loop;
#[doc = "Channel Descriptor Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch19_ctrl](ch19_ctrl) module"]
pub type CH19_CTRL = crate::Reg<u32, _CH19_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH19_CTRL;
#[doc = "`read()` method returns [ch19_ctrl::R](ch19_ctrl::R) reader structure"]
impl crate::Readable for CH19_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch19_ctrl::W](ch19_ctrl::W) writer structure"]
impl crate::Writable for CH19_CTRL {}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch19_ctrl;
#[doc = "Channel Descriptor Source Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch19_src](ch19_src) module"]
pub type CH19_SRC = crate::Reg<u32, _CH19_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH19_SRC;
#[doc = "`read()` method returns [ch19_src::R](ch19_src::R) reader structure"]
impl crate::Readable for CH19_SRC {}
#[doc = "`write(|w| ..)` method takes [ch19_src::W](ch19_src::W) writer structure"]
impl crate::Writable for CH19_SRC {}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch19_src;
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch19_dst](ch19_dst) module"]
pub type CH19_DST = crate::Reg<u32, _CH19_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH19_DST;
#[doc = "`read()` method returns [ch19_dst::R](ch19_dst::R) reader structure"]
impl crate::Readable for CH19_DST {}
#[doc = "`write(|w| ..)` method takes [ch19_dst::W](ch19_dst::W) writer structure"]
impl crate::Writable for CH19_DST {}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch19_dst;
#[doc = "Channel Descriptor Link Structure Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch19_link](ch19_link) module"]
pub type CH19_LINK = crate::Reg<u32, _CH19_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH19_LINK;
#[doc = "`read()` method returns [ch19_link::R](ch19_link::R) reader structure"]
impl crate::Readable for CH19_LINK {}
#[doc = "`write(|w| ..)` method takes [ch19_link::W](ch19_link::W) writer structure"]
impl crate::Writable for CH19_LINK {}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch19_link;
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch20_reqsel](ch20_reqsel) module"]
pub type CH20_REQSEL = crate::Reg<u32, _CH20_REQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH20_REQSEL;
#[doc = "`read()` method returns [ch20_reqsel::R](ch20_reqsel::R) reader structure"]
impl crate::Readable for CH20_REQSEL {}
#[doc = "`write(|w| ..)` method takes [ch20_reqsel::W](ch20_reqsel::W) writer structure"]
impl crate::Writable for CH20_REQSEL {}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch20_reqsel;
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch20_cfg](ch20_cfg) module"]
pub type CH20_CFG = crate::Reg<u32, _CH20_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH20_CFG;
#[doc = "`read()` method returns [ch20_cfg::R](ch20_cfg::R) reader structure"]
impl crate::Readable for CH20_CFG {}
#[doc = "`write(|w| ..)` method takes [ch20_cfg::W](ch20_cfg::W) writer structure"]
impl crate::Writable for CH20_CFG {}
#[doc = "Channel Configuration Register"]
pub mod ch20_cfg;
#[doc = "Channel Loop Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch20_loop](ch20_loop) module"]
pub type CH20_LOOP = crate::Reg<u32, _CH20_LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH20_LOOP;
#[doc = "`read()` method returns [ch20_loop::R](ch20_loop::R) reader structure"]
impl crate::Readable for CH20_LOOP {}
#[doc = "`write(|w| ..)` method takes [ch20_loop::W](ch20_loop::W) writer structure"]
impl crate::Writable for CH20_LOOP {}
#[doc = "Channel Loop Counter Register"]
pub mod ch20_loop;
#[doc = "Channel Descriptor Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch20_ctrl](ch20_ctrl) module"]
pub type CH20_CTRL = crate::Reg<u32, _CH20_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH20_CTRL;
#[doc = "`read()` method returns [ch20_ctrl::R](ch20_ctrl::R) reader structure"]
impl crate::Readable for CH20_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch20_ctrl::W](ch20_ctrl::W) writer structure"]
impl crate::Writable for CH20_CTRL {}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch20_ctrl;
#[doc = "Channel Descriptor Source Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch20_src](ch20_src) module"]
pub type CH20_SRC = crate::Reg<u32, _CH20_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH20_SRC;
#[doc = "`read()` method returns [ch20_src::R](ch20_src::R) reader structure"]
impl crate::Readable for CH20_SRC {}
#[doc = "`write(|w| ..)` method takes [ch20_src::W](ch20_src::W) writer structure"]
impl crate::Writable for CH20_SRC {}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch20_src;
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch20_dst](ch20_dst) module"]
pub type CH20_DST = crate::Reg<u32, _CH20_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH20_DST;
#[doc = "`read()` method returns [ch20_dst::R](ch20_dst::R) reader structure"]
impl crate::Readable for CH20_DST {}
#[doc = "`write(|w| ..)` method takes [ch20_dst::W](ch20_dst::W) writer structure"]
impl crate::Writable for CH20_DST {}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch20_dst;
#[doc = "Channel Descriptor Link Structure Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch20_link](ch20_link) module"]
pub type CH20_LINK = crate::Reg<u32, _CH20_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH20_LINK;
#[doc = "`read()` method returns [ch20_link::R](ch20_link::R) reader structure"]
impl crate::Readable for CH20_LINK {}
#[doc = "`write(|w| ..)` method takes [ch20_link::W](ch20_link::W) writer structure"]
impl crate::Writable for CH20_LINK {}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch20_link;
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch21_reqsel](ch21_reqsel) module"]
pub type CH21_REQSEL = crate::Reg<u32, _CH21_REQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH21_REQSEL;
#[doc = "`read()` method returns [ch21_reqsel::R](ch21_reqsel::R) reader structure"]
impl crate::Readable for CH21_REQSEL {}
#[doc = "`write(|w| ..)` method takes [ch21_reqsel::W](ch21_reqsel::W) writer structure"]
impl crate::Writable for CH21_REQSEL {}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch21_reqsel;
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch21_cfg](ch21_cfg) module"]
pub type CH21_CFG = crate::Reg<u32, _CH21_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH21_CFG;
#[doc = "`read()` method returns [ch21_cfg::R](ch21_cfg::R) reader structure"]
impl crate::Readable for CH21_CFG {}
#[doc = "`write(|w| ..)` method takes [ch21_cfg::W](ch21_cfg::W) writer structure"]
impl crate::Writable for CH21_CFG {}
#[doc = "Channel Configuration Register"]
pub mod ch21_cfg;
#[doc = "Channel Loop Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch21_loop](ch21_loop) module"]
pub type CH21_LOOP = crate::Reg<u32, _CH21_LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH21_LOOP;
#[doc = "`read()` method returns [ch21_loop::R](ch21_loop::R) reader structure"]
impl crate::Readable for CH21_LOOP {}
#[doc = "`write(|w| ..)` method takes [ch21_loop::W](ch21_loop::W) writer structure"]
impl crate::Writable for CH21_LOOP {}
#[doc = "Channel Loop Counter Register"]
pub mod ch21_loop;
#[doc = "Channel Descriptor Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch21_ctrl](ch21_ctrl) module"]
pub type CH21_CTRL = crate::Reg<u32, _CH21_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH21_CTRL;
#[doc = "`read()` method returns [ch21_ctrl::R](ch21_ctrl::R) reader structure"]
impl crate::Readable for CH21_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch21_ctrl::W](ch21_ctrl::W) writer structure"]
impl crate::Writable for CH21_CTRL {}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch21_ctrl;
#[doc = "Channel Descriptor Source Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch21_src](ch21_src) module"]
pub type CH21_SRC = crate::Reg<u32, _CH21_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH21_SRC;
#[doc = "`read()` method returns [ch21_src::R](ch21_src::R) reader structure"]
impl crate::Readable for CH21_SRC {}
#[doc = "`write(|w| ..)` method takes [ch21_src::W](ch21_src::W) writer structure"]
impl crate::Writable for CH21_SRC {}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch21_src;
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch21_dst](ch21_dst) module"]
pub type CH21_DST = crate::Reg<u32, _CH21_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH21_DST;
#[doc = "`read()` method returns [ch21_dst::R](ch21_dst::R) reader structure"]
impl crate::Readable for CH21_DST {}
#[doc = "`write(|w| ..)` method takes [ch21_dst::W](ch21_dst::W) writer structure"]
impl crate::Writable for CH21_DST {}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch21_dst;
#[doc = "Channel Descriptor Link Structure Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch21_link](ch21_link) module"]
pub type CH21_LINK = crate::Reg<u32, _CH21_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH21_LINK;
#[doc = "`read()` method returns [ch21_link::R](ch21_link::R) reader structure"]
impl crate::Readable for CH21_LINK {}
#[doc = "`write(|w| ..)` method takes [ch21_link::W](ch21_link::W) writer structure"]
impl crate::Writable for CH21_LINK {}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch21_link;
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch22_reqsel](ch22_reqsel) module"]
pub type CH22_REQSEL = crate::Reg<u32, _CH22_REQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH22_REQSEL;
#[doc = "`read()` method returns [ch22_reqsel::R](ch22_reqsel::R) reader structure"]
impl crate::Readable for CH22_REQSEL {}
#[doc = "`write(|w| ..)` method takes [ch22_reqsel::W](ch22_reqsel::W) writer structure"]
impl crate::Writable for CH22_REQSEL {}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch22_reqsel;
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch22_cfg](ch22_cfg) module"]
pub type CH22_CFG = crate::Reg<u32, _CH22_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH22_CFG;
#[doc = "`read()` method returns [ch22_cfg::R](ch22_cfg::R) reader structure"]
impl crate::Readable for CH22_CFG {}
#[doc = "`write(|w| ..)` method takes [ch22_cfg::W](ch22_cfg::W) writer structure"]
impl crate::Writable for CH22_CFG {}
#[doc = "Channel Configuration Register"]
pub mod ch22_cfg;
#[doc = "Channel Loop Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch22_loop](ch22_loop) module"]
pub type CH22_LOOP = crate::Reg<u32, _CH22_LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH22_LOOP;
#[doc = "`read()` method returns [ch22_loop::R](ch22_loop::R) reader structure"]
impl crate::Readable for CH22_LOOP {}
#[doc = "`write(|w| ..)` method takes [ch22_loop::W](ch22_loop::W) writer structure"]
impl crate::Writable for CH22_LOOP {}
#[doc = "Channel Loop Counter Register"]
pub mod ch22_loop;
#[doc = "Channel Descriptor Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch22_ctrl](ch22_ctrl) module"]
pub type CH22_CTRL = crate::Reg<u32, _CH22_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH22_CTRL;
#[doc = "`read()` method returns [ch22_ctrl::R](ch22_ctrl::R) reader structure"]
impl crate::Readable for CH22_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch22_ctrl::W](ch22_ctrl::W) writer structure"]
impl crate::Writable for CH22_CTRL {}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch22_ctrl;
#[doc = "Channel Descriptor Source Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch22_src](ch22_src) module"]
pub type CH22_SRC = crate::Reg<u32, _CH22_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH22_SRC;
#[doc = "`read()` method returns [ch22_src::R](ch22_src::R) reader structure"]
impl crate::Readable for CH22_SRC {}
#[doc = "`write(|w| ..)` method takes [ch22_src::W](ch22_src::W) writer structure"]
impl crate::Writable for CH22_SRC {}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch22_src;
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch22_dst](ch22_dst) module"]
pub type CH22_DST = crate::Reg<u32, _CH22_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH22_DST;
#[doc = "`read()` method returns [ch22_dst::R](ch22_dst::R) reader structure"]
impl crate::Readable for CH22_DST {}
#[doc = "`write(|w| ..)` method takes [ch22_dst::W](ch22_dst::W) writer structure"]
impl crate::Writable for CH22_DST {}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch22_dst;
#[doc = "Channel Descriptor Link Structure Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch22_link](ch22_link) module"]
pub type CH22_LINK = crate::Reg<u32, _CH22_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH22_LINK;
#[doc = "`read()` method returns [ch22_link::R](ch22_link::R) reader structure"]
impl crate::Readable for CH22_LINK {}
#[doc = "`write(|w| ..)` method takes [ch22_link::W](ch22_link::W) writer structure"]
impl crate::Writable for CH22_LINK {}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch22_link;
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch23_reqsel](ch23_reqsel) module"]
pub type CH23_REQSEL = crate::Reg<u32, _CH23_REQSEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH23_REQSEL;
#[doc = "`read()` method returns [ch23_reqsel::R](ch23_reqsel::R) reader structure"]
impl crate::Readable for CH23_REQSEL {}
#[doc = "`write(|w| ..)` method takes [ch23_reqsel::W](ch23_reqsel::W) writer structure"]
impl crate::Writable for CH23_REQSEL {}
#[doc = "Channel Peripheral Request Select Register"]
pub mod ch23_reqsel;
#[doc = "Channel Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch23_cfg](ch23_cfg) module"]
pub type CH23_CFG = crate::Reg<u32, _CH23_CFG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH23_CFG;
#[doc = "`read()` method returns [ch23_cfg::R](ch23_cfg::R) reader structure"]
impl crate::Readable for CH23_CFG {}
#[doc = "`write(|w| ..)` method takes [ch23_cfg::W](ch23_cfg::W) writer structure"]
impl crate::Writable for CH23_CFG {}
#[doc = "Channel Configuration Register"]
pub mod ch23_cfg;
#[doc = "Channel Loop Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch23_loop](ch23_loop) module"]
pub type CH23_LOOP = crate::Reg<u32, _CH23_LOOP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH23_LOOP;
#[doc = "`read()` method returns [ch23_loop::R](ch23_loop::R) reader structure"]
impl crate::Readable for CH23_LOOP {}
#[doc = "`write(|w| ..)` method takes [ch23_loop::W](ch23_loop::W) writer structure"]
impl crate::Writable for CH23_LOOP {}
#[doc = "Channel Loop Counter Register"]
pub mod ch23_loop;
#[doc = "Channel Descriptor Control Word Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch23_ctrl](ch23_ctrl) module"]
pub type CH23_CTRL = crate::Reg<u32, _CH23_CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH23_CTRL;
#[doc = "`read()` method returns [ch23_ctrl::R](ch23_ctrl::R) reader structure"]
impl crate::Readable for CH23_CTRL {}
#[doc = "`write(|w| ..)` method takes [ch23_ctrl::W](ch23_ctrl::W) writer structure"]
impl crate::Writable for CH23_CTRL {}
#[doc = "Channel Descriptor Control Word Register"]
pub mod ch23_ctrl;
#[doc = "Channel Descriptor Source Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch23_src](ch23_src) module"]
pub type CH23_SRC = crate::Reg<u32, _CH23_SRC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH23_SRC;
#[doc = "`read()` method returns [ch23_src::R](ch23_src::R) reader structure"]
impl crate::Readable for CH23_SRC {}
#[doc = "`write(|w| ..)` method takes [ch23_src::W](ch23_src::W) writer structure"]
impl crate::Writable for CH23_SRC {}
#[doc = "Channel Descriptor Source Data Address Register"]
pub mod ch23_src;
#[doc = "Channel Descriptor Destination Data Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch23_dst](ch23_dst) module"]
pub type CH23_DST = crate::Reg<u32, _CH23_DST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH23_DST;
#[doc = "`read()` method returns [ch23_dst::R](ch23_dst::R) reader structure"]
impl crate::Readable for CH23_DST {}
#[doc = "`write(|w| ..)` method takes [ch23_dst::W](ch23_dst::W) writer structure"]
impl crate::Writable for CH23_DST {}
#[doc = "Channel Descriptor Destination Data Address Register"]
pub mod ch23_dst;
#[doc = "Channel Descriptor Link Structure Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch23_link](ch23_link) module"]
pub type CH23_LINK = crate::Reg<u32, _CH23_LINK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CH23_LINK;
#[doc = "`read()` method returns [ch23_link::R](ch23_link::R) reader structure"]
impl crate::Readable for CH23_LINK {}
#[doc = "`write(|w| ..)` method takes [ch23_link::W](ch23_link::W) writer structure"]
impl crate::Writable for CH23_LINK {}
#[doc = "Channel Descriptor Link Structure Address Register"]
pub mod ch23_link;
