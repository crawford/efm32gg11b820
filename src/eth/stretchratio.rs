#[doc = "Reader of register STRETCHRATIO"]
pub type R = crate::R<u32, super::STRETCHRATIO>;
#[doc = "Writer for register STRETCHRATIO"]
pub type W = crate::W<u32, super::STRETCHRATIO>;
#[doc = "Register STRETCHRATIO `reset()`'s with value 0"]
impl crate::ResetValue for super::STRETCHRATIO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IPGSTRETCH`"]
pub type IPGSTRETCH_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `IPGSTRETCH`"]
pub struct IPGSTRETCH_W<'a> {
    w: &'a mut W,
}
impl<'a> IPGSTRETCH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - IPG Stretch"]
    #[inline(always)]
    pub fn ipgstretch(&self) -> IPGSTRETCH_R {
        IPGSTRETCH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IPG Stretch"]
    #[inline(always)]
    pub fn ipgstretch(&mut self) -> IPGSTRETCH_W {
        IPGSTRETCH_W { w: self }
    }
}
