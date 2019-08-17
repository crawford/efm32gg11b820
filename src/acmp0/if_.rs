#[doc = "Reader of register IF"]
pub type R = crate::R<u32, super::IF>;
#[doc = "Reader of field `EDGE`"]
pub type EDGE_R = crate::R<bool, bool>;
#[doc = "Reader of field `WARMUP`"]
pub type WARMUP_R = crate::R<bool, bool>;
#[doc = "Reader of field `APORTCONFLICT`"]
pub type APORTCONFLICT_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Edge Triggered Interrupt Flag"]
    #[inline(always)]
    pub fn edge(&self) -> EDGE_R {
        EDGE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Warm-up Interrupt Flag"]
    #[inline(always)]
    pub fn warmup(&self) -> WARMUP_R {
        WARMUP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - APORT Conflict Interrupt Flag"]
    #[inline(always)]
    pub fn aportconflict(&self) -> APORTCONFLICT_R {
        APORTCONFLICT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
