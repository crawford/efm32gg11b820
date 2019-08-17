#[doc = "Reader of register IF"]
pub type R = crate::R<u32, super::IF>;
#[doc = "Reader of field `SINGLE`"]
pub type SINGLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `SCAN`"]
pub type SCAN_R = crate::R<bool, bool>;
#[doc = "Reader of field `SINGLEOF`"]
pub type SINGLEOF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SCANOF`"]
pub type SCANOF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SINGLEUF`"]
pub type SINGLEUF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SCANUF`"]
pub type SCANUF_R = crate::R<bool, bool>;
#[doc = "Reader of field `SINGLECMP`"]
pub type SINGLECMP_R = crate::R<bool, bool>;
#[doc = "Reader of field `SCANCMP`"]
pub type SCANCMP_R = crate::R<bool, bool>;
#[doc = "Reader of field `VREFOV`"]
pub type VREFOV_R = crate::R<bool, bool>;
#[doc = "Reader of field `PROGERR`"]
pub type PROGERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `SCANEXTPEND`"]
pub type SCANEXTPEND_R = crate::R<bool, bool>;
#[doc = "Reader of field `SCANPEND`"]
pub type SCANPEND_R = crate::R<bool, bool>;
#[doc = "Reader of field `PRSTIMEDERR`"]
pub type PRSTIMEDERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `EM23ERR`"]
pub type EM23ERR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Single Conversion Complete Interrupt Flag"]
    #[inline(always)]
    pub fn single(&self) -> SINGLE_R {
        SINGLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Scan Conversion Complete Interrupt Flag"]
    #[inline(always)]
    pub fn scan(&self) -> SCAN_R {
        SCAN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Single FIFO Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn singleof(&self) -> SINGLEOF_R {
        SINGLEOF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Scan FIFO Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn scanof(&self) -> SCANOF_R {
        SCANOF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Single FIFO Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn singleuf(&self) -> SINGLEUF_R {
        SINGLEUF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Scan FIFO Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn scanuf(&self) -> SCANUF_R {
        SCANUF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Single Result Compare Match Interrupt Flag"]
    #[inline(always)]
    pub fn singlecmp(&self) -> SINGLECMP_R {
        SINGLECMP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Scan Result Compare Match Interrupt Flag"]
    #[inline(always)]
    pub fn scancmp(&self) -> SCANCMP_R {
        SCANCMP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 24 - VREF Over Voltage Interrupt Flag"]
    #[inline(always)]
    pub fn vrefov(&self) -> VREFOV_R {
        VREFOV_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Programming Error Interrupt Flag"]
    #[inline(always)]
    pub fn progerr(&self) -> PROGERR_R {
        PROGERR_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - External Scan Trigger Pending Flag"]
    #[inline(always)]
    pub fn scanextpend(&self) -> SCANEXTPEND_R {
        SCANEXTPEND_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Scan Trigger Pending Flag"]
    #[inline(always)]
    pub fn scanpend(&self) -> SCANPEND_R {
        SCANPEND_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - PRS Timed Mode Error Flag"]
    #[inline(always)]
    pub fn prstimederr(&self) -> PRSTIMEDERR_R {
        PRSTIMEDERR_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - EM23 Entry Error Flag"]
    #[inline(always)]
    pub fn em23err(&self) -> EM23ERR_R {
        EM23ERR_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
