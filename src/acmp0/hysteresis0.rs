#[doc = "Reader of register HYSTERESIS0"]
pub type R = crate::R<u32, super::HYSTERESIS0>;
#[doc = "Writer for register HYSTERESIS0"]
pub type W = crate::W<u32, super::HYSTERESIS0>;
#[doc = "Register HYSTERESIS0 `reset()`'s with value 0"]
impl crate::ResetValue for super::HYSTERESIS0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Hysteresis Select When ACMPOUT=0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HYST_A {
    #[doc = "0: No hysteresis"]
    HYST0,
    #[doc = "1: 14 mV hysteresis"]
    HYST1,
    #[doc = "2: 25 mV hysteresis"]
    HYST2,
    #[doc = "3: 30 mV hysteresis"]
    HYST3,
    #[doc = "4: 35 mV hysteresis"]
    HYST4,
    #[doc = "5: 39 mV hysteresis"]
    HYST5,
    #[doc = "6: 42 mV hysteresis"]
    HYST6,
    #[doc = "7: 45 mV hysteresis"]
    HYST7,
    #[doc = "8: No hysteresis"]
    HYST8,
    #[doc = "9: -14 mV hysteresis"]
    HYST9,
    #[doc = "10: -25 mV hysteresis"]
    HYST10,
    #[doc = "11: -30 mV hysteresis"]
    HYST11,
    #[doc = "12: -35 mV hysteresis"]
    HYST12,
    #[doc = "13: -39 mV hysteresis"]
    HYST13,
    #[doc = "14: -42 mV hysteresis"]
    HYST14,
    #[doc = "15: -45 mV hysteresis"]
    HYST15,
}
impl From<HYST_A> for u8 {
    #[inline(always)]
    fn from(variant: HYST_A) -> Self {
        match variant {
            HYST_A::HYST0 => 0,
            HYST_A::HYST1 => 1,
            HYST_A::HYST2 => 2,
            HYST_A::HYST3 => 3,
            HYST_A::HYST4 => 4,
            HYST_A::HYST5 => 5,
            HYST_A::HYST6 => 6,
            HYST_A::HYST7 => 7,
            HYST_A::HYST8 => 8,
            HYST_A::HYST9 => 9,
            HYST_A::HYST10 => 10,
            HYST_A::HYST11 => 11,
            HYST_A::HYST12 => 12,
            HYST_A::HYST13 => 13,
            HYST_A::HYST14 => 14,
            HYST_A::HYST15 => 15,
        }
    }
}
#[doc = "Reader of field `HYST`"]
pub type HYST_R = crate::R<u8, HYST_A>;
impl HYST_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYST_A {
        match self.bits {
            0 => HYST_A::HYST0,
            1 => HYST_A::HYST1,
            2 => HYST_A::HYST2,
            3 => HYST_A::HYST3,
            4 => HYST_A::HYST4,
            5 => HYST_A::HYST5,
            6 => HYST_A::HYST6,
            7 => HYST_A::HYST7,
            8 => HYST_A::HYST8,
            9 => HYST_A::HYST9,
            10 => HYST_A::HYST10,
            11 => HYST_A::HYST11,
            12 => HYST_A::HYST12,
            13 => HYST_A::HYST13,
            14 => HYST_A::HYST14,
            15 => HYST_A::HYST15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HYST0`"]
    #[inline(always)]
    pub fn is_hyst0(&self) -> bool {
        *self == HYST_A::HYST0
    }
    #[doc = "Checks if the value of the field is `HYST1`"]
    #[inline(always)]
    pub fn is_hyst1(&self) -> bool {
        *self == HYST_A::HYST1
    }
    #[doc = "Checks if the value of the field is `HYST2`"]
    #[inline(always)]
    pub fn is_hyst2(&self) -> bool {
        *self == HYST_A::HYST2
    }
    #[doc = "Checks if the value of the field is `HYST3`"]
    #[inline(always)]
    pub fn is_hyst3(&self) -> bool {
        *self == HYST_A::HYST3
    }
    #[doc = "Checks if the value of the field is `HYST4`"]
    #[inline(always)]
    pub fn is_hyst4(&self) -> bool {
        *self == HYST_A::HYST4
    }
    #[doc = "Checks if the value of the field is `HYST5`"]
    #[inline(always)]
    pub fn is_hyst5(&self) -> bool {
        *self == HYST_A::HYST5
    }
    #[doc = "Checks if the value of the field is `HYST6`"]
    #[inline(always)]
    pub fn is_hyst6(&self) -> bool {
        *self == HYST_A::HYST6
    }
    #[doc = "Checks if the value of the field is `HYST7`"]
    #[inline(always)]
    pub fn is_hyst7(&self) -> bool {
        *self == HYST_A::HYST7
    }
    #[doc = "Checks if the value of the field is `HYST8`"]
    #[inline(always)]
    pub fn is_hyst8(&self) -> bool {
        *self == HYST_A::HYST8
    }
    #[doc = "Checks if the value of the field is `HYST9`"]
    #[inline(always)]
    pub fn is_hyst9(&self) -> bool {
        *self == HYST_A::HYST9
    }
    #[doc = "Checks if the value of the field is `HYST10`"]
    #[inline(always)]
    pub fn is_hyst10(&self) -> bool {
        *self == HYST_A::HYST10
    }
    #[doc = "Checks if the value of the field is `HYST11`"]
    #[inline(always)]
    pub fn is_hyst11(&self) -> bool {
        *self == HYST_A::HYST11
    }
    #[doc = "Checks if the value of the field is `HYST12`"]
    #[inline(always)]
    pub fn is_hyst12(&self) -> bool {
        *self == HYST_A::HYST12
    }
    #[doc = "Checks if the value of the field is `HYST13`"]
    #[inline(always)]
    pub fn is_hyst13(&self) -> bool {
        *self == HYST_A::HYST13
    }
    #[doc = "Checks if the value of the field is `HYST14`"]
    #[inline(always)]
    pub fn is_hyst14(&self) -> bool {
        *self == HYST_A::HYST14
    }
    #[doc = "Checks if the value of the field is `HYST15`"]
    #[inline(always)]
    pub fn is_hyst15(&self) -> bool {
        *self == HYST_A::HYST15
    }
}
#[doc = "Write proxy for field `HYST`"]
pub struct HYST_W<'a> {
    w: &'a mut W,
}
impl<'a> HYST_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HYST_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn hyst0(self) -> &'a mut W {
        self.variant(HYST_A::HYST0)
    }
    #[doc = "14 mV hysteresis"]
    #[inline(always)]
    pub fn hyst1(self) -> &'a mut W {
        self.variant(HYST_A::HYST1)
    }
    #[doc = "25 mV hysteresis"]
    #[inline(always)]
    pub fn hyst2(self) -> &'a mut W {
        self.variant(HYST_A::HYST2)
    }
    #[doc = "30 mV hysteresis"]
    #[inline(always)]
    pub fn hyst3(self) -> &'a mut W {
        self.variant(HYST_A::HYST3)
    }
    #[doc = "35 mV hysteresis"]
    #[inline(always)]
    pub fn hyst4(self) -> &'a mut W {
        self.variant(HYST_A::HYST4)
    }
    #[doc = "39 mV hysteresis"]
    #[inline(always)]
    pub fn hyst5(self) -> &'a mut W {
        self.variant(HYST_A::HYST5)
    }
    #[doc = "42 mV hysteresis"]
    #[inline(always)]
    pub fn hyst6(self) -> &'a mut W {
        self.variant(HYST_A::HYST6)
    }
    #[doc = "45 mV hysteresis"]
    #[inline(always)]
    pub fn hyst7(self) -> &'a mut W {
        self.variant(HYST_A::HYST7)
    }
    #[doc = "No hysteresis"]
    #[inline(always)]
    pub fn hyst8(self) -> &'a mut W {
        self.variant(HYST_A::HYST8)
    }
    #[doc = "-14 mV hysteresis"]
    #[inline(always)]
    pub fn hyst9(self) -> &'a mut W {
        self.variant(HYST_A::HYST9)
    }
    #[doc = "-25 mV hysteresis"]
    #[inline(always)]
    pub fn hyst10(self) -> &'a mut W {
        self.variant(HYST_A::HYST10)
    }
    #[doc = "-30 mV hysteresis"]
    #[inline(always)]
    pub fn hyst11(self) -> &'a mut W {
        self.variant(HYST_A::HYST11)
    }
    #[doc = "-35 mV hysteresis"]
    #[inline(always)]
    pub fn hyst12(self) -> &'a mut W {
        self.variant(HYST_A::HYST12)
    }
    #[doc = "-39 mV hysteresis"]
    #[inline(always)]
    pub fn hyst13(self) -> &'a mut W {
        self.variant(HYST_A::HYST13)
    }
    #[doc = "-42 mV hysteresis"]
    #[inline(always)]
    pub fn hyst14(self) -> &'a mut W {
        self.variant(HYST_A::HYST14)
    }
    #[doc = "-45 mV hysteresis"]
    #[inline(always)]
    pub fn hyst15(self) -> &'a mut W {
        self.variant(HYST_A::HYST15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Reader of field `DIVVA`"]
pub type DIVVA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIVVA`"]
pub struct DIVVA_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVVA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `DIVVB`"]
pub type DIVVB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIVVB`"]
pub struct DIVVB_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVVB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Hysteresis Select When ACMPOUT=0"]
    #[inline(always)]
    pub fn hyst(&self) -> HYST_R {
        HYST_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Divider for VA Voltage When ACMPOUT=0"]
    #[inline(always)]
    pub fn divva(&self) -> DIVVA_R {
        DIVVA_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Divider for VB Voltage When ACMPOUT=0"]
    #[inline(always)]
    pub fn divvb(&self) -> DIVVB_R {
        DIVVB_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Hysteresis Select When ACMPOUT=0"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HYST_W {
        HYST_W { w: self }
    }
    #[doc = "Bits 16:21 - Divider for VA Voltage When ACMPOUT=0"]
    #[inline(always)]
    pub fn divva(&mut self) -> DIVVA_W {
        DIVVA_W { w: self }
    }
    #[doc = "Bits 24:29 - Divider for VB Voltage When ACMPOUT=0"]
    #[inline(always)]
    pub fn divvb(&mut self) -> DIVVB_W {
        DIVVB_W { w: self }
    }
}
