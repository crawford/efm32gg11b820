#[doc = "Reader of register IF"]
pub type R = crate::R<u32, super::IF>;
#[doc = "Reader of field `INSTRDONE`"]
pub type INSTRDONE_R = crate::R<bool, bool>;
#[doc = "Reader of field `SEQDONE`"]
pub type SEQDONE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Instruction Done"]
    #[inline(always)]
    pub fn instrdone(&self) -> INSTRDONE_R {
        INSTRDONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Sequence Done"]
    #[inline(always)]
    pub fn seqdone(&self) -> SEQDONE_R {
        SEQDONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
