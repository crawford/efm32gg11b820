#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "Update Data Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UDCTRL_A {
    #[doc = "0: The data transfer is controlled by SW. Transfer is performed as soon as possible"]
    REGULAR,
    #[doc = "1: The data transfer is done at the next event triggered by the Frame Counter"]
    FCEVENT,
    #[doc = "2: The data transfer is done continuously at every LCD frame start"]
    FRAMESTART,
}
impl From<UDCTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: UDCTRL_A) -> Self {
        match variant {
            UDCTRL_A::REGULAR => 0,
            UDCTRL_A::FCEVENT => 1,
            UDCTRL_A::FRAMESTART => 2,
        }
    }
}
#[doc = "Reader of field `UDCTRL`"]
pub type UDCTRL_R = crate::R<u8, UDCTRL_A>;
impl UDCTRL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, UDCTRL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(UDCTRL_A::REGULAR),
            1 => Val(UDCTRL_A::FCEVENT),
            2 => Val(UDCTRL_A::FRAMESTART),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `REGULAR`"]
    #[inline(always)]
    pub fn is_regular(&self) -> bool {
        *self == UDCTRL_A::REGULAR
    }
    #[doc = "Checks if the value of the field is `FCEVENT`"]
    #[inline(always)]
    pub fn is_fcevent(&self) -> bool {
        *self == UDCTRL_A::FCEVENT
    }
    #[doc = "Checks if the value of the field is `FRAMESTART`"]
    #[inline(always)]
    pub fn is_framestart(&self) -> bool {
        *self == UDCTRL_A::FRAMESTART
    }
}
#[doc = "Write proxy for field `UDCTRL`"]
pub struct UDCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> UDCTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UDCTRL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The data transfer is controlled by SW. Transfer is performed as soon as possible"]
    #[inline(always)]
    pub fn regular(self) -> &'a mut W {
        self.variant(UDCTRL_A::REGULAR)
    }
    #[doc = "The data transfer is done at the next event triggered by the Frame Counter"]
    #[inline(always)]
    pub fn fcevent(self) -> &'a mut W {
        self.variant(UDCTRL_A::FCEVENT)
    }
    #[doc = "The data transfer is done continuously at every LCD frame start"]
    #[inline(always)]
    pub fn framestart(self) -> &'a mut W {
        self.variant(UDCTRL_A::FRAMESTART)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `DSC`"]
pub type DSC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSC`"]
pub struct DSC_W<'a> {
    w: &'a mut W,
}
impl<'a> DSC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LCD Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Update Data Control"]
    #[inline(always)]
    pub fn udctrl(&self) -> UDCTRL_R {
        UDCTRL_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 23 - Direct Segment Control"]
    #[inline(always)]
    pub fn dsc(&self) -> DSC_R {
        DSC_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bits 1:2 - Update Data Control"]
    #[inline(always)]
    pub fn udctrl(&mut self) -> UDCTRL_W {
        UDCTRL_W { w: self }
    }
    #[doc = "Bit 23 - Direct Segment Control"]
    #[inline(always)]
    pub fn dsc(&mut self) -> DSC_W {
        DSC_W { w: self }
    }
}
