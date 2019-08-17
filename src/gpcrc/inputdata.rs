#[doc = "Reader of register INPUTDATA"]
pub type R = crate::R<u32, super::INPUTDATA>;
#[doc = "Writer for register INPUTDATA"]
pub type W = crate::W<u32, super::INPUTDATA>;
#[doc = "Register INPUTDATA `reset()`'s with value 0"]
impl crate::ResetValue for super::INPUTDATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `INPUTDATA`"]
pub type INPUTDATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `INPUTDATA`"]
pub struct INPUTDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUTDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Input Data for 32-bit"]
    #[inline(always)]
    pub fn inputdata(&self) -> INPUTDATA_R {
        INPUTDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Input Data for 32-bit"]
    #[inline(always)]
    pub fn inputdata(&mut self) -> INPUTDATA_W {
        INPUTDATA_W { w: self }
    }
}
