#[doc = "Reader of register INPUTDATABYTE"]
pub type R = crate::R<u32, super::INPUTDATABYTE>;
#[doc = "Writer for register INPUTDATABYTE"]
pub type W = crate::W<u32, super::INPUTDATABYTE>;
#[doc = "Register INPUTDATABYTE `reset()`'s with value 0"]
impl crate::ResetValue for super::INPUTDATABYTE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INPUTDATABYTE`"]
pub type INPUTDATABYTE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `INPUTDATABYTE`"]
pub struct INPUTDATABYTE_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUTDATABYTE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Input Data for 8-bit"]
    #[inline(always)]
    pub fn inputdatabyte(&self) -> INPUTDATABYTE_R {
        INPUTDATABYTE_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Input Data for 8-bit"]
    #[inline(always)]
    pub fn inputdatabyte(&mut self) -> INPUTDATABYTE_W {
        INPUTDATABYTE_W { w: self }
    }
}
