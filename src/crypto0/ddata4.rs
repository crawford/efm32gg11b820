#[doc = "Reader of register DDATA4"]
pub type R = crate::R<u32, super::DDATA4>;
#[doc = "Writer for register DDATA4"]
pub type W = crate::W<u32, super::DDATA4>;
#[doc = "Register DDATA4 `reset()`'s with value 0"]
impl crate::ResetValue for super::DDATA4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DDATA4`"]
pub type DDATA4_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DDATA4`"]
pub struct DDATA4_W<'a> {
    w: &'a mut W,
}
impl<'a> DDATA4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Double Data 0 Access"]
    #[inline(always)]
    pub fn ddata4(&self) -> DDATA4_R {
        DDATA4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Double Data 0 Access"]
    #[inline(always)]
    pub fn ddata4(&mut self) -> DDATA4_W {
        DDATA4_W { w: self }
    }
}
