#[doc = "Reader of register ETMCCER"]
pub type R = crate::R<u32, super::ETMCCER>;
#[doc = "Reader of field `EXTINPSEL`"]
pub type EXTINPSEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `EXTINPBUS`"]
pub type EXTINPBUS_R = crate::R<u8, u8>;
#[doc = "Reader of field `READREGS`"]
pub type READREGS_R = crate::R<bool, bool>;
#[doc = "Reader of field `DADDRCMP`"]
pub type DADDRCMP_R = crate::R<bool, bool>;
#[doc = "Reader of field `INSTRES`"]
pub type INSTRES_R = crate::R<u8, u8>;
#[doc = "Reader of field `EICEWPNT`"]
pub type EICEWPNT_R = crate::R<u8, u8>;
#[doc = "Reader of field `TEICEWPNT`"]
pub type TEICEWPNT_R = crate::R<bool, bool>;
#[doc = "Reader of field `EICEIMP`"]
pub type EICEIMP_R = crate::R<bool, bool>;
#[doc = "Reader of field `TIMP`"]
pub type TIMP_R = crate::R<bool, bool>;
#[doc = "Reader of field `RFCNT`"]
pub type RFCNT_R = crate::R<bool, bool>;
#[doc = "Reader of field `TENC`"]
pub type TENC_R = crate::R<bool, bool>;
#[doc = "Reader of field `TSIZE`"]
pub type TSIZE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:1 - Extended External Input Selectors"]
    #[inline(always)]
    pub fn extinpsel(&self) -> EXTINPSEL_R {
        EXTINPSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 3:10 - Extended External Input Bus"]
    #[inline(always)]
    pub fn extinpbus(&self) -> EXTINPBUS_R {
        EXTINPBUS_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bit 11 - Readable Registers"]
    #[inline(always)]
    pub fn readregs(&self) -> READREGS_R {
        READREGS_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Data Address comparisons"]
    #[inline(always)]
    pub fn daddrcmp(&self) -> DADDRCMP_R {
        DADDRCMP_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 13:15 - Instrumentation Resources"]
    #[inline(always)]
    pub fn instres(&self) -> INSTRES_R {
        INSTRES_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 16:19 - EmbeddedICE watchpoint inputs"]
    #[inline(always)]
    pub fn eicewpnt(&self) -> EICEWPNT_R {
        EICEWPNT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Trace Sart/Stop Block Uses EmbeddedICE watchpoint inputs"]
    #[inline(always)]
    pub fn teicewpnt(&self) -> TEICEWPNT_R {
        TEICEWPNT_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - EmbeddedICE Behavior control Implemented"]
    #[inline(always)]
    pub fn eiceimp(&self) -> EICEIMP_R {
        EICEIMP_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Timestamping Implemented"]
    #[inline(always)]
    pub fn timp(&self) -> TIMP_R {
        TIMP_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Reduced Function Counter"]
    #[inline(always)]
    pub fn rfcnt(&self) -> RFCNT_R {
        RFCNT_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Timestamp Encoding"]
    #[inline(always)]
    pub fn tenc(&self) -> TENC_R {
        TENC_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Timestamp Size"]
    #[inline(always)]
    pub fn tsize(&self) -> TSIZE_R {
        TSIZE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
