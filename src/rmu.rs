#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Reset Cause Register"]
    pub rstcause: RSTCAUSE,
    #[doc = "0x08 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x0c - Reset Control Register"]
    pub rst: RST,
    #[doc = "0x10 - Configuration Lock Register"]
    pub lock: LOCK,
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
#[doc = "Reset Cause Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rstcause](rstcause) module"]
pub type RSTCAUSE = crate::Reg<u32, _RSTCAUSE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTCAUSE;
#[doc = "`read()` method returns [rstcause::R](rstcause::R) reader structure"]
impl crate::Readable for RSTCAUSE {}
#[doc = "Reset Cause Register"]
pub mod rstcause;
#[doc = "Command Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "Command Register"]
pub mod cmd;
#[doc = "Reset Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [rst](rst) module"]
pub type RST = crate::Reg<u32, _RST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RST;
#[doc = "`read()` method returns [rst::R](rst::R) reader structure"]
impl crate::Readable for RST {}
#[doc = "`write(|w| ..)` method takes [rst::W](rst::W) writer structure"]
impl crate::Writable for RST {}
#[doc = "Reset Control Register"]
pub mod rst;
#[doc = "Configuration Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [lock](lock) module"]
pub type LOCK = crate::Reg<u32, _LOCK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LOCK;
#[doc = "`read()` method returns [lock::R](lock::R) reader structure"]
impl crate::Readable for LOCK {}
#[doc = "`write(|w| ..)` method takes [lock::W](lock::W) writer structure"]
impl crate::Writable for LOCK {}
#[doc = "Configuration Lock Register"]
pub mod lock;
