#[doc = "Reader of register JUMBOMAXLEN"]
pub type R = crate::R<u32, super::JUMBOMAXLEN>;
#[doc = "Writer for register JUMBOMAXLEN"]
pub type W = crate::W<u32, super::JUMBOMAXLEN>;
#[doc = "Register JUMBOMAXLEN `reset()`'s with value 0x2800"]
impl crate::ResetValue for super::JUMBOMAXLEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2800
    }
}
#[doc = "Reader of field `JUMBOMAXLEN`"]
pub type JUMBOMAXLEN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `JUMBOMAXLEN`"]
pub struct JUMBOMAXLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> JUMBOMAXLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - Maximum Jumbo Frame Size - resets to the gem_jumbo_max_length define value."]
    #[inline(always)]
    pub fn jumbomaxlen(&self) -> JUMBOMAXLEN_R {
        JUMBOMAXLEN_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Maximum Jumbo Frame Size - resets to the gem_jumbo_max_length define value."]
    #[inline(always)]
    pub fn jumbomaxlen(&mut self) -> JUMBOMAXLEN_W {
        JUMBOMAXLEN_W { w: self }
    }
}
