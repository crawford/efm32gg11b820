#[doc = "Reader of register DCDCLPEM01CFG"]
pub type R = crate::R<u32, super::DCDCLPEM01CFG>;
#[doc = "Writer for register DCDCLPEM01CFG"]
pub type W = crate::W<u32, super::DCDCLPEM01CFG>;
#[doc = "Register DCDCLPEM01CFG `reset()`'s with value 0x0300"]
impl crate::ResetValue for super::DCDCLPEM01CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0300
    }
}
#[doc = "LP Mode Comparator Bias Selection for EM01\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPCMPBIASEM01_A {
    #[doc = "0: Maximum load current less than 75uA."]
    BIAS0 = 0,
    #[doc = "1: Maximum load current less than 500uA."]
    BIAS1 = 1,
    #[doc = "2: Maximum load current less than 2.5mA."]
    BIAS2 = 2,
    #[doc = "3: Maximum load current less than 10mA."]
    BIAS3 = 3,
}
impl From<LPCMPBIASEM01_A> for u8 {
    #[inline(always)]
    fn from(variant: LPCMPBIASEM01_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LPCMPBIASEM01`"]
pub type LPCMPBIASEM01_R = crate::R<u8, LPCMPBIASEM01_A>;
impl LPCMPBIASEM01_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPCMPBIASEM01_A {
        match self.bits {
            0 => LPCMPBIASEM01_A::BIAS0,
            1 => LPCMPBIASEM01_A::BIAS1,
            2 => LPCMPBIASEM01_A::BIAS2,
            3 => LPCMPBIASEM01_A::BIAS3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS0`"]
    #[inline(always)]
    pub fn is_bias0(&self) -> bool {
        *self == LPCMPBIASEM01_A::BIAS0
    }
    #[doc = "Checks if the value of the field is `BIAS1`"]
    #[inline(always)]
    pub fn is_bias1(&self) -> bool {
        *self == LPCMPBIASEM01_A::BIAS1
    }
    #[doc = "Checks if the value of the field is `BIAS2`"]
    #[inline(always)]
    pub fn is_bias2(&self) -> bool {
        *self == LPCMPBIASEM01_A::BIAS2
    }
    #[doc = "Checks if the value of the field is `BIAS3`"]
    #[inline(always)]
    pub fn is_bias3(&self) -> bool {
        *self == LPCMPBIASEM01_A::BIAS3
    }
}
#[doc = "Write proxy for field `LPCMPBIASEM01`"]
pub struct LPCMPBIASEM01_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCMPBIASEM01_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPCMPBIASEM01_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Maximum load current less than 75uA."]
    #[inline(always)]
    pub fn bias0(self) -> &'a mut W {
        self.variant(LPCMPBIASEM01_A::BIAS0)
    }
    #[doc = "Maximum load current less than 500uA."]
    #[inline(always)]
    pub fn bias1(self) -> &'a mut W {
        self.variant(LPCMPBIASEM01_A::BIAS1)
    }
    #[doc = "Maximum load current less than 2.5mA."]
    #[inline(always)]
    pub fn bias2(self) -> &'a mut W {
        self.variant(LPCMPBIASEM01_A::BIAS2)
    }
    #[doc = "Maximum load current less than 10mA."]
    #[inline(always)]
    pub fn bias3(self) -> &'a mut W {
        self.variant(LPCMPBIASEM01_A::BIAS3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `LPCMPHYSSELEM01`"]
pub type LPCMPHYSSELEM01_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPCMPHYSSELEM01`"]
pub struct LPCMPHYSSELEM01_W<'a> {
    w: &'a mut W,
}
impl<'a> LPCMPHYSSELEM01_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:9 - LP Mode Comparator Bias Selection for EM01"]
    #[inline(always)]
    pub fn lpcmpbiasem01(&self) -> LPCMPBIASEM01_R {
        LPCMPBIASEM01_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:15 - LP Mode Hysteresis Selection for EM01"]
    #[inline(always)]
    pub fn lpcmphysselem01(&self) -> LPCMPHYSSELEM01_R {
        LPCMPHYSSELEM01_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - LP Mode Comparator Bias Selection for EM01"]
    #[inline(always)]
    pub fn lpcmpbiasem01(&mut self) -> LPCMPBIASEM01_W {
        LPCMPBIASEM01_W { w: self }
    }
    #[doc = "Bits 12:15 - LP Mode Hysteresis Selection for EM01"]
    #[inline(always)]
    pub fn lpcmphysselem01(&mut self) -> LPCMPHYSSELEM01_W {
        LPCMPHYSSELEM01_W { w: self }
    }
}
