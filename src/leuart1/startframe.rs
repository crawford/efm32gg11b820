#[doc = "Reader of register STARTFRAME"]
pub type R = crate::R<u32, super::STARTFRAME>;
#[doc = "Writer for register STARTFRAME"]
pub type W = crate::W<u32, super::STARTFRAME>;
#[doc = "Register STARTFRAME `reset()`'s with value 0"]
impl crate::ResetValue for super::STARTFRAME {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STARTFRAME`"]
pub type STARTFRAME_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `STARTFRAME`"]
pub struct STARTFRAME_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTFRAME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Start Frame"]
    #[inline(always)]
    pub fn startframe(&self) -> STARTFRAME_R {
        STARTFRAME_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Start Frame"]
    #[inline(always)]
    pub fn startframe(&mut self) -> STARTFRAME_W {
        STARTFRAME_W { w: self }
    }
}
