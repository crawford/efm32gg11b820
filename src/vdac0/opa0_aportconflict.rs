#[doc = "Reader of register OPA0_APORTCONFLICT"]
pub type R = crate::R<u32, super::OPA0_APORTCONFLICT>;
#[doc = "Reader of field `APORT1XCONFLICT`"]
pub type APORT1XCONFLICT_R = crate::R<bool, bool>;
#[doc = "Reader of field `APORT1YCONFLICT`"]
pub type APORT1YCONFLICT_R = crate::R<bool, bool>;
#[doc = "Reader of field `APORT2XCONFLICT`"]
pub type APORT2XCONFLICT_R = crate::R<bool, bool>;
#[doc = "Reader of field `APORT2YCONFLICT`"]
pub type APORT2YCONFLICT_R = crate::R<bool, bool>;
#[doc = "Reader of field `APORT3XCONFLICT`"]
pub type APORT3XCONFLICT_R = crate::R<bool, bool>;
#[doc = "Reader of field `APORT3YCONFLICT`"]
pub type APORT3YCONFLICT_R = crate::R<bool, bool>;
#[doc = "Reader of field `APORT4XCONFLICT`"]
pub type APORT4XCONFLICT_R = crate::R<bool, bool>;
#[doc = "Reader of field `APORT4YCONFLICT`"]
pub type APORT4YCONFLICT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 2 - 1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport1xconflict(&self) -> APORT1XCONFLICT_R {
        APORT1XCONFLICT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport1yconflict(&self) -> APORT1YCONFLICT_R {
        APORT1YCONFLICT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 1 If the Bus Connected to APORT2X is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport2xconflict(&self) -> APORT2XCONFLICT_R {
        APORT2XCONFLICT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 1 If the Bus Connected to APORT2Y is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport2yconflict(&self) -> APORT2YCONFLICT_R {
        APORT2YCONFLICT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 1 If the Bus Connected to APORT3X is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport3xconflict(&self) -> APORT3XCONFLICT_R {
        APORT3XCONFLICT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 1 If the Bus Connected to APORT3Y is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport3yconflict(&self) -> APORT3YCONFLICT_R {
        APORT3YCONFLICT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 1 If the Bus Connected to APORT4X is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport4xconflict(&self) -> APORT4XCONFLICT_R {
        APORT4XCONFLICT_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 1 If the Bus Connected to APORT4Y is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport4yconflict(&self) -> APORT4YCONFLICT_R {
        APORT4YCONFLICT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
