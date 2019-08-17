#[doc = "Reader of register ETMIDR"]
pub type R = crate::R<u32, super::ETMIDR>;
#[doc = "Reader of field `IMPVER`"]
pub type IMPVER_R = crate::R<u8, u8>;
#[doc = "Reader of field `ETMMINVER`"]
pub type ETMMINVER_R = crate::R<u8, u8>;
#[doc = "Reader of field `ETMMAJVER`"]
pub type ETMMAJVER_R = crate::R<u8, u8>;
#[doc = "Reader of field `PROCFAM`"]
pub type PROCFAM_R = crate::R<u8, u8>;
#[doc = "Reader of field `LPCF`"]
pub type LPCF_R = crate::R<bool, bool>;
#[doc = "Reader of field `THUMBT`"]
pub type THUMBT_R = crate::R<bool, bool>;
#[doc = "Reader of field `SECEXT`"]
pub type SECEXT_R = crate::R<bool, bool>;
#[doc = "Reader of field `BPE`"]
pub type BPE_R = crate::R<bool, bool>;
#[doc = "Reader of field `IMPCODE`"]
pub type IMPCODE_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Implementation Revision"]
    #[inline(always)]
    pub fn impver(&self) -> IMPVER_R {
        IMPVER_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Minor ETM Architecture Version"]
    #[inline(always)]
    pub fn etmminver(&self) -> ETMMINVER_R {
        ETMMINVER_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Major ETM Architecture Version"]
    #[inline(always)]
    pub fn etmmajver(&self) -> ETMMAJVER_R {
        ETMMAJVER_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Implementer Code"]
    #[inline(always)]
    pub fn procfam(&self) -> PROCFAM_R {
        PROCFAM_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Load PC First"]
    #[inline(always)]
    pub fn lpcf(&self) -> LPCF_R {
        LPCF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 18 - 32-bit Thumb Instruction Tracing"]
    #[inline(always)]
    pub fn thumbt(&self) -> THUMBT_R {
        THUMBT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Security Extension Support"]
    #[inline(always)]
    pub fn secext(&self) -> SECEXT_R {
        SECEXT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Branch Packet Encoding"]
    #[inline(always)]
    pub fn bpe(&self) -> BPE_R {
        BPE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 24:31 - Implementer Code"]
    #[inline(always)]
    pub fn impcode(&self) -> IMPCODE_R {
        IMPCODE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
