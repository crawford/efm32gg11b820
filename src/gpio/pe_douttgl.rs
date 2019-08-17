#[doc = "Writer for register PE_DOUTTGL"]
pub type W = crate::W<u32, super::PE_DOUTTGL>;
#[doc = "Register PE_DOUTTGL `reset()`'s with value 0"]
impl crate::ResetValue for super::PE_DOUTTGL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `DOUTTGL`"]
pub struct DOUTTGL_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUTTGL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - Data Out Toggle"]
    #[inline(always)]
    pub fn douttgl(&mut self) -> DOUTTGL_W {
        DOUTTGL_W { w: self }
    }
}
