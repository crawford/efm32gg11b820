#[doc = "Reader of register TXPAUSEQUANT1"]
pub type R = crate::R<u32, super::TXPAUSEQUANT1>;
#[doc = "Writer for register TXPAUSEQUANT1"]
pub type W = crate::W<u32, super::TXPAUSEQUANT1>;
#[doc = "Register TXPAUSEQUANT1 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::TXPAUSEQUANT1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `QUANTP2`"]
pub type QUANTP2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `QUANTP2`"]
pub struct QUANTP2_W<'a> {
    w: &'a mut W,
}
impl<'a> QUANTP2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
#[doc = "Reader of field `QUANTP3`"]
pub type QUANTP3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `QUANTP3`"]
pub struct QUANTP3_W<'a> {
    w: &'a mut W,
}
impl<'a> QUANTP3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 2."]
    #[inline(always)]
    pub fn quantp2(&self) -> QUANTP2_R {
        QUANTP2_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 3."]
    #[inline(always)]
    pub fn quantp3(&self) -> QUANTP3_R {
        QUANTP3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 2."]
    #[inline(always)]
    pub fn quantp2(&mut self) -> QUANTP2_W {
        QUANTP2_W { w: self }
    }
    #[doc = "Bits 16:31 - Transmit pause quantum - written with the pause quantum value for pause frame transmission of priority 3."]
    #[inline(always)]
    pub fn quantp3(&mut self) -> QUANTP3_W {
        QUANTP3_W { w: self }
    }
}
