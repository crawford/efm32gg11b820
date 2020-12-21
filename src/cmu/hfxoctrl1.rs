#[doc = "Reader of register HFXOCTRL1"]
pub type R = crate::R<u32, super::HFXOCTRL1>;
#[doc = "Writer for register HFXOCTRL1"]
pub type W = crate::W<u32, super::HFXOCTRL1>;
#[doc = "Register HFXOCTRL1 `reset()`'s with value 0x2000"]
impl crate::ResetValue for super::HFXOCTRL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x2000
    }
}
#[doc = "Sets the Amplitude Detection Level (mV)\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PEAKDETTHR_A {
    #[doc = "0: 50mV amplitude detection level"]
    THR0 = 0,
    #[doc = "1: 75mV amplitude detection level"]
    THR1 = 1,
    #[doc = "2: 115mV amplitude detection level"]
    THR2 = 2,
    #[doc = "3: 160mV amplitude detection level"]
    THR3 = 3,
    #[doc = "4: 220mV amplitude detection level"]
    THR4 = 4,
    #[doc = "5: 260mV amplitude detection level"]
    THR5 = 5,
    #[doc = "6: 320mV amplitude detection level"]
    THR6 = 6,
    #[doc = "7: Same as THR6"]
    THR7 = 7,
}
impl From<PEAKDETTHR_A> for u8 {
    #[inline(always)]
    fn from(variant: PEAKDETTHR_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PEAKDETTHR`"]
pub type PEAKDETTHR_R = crate::R<u8, PEAKDETTHR_A>;
impl PEAKDETTHR_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEAKDETTHR_A {
        match self.bits {
            0 => PEAKDETTHR_A::THR0,
            1 => PEAKDETTHR_A::THR1,
            2 => PEAKDETTHR_A::THR2,
            3 => PEAKDETTHR_A::THR3,
            4 => PEAKDETTHR_A::THR4,
            5 => PEAKDETTHR_A::THR5,
            6 => PEAKDETTHR_A::THR6,
            7 => PEAKDETTHR_A::THR7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `THR0`"]
    #[inline(always)]
    pub fn is_thr0(&self) -> bool {
        *self == PEAKDETTHR_A::THR0
    }
    #[doc = "Checks if the value of the field is `THR1`"]
    #[inline(always)]
    pub fn is_thr1(&self) -> bool {
        *self == PEAKDETTHR_A::THR1
    }
    #[doc = "Checks if the value of the field is `THR2`"]
    #[inline(always)]
    pub fn is_thr2(&self) -> bool {
        *self == PEAKDETTHR_A::THR2
    }
    #[doc = "Checks if the value of the field is `THR3`"]
    #[inline(always)]
    pub fn is_thr3(&self) -> bool {
        *self == PEAKDETTHR_A::THR3
    }
    #[doc = "Checks if the value of the field is `THR4`"]
    #[inline(always)]
    pub fn is_thr4(&self) -> bool {
        *self == PEAKDETTHR_A::THR4
    }
    #[doc = "Checks if the value of the field is `THR5`"]
    #[inline(always)]
    pub fn is_thr5(&self) -> bool {
        *self == PEAKDETTHR_A::THR5
    }
    #[doc = "Checks if the value of the field is `THR6`"]
    #[inline(always)]
    pub fn is_thr6(&self) -> bool {
        *self == PEAKDETTHR_A::THR6
    }
    #[doc = "Checks if the value of the field is `THR7`"]
    #[inline(always)]
    pub fn is_thr7(&self) -> bool {
        *self == PEAKDETTHR_A::THR7
    }
}
#[doc = "Write proxy for field `PEAKDETTHR`"]
pub struct PEAKDETTHR_W<'a> {
    w: &'a mut W,
}
impl<'a> PEAKDETTHR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PEAKDETTHR_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "50mV amplitude detection level"]
    #[inline(always)]
    pub fn thr0(self) -> &'a mut W {
        self.variant(PEAKDETTHR_A::THR0)
    }
    #[doc = "75mV amplitude detection level"]
    #[inline(always)]
    pub fn thr1(self) -> &'a mut W {
        self.variant(PEAKDETTHR_A::THR1)
    }
    #[doc = "115mV amplitude detection level"]
    #[inline(always)]
    pub fn thr2(self) -> &'a mut W {
        self.variant(PEAKDETTHR_A::THR2)
    }
    #[doc = "160mV amplitude detection level"]
    #[inline(always)]
    pub fn thr3(self) -> &'a mut W {
        self.variant(PEAKDETTHR_A::THR3)
    }
    #[doc = "220mV amplitude detection level"]
    #[inline(always)]
    pub fn thr4(self) -> &'a mut W {
        self.variant(PEAKDETTHR_A::THR4)
    }
    #[doc = "260mV amplitude detection level"]
    #[inline(always)]
    pub fn thr5(self) -> &'a mut W {
        self.variant(PEAKDETTHR_A::THR5)
    }
    #[doc = "320mV amplitude detection level"]
    #[inline(always)]
    pub fn thr6(self) -> &'a mut W {
        self.variant(PEAKDETTHR_A::THR6)
    }
    #[doc = "Same as THR6"]
    #[inline(always)]
    pub fn thr7(self) -> &'a mut W {
        self.variant(PEAKDETTHR_A::THR7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 12:14 - Sets the Amplitude Detection Level (mV)"]
    #[inline(always)]
    pub fn peakdetthr(&self) -> PEAKDETTHR_R {
        PEAKDETTHR_R::new(((self.bits >> 12) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 12:14 - Sets the Amplitude Detection Level (mV)"]
    #[inline(always)]
    pub fn peakdetthr(&mut self) -> PEAKDETTHR_W {
        PEAKDETTHR_W { w: self }
    }
}
