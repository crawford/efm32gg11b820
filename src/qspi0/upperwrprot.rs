#[doc = "Reader of register UPPERWRPROT"]
pub type R = crate::R<u32, super::UPPERWRPROT>;
#[doc = "Writer for register UPPERWRPROT"]
pub type W = crate::W<u32, super::UPPERWRPROT>;
#[doc = "Register UPPERWRPROT `reset()`'s with value 0"]
impl crate::ResetValue for super::UPPERWRPROT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SUBSECTOR`"]
pub type SUBSECTOR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SUBSECTOR`"]
pub struct SUBSECTOR_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBSECTOR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Upper Block Number"]
    #[inline(always)]
    pub fn subsector(&self) -> SUBSECTOR_R {
        SUBSECTOR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Upper Block Number"]
    #[inline(always)]
    pub fn subsector(&mut self) -> SUBSECTOR_W {
        SUBSECTOR_W { w: self }
    }
}
