#[doc = "Reader of register IF"]
pub type R = crate::R<u32, super::IF>;
#[doc = "Reader of field `EXT`"]
pub type EXT_R = crate::R<u16, u16>;
#[doc = "Reader of field `EM4WU`"]
pub type EM4WU_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - External Pin Interrupt Flag"]
    #[inline(always)]
    pub fn ext(&self) -> EXT_R {
        EXT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - EM4 Wake Up Pin Interrupt Flag"]
    #[inline(always)]
    pub fn em4wu(&self) -> EM4WU_R {
        EM4WU_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
