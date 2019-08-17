#[doc = "Reader of register DCDCCLIMCTRL"]
pub type R = crate::R<u32, super::DCDCCLIMCTRL>;
#[doc = "Writer for register DCDCCLIMCTRL"]
pub type W = crate::W<u32, super::DCDCCLIMCTRL>;
#[doc = "Register DCDCCLIMCTRL `reset()`'s with value 0x0100"]
impl crate::ResetValue for super::DCDCCLIMCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0100
    }
}
#[doc = "Reader of field `CLIMBLANKDLY`"]
pub type CLIMBLANKDLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CLIMBLANKDLY`"]
pub struct CLIMBLANKDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> CLIMBLANKDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `BYPLIMEN`"]
pub type BYPLIMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYPLIMEN`"]
pub struct BYPLIMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPLIMEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:9 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn climblankdly(&self) -> CLIMBLANKDLY_R {
        CLIMBLANKDLY_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bit 13 - Bypass Current Limit Enable"]
    #[inline(always)]
    pub fn byplimen(&self) -> BYPLIMEN_R {
        BYPLIMEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:9 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn climblankdly(&mut self) -> CLIMBLANKDLY_W {
        CLIMBLANKDLY_W { w: self }
    }
    #[doc = "Bit 13 - Bypass Current Limit Enable"]
    #[inline(always)]
    pub fn byplimen(&mut self) -> BYPLIMEN_W {
        BYPLIMEN_W { w: self }
    }
}
