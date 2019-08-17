#[doc = "Reader of register IF"]
pub type R = crate::R<u32, super::IF>;
#[doc = "Reader of field `CURSTABLE`"]
pub type CURSTABLE_R = crate::R<bool, bool>;
#[doc = "Reader of field `APORTCONFLICT`"]
pub type APORTCONFLICT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Edge Triggered Interrupt Flag"]
    #[inline(always)]
    pub fn curstable(&self) -> CURSTABLE_R {
        CURSTABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - APORT Conflict Interrupt Flag"]
    #[inline(always)]
    pub fn aportconflict(&self) -> APORTCONFLICT_R {
        APORTCONFLICT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
