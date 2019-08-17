#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Display Control Register"]
    pub dispctrl: DISPCTRL,
    #[doc = "0x08 - Segment Enable Register"]
    pub segen: SEGEN,
    #[doc = "0x0c - Blink and Animation Control Register"]
    pub bactrl: BACTRL,
    #[doc = "0x10 - Status Register"]
    pub status: STATUS,
    #[doc = "0x14 - Animation Register a"]
    pub arega: AREGA,
    #[doc = "0x18 - Animation Register B"]
    pub aregb: AREGB,
    #[doc = "0x1c - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x20 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x24 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x28 - Interrupt Enable Register"]
    pub ien: IEN,
    _reserved11: [u8; 4usize],
    #[doc = "0x30 - Analog BIAS Control"]
    pub biasctrl: BIASCTRL,
    _reserved12: [u8; 12usize],
    #[doc = "0x40 - Segment Data Low Register 0"]
    pub segd0l: SEGD0L,
    #[doc = "0x44 - Segment Data Low Register 1"]
    pub segd1l: SEGD1L,
    #[doc = "0x48 - Segment Data Low Register 2"]
    pub segd2l: SEGD2L,
    #[doc = "0x4c - Segment Data Low Register 3"]
    pub segd3l: SEGD3L,
    #[doc = "0x50 - Segment Data High Register 0"]
    pub segd0h: SEGD0H,
    #[doc = "0x54 - Segment Data High Register 1"]
    pub segd1h: SEGD1H,
    #[doc = "0x58 - Segment Data High Register 2"]
    pub segd2h: SEGD2H,
    #[doc = "0x5c - Segment Data High Register 3"]
    pub segd3h: SEGD3H,
    #[doc = "0x60 - Segment Data Low Register 4"]
    pub segd4l: SEGD4L,
    #[doc = "0x64 - Segment Data Low Register 5"]
    pub segd5l: SEGD5L,
    #[doc = "0x68 - Segment Data Low Register 6"]
    pub segd6l: SEGD6L,
    #[doc = "0x6c - Segment Data Low Register 7"]
    pub segd7l: SEGD7L,
    #[doc = "0x70 - Segment Data High Register 4"]
    pub segd4h: SEGD4H,
    #[doc = "0x74 - Segment Data High Register 5"]
    pub segd5h: SEGD5H,
    #[doc = "0x78 - Segment Data High Register 6"]
    pub segd6h: SEGD6H,
    #[doc = "0x7c - Segment Data High Register 7"]
    pub segd7h: SEGD7H,
    _reserved28: [u8; 64usize],
    #[doc = "0xc0 - Freeze Register"]
    pub freeze: FREEZE,
    #[doc = "0xc4 - Synchronization Busy Register"]
    pub syncbusy: SYNCBUSY,
    _reserved30: [u8; 40usize],
    #[doc = "0xf0 - Frame Rate"]
    pub framerate: FRAMERATE,
    #[doc = "0xf4 - Segment Enable (32 to 39)"]
    pub segen2: SEGEN2,
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "Display Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dispctrl](dispctrl) module"]
pub type DISPCTRL = crate::Reg<u32, _DISPCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DISPCTRL;
#[doc = "`read()` method returns [dispctrl::R](dispctrl::R) reader structure"]
impl crate::Readable for DISPCTRL {}
#[doc = "`write(|w| ..)` method takes [dispctrl::W](dispctrl::W) writer structure"]
impl crate::Writable for DISPCTRL {}
#[doc = "Display Control Register"]
pub mod dispctrl;
#[doc = "Segment Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [segen](segen) module"]
pub type SEGEN = crate::Reg<u32, _SEGEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEGEN;
#[doc = "`read()` method returns [segen::R](segen::R) reader structure"]
impl crate::Readable for SEGEN {}
#[doc = "`write(|w| ..)` method takes [segen::W](segen::W) writer structure"]
impl crate::Writable for SEGEN {}
#[doc = "Segment Enable Register"]
pub mod segen;
#[doc = "Blink and Animation Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bactrl](bactrl) module"]
pub type BACTRL = crate::Reg<u32, _BACTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BACTRL;
#[doc = "`read()` method returns [bactrl::R](bactrl::R) reader structure"]
impl crate::Readable for BACTRL {}
#[doc = "`write(|w| ..)` method takes [bactrl::W](bactrl::W) writer structure"]
impl crate::Writable for BACTRL {}
#[doc = "Blink and Animation Control Register"]
pub mod bactrl;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status Register"]
pub mod status;
#[doc = "Animation Register a\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [arega](arega) module"]
pub type AREGA = crate::Reg<u32, _AREGA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AREGA;
#[doc = "`read()` method returns [arega::R](arega::R) reader structure"]
impl crate::Readable for AREGA {}
#[doc = "`write(|w| ..)` method takes [arega::W](arega::W) writer structure"]
impl crate::Writable for AREGA {}
#[doc = "Animation Register a"]
pub mod arega;
#[doc = "Animation Register B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [aregb](aregb) module"]
pub type AREGB = crate::Reg<u32, _AREGB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AREGB;
#[doc = "`read()` method returns [aregb::R](aregb::R) reader structure"]
impl crate::Readable for AREGB {}
#[doc = "`write(|w| ..)` method takes [aregb::W](aregb::W) writer structure"]
impl crate::Writable for AREGB {}
#[doc = "Animation Register B"]
pub mod aregb;
#[doc = "Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [if_](if_) module"]
pub type IF = crate::Reg<u32, _IF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF;
#[doc = "`read()` method returns [if_::R](if_::R) reader structure"]
impl crate::Readable for IF {}
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "Interrupt Flag Set Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ifs](ifs) module"]
pub type IFS = crate::Reg<u32, _IFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFS;
#[doc = "`write(|w| ..)` method takes [ifs::W](ifs::W) writer structure"]
impl crate::Writable for IFS {}
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "Interrupt Flag Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ifc](ifc) module"]
pub type IFC = crate::Reg<u32, _IFC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFC;
#[doc = "`write(|w| ..)` method takes [ifc::W](ifc::W) writer structure"]
impl crate::Writable for IFC {}
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ien](ien) module"]
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
#[doc = "Analog BIAS Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [biasctrl](biasctrl) module"]
pub type BIASCTRL = crate::Reg<u32, _BIASCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BIASCTRL;
#[doc = "`read()` method returns [biasctrl::R](biasctrl::R) reader structure"]
impl crate::Readable for BIASCTRL {}
#[doc = "`write(|w| ..)` method takes [biasctrl::W](biasctrl::W) writer structure"]
impl crate::Writable for BIASCTRL {}
#[doc = "Analog BIAS Control"]
pub mod biasctrl;
#[doc = "Segment Data Low Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [segd0l](segd0l) module"]
pub type SEGD0L = crate::Reg<u32, _SEGD0L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEGD0L;
#[doc = "`read()` method returns [segd0l::R](segd0l::R) reader structure"]
impl crate::Readable for SEGD0L {}
#[doc = "`write(|w| ..)` method takes [segd0l::W](segd0l::W) writer structure"]
impl crate::Writable for SEGD0L {}
#[doc = "Segment Data Low Register 0"]
pub mod segd0l;
#[doc = "Segment Data Low Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [segd1l](segd1l) module"]
pub type SEGD1L = crate::Reg<u32, _SEGD1L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEGD1L;
#[doc = "`read()` method returns [segd1l::R](segd1l::R) reader structure"]
impl crate::Readable for SEGD1L {}
#[doc = "`write(|w| ..)` method takes [segd1l::W](segd1l::W) writer structure"]
impl crate::Writable for SEGD1L {}
#[doc = "Segment Data Low Register 1"]
pub mod segd1l;
#[doc = "Segment Data Low Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [segd2l](segd2l) module"]
pub type SEGD2L = crate::Reg<u32, _SEGD2L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEGD2L;
#[doc = "`read()` method returns [segd2l::R](segd2l::R) reader structure"]
impl crate::Readable for SEGD2L {}
#[doc = "`write(|w| ..)` method takes [segd2l::W](segd2l::W) writer structure"]
impl crate::Writable for SEGD2L {}
#[doc = "Segment Data Low Register 2"]
pub mod segd2l;
#[doc = "Segment Data Low Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [segd3l](segd3l) module"]
pub type SEGD3L = crate::Reg<u32, _SEGD3L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEGD3L;
#[doc = "`read()` method returns [segd3l::R](segd3l::R) reader structure"]
impl crate::Readable for SEGD3L {}
#[doc = "`write(|w| ..)` method takes [segd3l::W](segd3l::W) writer structure"]
impl crate::Writable for SEGD3L {}
#[doc = "Segment Data Low Register 3"]
pub mod segd3l;
#[doc = "Segment Data High Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [segd0h](segd0h) module"]
pub type SEGD0H = crate::Reg<u32, _SEGD0H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEGD0H;
#[doc = "`read()` method returns [segd0h::R](segd0h::R) reader structure"]
impl crate::Readable for SEGD0H {}
#[doc = "`write(|w| ..)` method takes [segd0h::W](segd0h::W) writer structure"]
impl crate::Writable for SEGD0H {}
#[doc = "Segment Data High Register 0"]
pub mod segd0h;
#[doc = "Segment Data High Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [segd1h](segd1h) module"]
pub type SEGD1H = crate::Reg<u32, _SEGD1H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEGD1H;
#[doc = "`read()` method returns [segd1h::R](segd1h::R) reader structure"]
impl crate::Readable for SEGD1H {}
#[doc = "`write(|w| ..)` method takes [segd1h::W](segd1h::W) writer structure"]
impl crate::Writable for SEGD1H {}
#[doc = "Segment Data High Register 1"]
pub mod segd1h;
#[doc = "Segment Data High Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [segd2h](segd2h) module"]
pub type SEGD2H = crate::Reg<u32, _SEGD2H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEGD2H;
#[doc = "`read()` method returns [segd2h::R](segd2h::R) reader structure"]
impl crate::Readable for SEGD2H {}
#[doc = "`write(|w| ..)` method takes [segd2h::W](segd2h::W) writer structure"]
impl crate::Writable for SEGD2H {}
#[doc = "Segment Data High Register 2"]
pub mod segd2h;
#[doc = "Segment Data High Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [segd3h](segd3h) module"]
pub type SEGD3H = crate::Reg<u32, _SEGD3H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEGD3H;
#[doc = "`read()` method returns [segd3h::R](segd3h::R) reader structure"]
impl crate::Readable for SEGD3H {}
#[doc = "`write(|w| ..)` method takes [segd3h::W](segd3h::W) writer structure"]
impl crate::Writable for SEGD3H {}
#[doc = "Segment Data High Register 3"]
pub mod segd3h;
#[doc = "Segment Data Low Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [segd4l](segd4l) module"]
pub type SEGD4L = crate::Reg<u32, _SEGD4L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEGD4L;
#[doc = "`read()` method returns [segd4l::R](segd4l::R) reader structure"]
impl crate::Readable for SEGD4L {}
#[doc = "`write(|w| ..)` method takes [segd4l::W](segd4l::W) writer structure"]
impl crate::Writable for SEGD4L {}
#[doc = "Segment Data Low Register 4"]
pub mod segd4l;
#[doc = "Segment Data Low Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [segd5l](segd5l) module"]
pub type SEGD5L = crate::Reg<u32, _SEGD5L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEGD5L;
#[doc = "`read()` method returns [segd5l::R](segd5l::R) reader structure"]
impl crate::Readable for SEGD5L {}
#[doc = "`write(|w| ..)` method takes [segd5l::W](segd5l::W) writer structure"]
impl crate::Writable for SEGD5L {}
#[doc = "Segment Data Low Register 5"]
pub mod segd5l;
#[doc = "Segment Data Low Register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [segd6l](segd6l) module"]
pub type SEGD6L = crate::Reg<u32, _SEGD6L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEGD6L;
#[doc = "`read()` method returns [segd6l::R](segd6l::R) reader structure"]
impl crate::Readable for SEGD6L {}
#[doc = "`write(|w| ..)` method takes [segd6l::W](segd6l::W) writer structure"]
impl crate::Writable for SEGD6L {}
#[doc = "Segment Data Low Register 6"]
pub mod segd6l;
#[doc = "Segment Data Low Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [segd7l](segd7l) module"]
pub type SEGD7L = crate::Reg<u32, _SEGD7L>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEGD7L;
#[doc = "`read()` method returns [segd7l::R](segd7l::R) reader structure"]
impl crate::Readable for SEGD7L {}
#[doc = "`write(|w| ..)` method takes [segd7l::W](segd7l::W) writer structure"]
impl crate::Writable for SEGD7L {}
#[doc = "Segment Data Low Register 7"]
pub mod segd7l;
#[doc = "Segment Data High Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [segd4h](segd4h) module"]
pub type SEGD4H = crate::Reg<u32, _SEGD4H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEGD4H;
#[doc = "`read()` method returns [segd4h::R](segd4h::R) reader structure"]
impl crate::Readable for SEGD4H {}
#[doc = "`write(|w| ..)` method takes [segd4h::W](segd4h::W) writer structure"]
impl crate::Writable for SEGD4H {}
#[doc = "Segment Data High Register 4"]
pub mod segd4h;
#[doc = "Segment Data High Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [segd5h](segd5h) module"]
pub type SEGD5H = crate::Reg<u32, _SEGD5H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEGD5H;
#[doc = "`read()` method returns [segd5h::R](segd5h::R) reader structure"]
impl crate::Readable for SEGD5H {}
#[doc = "`write(|w| ..)` method takes [segd5h::W](segd5h::W) writer structure"]
impl crate::Writable for SEGD5H {}
#[doc = "Segment Data High Register 5"]
pub mod segd5h;
#[doc = "Segment Data High Register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [segd6h](segd6h) module"]
pub type SEGD6H = crate::Reg<u32, _SEGD6H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEGD6H;
#[doc = "`read()` method returns [segd6h::R](segd6h::R) reader structure"]
impl crate::Readable for SEGD6H {}
#[doc = "`write(|w| ..)` method takes [segd6h::W](segd6h::W) writer structure"]
impl crate::Writable for SEGD6H {}
#[doc = "Segment Data High Register 6"]
pub mod segd6h;
#[doc = "Segment Data High Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [segd7h](segd7h) module"]
pub type SEGD7H = crate::Reg<u32, _SEGD7H>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEGD7H;
#[doc = "`read()` method returns [segd7h::R](segd7h::R) reader structure"]
impl crate::Readable for SEGD7H {}
#[doc = "`write(|w| ..)` method takes [segd7h::W](segd7h::W) writer structure"]
impl crate::Writable for SEGD7H {}
#[doc = "Segment Data High Register 7"]
pub mod segd7h;
#[doc = "Freeze Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [freeze](freeze) module"]
pub type FREEZE = crate::Reg<u32, _FREEZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FREEZE;
#[doc = "`read()` method returns [freeze::R](freeze::R) reader structure"]
impl crate::Readable for FREEZE {}
#[doc = "`write(|w| ..)` method takes [freeze::W](freeze::W) writer structure"]
impl crate::Writable for FREEZE {}
#[doc = "Freeze Register"]
pub mod freeze;
#[doc = "Synchronization Busy Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [syncbusy](syncbusy) module"]
pub type SYNCBUSY = crate::Reg<u32, _SYNCBUSY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYNCBUSY;
#[doc = "`read()` method returns [syncbusy::R](syncbusy::R) reader structure"]
impl crate::Readable for SYNCBUSY {}
#[doc = "Synchronization Busy Register"]
pub mod syncbusy;
#[doc = "Frame Rate\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [framerate](framerate) module"]
pub type FRAMERATE = crate::Reg<u32, _FRAMERATE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FRAMERATE;
#[doc = "`read()` method returns [framerate::R](framerate::R) reader structure"]
impl crate::Readable for FRAMERATE {}
#[doc = "`write(|w| ..)` method takes [framerate::W](framerate::W) writer structure"]
impl crate::Writable for FRAMERATE {}
#[doc = "Frame Rate"]
pub mod framerate;
#[doc = "Segment Enable (32 to 39)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [segen2](segen2) module"]
pub type SEGEN2 = crate::Reg<u32, _SEGEN2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEGEN2;
#[doc = "`read()` method returns [segen2::R](segen2::R) reader structure"]
impl crate::Readable for SEGEN2 {}
#[doc = "`write(|w| ..)` method takes [segen2::W](segen2::W) writer structure"]
impl crate::Writable for SEGEN2 {}
#[doc = "Segment Enable (32 to 39)"]
pub mod segen2;
