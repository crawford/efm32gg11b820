#[doc = "Writer for register IFC"]
pub type W = crate::W<u32, super::IFC>;
#[doc = "Register IFC `reset()`'s with value 0"]
impl crate::ResetValue for super::IFC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `EXT`"]
pub struct EXT_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Write proxy for field `EM4WU`"]
pub struct EM4WU_W<'a> {
    w: &'a mut W,
}
impl<'a> EM4WU_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - Clear EXT Interrupt Flag"]
    #[inline(always)]
    pub fn ext(&mut self) -> EXT_W {
        EXT_W { w: self }
    }
    #[doc = "Bits 16:31 - Clear EM4WU Interrupt Flag"]
    #[inline(always)]
    pub fn em4wu(&mut self) -> EM4WU_W {
        EM4WU_W { w: self }
    }
}
