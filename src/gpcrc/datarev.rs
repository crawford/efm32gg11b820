#[doc = "Reader of register DATAREV"]
pub type R = crate::R<u32, super::DATAREV>;
#[doc = "Reader of field `DATAREV`"]
pub type DATAREV_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data Reverse Value"]
    #[inline(always)]
    pub fn datarev(&self) -> DATAREV_R {
        DATAREV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
