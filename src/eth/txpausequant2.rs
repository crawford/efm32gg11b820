#[doc = "Reader of register TXPAUSEQUANT2"]
pub type R = crate::R<u32, super::TXPAUSEQUANT2>;
#[doc = "Writer for register TXPAUSEQUANT2"]
pub type W = crate::W<u32, super::TXPAUSEQUANT2>;
#[doc = "Register TXPAUSEQUANT2 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::TXPAUSEQUANT2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `QUANTP4`"]
pub type QUANTP4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `QUANTP4`"]
pub struct QUANTP4_W<'a> {
    w: &'a mut W,
}
impl<'a> QUANTP4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `QUANTP5`"]
pub type QUANTP5_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `QUANTP5`"]
pub struct QUANTP5_W<'a> {
    w: &'a mut W,
}
impl<'a> QUANTP5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 4."]
    #[inline(always)]
    pub fn quantp4(&self) -> QUANTP4_R {
        QUANTP4_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 5."]
    #[inline(always)]
    pub fn quantp5(&self) -> QUANTP5_R {
        QUANTP5_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 4."]
    #[inline(always)]
    pub fn quantp4(&mut self) -> QUANTP4_W {
        QUANTP4_W { w: self }
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 5."]
    #[inline(always)]
    pub fn quantp5(&mut self) -> QUANTP5_W {
        QUANTP5_W { w: self }
    }
}
