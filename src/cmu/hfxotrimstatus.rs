#[doc = "Reader of register HFXOTRIMSTATUS"]
pub type R = crate::R<u32, super::HFXOTRIMSTATUS>;
#[doc = "Reader of field `IBTRIMXOCORE`"]
pub type IBTRIMXOCORE_R = crate::R<u16, u16>;
#[doc = "Reader of field `IBTRIMXOCOREMON`"]
pub type IBTRIMXOCOREMON_R = crate::R<u16, u16>;
#[doc = "Reader of field `VALID`"]
pub type VALID_R = crate::R<bool, bool>;
#[doc = "Reader of field `MONVALID`"]
pub type MONVALID_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:10 - Value of IBTRIMXOCORE Found By Automatic HFXO Peak Detection Algorithm"]
    #[inline(always)]
    pub fn ibtrimxocore(&self) -> IBTRIMXOCORE_R {
        IBTRIMXOCORE_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Value of IBTRIMXOCORE Found By Automatic HFXO Peak Detection Algorithm or Peak Monitoring Algorithm (completion of Either Algorithm Will Cause an Update of IBTRIMXOCOREMON)"]
    #[inline(always)]
    pub fn ibtrimxocoremon(&self) -> IBTRIMXOCOREMON_R {
        IBTRIMXOCOREMON_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 30 - Peak Detection Algorithm Found a Value for IBTRIMXOCORE"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Peak Detection Algorithm or Peak Monitoring Algorithm Found a Value for IBTRIMXOCOREMON"]
    #[inline(always)]
    pub fn monvalid(&self) -> MONVALID_R {
        MONVALID_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
