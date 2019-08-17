#[doc = "Reader of register IF"]
pub type R = crate::R<u32, super::IF>;
#[doc = "Reader of field `FC`"]
pub type FC_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Frame Counter Interrupt Flag"]
    #[inline(always)]
    pub fn fc(&self) -> FC_R {
        FC_R::new((self.bits & 0x01) != 0)
    }
}
