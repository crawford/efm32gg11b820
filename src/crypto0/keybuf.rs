#[doc = "Reader of register KEYBUF"]
pub type R = crate::R<u32, super::KEYBUF>;
#[doc = "Writer for register KEYBUF"]
pub type W = crate::W<u32, super::KEYBUF>;
#[doc = "Register KEYBUF `reset()`'s with value 0"]
impl crate::ResetValue for super::KEYBUF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `KEYBUF`"]
pub type KEYBUF_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `KEYBUF`"]
pub struct KEYBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYBUF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Key Buffer Access"]
    #[inline(always)]
    pub fn keybuf(&self) -> KEYBUF_R {
        KEYBUF_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key Buffer Access"]
    #[inline(always)]
    pub fn keybuf(&mut self) -> KEYBUF_W {
        KEYBUF_W { w: self }
    }
}
