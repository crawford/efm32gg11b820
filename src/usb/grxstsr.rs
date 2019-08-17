#[doc = "Reader of register GRXSTSR"]
pub type R = crate::R<u32, super::GRXSTSR>;
#[doc = "Reader of field `CHNUM`"]
pub type CHNUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `BCNT`"]
pub type BCNT_R = crate::R<u16, u16>;
#[doc = "Reader of field `DPID`"]
pub type DPID_R = crate::R<u8, u8>;
#[doc = "Reader of field `PKTSTS`"]
pub type PKTSTS_R = crate::R<u8, u8>;
#[doc = "Reader of field `FN`"]
pub type FN_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Channel Number"]
    #[inline(always)]
    pub fn chnum(&self) -> CHNUM_R {
        CHNUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:14 - Byte Count"]
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bits 15:16 - Data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DPID_R {
        DPID_R::new(((self.bits >> 15) & 0x03) as u8)
    }
    #[doc = "Bits 17:20 - Packet Status"]
    #[inline(always)]
    pub fn pktsts(&self) -> PKTSTS_R {
        PKTSTS_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:24 - Frame Number"]
    #[inline(always)]
    pub fn fn_(&self) -> FN_R {
        FN_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
}
