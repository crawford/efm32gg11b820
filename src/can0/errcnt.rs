#[doc = "Reader of register ERRCNT"]
pub type R = crate::R<u32, super::ERRCNT>;
#[doc = "Reader of field `TEC`"]
pub type TEC_R = crate::R<u8, u8>;
#[doc = "Reader of field `REC`"]
pub type REC_R = crate::R<u8, u8>;
#[doc = "Reader of field `RECERRP`"]
pub type RECERRP_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:7 - Transmit Error Counter"]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - Receive Error Counter"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Receive Error Passive"]
    #[inline(always)]
    pub fn recerrp(&self) -> RECERRP_R {
        RECERRP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
