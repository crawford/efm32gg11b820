#[doc = "Reader of register CACHEHITS"]
pub type R = crate::R<u32, super::CACHEHITS>;
#[doc = "Reader of field `CACHEHITS`"]
pub type CACHEHITS_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:19 - Cache Hits Since Last Performance Counter Start Command"]
    #[inline(always)]
    pub fn cachehits(&self) -> CACHEHITS_R {
        CACHEHITS_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
