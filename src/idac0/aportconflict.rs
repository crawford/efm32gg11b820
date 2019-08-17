#[doc = "Reader of register APORTCONFLICT"]
pub type R = crate::R<u32, super::APORTCONFLICT>;
#[doc = "Reader of field `APORT1XCONFLICT`"]
pub type APORT1XCONFLICT_R = crate::R<bool, bool>;
#[doc = "Reader of field `APORT1YCONFLICT`"]
pub type APORT1YCONFLICT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 2 - 1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport1xconflict(&self) -> APORT1XCONFLICT_R {
        APORT1XCONFLICT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1 If the Bus Connected to APORT1Y is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport1yconflict(&self) -> APORT1YCONFLICT_R {
        APORT1YCONFLICT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
