#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Counter Value Register"]
    pub cnt: CNT,
    #[doc = "0x08 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x0c - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x10 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub ien: IEN,
    _reserved6: [u8; 8usize],
    #[doc = "0x20 - Compare Value Register X"]
    pub compa_comp: COMPA_COMP,
    #[doc = "0x24 - Compare Value Register X"]
    pub compb_comp: COMPB_COMP,
    #[doc = "0x28 - Compare Value Register X"]
    pub compc_comp: COMPC_COMP,
    #[doc = "0x2c - Compare Value Register X"]
    pub compd_comp: COMPD_COMP,
    #[doc = "0x30 - Compare Value Register X"]
    pub compe_comp: COMPE_COMP,
    #[doc = "0x34 - Compare Value Register X"]
    pub compf_comp: COMPF_COMP,
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
#[doc = "Counter Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cnt](cnt) module"]
pub type CNT = crate::Reg<u32, _CNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNT;
#[doc = "`read()` method returns [cnt::R](cnt::R) reader structure"]
impl crate::Readable for CNT {}
#[doc = "`write(|w| ..)` method takes [cnt::W](cnt::W) writer structure"]
impl crate::Writable for CNT {}
#[doc = "Counter Value Register"]
pub mod cnt;
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
#[doc = "Compare Value Register X\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [compa_comp](compa_comp) module"]
pub type COMPA_COMP = crate::Reg<u32, _COMPA_COMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMPA_COMP;
#[doc = "`read()` method returns [compa_comp::R](compa_comp::R) reader structure"]
impl crate::Readable for COMPA_COMP {}
#[doc = "`write(|w| ..)` method takes [compa_comp::W](compa_comp::W) writer structure"]
impl crate::Writable for COMPA_COMP {}
#[doc = "Compare Value Register X"]
pub mod compa_comp;
#[doc = "Compare Value Register X\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [compb_comp](compb_comp) module"]
pub type COMPB_COMP = crate::Reg<u32, _COMPB_COMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMPB_COMP;
#[doc = "`read()` method returns [compb_comp::R](compb_comp::R) reader structure"]
impl crate::Readable for COMPB_COMP {}
#[doc = "`write(|w| ..)` method takes [compb_comp::W](compb_comp::W) writer structure"]
impl crate::Writable for COMPB_COMP {}
#[doc = "Compare Value Register X"]
pub mod compb_comp;
#[doc = "Compare Value Register X\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [compc_comp](compc_comp) module"]
pub type COMPC_COMP = crate::Reg<u32, _COMPC_COMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMPC_COMP;
#[doc = "`read()` method returns [compc_comp::R](compc_comp::R) reader structure"]
impl crate::Readable for COMPC_COMP {}
#[doc = "`write(|w| ..)` method takes [compc_comp::W](compc_comp::W) writer structure"]
impl crate::Writable for COMPC_COMP {}
#[doc = "Compare Value Register X"]
pub mod compc_comp;
#[doc = "Compare Value Register X\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [compd_comp](compd_comp) module"]
pub type COMPD_COMP = crate::Reg<u32, _COMPD_COMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMPD_COMP;
#[doc = "`read()` method returns [compd_comp::R](compd_comp::R) reader structure"]
impl crate::Readable for COMPD_COMP {}
#[doc = "`write(|w| ..)` method takes [compd_comp::W](compd_comp::W) writer structure"]
impl crate::Writable for COMPD_COMP {}
#[doc = "Compare Value Register X"]
pub mod compd_comp;
#[doc = "Compare Value Register X\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [compe_comp](compe_comp) module"]
pub type COMPE_COMP = crate::Reg<u32, _COMPE_COMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMPE_COMP;
#[doc = "`read()` method returns [compe_comp::R](compe_comp::R) reader structure"]
impl crate::Readable for COMPE_COMP {}
#[doc = "`write(|w| ..)` method takes [compe_comp::W](compe_comp::W) writer structure"]
impl crate::Writable for COMPE_COMP {}
#[doc = "Compare Value Register X"]
pub mod compe_comp;
#[doc = "Compare Value Register X\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [compf_comp](compf_comp) module"]
pub type COMPF_COMP = crate::Reg<u32, _COMPF_COMP>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMPF_COMP;
#[doc = "`read()` method returns [compf_comp::R](compf_comp::R) reader structure"]
impl crate::Readable for COMPF_COMP {}
#[doc = "`write(|w| ..)` method takes [compf_comp::W](compf_comp::W) writer structure"]
impl crate::Writable for COMPF_COMP {}
#[doc = "Compare Value Register X"]
pub mod compf_comp;
