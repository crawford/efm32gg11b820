#[doc = "Reader of register USBCTRL"]
pub type R = crate::R<u32, super::USBCTRL>;
#[doc = "Writer for register USBCTRL"]
pub type W = crate::W<u32, super::USBCTRL>;
#[doc = "Register USBCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::USBCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "USB Rate Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USBCLKSEL_A {
    #[doc = "0: USHFRCO (clock recovery) is clocking USB"]
    USHFRCO = 0,
    #[doc = "1: HFXO clock is used to clock USB"]
    HFXO = 1,
    #[doc = "2: HFXO clock doubler is used to clock USB"]
    HFXOX2 = 2,
    #[doc = "3: HFRCO clock is used to clock USB"]
    HFRCO = 3,
    #[doc = "4: LFXO clock is used to clock USB"]
    LFXO = 4,
    #[doc = "5: LFRCO clock is used to clock USB"]
    LFRCO = 5,
}
impl From<USBCLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USBCLKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `USBCLKSEL`"]
pub type USBCLKSEL_R = crate::R<u8, USBCLKSEL_A>;
impl USBCLKSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, USBCLKSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(USBCLKSEL_A::USHFRCO),
            1 => Val(USBCLKSEL_A::HFXO),
            2 => Val(USBCLKSEL_A::HFXOX2),
            3 => Val(USBCLKSEL_A::HFRCO),
            4 => Val(USBCLKSEL_A::LFXO),
            5 => Val(USBCLKSEL_A::LFRCO),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `USHFRCO`"]
    #[inline(always)]
    pub fn is_ushfrco(&self) -> bool {
        *self == USBCLKSEL_A::USHFRCO
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == USBCLKSEL_A::HFXO
    }
    #[doc = "Checks if the value of the field is `HFXOX2`"]
    #[inline(always)]
    pub fn is_hfxox2(&self) -> bool {
        *self == USBCLKSEL_A::HFXOX2
    }
    #[doc = "Checks if the value of the field is `HFRCO`"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == USBCLKSEL_A::HFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == USBCLKSEL_A::LFXO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == USBCLKSEL_A::LFRCO
    }
}
#[doc = "Write proxy for field `USBCLKSEL`"]
pub struct USBCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USBCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBCLKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "USHFRCO (clock recovery) is clocking USB"]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut W {
        self.variant(USBCLKSEL_A::USHFRCO)
    }
    #[doc = "HFXO clock is used to clock USB"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(USBCLKSEL_A::HFXO)
    }
    #[doc = "HFXO clock doubler is used to clock USB"]
    #[inline(always)]
    pub fn hfxox2(self) -> &'a mut W {
        self.variant(USBCLKSEL_A::HFXOX2)
    }
    #[doc = "HFRCO clock is used to clock USB"]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(USBCLKSEL_A::HFRCO)
    }
    #[doc = "LFXO clock is used to clock USB"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(USBCLKSEL_A::LFXO)
    }
    #[doc = "LFRCO clock is used to clock USB"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(USBCLKSEL_A::LFRCO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `USBCLKEN`"]
pub type USBCLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBCLKEN`"]
pub struct USBCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBCLKEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - USB Rate Clock Select"]
    #[inline(always)]
    pub fn usbclksel(&self) -> USBCLKSEL_R {
        USBCLKSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 7 - USB Rate Clock Enable"]
    #[inline(always)]
    pub fn usbclken(&self) -> USBCLKEN_R {
        USBCLKEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - USB Rate Clock Select"]
    #[inline(always)]
    pub fn usbclksel(&mut self) -> USBCLKSEL_W {
        USBCLKSEL_W { w: self }
    }
    #[doc = "Bit 7 - USB Rate Clock Enable"]
    #[inline(always)]
    pub fn usbclken(&mut self) -> USBCLKEN_W {
        USBCLKEN_W { w: self }
    }
}
