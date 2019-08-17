#[doc = "Reader of register CC0_CCVP"]
pub type R = crate::R<u32, super::CC0_CCVP>;
#[doc = "Reader of field `CCVP`"]
pub type CCVP_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CC Channel Value Peek"]
    #[inline(always)]
    pub fn ccvp(&self) -> CCVP_R {
        CCVP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
