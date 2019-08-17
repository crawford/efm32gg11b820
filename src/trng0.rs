#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Main Control Register"]
    pub control: CONTROL,
    #[doc = "0x04 - FIFO Level Register"]
    pub fifolevel: FIFOLEVEL,
    _reserved2: [u8; 4usize],
    #[doc = "0x0c - FIFO Depth Register"]
    pub fifodepth: FIFODEPTH,
    #[doc = "0x10 - Key Register 0"]
    pub key0: KEY0,
    #[doc = "0x14 - Key Register 1"]
    pub key1: KEY1,
    #[doc = "0x18 - Key Register 2"]
    pub key2: KEY2,
    #[doc = "0x1c - Key Register 3"]
    pub key3: KEY3,
    #[doc = "0x20 - Test Data Register"]
    pub testdata: TESTDATA,
    _reserved8: [u8; 12usize],
    #[doc = "0x30 - Status Register"]
    pub status: STATUS,
    #[doc = "0x34 - Initial Wait Counter"]
    pub initwaitval: INITWAITVAL,
    _reserved10: [u8; 200usize],
    #[doc = "0x100 - FIFO Data"]
    pub fifo: FIFO,
}
#[doc = "Main Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [control](control) module"]
pub type CONTROL = crate::Reg<u32, _CONTROL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CONTROL;
#[doc = "`read()` method returns [control::R](control::R) reader structure"]
impl crate::Readable for CONTROL {}
#[doc = "`write(|w| ..)` method takes [control::W](control::W) writer structure"]
impl crate::Writable for CONTROL {}
#[doc = "Main Control Register"]
pub mod control;
#[doc = "FIFO Level Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fifolevel](fifolevel) module"]
pub type FIFOLEVEL = crate::Reg<u32, _FIFOLEVEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFOLEVEL;
#[doc = "`read()` method returns [fifolevel::R](fifolevel::R) reader structure"]
impl crate::Readable for FIFOLEVEL {}
#[doc = "FIFO Level Register"]
pub mod fifolevel;
#[doc = "FIFO Depth Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fifodepth](fifodepth) module"]
pub type FIFODEPTH = crate::Reg<u32, _FIFODEPTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFODEPTH;
#[doc = "`read()` method returns [fifodepth::R](fifodepth::R) reader structure"]
impl crate::Readable for FIFODEPTH {}
#[doc = "FIFO Depth Register"]
pub mod fifodepth;
#[doc = "Key Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [key0](key0) module"]
pub type KEY0 = crate::Reg<u32, _KEY0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY0;
#[doc = "`read()` method returns [key0::R](key0::R) reader structure"]
impl crate::Readable for KEY0 {}
#[doc = "`write(|w| ..)` method takes [key0::W](key0::W) writer structure"]
impl crate::Writable for KEY0 {}
#[doc = "Key Register 0"]
pub mod key0;
#[doc = "Key Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [key1](key1) module"]
pub type KEY1 = crate::Reg<u32, _KEY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY1;
#[doc = "`read()` method returns [key1::R](key1::R) reader structure"]
impl crate::Readable for KEY1 {}
#[doc = "`write(|w| ..)` method takes [key1::W](key1::W) writer structure"]
impl crate::Writable for KEY1 {}
#[doc = "Key Register 1"]
pub mod key1;
#[doc = "Key Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [key2](key2) module"]
pub type KEY2 = crate::Reg<u32, _KEY2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY2;
#[doc = "`read()` method returns [key2::R](key2::R) reader structure"]
impl crate::Readable for KEY2 {}
#[doc = "`write(|w| ..)` method takes [key2::W](key2::W) writer structure"]
impl crate::Writable for KEY2 {}
#[doc = "Key Register 2"]
pub mod key2;
#[doc = "Key Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [key3](key3) module"]
pub type KEY3 = crate::Reg<u32, _KEY3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY3;
#[doc = "`read()` method returns [key3::R](key3::R) reader structure"]
impl crate::Readable for KEY3 {}
#[doc = "`write(|w| ..)` method takes [key3::W](key3::W) writer structure"]
impl crate::Writable for KEY3 {}
#[doc = "Key Register 3"]
pub mod key3;
#[doc = "Test Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [testdata](testdata) module"]
pub type TESTDATA = crate::Reg<u32, _TESTDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TESTDATA;
#[doc = "`read()` method returns [testdata::R](testdata::R) reader structure"]
impl crate::Readable for TESTDATA {}
#[doc = "`write(|w| ..)` method takes [testdata::W](testdata::W) writer structure"]
impl crate::Writable for TESTDATA {}
#[doc = "Test Data Register"]
pub mod testdata;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "`write(|w| ..)` method takes [status::W](status::W) writer structure"]
impl crate::Writable for STATUS {}
#[doc = "Status Register"]
pub mod status;
#[doc = "Initial Wait Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [initwaitval](initwaitval) module"]
pub type INITWAITVAL = crate::Reg<u32, _INITWAITVAL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INITWAITVAL;
#[doc = "`read()` method returns [initwaitval::R](initwaitval::R) reader structure"]
impl crate::Readable for INITWAITVAL {}
#[doc = "`write(|w| ..)` method takes [initwaitval::W](initwaitval::W) writer structure"]
impl crate::Writable for INITWAITVAL {}
#[doc = "Initial Wait Counter"]
pub mod initwaitval;
#[doc = "FIFO Data\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fifo](fifo) module"]
pub type FIFO = crate::Reg<u32, _FIFO>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FIFO;
#[doc = "`read()` method returns [fifo::R](fifo::R) reader structure"]
impl crate::Readable for FIFO {}
#[doc = "FIFO Data"]
pub mod fifo;
