#[doc = "Reader of register TXPAUSEQUANT3"]
pub type R = crate::R<u32, super::TXPAUSEQUANT3>;
#[doc = "Writer for register TXPAUSEQUANT3"]
pub type W = crate::W<u32, super::TXPAUSEQUANT3>;
#[doc = "Register TXPAUSEQUANT3 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::TXPAUSEQUANT3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `QUANTP6`"]
pub type QUANTP6_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `QUANTP6`"]
pub struct QUANTP6_W<'a> {
    w: &'a mut W,
}
impl<'a> QUANTP6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `QUANTP7`"]
pub type QUANTP7_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `QUANTP7`"]
pub struct QUANTP7_W<'a> {
    w: &'a mut W,
}
impl<'a> QUANTP7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 6."]
    #[inline(always)]
    pub fn quantp6(&self) -> QUANTP6_R {
        QUANTP6_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 7."]
    #[inline(always)]
    pub fn quantp7(&self) -> QUANTP7_R {
        QUANTP7_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 6."]
    #[inline(always)]
    pub fn quantp6(&mut self) -> QUANTP6_W {
        QUANTP6_W { w: self }
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 7."]
    #[inline(always)]
    pub fn quantp7(&mut self) -> QUANTP7_W {
        QUANTP7_W { w: self }
    }
}
