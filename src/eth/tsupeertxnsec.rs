#[doc = "Reader of register TSUPEERTXNSEC"]
pub type R = crate::R<u32, super::TSUPEERTXNSEC>;
#[doc = "Reader of field `TIMER`"]
pub type TIMER_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:29 - PTP Peer Event Frame Transmitted Nanoseconds"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
