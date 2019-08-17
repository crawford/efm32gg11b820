#[doc = "Reader of register CURCH"]
pub type R = crate::R<u32, super::CURCH>;
#[doc = "Reader of field `CURCH`"]
pub type CURCH_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Current Channel Index"]
    #[inline(always)]
    pub fn curch(&self) -> CURCH_R {
        CURCH_R::new((self.bits & 0x0f) as u8)
    }
}
