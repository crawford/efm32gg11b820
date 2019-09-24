#[doc = "Reader of register DCDCMISCCTRL"]
pub type R = crate::R<u32, super::DCDCMISCCTRL>;
#[doc = "Writer for register DCDCMISCCTRL"]
pub type W = crate::W<u32, super::DCDCMISCCTRL>;
#[doc = "Register DCDCMISCCTRL `reset()`'s with value 0x0310_7706"]
impl crate::ResetValue for super::DCDCMISCCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0310_7706
    }
}
#[doc = "Reader of field `LNFORCECCM`"]
pub type LNFORCECCM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LNFORCECCM`"]
pub struct LNFORCECCM_W<'a> {
    w: &'a mut W,
}
impl<'a> LNFORCECCM_W<'a> {
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
#[doc = "Reader of field `LPCMPHYSDIS`"]
pub type LPCMPHYSDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPCMPHYSDIS`"]
pub struct LPCMPHYSDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCMPHYSDIS_W<'a> {
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
#[doc = "Reader of field `LPCMPHYSHI`"]
pub type LPCMPHYSHI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPCMPHYSHI`"]
pub struct LPCMPHYSHI_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCMPHYSHI_W<'a> {
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
#[doc = "Reader of field `LNFORCECCMIMM`"]
pub type LNFORCECCMIMM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LNFORCECCMIMM`"]
pub struct LNFORCECCMIMM_W<'a> {
    w: &'a mut W,
}
impl<'a> LNFORCECCMIMM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `PFETCNT`"]
pub type PFETCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PFETCNT`"]
pub struct PFETCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PFETCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `NFETCNT`"]
pub type NFETCNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NFETCNT`"]
pub struct NFETCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> NFETCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `BYPLIMSEL`"]
pub type BYPLIMSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BYPLIMSEL`"]
pub struct BYPLIMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPLIMSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `LPCLIMILIMSEL`"]
pub type LPCLIMILIMSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPCLIMILIMSEL`"]
pub struct LPCLIMILIMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCLIMILIMSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | (((value as u32) & 0x07) << 20);
        self.w
    }
}
#[doc = "Reader of field `LNCLIMILIMSEL`"]
pub type LNCLIMILIMSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LNCLIMILIMSEL`"]
pub struct LNCLIMILIMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LNCLIMILIMSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | (((value as u32) & 0x07) << 24);
        self.w
    }
}
#[doc = "LP Mode Comparator Bias Selection for EM23 or EM4H\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LPCMPBIASEM234H_A {
    #[doc = "0: Maximum load current less than 75uA."]
    BIAS0,
    #[doc = "1: Maximum load current less than 500uA."]
    BIAS1,
    #[doc = "2: Maximum load current less than 2.5mA."]
    BIAS2,
    #[doc = "3: Maximum load current less than 10mA."]
    BIAS3,
}
impl From<LPCMPBIASEM234H_A> for u8 {
    #[inline(always)]
    fn from(variant: LPCMPBIASEM234H_A) -> Self {
        match variant {
            LPCMPBIASEM234H_A::BIAS0 => 0,
            LPCMPBIASEM234H_A::BIAS1 => 1,
            LPCMPBIASEM234H_A::BIAS2 => 2,
            LPCMPBIASEM234H_A::BIAS3 => 3,
        }
    }
}
#[doc = "Reader of field `LPCMPBIASEM234H`"]
pub type LPCMPBIASEM234H_R = crate::R<u8, LPCMPBIASEM234H_A>;
impl LPCMPBIASEM234H_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPCMPBIASEM234H_A {
        match self.bits {
            0 => LPCMPBIASEM234H_A::BIAS0,
            1 => LPCMPBIASEM234H_A::BIAS1,
            2 => LPCMPBIASEM234H_A::BIAS2,
            3 => LPCMPBIASEM234H_A::BIAS3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS0`"]
    #[inline(always)]
    pub fn is_bias0(&self) -> bool {
        *self == LPCMPBIASEM234H_A::BIAS0
    }
    #[doc = "Checks if the value of the field is `BIAS1`"]
    #[inline(always)]
    pub fn is_bias1(&self) -> bool {
        *self == LPCMPBIASEM234H_A::BIAS1
    }
    #[doc = "Checks if the value of the field is `BIAS2`"]
    #[inline(always)]
    pub fn is_bias2(&self) -> bool {
        *self == LPCMPBIASEM234H_A::BIAS2
    }
    #[doc = "Checks if the value of the field is `BIAS3`"]
    #[inline(always)]
    pub fn is_bias3(&self) -> bool {
        *self == LPCMPBIASEM234H_A::BIAS3
    }
}
#[doc = "Write proxy for field `LPCMPBIASEM234H`"]
pub struct LPCMPBIASEM234H_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCMPBIASEM234H_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPCMPBIASEM234H_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Maximum load current less than 75uA."]
    #[inline(always)]
    pub fn bias0(self) -> &'a mut W {
        self.variant(LPCMPBIASEM234H_A::BIAS0)
    }
    #[doc = "Maximum load current less than 500uA."]
    #[inline(always)]
    pub fn bias1(self) -> &'a mut W {
        self.variant(LPCMPBIASEM234H_A::BIAS1)
    }
    #[doc = "Maximum load current less than 2.5mA."]
    #[inline(always)]
    pub fn bias2(self) -> &'a mut W {
        self.variant(LPCMPBIASEM234H_A::BIAS2)
    }
    #[doc = "Maximum load current less than 10mA."]
    #[inline(always)]
    pub fn bias3(self) -> &'a mut W {
        self.variant(LPCMPBIASEM234H_A::BIAS3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Force DCDC Into CCM Mode in Low Noise Operation"]
    #[inline(always)]
    pub fn lnforceccm(&self) -> LNFORCECCM_R {
        LNFORCECCM_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Disable LP Mode Hysteresis in the State Machine Control"]
    #[inline(always)]
    pub fn lpcmphysdis(&self) -> LPCMPHYSDIS_R {
        LPCMPHYSDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Comparator Threshold on the High Side"]
    #[inline(always)]
    pub fn lpcmphyshi(&self) -> LPCMPHYSHI_R {
        LPCMPHYSHI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Force DCDC Into CCM Mode Immediately, Based on LNFORCECCM"]
    #[inline(always)]
    pub fn lnforceccmimm(&self) -> LNFORCECCMIMM_R {
        LNFORCECCMIMM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - PFET Switch Number Selection"]
    #[inline(always)]
    pub fn pfetcnt(&self) -> PFETCNT_R {
        PFETCNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - NFET Switch Number Selection"]
    #[inline(always)]
    pub fn nfetcnt(&self) -> NFETCNT_R {
        NFETCNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Current Limit in Bypass Mode"]
    #[inline(always)]
    pub fn byplimsel(&self) -> BYPLIMSEL_R {
        BYPLIMSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Current Limit Level Selection for Current Limiter in LP Mode"]
    #[inline(always)]
    pub fn lpclimilimsel(&self) -> LPCLIMILIMSEL_R {
        LPCLIMILIMSEL_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - Current Limit Level Selection for Current Limiter in LN Mode"]
    #[inline(always)]
    pub fn lnclimilimsel(&self) -> LNCLIMILIMSEL_R {
        LNCLIMILIMSEL_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 28:29 - LP Mode Comparator Bias Selection for EM23 or EM4H"]
    #[inline(always)]
    pub fn lpcmpbiasem234h(&self) -> LPCMPBIASEM234H_R {
        LPCMPBIASEM234H_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Force DCDC Into CCM Mode in Low Noise Operation"]
    #[inline(always)]
    pub fn lnforceccm(&mut self) -> LNFORCECCM_W {
        LNFORCECCM_W { w: self }
    }
    #[doc = "Bit 1 - Disable LP Mode Hysteresis in the State Machine Control"]
    #[inline(always)]
    pub fn lpcmphysdis(&mut self) -> LPCMPHYSDIS_W {
        LPCMPHYSDIS_W { w: self }
    }
    #[doc = "Bit 2 - Comparator Threshold on the High Side"]
    #[inline(always)]
    pub fn lpcmphyshi(&mut self) -> LPCMPHYSHI_W {
        LPCMPHYSHI_W { w: self }
    }
    #[doc = "Bit 5 - Force DCDC Into CCM Mode Immediately, Based on LNFORCECCM"]
    #[inline(always)]
    pub fn lnforceccmimm(&mut self) -> LNFORCECCMIMM_W {
        LNFORCECCMIMM_W { w: self }
    }
    #[doc = "Bits 8:11 - PFET Switch Number Selection"]
    #[inline(always)]
    pub fn pfetcnt(&mut self) -> PFETCNT_W {
        PFETCNT_W { w: self }
    }
    #[doc = "Bits 12:15 - NFET Switch Number Selection"]
    #[inline(always)]
    pub fn nfetcnt(&mut self) -> NFETCNT_W {
        NFETCNT_W { w: self }
    }
    #[doc = "Bits 16:19 - Current Limit in Bypass Mode"]
    #[inline(always)]
    pub fn byplimsel(&mut self) -> BYPLIMSEL_W {
        BYPLIMSEL_W { w: self }
    }
    #[doc = "Bits 20:22 - Current Limit Level Selection for Current Limiter in LP Mode"]
    #[inline(always)]
    pub fn lpclimilimsel(&mut self) -> LPCLIMILIMSEL_W {
        LPCLIMILIMSEL_W { w: self }
    }
    #[doc = "Bits 24:26 - Current Limit Level Selection for Current Limiter in LN Mode"]
    #[inline(always)]
    pub fn lnclimilimsel(&mut self) -> LNCLIMILIMSEL_W {
        LNCLIMILIMSEL_W { w: self }
    }
    #[doc = "Bits 28:29 - LP Mode Comparator Bias Selection for EM23 or EM4H"]
    #[inline(always)]
    pub fn lpcmpbiasem234h(&mut self) -> LPCMPBIASEM234H_W {
        LPCMPBIASEM234H_W { w: self }
    }
}
