#[doc = "Reader of register TSUPTPRXSEC"]
pub type R = crate::R<u32, super::TSUPTPRXSEC>;
#[doc = "Reader of field `TIMER`"]
pub type TIMER_R = crate::R<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - PTP Event Frame Received Seconds"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
