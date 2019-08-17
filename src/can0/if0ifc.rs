#[doc = "Writer for register IF0IFC"]
pub type W = crate::W<u32, super::IF0IFC>;
#[doc = "Register IF0IFC `reset()`'s with value 0"]
impl crate::ResetValue for super::IF0IFC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Write proxy for field `MESSAGE`"]
pub struct MESSAGE_W<'a> {
    w: &'a mut W,
}
impl<'a> MESSAGE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Clear MESSAGE Interrupt Flag"]
    #[inline(always)]
    pub fn message(&mut self) -> MESSAGE_W {
        MESSAGE_W { w: self }
    }
}
