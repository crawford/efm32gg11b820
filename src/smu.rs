#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 12usize],
    #[doc = "0x0c - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x10 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x14 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x18 - Interrupt Enable Register"]
    pub ien: IEN,
    _reserved4: [u8; 36usize],
    #[doc = "0x40 - PPU Control Register"]
    pub ppuctrl: PPUCTRL,
    _reserved5: [u8; 12usize],
    #[doc = "0x50 - PPU Privilege Access Type Descriptor 0"]
    pub ppupatd0: PPUPATD0,
    #[doc = "0x54 - PPU Privilege Access Type Descriptor 1"]
    pub ppupatd1: PPUPATD1,
    #[doc = "0x58 - PPU Privilege Access Type Descriptor 2"]
    pub ppupatd2: PPUPATD2,
    _reserved8: [u8; 52usize],
    #[doc = "0x90 - PPU Fault Status"]
    pub ppufs: PPUFS,
}
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
#[doc = "PPU Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppuctrl](ppuctrl) module"]
pub type PPUCTRL = crate::Reg<u32, _PPUCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPUCTRL;
#[doc = "`read()` method returns [ppuctrl::R](ppuctrl::R) reader structure"]
impl crate::Readable for PPUCTRL {}
#[doc = "`write(|w| ..)` method takes [ppuctrl::W](ppuctrl::W) writer structure"]
impl crate::Writable for PPUCTRL {}
#[doc = "PPU Control Register"]
pub mod ppuctrl;
#[doc = "PPU Privilege Access Type Descriptor 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppupatd0](ppupatd0) module"]
pub type PPUPATD0 = crate::Reg<u32, _PPUPATD0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPUPATD0;
#[doc = "`read()` method returns [ppupatd0::R](ppupatd0::R) reader structure"]
impl crate::Readable for PPUPATD0 {}
#[doc = "`write(|w| ..)` method takes [ppupatd0::W](ppupatd0::W) writer structure"]
impl crate::Writable for PPUPATD0 {}
#[doc = "PPU Privilege Access Type Descriptor 0"]
pub mod ppupatd0;
#[doc = "PPU Privilege Access Type Descriptor 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppupatd1](ppupatd1) module"]
pub type PPUPATD1 = crate::Reg<u32, _PPUPATD1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPUPATD1;
#[doc = "`read()` method returns [ppupatd1::R](ppupatd1::R) reader structure"]
impl crate::Readable for PPUPATD1 {}
#[doc = "`write(|w| ..)` method takes [ppupatd1::W](ppupatd1::W) writer structure"]
impl crate::Writable for PPUPATD1 {}
#[doc = "PPU Privilege Access Type Descriptor 1"]
pub mod ppupatd1;
#[doc = "PPU Privilege Access Type Descriptor 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppupatd2](ppupatd2) module"]
pub type PPUPATD2 = crate::Reg<u32, _PPUPATD2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPUPATD2;
#[doc = "`read()` method returns [ppupatd2::R](ppupatd2::R) reader structure"]
impl crate::Readable for PPUPATD2 {}
#[doc = "`write(|w| ..)` method takes [ppupatd2::W](ppupatd2::W) writer structure"]
impl crate::Writable for PPUPATD2 {}
#[doc = "PPU Privilege Access Type Descriptor 2"]
pub mod ppupatd2;
#[doc = "PPU Fault Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppufs](ppufs) module"]
pub type PPUFS = crate::Reg<u32, _PPUFS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PPUFS;
#[doc = "`read()` method returns [ppufs::R](ppufs::R) reader structure"]
impl crate::Readable for PPUFS {}
#[doc = "PPU Fault Status"]
pub mod ppufs;
