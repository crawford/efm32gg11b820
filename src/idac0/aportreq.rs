#[doc = "Reader of register APORTREQ"]
pub type R = crate::R<u32, super::APORTREQ>;
#[doc = "Reader of field `APORT1XREQ`"]
pub type APORT1XREQ_R = crate::R<bool, bool>;
#[doc = "Reader of field `APORT1YREQ`"]
pub type APORT1YREQ_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 2 - 1 If the APORT Bus Connected to APORT1X is Requested"]
    #[inline(always)]
    pub fn aport1xreq(&self) -> APORT1XREQ_R {
        APORT1XREQ_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1 If the Bus Connected to APORT1Y is Requested"]
    #[inline(always)]
    pub fn aport1yreq(&self) -> APORT1YREQ_R {
        APORT1YREQ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
