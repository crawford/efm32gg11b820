#[doc = "Reader of register WDATA"]
pub type R = crate::R<u32, super::WDATA>;
#[doc = "Writer for register WDATA"]
pub type W = crate::W<u32, super::WDATA>;
#[doc = "Register WDATA `reset()`'s with value 0"]
impl crate::ResetValue for super::WDATA {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `WDATA`"]
pub type WDATA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `WDATA`"]
pub struct WDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> WDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Write Data"]
    #[inline(always)]
    pub fn wdata(&self) -> WDATA_R {
        WDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write Data"]
    #[inline(always)]
    pub fn wdata(&mut self) -> WDATA_W {
        WDATA_W { w: self }
    }
}
