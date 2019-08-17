#[doc = "Reader of register IF0IF"]
pub type R = crate::R<u32, super::IF0IF>;
#[doc = "Reader of field `MESSAGE`"]
pub type MESSAGE_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Message Object Interrupt Flag"]
    #[inline(always)]
    pub fn message(&self) -> MESSAGE_R {
        MESSAGE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
