#[doc = "Reader of register DSTS"]
pub type R = crate::R<u32, super::DSTS>;
#[doc = "Reader of field `SUSPSTS`"]
pub type SUSPSTS_R = crate::R<bool, bool>;
#[doc = "Reader of field `ENUMSPD`"]
pub type ENUMSPD_R = crate::R<u8, u8>;
#[doc = "Reader of field `ERRTICERR`"]
pub type ERRTICERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `SOFFN`"]
pub type SOFFN_R = crate::R<u16, u16>;
#[doc = "Reader of field `DEVLNSTS`"]
pub type DEVLNSTS_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Suspend Status"]
    #[inline(always)]
    pub fn suspsts(&self) -> SUSPSTS_R {
        SUSPSTS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Enumerated Speed"]
    #[inline(always)]
    pub fn enumspd(&self) -> ENUMSPD_R {
        ENUMSPD_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - Erratic Error"]
    #[inline(always)]
    pub fn errticerr(&self) -> ERRTICERR_R {
        ERRTICERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 8:21 - Frame or Microframe Number of the Received SOF"]
    #[inline(always)]
    pub fn soffn(&self) -> SOFFN_R {
        SOFFN_R::new(((self.bits >> 8) & 0x3fff) as u16)
    }
    #[doc = "Bits 22:23 - Device Line Status"]
    #[inline(always)]
    pub fn devlnsts(&self) -> DEVLNSTS_R {
        DEVLNSTS_R::new(((self.bits >> 22) & 0x03) as u8)
    }
}
