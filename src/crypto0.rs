#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Wide Arithmetic Configuration"]
    pub wac: WAC,
    #[doc = "0x08 - Command Register"]
    pub cmd: CMD,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - Status Register"]
    pub status: STATUS,
    #[doc = "0x14 - Data Status Register"]
    pub dstatus: DSTATUS,
    #[doc = "0x18 - Control Status Register"]
    pub cstatus: CSTATUS,
    _reserved6: [u8; 4usize],
    #[doc = "0x20 - KEY Register Access"]
    pub key: KEY,
    #[doc = "0x24 - KEY Buffer Register Access"]
    pub keybuf: KEYBUF,
    _reserved8: [u8; 8usize],
    #[doc = "0x30 - Sequence Control"]
    pub seqctrl: SEQCTRL,
    #[doc = "0x34 - Sequence Control B"]
    pub seqctrlb: SEQCTRLB,
    _reserved10: [u8; 8usize],
    #[doc = "0x40 - AES Interrupt Flags"]
    pub if_: IF,
    #[doc = "0x44 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x48 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x4c - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x50 - Sequence Register 0"]
    pub seq0: SEQ0,
    #[doc = "0x54 - Sequence Register 1"]
    pub seq1: SEQ1,
    #[doc = "0x58 - Sequence Register 2"]
    pub seq2: SEQ2,
    #[doc = "0x5c - Sequence Register 3"]
    pub seq3: SEQ3,
    #[doc = "0x60 - Sequence Register 4"]
    pub seq4: SEQ4,
    _reserved19: [u8; 28usize],
    #[doc = "0x80 - DATA0 Register Access"]
    pub data0: DATA0,
    #[doc = "0x84 - DATA1 Register Access"]
    pub data1: DATA1,
    #[doc = "0x88 - DATA2 Register Access"]
    pub data2: DATA2,
    #[doc = "0x8c - DATA3 Register Access"]
    pub data3: DATA3,
    _reserved23: [u8; 16usize],
    #[doc = "0xa0 - DATA0XOR Register Access"]
    pub data0xor: DATA0XOR,
    _reserved24: [u8; 12usize],
    #[doc = "0xb0 - DATA0 Register Byte Access"]
    pub data0byte: DATA0BYTE,
    #[doc = "0xb4 - DATA1 Register Byte Access"]
    pub data1byte: DATA1BYTE,
    _reserved26: [u8; 4usize],
    #[doc = "0xbc - DATA0 Register Byte XOR Access"]
    pub data0xorbyte: DATA0XORBYTE,
    #[doc = "0xc0 - DATA0 Register Byte 12 Access"]
    pub data0byte12: DATA0BYTE12,
    #[doc = "0xc4 - DATA0 Register Byte 13 Access"]
    pub data0byte13: DATA0BYTE13,
    #[doc = "0xc8 - DATA0 Register Byte 14 Access"]
    pub data0byte14: DATA0BYTE14,
    #[doc = "0xcc - DATA0 Register Byte 15 Access"]
    pub data0byte15: DATA0BYTE15,
    _reserved31: [u8; 48usize],
    #[doc = "0x100 - DDATA0 Register Access"]
    pub ddata0: DDATA0,
    #[doc = "0x104 - DDATA1 Register Access"]
    pub ddata1: DDATA1,
    #[doc = "0x108 - DDATA2 Register Access"]
    pub ddata2: DDATA2,
    #[doc = "0x10c - DDATA3 Register Access"]
    pub ddata3: DDATA3,
    #[doc = "0x110 - DDATA4 Register Access"]
    pub ddata4: DDATA4,
    _reserved36: [u8; 28usize],
    #[doc = "0x130 - DDATA0 Register Big Endian Access"]
    pub ddata0big: DDATA0BIG,
    _reserved37: [u8; 12usize],
    #[doc = "0x140 - DDATA0 Register Byte Access"]
    pub ddata0byte: DDATA0BYTE,
    #[doc = "0x144 - DDATA1 Register Byte Access"]
    pub ddata1byte: DDATA1BYTE,
    #[doc = "0x148 - DDATA0 Register Byte 32 Access"]
    pub ddata0byte32: DDATA0BYTE32,
    _reserved40: [u8; 52usize],
    #[doc = "0x180 - QDATA0 Register Access"]
    pub qdata0: QDATA0,
    #[doc = "0x184 - QDATA1 Register Access"]
    pub qdata1: QDATA1,
    _reserved42: [u8; 28usize],
    #[doc = "0x1a4 - QDATA1 Register Big Endian Access"]
    pub qdata1big: QDATA1BIG,
    _reserved43: [u8; 24usize],
    #[doc = "0x1c0 - QDATA0 Register Byte Access"]
    pub qdata0byte: QDATA0BYTE,
    #[doc = "0x1c4 - QDATA1 Register Byte Access"]
    pub qdata1byte: QDATA1BYTE,
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
#[doc = "Wide Arithmetic Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [wac](wac) module"]
pub type WAC = crate::Reg<u32, _WAC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WAC;
#[doc = "`read()` method returns [wac::R](wac::R) reader structure"]
impl crate::Readable for WAC {}
#[doc = "`write(|w| ..)` method takes [wac::W](wac::W) writer structure"]
impl crate::Writable for WAC {}
#[doc = "Wide Arithmetic Configuration"]
pub mod wac;
#[doc = "Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`read()` method returns [cmd::R](cmd::R) reader structure"]
impl crate::Readable for CMD {}
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "Command Register"]
pub mod cmd;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status Register"]
pub mod status;
#[doc = "Data Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [dstatus](dstatus) module"]
pub type DSTATUS = crate::Reg<u32, _DSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DSTATUS;
#[doc = "`read()` method returns [dstatus::R](dstatus::R) reader structure"]
impl crate::Readable for DSTATUS {}
#[doc = "Data Status Register"]
pub mod dstatus;
#[doc = "Control Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cstatus](cstatus) module"]
pub type CSTATUS = crate::Reg<u32, _CSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSTATUS;
#[doc = "`read()` method returns [cstatus::R](cstatus::R) reader structure"]
impl crate::Readable for CSTATUS {}
#[doc = "Control Status Register"]
pub mod cstatus;
#[doc = "KEY Register Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [key](key) module"]
pub type KEY = crate::Reg<u32, _KEY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEY;
#[doc = "`read()` method returns [key::R](key::R) reader structure"]
impl crate::Readable for KEY {}
#[doc = "`write(|w| ..)` method takes [key::W](key::W) writer structure"]
impl crate::Writable for KEY {}
#[doc = "KEY Register Access"]
pub mod key;
#[doc = "KEY Buffer Register Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [keybuf](keybuf) module"]
pub type KEYBUF = crate::Reg<u32, _KEYBUF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _KEYBUF;
#[doc = "`read()` method returns [keybuf::R](keybuf::R) reader structure"]
impl crate::Readable for KEYBUF {}
#[doc = "`write(|w| ..)` method takes [keybuf::W](keybuf::W) writer structure"]
impl crate::Writable for KEYBUF {}
#[doc = "KEY Buffer Register Access"]
pub mod keybuf;
#[doc = "Sequence Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [seqctrl](seqctrl) module"]
pub type SEQCTRL = crate::Reg<u32, _SEQCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQCTRL;
#[doc = "`read()` method returns [seqctrl::R](seqctrl::R) reader structure"]
impl crate::Readable for SEQCTRL {}
#[doc = "`write(|w| ..)` method takes [seqctrl::W](seqctrl::W) writer structure"]
impl crate::Writable for SEQCTRL {}
#[doc = "Sequence Control"]
pub mod seqctrl;
#[doc = "Sequence Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [seqctrlb](seqctrlb) module"]
pub type SEQCTRLB = crate::Reg<u32, _SEQCTRLB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQCTRLB;
#[doc = "`read()` method returns [seqctrlb::R](seqctrlb::R) reader structure"]
impl crate::Readable for SEQCTRLB {}
#[doc = "`write(|w| ..)` method takes [seqctrlb::W](seqctrlb::W) writer structure"]
impl crate::Writable for SEQCTRLB {}
#[doc = "Sequence Control B"]
pub mod seqctrlb;
#[doc = "AES Interrupt Flags\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [if_](if_) module"]
pub type IF = crate::Reg<u32, _IF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IF;
#[doc = "`read()` method returns [if_::R](if_::R) reader structure"]
impl crate::Readable for IF {}
#[doc = "AES Interrupt Flags"]
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
#[doc = "Sequence Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [seq0](seq0) module"]
pub type SEQ0 = crate::Reg<u32, _SEQ0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQ0;
#[doc = "`read()` method returns [seq0::R](seq0::R) reader structure"]
impl crate::Readable for SEQ0 {}
#[doc = "`write(|w| ..)` method takes [seq0::W](seq0::W) writer structure"]
impl crate::Writable for SEQ0 {}
#[doc = "Sequence Register 0"]
pub mod seq0;
#[doc = "Sequence Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [seq1](seq1) module"]
pub type SEQ1 = crate::Reg<u32, _SEQ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQ1;
#[doc = "`read()` method returns [seq1::R](seq1::R) reader structure"]
impl crate::Readable for SEQ1 {}
#[doc = "`write(|w| ..)` method takes [seq1::W](seq1::W) writer structure"]
impl crate::Writable for SEQ1 {}
#[doc = "Sequence Register 1"]
pub mod seq1;
#[doc = "Sequence Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [seq2](seq2) module"]
pub type SEQ2 = crate::Reg<u32, _SEQ2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQ2;
#[doc = "`read()` method returns [seq2::R](seq2::R) reader structure"]
impl crate::Readable for SEQ2 {}
#[doc = "`write(|w| ..)` method takes [seq2::W](seq2::W) writer structure"]
impl crate::Writable for SEQ2 {}
#[doc = "Sequence Register 2"]
pub mod seq2;
#[doc = "Sequence Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [seq3](seq3) module"]
pub type SEQ3 = crate::Reg<u32, _SEQ3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQ3;
#[doc = "`read()` method returns [seq3::R](seq3::R) reader structure"]
impl crate::Readable for SEQ3 {}
#[doc = "`write(|w| ..)` method takes [seq3::W](seq3::W) writer structure"]
impl crate::Writable for SEQ3 {}
#[doc = "Sequence Register 3"]
pub mod seq3;
#[doc = "Sequence Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [seq4](seq4) module"]
pub type SEQ4 = crate::Reg<u32, _SEQ4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SEQ4;
#[doc = "`read()` method returns [seq4::R](seq4::R) reader structure"]
impl crate::Readable for SEQ4 {}
#[doc = "`write(|w| ..)` method takes [seq4::W](seq4::W) writer structure"]
impl crate::Writable for SEQ4 {}
#[doc = "Sequence Register 4"]
pub mod seq4;
#[doc = "DATA0 Register Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data0](data0) module"]
pub type DATA0 = crate::Reg<u32, _DATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA0;
#[doc = "`read()` method returns [data0::R](data0::R) reader structure"]
impl crate::Readable for DATA0 {}
#[doc = "`write(|w| ..)` method takes [data0::W](data0::W) writer structure"]
impl crate::Writable for DATA0 {}
#[doc = "DATA0 Register Access"]
pub mod data0;
#[doc = "DATA1 Register Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data1](data1) module"]
pub type DATA1 = crate::Reg<u32, _DATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA1;
#[doc = "`read()` method returns [data1::R](data1::R) reader structure"]
impl crate::Readable for DATA1 {}
#[doc = "`write(|w| ..)` method takes [data1::W](data1::W) writer structure"]
impl crate::Writable for DATA1 {}
#[doc = "DATA1 Register Access"]
pub mod data1;
#[doc = "DATA2 Register Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data2](data2) module"]
pub type DATA2 = crate::Reg<u32, _DATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA2;
#[doc = "`read()` method returns [data2::R](data2::R) reader structure"]
impl crate::Readable for DATA2 {}
#[doc = "`write(|w| ..)` method takes [data2::W](data2::W) writer structure"]
impl crate::Writable for DATA2 {}
#[doc = "DATA2 Register Access"]
pub mod data2;
#[doc = "DATA3 Register Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data3](data3) module"]
pub type DATA3 = crate::Reg<u32, _DATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA3;
#[doc = "`read()` method returns [data3::R](data3::R) reader structure"]
impl crate::Readable for DATA3 {}
#[doc = "`write(|w| ..)` method takes [data3::W](data3::W) writer structure"]
impl crate::Writable for DATA3 {}
#[doc = "DATA3 Register Access"]
pub mod data3;
#[doc = "DATA0XOR Register Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data0xor](data0xor) module"]
pub type DATA0XOR = crate::Reg<u32, _DATA0XOR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA0XOR;
#[doc = "`read()` method returns [data0xor::R](data0xor::R) reader structure"]
impl crate::Readable for DATA0XOR {}
#[doc = "`write(|w| ..)` method takes [data0xor::W](data0xor::W) writer structure"]
impl crate::Writable for DATA0XOR {}
#[doc = "DATA0XOR Register Access"]
pub mod data0xor;
#[doc = "DATA0 Register Byte Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data0byte](data0byte) module"]
pub type DATA0BYTE = crate::Reg<u32, _DATA0BYTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA0BYTE;
#[doc = "`read()` method returns [data0byte::R](data0byte::R) reader structure"]
impl crate::Readable for DATA0BYTE {}
#[doc = "`write(|w| ..)` method takes [data0byte::W](data0byte::W) writer structure"]
impl crate::Writable for DATA0BYTE {}
#[doc = "DATA0 Register Byte Access"]
pub mod data0byte;
#[doc = "DATA1 Register Byte Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data1byte](data1byte) module"]
pub type DATA1BYTE = crate::Reg<u32, _DATA1BYTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA1BYTE;
#[doc = "`read()` method returns [data1byte::R](data1byte::R) reader structure"]
impl crate::Readable for DATA1BYTE {}
#[doc = "`write(|w| ..)` method takes [data1byte::W](data1byte::W) writer structure"]
impl crate::Writable for DATA1BYTE {}
#[doc = "DATA1 Register Byte Access"]
pub mod data1byte;
#[doc = "DATA0 Register Byte XOR Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data0xorbyte](data0xorbyte) module"]
pub type DATA0XORBYTE = crate::Reg<u32, _DATA0XORBYTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA0XORBYTE;
#[doc = "`read()` method returns [data0xorbyte::R](data0xorbyte::R) reader structure"]
impl crate::Readable for DATA0XORBYTE {}
#[doc = "`write(|w| ..)` method takes [data0xorbyte::W](data0xorbyte::W) writer structure"]
impl crate::Writable for DATA0XORBYTE {}
#[doc = "DATA0 Register Byte XOR Access"]
pub mod data0xorbyte;
#[doc = "DATA0 Register Byte 12 Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data0byte12](data0byte12) module"]
pub type DATA0BYTE12 = crate::Reg<u32, _DATA0BYTE12>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA0BYTE12;
#[doc = "`read()` method returns [data0byte12::R](data0byte12::R) reader structure"]
impl crate::Readable for DATA0BYTE12 {}
#[doc = "`write(|w| ..)` method takes [data0byte12::W](data0byte12::W) writer structure"]
impl crate::Writable for DATA0BYTE12 {}
#[doc = "DATA0 Register Byte 12 Access"]
pub mod data0byte12;
#[doc = "DATA0 Register Byte 13 Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data0byte13](data0byte13) module"]
pub type DATA0BYTE13 = crate::Reg<u32, _DATA0BYTE13>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA0BYTE13;
#[doc = "`read()` method returns [data0byte13::R](data0byte13::R) reader structure"]
impl crate::Readable for DATA0BYTE13 {}
#[doc = "`write(|w| ..)` method takes [data0byte13::W](data0byte13::W) writer structure"]
impl crate::Writable for DATA0BYTE13 {}
#[doc = "DATA0 Register Byte 13 Access"]
pub mod data0byte13;
#[doc = "DATA0 Register Byte 14 Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data0byte14](data0byte14) module"]
pub type DATA0BYTE14 = crate::Reg<u32, _DATA0BYTE14>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA0BYTE14;
#[doc = "`read()` method returns [data0byte14::R](data0byte14::R) reader structure"]
impl crate::Readable for DATA0BYTE14 {}
#[doc = "`write(|w| ..)` method takes [data0byte14::W](data0byte14::W) writer structure"]
impl crate::Writable for DATA0BYTE14 {}
#[doc = "DATA0 Register Byte 14 Access"]
pub mod data0byte14;
#[doc = "DATA0 Register Byte 15 Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [data0byte15](data0byte15) module"]
pub type DATA0BYTE15 = crate::Reg<u32, _DATA0BYTE15>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DATA0BYTE15;
#[doc = "`read()` method returns [data0byte15::R](data0byte15::R) reader structure"]
impl crate::Readable for DATA0BYTE15 {}
#[doc = "`write(|w| ..)` method takes [data0byte15::W](data0byte15::W) writer structure"]
impl crate::Writable for DATA0BYTE15 {}
#[doc = "DATA0 Register Byte 15 Access"]
pub mod data0byte15;
#[doc = "DDATA0 Register Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ddata0](ddata0) module"]
pub type DDATA0 = crate::Reg<u32, _DDATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDATA0;
#[doc = "`read()` method returns [ddata0::R](ddata0::R) reader structure"]
impl crate::Readable for DDATA0 {}
#[doc = "`write(|w| ..)` method takes [ddata0::W](ddata0::W) writer structure"]
impl crate::Writable for DDATA0 {}
#[doc = "DDATA0 Register Access"]
pub mod ddata0;
#[doc = "DDATA1 Register Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ddata1](ddata1) module"]
pub type DDATA1 = crate::Reg<u32, _DDATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDATA1;
#[doc = "`read()` method returns [ddata1::R](ddata1::R) reader structure"]
impl crate::Readable for DDATA1 {}
#[doc = "`write(|w| ..)` method takes [ddata1::W](ddata1::W) writer structure"]
impl crate::Writable for DDATA1 {}
#[doc = "DDATA1 Register Access"]
pub mod ddata1;
#[doc = "DDATA2 Register Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ddata2](ddata2) module"]
pub type DDATA2 = crate::Reg<u32, _DDATA2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDATA2;
#[doc = "`read()` method returns [ddata2::R](ddata2::R) reader structure"]
impl crate::Readable for DDATA2 {}
#[doc = "`write(|w| ..)` method takes [ddata2::W](ddata2::W) writer structure"]
impl crate::Writable for DDATA2 {}
#[doc = "DDATA2 Register Access"]
pub mod ddata2;
#[doc = "DDATA3 Register Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ddata3](ddata3) module"]
pub type DDATA3 = crate::Reg<u32, _DDATA3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDATA3;
#[doc = "`read()` method returns [ddata3::R](ddata3::R) reader structure"]
impl crate::Readable for DDATA3 {}
#[doc = "`write(|w| ..)` method takes [ddata3::W](ddata3::W) writer structure"]
impl crate::Writable for DDATA3 {}
#[doc = "DDATA3 Register Access"]
pub mod ddata3;
#[doc = "DDATA4 Register Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ddata4](ddata4) module"]
pub type DDATA4 = crate::Reg<u32, _DDATA4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDATA4;
#[doc = "`read()` method returns [ddata4::R](ddata4::R) reader structure"]
impl crate::Readable for DDATA4 {}
#[doc = "`write(|w| ..)` method takes [ddata4::W](ddata4::W) writer structure"]
impl crate::Writable for DDATA4 {}
#[doc = "DDATA4 Register Access"]
pub mod ddata4;
#[doc = "DDATA0 Register Big Endian Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ddata0big](ddata0big) module"]
pub type DDATA0BIG = crate::Reg<u32, _DDATA0BIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDATA0BIG;
#[doc = "`read()` method returns [ddata0big::R](ddata0big::R) reader structure"]
impl crate::Readable for DDATA0BIG {}
#[doc = "`write(|w| ..)` method takes [ddata0big::W](ddata0big::W) writer structure"]
impl crate::Writable for DDATA0BIG {}
#[doc = "DDATA0 Register Big Endian Access"]
pub mod ddata0big;
#[doc = "DDATA0 Register Byte Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ddata0byte](ddata0byte) module"]
pub type DDATA0BYTE = crate::Reg<u32, _DDATA0BYTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDATA0BYTE;
#[doc = "`read()` method returns [ddata0byte::R](ddata0byte::R) reader structure"]
impl crate::Readable for DDATA0BYTE {}
#[doc = "`write(|w| ..)` method takes [ddata0byte::W](ddata0byte::W) writer structure"]
impl crate::Writable for DDATA0BYTE {}
#[doc = "DDATA0 Register Byte Access"]
pub mod ddata0byte;
#[doc = "DDATA1 Register Byte Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ddata1byte](ddata1byte) module"]
pub type DDATA1BYTE = crate::Reg<u32, _DDATA1BYTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDATA1BYTE;
#[doc = "`read()` method returns [ddata1byte::R](ddata1byte::R) reader structure"]
impl crate::Readable for DDATA1BYTE {}
#[doc = "`write(|w| ..)` method takes [ddata1byte::W](ddata1byte::W) writer structure"]
impl crate::Writable for DDATA1BYTE {}
#[doc = "DDATA1 Register Byte Access"]
pub mod ddata1byte;
#[doc = "DDATA0 Register Byte 32 Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ddata0byte32](ddata0byte32) module"]
pub type DDATA0BYTE32 = crate::Reg<u32, _DDATA0BYTE32>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DDATA0BYTE32;
#[doc = "`read()` method returns [ddata0byte32::R](ddata0byte32::R) reader structure"]
impl crate::Readable for DDATA0BYTE32 {}
#[doc = "`write(|w| ..)` method takes [ddata0byte32::W](ddata0byte32::W) writer structure"]
impl crate::Writable for DDATA0BYTE32 {}
#[doc = "DDATA0 Register Byte 32 Access"]
pub mod ddata0byte32;
#[doc = "QDATA0 Register Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qdata0](qdata0) module"]
pub type QDATA0 = crate::Reg<u32, _QDATA0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QDATA0;
#[doc = "`read()` method returns [qdata0::R](qdata0::R) reader structure"]
impl crate::Readable for QDATA0 {}
#[doc = "`write(|w| ..)` method takes [qdata0::W](qdata0::W) writer structure"]
impl crate::Writable for QDATA0 {}
#[doc = "QDATA0 Register Access"]
pub mod qdata0;
#[doc = "QDATA1 Register Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qdata1](qdata1) module"]
pub type QDATA1 = crate::Reg<u32, _QDATA1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QDATA1;
#[doc = "`read()` method returns [qdata1::R](qdata1::R) reader structure"]
impl crate::Readable for QDATA1 {}
#[doc = "`write(|w| ..)` method takes [qdata1::W](qdata1::W) writer structure"]
impl crate::Writable for QDATA1 {}
#[doc = "QDATA1 Register Access"]
pub mod qdata1;
#[doc = "QDATA1 Register Big Endian Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qdata1big](qdata1big) module"]
pub type QDATA1BIG = crate::Reg<u32, _QDATA1BIG>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QDATA1BIG;
#[doc = "`read()` method returns [qdata1big::R](qdata1big::R) reader structure"]
impl crate::Readable for QDATA1BIG {}
#[doc = "`write(|w| ..)` method takes [qdata1big::W](qdata1big::W) writer structure"]
impl crate::Writable for QDATA1BIG {}
#[doc = "QDATA1 Register Big Endian Access"]
pub mod qdata1big;
#[doc = "QDATA0 Register Byte Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qdata0byte](qdata0byte) module"]
pub type QDATA0BYTE = crate::Reg<u32, _QDATA0BYTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QDATA0BYTE;
#[doc = "`read()` method returns [qdata0byte::R](qdata0byte::R) reader structure"]
impl crate::Readable for QDATA0BYTE {}
#[doc = "`write(|w| ..)` method takes [qdata0byte::W](qdata0byte::W) writer structure"]
impl crate::Writable for QDATA0BYTE {}
#[doc = "QDATA0 Register Byte Access"]
pub mod qdata0byte;
#[doc = "QDATA1 Register Byte Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [qdata1byte](qdata1byte) module"]
pub type QDATA1BYTE = crate::Reg<u32, _QDATA1BYTE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _QDATA1BYTE;
#[doc = "`read()` method returns [qdata1byte::R](qdata1byte::R) reader structure"]
impl crate::Readable for QDATA1BYTE {}
#[doc = "`write(|w| ..)` method takes [qdata1byte::W](qdata1byte::W) writer structure"]
impl crate::Writable for QDATA1BYTE {}
#[doc = "QDATA1 Register Byte Access"]
pub mod qdata1byte;
