#[doc = "Reader of register PRECNT"]
pub type R = crate::R<u32, super::PRECNT>;
#[doc = "Writer for register PRECNT"]
pub type W = crate::W<u32, super::PRECNT>;
#[doc = "Register PRECNT `reset()`'s with value 0"]
impl crate::ResetValue for super::PRECNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRECNT`"]
pub type PRECNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PRECNT`"]
pub struct PRECNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PRECNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff) | ((value as u32) & 0x7fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:14 - Pre-Counter Value"]
    #[inline(always)]
    pub fn precnt(&self) -> PRECNT_R {
        PRECNT_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - Pre-Counter Value"]
    #[inline(always)]
    pub fn precnt(&mut self) -> PRECNT_W {
        PRECNT_W { w: self }
    }
}
