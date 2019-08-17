#[doc = "Reader of register IF"]
pub type R = crate::R<u32, super::IF>;
#[doc = "Reader of field `OF`"]
pub type OF_R = crate::R<bool, bool>;
#[doc = "Reader of field `COMP`"]
pub type COMP_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:6 - Compare Match X Interrupt Flag"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
}
