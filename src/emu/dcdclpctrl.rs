#[doc = "Reader of register DCDCLPCTRL"]
pub type R = crate::R<u32, super::DCDCLPCTRL>;
#[doc = "Writer for register DCDCLPCTRL"]
pub type W = crate::W<u32, super::DCDCLPCTRL>;
#[doc = "Register DCDCLPCTRL `reset()`'s with value 0x0300_0000"]
impl crate::ResetValue for super::DCDCLPCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0300_0000
    }
}
#[doc = "Reader of field `LPCMPHYSSELEM234H`"]
pub type LPCMPHYSSELEM234H_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPCMPHYSSELEM234H`"]
pub struct LPCMPHYSSELEM234H_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCMPHYSSELEM234H_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `LPVREFDUTYEN`"]
pub type LPVREFDUTYEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPVREFDUTYEN`"]
pub struct LPVREFDUTYEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPVREFDUTYEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `LPBLANK`"]
pub type LPBLANK_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPBLANK`"]
pub struct LPBLANK_W<'a> {
    w: &'a mut W,
}
impl<'a> LPBLANK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 25)) | (((value as u32) & 0x03) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:15 - LP Mode Hysteresis Selection for EM23 and EM4H"]
    #[inline(always)]
    pub fn lpcmphysselem234h(&self) -> LPCMPHYSSELEM234H_R {
        LPCMPHYSSELEM234H_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - LP Mode Duty Cycling Enable"]
    #[inline(always)]
    pub fn lpvrefdutyen(&self) -> LPVREFDUTYEN_R {
        LPVREFDUTYEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:26 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn lpblank(&self) -> LPBLANK_R {
        LPBLANK_R::new(((self.bits >> 25) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - LP Mode Hysteresis Selection for EM23 and EM4H"]
    #[inline(always)]
    pub fn lpcmphysselem234h(&mut self) -> LPCMPHYSSELEM234H_W {
        LPCMPHYSSELEM234H_W { w: self }
    }
    #[doc = "Bit 24 - LP Mode Duty Cycling Enable"]
    #[inline(always)]
    pub fn lpvrefdutyen(&mut self) -> LPVREFDUTYEN_W {
        LPVREFDUTYEN_W { w: self }
    }
    #[doc = "Bits 25:26 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn lpblank(&mut self) -> LPBLANK_W {
        LPBLANK_W { w: self }
    }
}
