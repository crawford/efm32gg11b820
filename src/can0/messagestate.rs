#[doc = "Reader of register MESSAGESTATE"]
pub type R = crate::R<u32, super::MESSAGESTATE>;
#[doc = "Reader of field `VALID`"]
pub type VALID_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Message Valid Bits (of All Message Objects)"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
