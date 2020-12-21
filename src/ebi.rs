#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - Address Timing Register"]
    pub addrtiming: ADDRTIMING,
    #[doc = "0x08 - Read Timing Register"]
    pub rdtiming: RDTIMING,
    #[doc = "0x0c - Write Timing Register"]
    pub wrtiming: WRTIMING,
    #[doc = "0x10 - Polarity Register"]
    pub polarity: POLARITY,
    _reserved5: [u8; 4usize],
    #[doc = "0x18 - Address Timing Register 1"]
    pub addrtiming1: ADDRTIMING1,
    #[doc = "0x1c - Read Timing Register 1"]
    pub rdtiming1: RDTIMING1,
    #[doc = "0x20 - Write Timing Register 1"]
    pub wrtiming1: WRTIMING1,
    #[doc = "0x24 - Polarity Register 1"]
    pub polarity1: POLARITY1,
    #[doc = "0x28 - Address Timing Register 2"]
    pub addrtiming2: ADDRTIMING2,
    #[doc = "0x2c - Read Timing Register 2"]
    pub rdtiming2: RDTIMING2,
    #[doc = "0x30 - Write Timing Register 2"]
    pub wrtiming2: WRTIMING2,
    #[doc = "0x34 - Polarity Register 2"]
    pub polarity2: POLARITY2,
    #[doc = "0x38 - Address Timing Register 3"]
    pub addrtiming3: ADDRTIMING3,
    #[doc = "0x3c - Read Timing Register 3"]
    pub rdtiming3: RDTIMING3,
    #[doc = "0x40 - Write Timing Register 3"]
    pub wrtiming3: WRTIMING3,
    #[doc = "0x44 - Polarity Register 3"]
    pub polarity3: POLARITY3,
    #[doc = "0x48 - Page Control Register"]
    pub pagectrl: PAGECTRL,
    #[doc = "0x4c - NAND Control Register"]
    pub nandctrl: NANDCTRL,
    #[doc = "0x50 - Command Register"]
    pub cmd: CMD,
    #[doc = "0x54 - Status Register"]
    pub status: STATUS,
    #[doc = "0x58 - ECC Parity Register"]
    pub eccparity: ECCPARITY,
    #[doc = "0x5c - TFT Control Register"]
    pub tftctrl: TFTCTRL,
    #[doc = "0x60 - TFT Status Register"]
    pub tftstatus: TFTSTATUS,
    #[doc = "0x64 - Color Format Register"]
    pub tftcolorformat: TFTCOLORFORMAT,
    #[doc = "0x68 - TFT Frame Base Register"]
    pub tftframebase: TFTFRAMEBASE,
    _reserved26: [u8; 4usize],
    #[doc = "0x70 - TFT Stride Register"]
    pub tftstride: TFTSTRIDE,
    #[doc = "0x74 - TFT Size Register"]
    pub tftsize: TFTSIZE,
    #[doc = "0x78 - TFT Horizontal Porch Register"]
    pub tfthporch: TFTHPORCH,
    #[doc = "0x7c - TFT Vertical Porch Register"]
    pub tftvporch: TFTVPORCH,
    #[doc = "0x80 - TFT Timing Register"]
    pub tfttiming: TFTTIMING,
    #[doc = "0x84 - TFT Polarity Register"]
    pub tftpolarity: TFTPOLARITY,
    #[doc = "0x88 - TFT Direct Drive Data Register"]
    pub tftdd: TFTDD,
    #[doc = "0x8c - TFT Alpha Blending Register"]
    pub tftalpha: TFTALPHA,
    #[doc = "0x90 - TFT Pixel 0 Register"]
    pub tftpixel0: TFTPIXEL0,
    #[doc = "0x94 - TFT Pixel 1 Register"]
    pub tftpixel1: TFTPIXEL1,
    #[doc = "0x98 - TFT Alpha Blending Result Pixel Register"]
    pub tftpixel: TFTPIXEL,
    #[doc = "0x9c - TFT Masking Register"]
    pub tftmask: TFTMASK,
    #[doc = "0xa0 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0xa4 - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0xa8 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0xac - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0xb0 - I/O Routing Register"]
    pub routepen: ROUTEPEN,
    #[doc = "0xb4 - I/O Routing Location Register"]
    pub routeloc0: ROUTELOC0,
    #[doc = "0xb8 - I/O Routing Location Register"]
    pub routeloc1: ROUTELOC1,
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](ctrl) module"]
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
#[doc = "Address Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addrtiming](addrtiming) module"]
pub type ADDRTIMING = crate::Reg<u32, _ADDRTIMING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDRTIMING;
#[doc = "`read()` method returns [addrtiming::R](addrtiming::R) reader structure"]
impl crate::Readable for ADDRTIMING {}
#[doc = "`write(|w| ..)` method takes [addrtiming::W](addrtiming::W) writer structure"]
impl crate::Writable for ADDRTIMING {}
#[doc = "Address Timing Register"]
pub mod addrtiming;
#[doc = "Read Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdtiming](rdtiming) module"]
pub type RDTIMING = crate::Reg<u32, _RDTIMING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDTIMING;
#[doc = "`read()` method returns [rdtiming::R](rdtiming::R) reader structure"]
impl crate::Readable for RDTIMING {}
#[doc = "`write(|w| ..)` method takes [rdtiming::W](rdtiming::W) writer structure"]
impl crate::Writable for RDTIMING {}
#[doc = "Read Timing Register"]
pub mod rdtiming;
#[doc = "Write Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrtiming](wrtiming) module"]
pub type WRTIMING = crate::Reg<u32, _WRTIMING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRTIMING;
#[doc = "`read()` method returns [wrtiming::R](wrtiming::R) reader structure"]
impl crate::Readable for WRTIMING {}
#[doc = "`write(|w| ..)` method takes [wrtiming::W](wrtiming::W) writer structure"]
impl crate::Writable for WRTIMING {}
#[doc = "Write Timing Register"]
pub mod wrtiming;
#[doc = "Polarity Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [polarity](polarity) module"]
pub type POLARITY = crate::Reg<u32, _POLARITY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POLARITY;
#[doc = "`read()` method returns [polarity::R](polarity::R) reader structure"]
impl crate::Readable for POLARITY {}
#[doc = "`write(|w| ..)` method takes [polarity::W](polarity::W) writer structure"]
impl crate::Writable for POLARITY {}
#[doc = "Polarity Register"]
pub mod polarity;
#[doc = "Address Timing Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addrtiming1](addrtiming1) module"]
pub type ADDRTIMING1 = crate::Reg<u32, _ADDRTIMING1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDRTIMING1;
#[doc = "`read()` method returns [addrtiming1::R](addrtiming1::R) reader structure"]
impl crate::Readable for ADDRTIMING1 {}
#[doc = "`write(|w| ..)` method takes [addrtiming1::W](addrtiming1::W) writer structure"]
impl crate::Writable for ADDRTIMING1 {}
#[doc = "Address Timing Register 1"]
pub mod addrtiming1;
#[doc = "Read Timing Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdtiming1](rdtiming1) module"]
pub type RDTIMING1 = crate::Reg<u32, _RDTIMING1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDTIMING1;
#[doc = "`read()` method returns [rdtiming1::R](rdtiming1::R) reader structure"]
impl crate::Readable for RDTIMING1 {}
#[doc = "`write(|w| ..)` method takes [rdtiming1::W](rdtiming1::W) writer structure"]
impl crate::Writable for RDTIMING1 {}
#[doc = "Read Timing Register 1"]
pub mod rdtiming1;
#[doc = "Write Timing Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrtiming1](wrtiming1) module"]
pub type WRTIMING1 = crate::Reg<u32, _WRTIMING1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRTIMING1;
#[doc = "`read()` method returns [wrtiming1::R](wrtiming1::R) reader structure"]
impl crate::Readable for WRTIMING1 {}
#[doc = "`write(|w| ..)` method takes [wrtiming1::W](wrtiming1::W) writer structure"]
impl crate::Writable for WRTIMING1 {}
#[doc = "Write Timing Register 1"]
pub mod wrtiming1;
#[doc = "Polarity Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [polarity1](polarity1) module"]
pub type POLARITY1 = crate::Reg<u32, _POLARITY1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POLARITY1;
#[doc = "`read()` method returns [polarity1::R](polarity1::R) reader structure"]
impl crate::Readable for POLARITY1 {}
#[doc = "`write(|w| ..)` method takes [polarity1::W](polarity1::W) writer structure"]
impl crate::Writable for POLARITY1 {}
#[doc = "Polarity Register 1"]
pub mod polarity1;
#[doc = "Address Timing Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addrtiming2](addrtiming2) module"]
pub type ADDRTIMING2 = crate::Reg<u32, _ADDRTIMING2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDRTIMING2;
#[doc = "`read()` method returns [addrtiming2::R](addrtiming2::R) reader structure"]
impl crate::Readable for ADDRTIMING2 {}
#[doc = "`write(|w| ..)` method takes [addrtiming2::W](addrtiming2::W) writer structure"]
impl crate::Writable for ADDRTIMING2 {}
#[doc = "Address Timing Register 2"]
pub mod addrtiming2;
#[doc = "Read Timing Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdtiming2](rdtiming2) module"]
pub type RDTIMING2 = crate::Reg<u32, _RDTIMING2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDTIMING2;
#[doc = "`read()` method returns [rdtiming2::R](rdtiming2::R) reader structure"]
impl crate::Readable for RDTIMING2 {}
#[doc = "`write(|w| ..)` method takes [rdtiming2::W](rdtiming2::W) writer structure"]
impl crate::Writable for RDTIMING2 {}
#[doc = "Read Timing Register 2"]
pub mod rdtiming2;
#[doc = "Write Timing Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrtiming2](wrtiming2) module"]
pub type WRTIMING2 = crate::Reg<u32, _WRTIMING2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRTIMING2;
#[doc = "`read()` method returns [wrtiming2::R](wrtiming2::R) reader structure"]
impl crate::Readable for WRTIMING2 {}
#[doc = "`write(|w| ..)` method takes [wrtiming2::W](wrtiming2::W) writer structure"]
impl crate::Writable for WRTIMING2 {}
#[doc = "Write Timing Register 2"]
pub mod wrtiming2;
#[doc = "Polarity Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [polarity2](polarity2) module"]
pub type POLARITY2 = crate::Reg<u32, _POLARITY2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POLARITY2;
#[doc = "`read()` method returns [polarity2::R](polarity2::R) reader structure"]
impl crate::Readable for POLARITY2 {}
#[doc = "`write(|w| ..)` method takes [polarity2::W](polarity2::W) writer structure"]
impl crate::Writable for POLARITY2 {}
#[doc = "Polarity Register 2"]
pub mod polarity2;
#[doc = "Address Timing Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addrtiming3](addrtiming3) module"]
pub type ADDRTIMING3 = crate::Reg<u32, _ADDRTIMING3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADDRTIMING3;
#[doc = "`read()` method returns [addrtiming3::R](addrtiming3::R) reader structure"]
impl crate::Readable for ADDRTIMING3 {}
#[doc = "`write(|w| ..)` method takes [addrtiming3::W](addrtiming3::W) writer structure"]
impl crate::Writable for ADDRTIMING3 {}
#[doc = "Address Timing Register 3"]
pub mod addrtiming3;
#[doc = "Read Timing Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdtiming3](rdtiming3) module"]
pub type RDTIMING3 = crate::Reg<u32, _RDTIMING3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDTIMING3;
#[doc = "`read()` method returns [rdtiming3::R](rdtiming3::R) reader structure"]
impl crate::Readable for RDTIMING3 {}
#[doc = "`write(|w| ..)` method takes [rdtiming3::W](rdtiming3::W) writer structure"]
impl crate::Writable for RDTIMING3 {}
#[doc = "Read Timing Register 3"]
pub mod rdtiming3;
#[doc = "Write Timing Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrtiming3](wrtiming3) module"]
pub type WRTIMING3 = crate::Reg<u32, _WRTIMING3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WRTIMING3;
#[doc = "`read()` method returns [wrtiming3::R](wrtiming3::R) reader structure"]
impl crate::Readable for WRTIMING3 {}
#[doc = "`write(|w| ..)` method takes [wrtiming3::W](wrtiming3::W) writer structure"]
impl crate::Writable for WRTIMING3 {}
#[doc = "Write Timing Register 3"]
pub mod wrtiming3;
#[doc = "Polarity Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [polarity3](polarity3) module"]
pub type POLARITY3 = crate::Reg<u32, _POLARITY3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POLARITY3;
#[doc = "`read()` method returns [polarity3::R](polarity3::R) reader structure"]
impl crate::Readable for POLARITY3 {}
#[doc = "`write(|w| ..)` method takes [polarity3::W](polarity3::W) writer structure"]
impl crate::Writable for POLARITY3 {}
#[doc = "Polarity Register 3"]
pub mod polarity3;
#[doc = "Page Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pagectrl](pagectrl) module"]
pub type PAGECTRL = crate::Reg<u32, _PAGECTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PAGECTRL;
#[doc = "`read()` method returns [pagectrl::R](pagectrl::R) reader structure"]
impl crate::Readable for PAGECTRL {}
#[doc = "`write(|w| ..)` method takes [pagectrl::W](pagectrl::W) writer structure"]
impl crate::Writable for PAGECTRL {}
#[doc = "Page Control Register"]
pub mod pagectrl;
#[doc = "NAND Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nandctrl](nandctrl) module"]
pub type NANDCTRL = crate::Reg<u32, _NANDCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NANDCTRL;
#[doc = "`read()` method returns [nandctrl::R](nandctrl::R) reader structure"]
impl crate::Readable for NANDCTRL {}
#[doc = "`write(|w| ..)` method takes [nandctrl::W](nandctrl::W) writer structure"]
impl crate::Writable for NANDCTRL {}
#[doc = "NAND Control Register"]
pub mod nandctrl;
#[doc = "Command Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](cmd) module"]
pub type CMD = crate::Reg<u32, _CMD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMD;
#[doc = "`write(|w| ..)` method takes [cmd::W](cmd::W) writer structure"]
impl crate::Writable for CMD {}
#[doc = "Command Register"]
pub mod cmd;
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](status) module"]
pub type STATUS = crate::Reg<u32, _STATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _STATUS;
#[doc = "`read()` method returns [status::R](status::R) reader structure"]
impl crate::Readable for STATUS {}
#[doc = "Status Register"]
pub mod status;
#[doc = "ECC Parity Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eccparity](eccparity) module"]
pub type ECCPARITY = crate::Reg<u32, _ECCPARITY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ECCPARITY;
#[doc = "`read()` method returns [eccparity::R](eccparity::R) reader structure"]
impl crate::Readable for ECCPARITY {}
#[doc = "ECC Parity Register"]
pub mod eccparity;
#[doc = "TFT Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tftctrl](tftctrl) module"]
pub type TFTCTRL = crate::Reg<u32, _TFTCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TFTCTRL;
#[doc = "`read()` method returns [tftctrl::R](tftctrl::R) reader structure"]
impl crate::Readable for TFTCTRL {}
#[doc = "`write(|w| ..)` method takes [tftctrl::W](tftctrl::W) writer structure"]
impl crate::Writable for TFTCTRL {}
#[doc = "TFT Control Register"]
pub mod tftctrl;
#[doc = "TFT Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tftstatus](tftstatus) module"]
pub type TFTSTATUS = crate::Reg<u32, _TFTSTATUS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TFTSTATUS;
#[doc = "`read()` method returns [tftstatus::R](tftstatus::R) reader structure"]
impl crate::Readable for TFTSTATUS {}
#[doc = "TFT Status Register"]
pub mod tftstatus;
#[doc = "Color Format Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tftcolorformat](tftcolorformat) module"]
pub type TFTCOLORFORMAT = crate::Reg<u32, _TFTCOLORFORMAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TFTCOLORFORMAT;
#[doc = "`read()` method returns [tftcolorformat::R](tftcolorformat::R) reader structure"]
impl crate::Readable for TFTCOLORFORMAT {}
#[doc = "`write(|w| ..)` method takes [tftcolorformat::W](tftcolorformat::W) writer structure"]
impl crate::Writable for TFTCOLORFORMAT {}
#[doc = "Color Format Register"]
pub mod tftcolorformat;
#[doc = "TFT Frame Base Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tftframebase](tftframebase) module"]
pub type TFTFRAMEBASE = crate::Reg<u32, _TFTFRAMEBASE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TFTFRAMEBASE;
#[doc = "`read()` method returns [tftframebase::R](tftframebase::R) reader structure"]
impl crate::Readable for TFTFRAMEBASE {}
#[doc = "`write(|w| ..)` method takes [tftframebase::W](tftframebase::W) writer structure"]
impl crate::Writable for TFTFRAMEBASE {}
#[doc = "TFT Frame Base Register"]
pub mod tftframebase;
#[doc = "TFT Stride Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tftstride](tftstride) module"]
pub type TFTSTRIDE = crate::Reg<u32, _TFTSTRIDE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TFTSTRIDE;
#[doc = "`read()` method returns [tftstride::R](tftstride::R) reader structure"]
impl crate::Readable for TFTSTRIDE {}
#[doc = "`write(|w| ..)` method takes [tftstride::W](tftstride::W) writer structure"]
impl crate::Writable for TFTSTRIDE {}
#[doc = "TFT Stride Register"]
pub mod tftstride;
#[doc = "TFT Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tftsize](tftsize) module"]
pub type TFTSIZE = crate::Reg<u32, _TFTSIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TFTSIZE;
#[doc = "`read()` method returns [tftsize::R](tftsize::R) reader structure"]
impl crate::Readable for TFTSIZE {}
#[doc = "`write(|w| ..)` method takes [tftsize::W](tftsize::W) writer structure"]
impl crate::Writable for TFTSIZE {}
#[doc = "TFT Size Register"]
pub mod tftsize;
#[doc = "TFT Horizontal Porch Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tfthporch](tfthporch) module"]
pub type TFTHPORCH = crate::Reg<u32, _TFTHPORCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TFTHPORCH;
#[doc = "`read()` method returns [tfthporch::R](tfthporch::R) reader structure"]
impl crate::Readable for TFTHPORCH {}
#[doc = "`write(|w| ..)` method takes [tfthporch::W](tfthporch::W) writer structure"]
impl crate::Writable for TFTHPORCH {}
#[doc = "TFT Horizontal Porch Register"]
pub mod tfthporch;
#[doc = "TFT Vertical Porch Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tftvporch](tftvporch) module"]
pub type TFTVPORCH = crate::Reg<u32, _TFTVPORCH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TFTVPORCH;
#[doc = "`read()` method returns [tftvporch::R](tftvporch::R) reader structure"]
impl crate::Readable for TFTVPORCH {}
#[doc = "`write(|w| ..)` method takes [tftvporch::W](tftvporch::W) writer structure"]
impl crate::Writable for TFTVPORCH {}
#[doc = "TFT Vertical Porch Register"]
pub mod tftvporch;
#[doc = "TFT Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tfttiming](tfttiming) module"]
pub type TFTTIMING = crate::Reg<u32, _TFTTIMING>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TFTTIMING;
#[doc = "`read()` method returns [tfttiming::R](tfttiming::R) reader structure"]
impl crate::Readable for TFTTIMING {}
#[doc = "`write(|w| ..)` method takes [tfttiming::W](tfttiming::W) writer structure"]
impl crate::Writable for TFTTIMING {}
#[doc = "TFT Timing Register"]
pub mod tfttiming;
#[doc = "TFT Polarity Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tftpolarity](tftpolarity) module"]
pub type TFTPOLARITY = crate::Reg<u32, _TFTPOLARITY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TFTPOLARITY;
#[doc = "`read()` method returns [tftpolarity::R](tftpolarity::R) reader structure"]
impl crate::Readable for TFTPOLARITY {}
#[doc = "`write(|w| ..)` method takes [tftpolarity::W](tftpolarity::W) writer structure"]
impl crate::Writable for TFTPOLARITY {}
#[doc = "TFT Polarity Register"]
pub mod tftpolarity;
#[doc = "TFT Direct Drive Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tftdd](tftdd) module"]
pub type TFTDD = crate::Reg<u32, _TFTDD>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TFTDD;
#[doc = "`read()` method returns [tftdd::R](tftdd::R) reader structure"]
impl crate::Readable for TFTDD {}
#[doc = "`write(|w| ..)` method takes [tftdd::W](tftdd::W) writer structure"]
impl crate::Writable for TFTDD {}
#[doc = "TFT Direct Drive Data Register"]
pub mod tftdd;
#[doc = "TFT Alpha Blending Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tftalpha](tftalpha) module"]
pub type TFTALPHA = crate::Reg<u32, _TFTALPHA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TFTALPHA;
#[doc = "`read()` method returns [tftalpha::R](tftalpha::R) reader structure"]
impl crate::Readable for TFTALPHA {}
#[doc = "`write(|w| ..)` method takes [tftalpha::W](tftalpha::W) writer structure"]
impl crate::Writable for TFTALPHA {}
#[doc = "TFT Alpha Blending Register"]
pub mod tftalpha;
#[doc = "TFT Pixel 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tftpixel0](tftpixel0) module"]
pub type TFTPIXEL0 = crate::Reg<u32, _TFTPIXEL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TFTPIXEL0;
#[doc = "`read()` method returns [tftpixel0::R](tftpixel0::R) reader structure"]
impl crate::Readable for TFTPIXEL0 {}
#[doc = "`write(|w| ..)` method takes [tftpixel0::W](tftpixel0::W) writer structure"]
impl crate::Writable for TFTPIXEL0 {}
#[doc = "TFT Pixel 0 Register"]
pub mod tftpixel0;
#[doc = "TFT Pixel 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tftpixel1](tftpixel1) module"]
pub type TFTPIXEL1 = crate::Reg<u32, _TFTPIXEL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TFTPIXEL1;
#[doc = "`read()` method returns [tftpixel1::R](tftpixel1::R) reader structure"]
impl crate::Readable for TFTPIXEL1 {}
#[doc = "`write(|w| ..)` method takes [tftpixel1::W](tftpixel1::W) writer structure"]
impl crate::Writable for TFTPIXEL1 {}
#[doc = "TFT Pixel 1 Register"]
pub mod tftpixel1;
#[doc = "TFT Alpha Blending Result Pixel Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tftpixel](tftpixel) module"]
pub type TFTPIXEL = crate::Reg<u32, _TFTPIXEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TFTPIXEL;
#[doc = "`read()` method returns [tftpixel::R](tftpixel::R) reader structure"]
impl crate::Readable for TFTPIXEL {}
#[doc = "TFT Alpha Blending Result Pixel Register"]
pub mod tftpixel;
#[doc = "TFT Masking Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tftmask](tftmask) module"]
pub type TFTMASK = crate::Reg<u32, _TFTMASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TFTMASK;
#[doc = "`read()` method returns [tftmask::R](tftmask::R) reader structure"]
impl crate::Readable for TFTMASK {}
#[doc = "`write(|w| ..)` method takes [tftmask::W](tftmask::W) writer structure"]
impl crate::Writable for TFTMASK {}
#[doc = "TFT Masking Register"]
pub mod tftmask;
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
#[doc = "I/O Routing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routepen](routepen) module"]
pub type ROUTEPEN = crate::Reg<u32, _ROUTEPEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROUTEPEN;
#[doc = "`read()` method returns [routepen::R](routepen::R) reader structure"]
impl crate::Readable for ROUTEPEN {}
#[doc = "`write(|w| ..)` method takes [routepen::W](routepen::W) writer structure"]
impl crate::Writable for ROUTEPEN {}
#[doc = "I/O Routing Register"]
pub mod routepen;
#[doc = "I/O Routing Location Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routeloc0](routeloc0) module"]
pub type ROUTELOC0 = crate::Reg<u32, _ROUTELOC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROUTELOC0;
#[doc = "`read()` method returns [routeloc0::R](routeloc0::R) reader structure"]
impl crate::Readable for ROUTELOC0 {}
#[doc = "`write(|w| ..)` method takes [routeloc0::W](routeloc0::W) writer structure"]
impl crate::Writable for ROUTELOC0 {}
#[doc = "I/O Routing Location Register"]
pub mod routeloc0;
#[doc = "I/O Routing Location Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routeloc1](routeloc1) module"]
pub type ROUTELOC1 = crate::Reg<u32, _ROUTELOC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROUTELOC1;
#[doc = "`read()` method returns [routeloc1::R](routeloc1::R) reader structure"]
impl crate::Readable for ROUTELOC1 {}
#[doc = "`write(|w| ..)` method takes [routeloc1::W](routeloc1::W) writer structure"]
impl crate::Writable for ROUTELOC1 {}
#[doc = "I/O Routing Location Register"]
pub mod routeloc1;
