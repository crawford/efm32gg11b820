#[doc = "Reader of register APORTREQ"]
pub type R = crate::R<u32, super::APORTREQ>;
#[doc = "Reader of field `APORT0XREQ`"]
pub type APORT0XREQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `APORT0YREQ`"]
pub type APORT0YREQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `APORT1XREQ`"]
pub type APORT1XREQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `APORT1YREQ`"]
pub type APORT1YREQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `APORT2XREQ`"]
pub type APORT2XREQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `APORT2YREQ`"]
pub type APORT2YREQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `APORT3XREQ`"]
pub type APORT3XREQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `APORT3YREQ`"]
pub type APORT3YREQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `APORT4XREQ`"]
pub type APORT4XREQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `APORT4YREQ`"]
pub type APORT4YREQ_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - 1 If the Bus Connected to APORT0X is Requested"]
    #[inline(always)]
    pub fn aport0xreq(&self) -> APORT0XREQ_R {
        APORT0XREQ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1 If the Bus Connected to APORT0Y is Requested"]
    #[inline(always)]
    pub fn aport0yreq(&self) -> APORT0YREQ_R {
        APORT0YREQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 1 If the Bus Connected to APORT2X is Requested"]
    #[inline(always)]
    pub fn aport1xreq(&self) -> APORT1XREQ_R {
        APORT1XREQ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1 If the Bus Connected to APORT1X is Requested"]
    #[inline(always)]
    pub fn aport1yreq(&self) -> APORT1YREQ_R {
        APORT1YREQ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 1 If the Bus Connected to APORT2X is Requested"]
    #[inline(always)]
    pub fn aport2xreq(&self) -> APORT2XREQ_R {
        APORT2XREQ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - 1 If the Bus Connected to APORT2Y is Requested"]
    #[inline(always)]
    pub fn aport2yreq(&self) -> APORT2YREQ_R {
        APORT2YREQ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 1 If the Bus Connected to APORT3X is Requested"]
    #[inline(always)]
    pub fn aport3xreq(&self) -> APORT3XREQ_R {
        APORT3XREQ_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - 1 If the Bus Connected to APORT3Y is Requested"]
    #[inline(always)]
    pub fn aport3yreq(&self) -> APORT3YREQ_R {
        APORT3YREQ_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - 1 If the Bus Connected to APORT4X is Requested"]
    #[inline(always)]
    pub fn aport4xreq(&self) -> APORT4XREQ_R {
        APORT4XREQ_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - 1 If the Bus Connected to APORT4Y is Requested"]
    #[inline(always)]
    pub fn aport4yreq(&self) -> APORT4YREQ_R {
        APORT4YREQ_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
