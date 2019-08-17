#[doc = "Reader of register SINGLECOLS"]
pub type R = crate::R<u32, super::SINGLECOLS>;
#[doc = "Writer for register SINGLECOLS"]
pub type W = crate::W<u32, super::SINGLECOLS>;
#[doc = "Register SINGLECOLS `reset()`'s with value 0"]
impl crate::ResetValue for super::SINGLECOLS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `COUNT`"]
pub type COUNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `COUNT`"]
pub struct COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | ((value as u32) & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - Single collision frames"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - Single collision frames"]
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W {
        COUNT_W { w: self }
    }
}
