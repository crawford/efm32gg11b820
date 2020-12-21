#[doc = "Reader of register EM4CTRL"]
pub type R = crate::R<u32, super::EM4CTRL>;
#[doc = "Writer for register EM4CTRL"]
pub type W = crate::W<u32, super::EM4CTRL>;
#[doc = "Register EM4CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::EM4CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EM4STATE`"]
pub type EM4STATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EM4STATE`"]
pub struct EM4STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> EM4STATE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `RETAINLFRCO`"]
pub type RETAINLFRCO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RETAINLFRCO`"]
pub struct RETAINLFRCO_W<'a> {
    w: &'a mut W,
}
impl<'a> RETAINLFRCO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `RETAINLFXO`"]
pub type RETAINLFXO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RETAINLFXO`"]
pub struct RETAINLFXO_W<'a> {
    w: &'a mut W,
}
impl<'a> RETAINLFXO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `RETAINULFRCO`"]
pub type RETAINULFRCO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RETAINULFRCO`"]
pub struct RETAINULFRCO_W<'a> {
    w: &'a mut W,
}
impl<'a> RETAINULFRCO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "EM4 IO Retention Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EM4IORETMODE_A {
    #[doc = "0: No Retention: Pads enter reset state when entering EM4"]
    DISABLE = 0,
    #[doc = "1: Retention through EM4: Pads enter reset state when exiting EM4"]
    EM4EXIT = 1,
    #[doc = "2: Retention through EM4 and Wakeup: software writes UNLATCH register to remove retention"]
    SWUNLATCH = 2,
}
impl From<EM4IORETMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: EM4IORETMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EM4IORETMODE`"]
pub type EM4IORETMODE_R = crate::R<u8, EM4IORETMODE_A>;
impl EM4IORETMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EM4IORETMODE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EM4IORETMODE_A::DISABLE),
            1 => Val(EM4IORETMODE_A::EM4EXIT),
            2 => Val(EM4IORETMODE_A::SWUNLATCH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EM4IORETMODE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `EM4EXIT`"]
    #[inline(always)]
    pub fn is_em4exit(&self) -> bool {
        *self == EM4IORETMODE_A::EM4EXIT
    }
    #[doc = "Checks if the value of the field is `SWUNLATCH`"]
    #[inline(always)]
    pub fn is_swunlatch(&self) -> bool {
        *self == EM4IORETMODE_A::SWUNLATCH
    }
}
#[doc = "Write proxy for field `EM4IORETMODE`"]
pub struct EM4IORETMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EM4IORETMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EM4IORETMODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No Retention: Pads enter reset state when entering EM4"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EM4IORETMODE_A::DISABLE)
    }
    #[doc = "Retention through EM4: Pads enter reset state when exiting EM4"]
    #[inline(always)]
    pub fn em4exit(self) -> &'a mut W {
        self.variant(EM4IORETMODE_A::EM4EXIT)
    }
    #[doc = "Retention through EM4 and Wakeup: software writes UNLATCH register to remove retention"]
    #[inline(always)]
    pub fn swunlatch(self) -> &'a mut W {
        self.variant(EM4IORETMODE_A::SWUNLATCH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Write proxy for field `EM4ENTRY`"]
pub struct EM4ENTRY_W<'a> {
    w: &'a mut W,
}
impl<'a> EM4ENTRY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Energy Mode 4 State"]
    #[inline(always)]
    pub fn em4state(&self) -> EM4STATE_R {
        EM4STATE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LFRCO Retain During EM4"]
    #[inline(always)]
    pub fn retainlfrco(&self) -> RETAINLFRCO_R {
        RETAINLFRCO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LFXO Retain During EM4"]
    #[inline(always)]
    pub fn retainlfxo(&self) -> RETAINLFXO_R {
        RETAINLFXO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - ULFRCO Retain During EM4S"]
    #[inline(always)]
    pub fn retainulfrco(&self) -> RETAINULFRCO_R {
        RETAINULFRCO_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - EM4 IO Retention Disable"]
    #[inline(always)]
    pub fn em4ioretmode(&self) -> EM4IORETMODE_R {
        EM4IORETMODE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Energy Mode 4 State"]
    #[inline(always)]
    pub fn em4state(&mut self) -> EM4STATE_W {
        EM4STATE_W { w: self }
    }
    #[doc = "Bit 1 - LFRCO Retain During EM4"]
    #[inline(always)]
    pub fn retainlfrco(&mut self) -> RETAINLFRCO_W {
        RETAINLFRCO_W { w: self }
    }
    #[doc = "Bit 2 - LFXO Retain During EM4"]
    #[inline(always)]
    pub fn retainlfxo(&mut self) -> RETAINLFXO_W {
        RETAINLFXO_W { w: self }
    }
    #[doc = "Bit 3 - ULFRCO Retain During EM4S"]
    #[inline(always)]
    pub fn retainulfrco(&mut self) -> RETAINULFRCO_W {
        RETAINULFRCO_W { w: self }
    }
    #[doc = "Bits 4:5 - EM4 IO Retention Disable"]
    #[inline(always)]
    pub fn em4ioretmode(&mut self) -> EM4IORETMODE_W {
        EM4IORETMODE_W { w: self }
    }
    #[doc = "Bits 16:17 - Energy Mode 4 Entry"]
    #[inline(always)]
    pub fn em4entry(&mut self) -> EM4ENTRY_W {
        EM4ENTRY_W { w: self }
    }
}
