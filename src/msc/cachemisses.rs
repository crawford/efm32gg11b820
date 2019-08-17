#[doc = "Reader of register CACHEMISSES"]
pub type R = crate::R<u32, super::CACHEMISSES>;
#[doc = "Reader of field `CACHEMISSES`"]
pub type CACHEMISSES_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:19 - Cache Misses Since Last Performance Counter Start Command"]
    #[inline(always)]
    pub fn cachemisses(&self) -> CACHEMISSES_R {
        CACHEMISSES_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
