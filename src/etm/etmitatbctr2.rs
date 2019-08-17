#[doc = "Reader of register ETMITATBCTR2"]
pub type R = crate::R<u32, super::ETMITATBCTR2>;
#[doc = "Reader of field `ATREADY`"]
pub type ATREADY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - ATREADY Input Value"]
    #[inline(always)]
    pub fn atready(&self) -> ATREADY_R {
        ATREADY_R::new((self.bits & 0x01) != 0)
    }
}
