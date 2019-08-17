#[doc = "Reader of register ETMIDR2"]
pub type R = crate::R<u32, super::ETMIDR2>;
#[doc = "Reader of field `RFE`"]
pub type RFE_R = crate::R<bool, bool>;
#[doc = "Reader of field `SWP`"]
pub type SWP_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - RFE Transfer Order"]
    #[inline(always)]
    pub fn rfe(&self) -> RFE_R {
        RFE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SWP Transfer Order"]
    #[inline(always)]
    pub fn swp(&self) -> SWP_R {
        SWP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
