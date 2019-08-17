#[doc = "Reader of register STATUS"]
pub type R = crate::R<u32, super::STATUS>;
#[doc = "Reader of field `ANYBUSY`"]
pub type ANYBUSY_R = crate::R<bool, bool>;
#[doc = "Reader of field `ANYREQ`"]
pub type ANYREQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHGRANT`"]
pub type CHGRANT_R = crate::R<u8, u8>;
#[doc = "Reader of field `CHERROR`"]
pub type CHERROR_R = crate::R<u8, u8>;
#[doc = "Reader of field `FIFOLEVEL`"]
pub type FIFOLEVEL_R = crate::R<u8, u8>;
#[doc = "Reader of field `CHNUM`"]
pub type CHNUM_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Any DMA Channel Busy"]
    #[inline(always)]
    pub fn anybusy(&self) -> ANYBUSY_R {
        ANYBUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Any DMA Channel Request Pending"]
    #[inline(always)]
    pub fn anyreq(&self) -> ANYREQ_R {
        ANYREQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 3:7 - Granted Channel Number"]
    #[inline(always)]
    pub fn chgrant(&self) -> CHGRANT_R {
        CHGRANT_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Errant Channel Number"]
    #[inline(always)]
    pub fn cherror(&self) -> CHERROR_R {
        CHERROR_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - FIFO Level"]
    #[inline(always)]
    pub fn fifolevel(&self) -> FIFOLEVEL_R {
        FIFOLEVEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Number of Channels"]
    #[inline(always)]
    pub fn chnum(&self) -> CHNUM_R {
        CHNUM_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
