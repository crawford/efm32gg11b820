#[doc = "Reader of register CURPROG"]
pub type R = crate::R<u32, super::CURPROG>;
#[doc = "Writer for register CURPROG"]
pub type W = crate::W<u32, super::CURPROG>;
#[doc = "Register CURPROG `reset()`'s with value 0x009b_0000"]
impl crate::ResetValue for super::CURPROG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x009b_0000
    }
}
#[doc = "Current Range Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RANGESEL_A {
    #[doc = "0: Current range set to 0 - 1.6 uA."]
    RANGE0,
    #[doc = "1: Current range set to 1.6 - 4.7 uA."]
    RANGE1,
    #[doc = "2: Current range set to 0.5 - 16 uA."]
    RANGE2,
    #[doc = "3: Current range set to 2 - 64 uA."]
    RANGE3,
}
impl From<RANGESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RANGESEL_A) -> Self {
        match variant {
            RANGESEL_A::RANGE0 => 0,
            RANGESEL_A::RANGE1 => 1,
            RANGESEL_A::RANGE2 => 2,
            RANGESEL_A::RANGE3 => 3,
        }
    }
}
#[doc = "Reader of field `RANGESEL`"]
pub type RANGESEL_R = crate::R<u8, RANGESEL_A>;
impl RANGESEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RANGESEL_A {
        match self.bits {
            0 => RANGESEL_A::RANGE0,
            1 => RANGESEL_A::RANGE1,
            2 => RANGESEL_A::RANGE2,
            3 => RANGESEL_A::RANGE3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RANGE0`"]
    #[inline(always)]
    pub fn is_range0(&self) -> bool {
        *self == RANGESEL_A::RANGE0
    }
    #[doc = "Checks if the value of the field is `RANGE1`"]
    #[inline(always)]
    pub fn is_range1(&self) -> bool {
        *self == RANGESEL_A::RANGE1
    }
    #[doc = "Checks if the value of the field is `RANGE2`"]
    #[inline(always)]
    pub fn is_range2(&self) -> bool {
        *self == RANGESEL_A::RANGE2
    }
    #[doc = "Checks if the value of the field is `RANGE3`"]
    #[inline(always)]
    pub fn is_range3(&self) -> bool {
        *self == RANGESEL_A::RANGE3
    }
}
#[doc = "Write proxy for field `RANGESEL`"]
pub struct RANGESEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RANGESEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RANGESEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Current range set to 0 - 1.6 uA."]
    #[inline(always)]
    pub fn range0(self) -> &'a mut W {
        self.variant(RANGESEL_A::RANGE0)
    }
    #[doc = "Current range set to 1.6 - 4.7 uA."]
    #[inline(always)]
    pub fn range1(self) -> &'a mut W {
        self.variant(RANGESEL_A::RANGE1)
    }
    #[doc = "Current range set to 0.5 - 16 uA."]
    #[inline(always)]
    pub fn range2(self) -> &'a mut W {
        self.variant(RANGESEL_A::RANGE2)
    }
    #[doc = "Current range set to 2 - 64 uA."]
    #[inline(always)]
    pub fn range3(self) -> &'a mut W {
        self.variant(RANGESEL_A::RANGE3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `STEPSEL`"]
pub type STEPSEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STEPSEL`"]
pub struct STEPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> STEPSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `TUNING`"]
pub type TUNING_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TUNING`"]
pub struct TUNING_W<'a> {
    w: &'a mut W,
}
impl<'a> TUNING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Current Range Select"]
    #[inline(always)]
    pub fn rangesel(&self) -> RANGESEL_R {
        RANGESEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:12 - Current Step Size Select"]
    #[inline(always)]
    pub fn stepsel(&self) -> STEPSEL_R {
        STEPSEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - Tune the Current to Given Accuracy"]
    #[inline(always)]
    pub fn tuning(&self) -> TUNING_R {
        TUNING_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Current Range Select"]
    #[inline(always)]
    pub fn rangesel(&mut self) -> RANGESEL_W {
        RANGESEL_W { w: self }
    }
    #[doc = "Bits 8:12 - Current Step Size Select"]
    #[inline(always)]
    pub fn stepsel(&mut self) -> STEPSEL_W {
        STEPSEL_W { w: self }
    }
    #[doc = "Bits 16:23 - Tune the Current to Given Accuracy"]
    #[inline(always)]
    pub fn tuning(&mut self) -> TUNING_W {
        TUNING_W { w: self }
    }
}
