#[doc = "Reader of register TSUPTPRXNSEC"]
pub type R = crate::R<u32, super::TSUPTPRXNSEC>;
#[doc = "Reader of field `TIMER`"]
pub type TIMER_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:29 - PTP Event Frame Received Nanoseconds"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
