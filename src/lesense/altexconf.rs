#[doc = "Reader of register ALTEXCONF"]
pub type R = crate::R<u32, super::ALTEXCONF>;
#[doc = "Writer for register ALTEXCONF"]
pub type W = crate::W<u32, super::ALTEXCONF>;
#[doc = "Register ALTEXCONF `reset()`'s with value 0"]
impl crate::ResetValue for super::ALTEXCONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "ALTEX0 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLECONF0_A {
    #[doc = "0: ALTEX0 output is disabled in idle phase"]
    DISABLE,
    #[doc = "1: ALTEX0 output is high in idle phase"]
    HIGH,
    #[doc = "2: ALTEX0 output is low in idle phase"]
    LOW,
}
impl From<IDLECONF0_A> for u8 {
    #[inline(always)]
    fn from(variant: IDLECONF0_A) -> Self {
        match variant {
            IDLECONF0_A::DISABLE => 0,
            IDLECONF0_A::HIGH => 1,
            IDLECONF0_A::LOW => 2,
        }
    }
}
#[doc = "Reader of field `IDLECONF0`"]
pub type IDLECONF0_R = crate::R<u8, IDLECONF0_A>;
impl IDLECONF0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, IDLECONF0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(IDLECONF0_A::DISABLE),
            1 => Val(IDLECONF0_A::HIGH),
            2 => Val(IDLECONF0_A::LOW),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IDLECONF0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IDLECONF0_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IDLECONF0_A::LOW
    }
}
#[doc = "Write proxy for field `IDLECONF0`"]
pub struct IDLECONF0_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLECONF0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDLECONF0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ALTEX0 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IDLECONF0_A::DISABLE)
    }
    #[doc = "ALTEX0 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(IDLECONF0_A::HIGH)
    }
    #[doc = "ALTEX0 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(IDLECONF0_A::LOW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "ALTEX1 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLECONF1_A {
    #[doc = "0: ALTEX1 output is disabled in idle phase"]
    DISABLE,
    #[doc = "1: ALTEX1 output is high in idle phase"]
    HIGH,
    #[doc = "2: ALTEX1 output is low in idle phase"]
    LOW,
}
impl From<IDLECONF1_A> for u8 {
    #[inline(always)]
    fn from(variant: IDLECONF1_A) -> Self {
        match variant {
            IDLECONF1_A::DISABLE => 0,
            IDLECONF1_A::HIGH => 1,
            IDLECONF1_A::LOW => 2,
        }
    }
}
#[doc = "Reader of field `IDLECONF1`"]
pub type IDLECONF1_R = crate::R<u8, IDLECONF1_A>;
impl IDLECONF1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, IDLECONF1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(IDLECONF1_A::DISABLE),
            1 => Val(IDLECONF1_A::HIGH),
            2 => Val(IDLECONF1_A::LOW),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IDLECONF1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IDLECONF1_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IDLECONF1_A::LOW
    }
}
#[doc = "Write proxy for field `IDLECONF1`"]
pub struct IDLECONF1_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLECONF1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDLECONF1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ALTEX1 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IDLECONF1_A::DISABLE)
    }
    #[doc = "ALTEX1 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(IDLECONF1_A::HIGH)
    }
    #[doc = "ALTEX1 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(IDLECONF1_A::LOW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "ALTEX2 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLECONF2_A {
    #[doc = "0: ALTEX2 output is disabled in idle phase"]
    DISABLE,
    #[doc = "1: ALTEX2 output is high in idle phase"]
    HIGH,
    #[doc = "2: ALTEX2 output is low in idle phase"]
    LOW,
}
impl From<IDLECONF2_A> for u8 {
    #[inline(always)]
    fn from(variant: IDLECONF2_A) -> Self {
        match variant {
            IDLECONF2_A::DISABLE => 0,
            IDLECONF2_A::HIGH => 1,
            IDLECONF2_A::LOW => 2,
        }
    }
}
#[doc = "Reader of field `IDLECONF2`"]
pub type IDLECONF2_R = crate::R<u8, IDLECONF2_A>;
impl IDLECONF2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, IDLECONF2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(IDLECONF2_A::DISABLE),
            1 => Val(IDLECONF2_A::HIGH),
            2 => Val(IDLECONF2_A::LOW),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IDLECONF2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IDLECONF2_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IDLECONF2_A::LOW
    }
}
#[doc = "Write proxy for field `IDLECONF2`"]
pub struct IDLECONF2_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLECONF2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDLECONF2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ALTEX2 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IDLECONF2_A::DISABLE)
    }
    #[doc = "ALTEX2 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(IDLECONF2_A::HIGH)
    }
    #[doc = "ALTEX2 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(IDLECONF2_A::LOW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "ALTEX3 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLECONF3_A {
    #[doc = "0: ALTEX3 output is disabled in idle phase"]
    DISABLE,
    #[doc = "1: ALTEX3 output is high in idle phase"]
    HIGH,
    #[doc = "2: ALTEX3 output is low in idle phase"]
    LOW,
}
impl From<IDLECONF3_A> for u8 {
    #[inline(always)]
    fn from(variant: IDLECONF3_A) -> Self {
        match variant {
            IDLECONF3_A::DISABLE => 0,
            IDLECONF3_A::HIGH => 1,
            IDLECONF3_A::LOW => 2,
        }
    }
}
#[doc = "Reader of field `IDLECONF3`"]
pub type IDLECONF3_R = crate::R<u8, IDLECONF3_A>;
impl IDLECONF3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, IDLECONF3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(IDLECONF3_A::DISABLE),
            1 => Val(IDLECONF3_A::HIGH),
            2 => Val(IDLECONF3_A::LOW),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IDLECONF3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IDLECONF3_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IDLECONF3_A::LOW
    }
}
#[doc = "Write proxy for field `IDLECONF3`"]
pub struct IDLECONF3_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLECONF3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDLECONF3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ALTEX3 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IDLECONF3_A::DISABLE)
    }
    #[doc = "ALTEX3 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(IDLECONF3_A::HIGH)
    }
    #[doc = "ALTEX3 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(IDLECONF3_A::LOW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "ALTEX4 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLECONF4_A {
    #[doc = "0: ALTEX4 output is disabled in idle phase"]
    DISABLE,
    #[doc = "1: ALTEX4 output is high in idle phase"]
    HIGH,
    #[doc = "2: ALTEX4 output is low in idle phase"]
    LOW,
}
impl From<IDLECONF4_A> for u8 {
    #[inline(always)]
    fn from(variant: IDLECONF4_A) -> Self {
        match variant {
            IDLECONF4_A::DISABLE => 0,
            IDLECONF4_A::HIGH => 1,
            IDLECONF4_A::LOW => 2,
        }
    }
}
#[doc = "Reader of field `IDLECONF4`"]
pub type IDLECONF4_R = crate::R<u8, IDLECONF4_A>;
impl IDLECONF4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, IDLECONF4_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(IDLECONF4_A::DISABLE),
            1 => Val(IDLECONF4_A::HIGH),
            2 => Val(IDLECONF4_A::LOW),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IDLECONF4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IDLECONF4_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IDLECONF4_A::LOW
    }
}
#[doc = "Write proxy for field `IDLECONF4`"]
pub struct IDLECONF4_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLECONF4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDLECONF4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ALTEX4 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IDLECONF4_A::DISABLE)
    }
    #[doc = "ALTEX4 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(IDLECONF4_A::HIGH)
    }
    #[doc = "ALTEX4 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(IDLECONF4_A::LOW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "ALTEX5 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLECONF5_A {
    #[doc = "0: ALTEX5 output is disabled in idle phase"]
    DISABLE,
    #[doc = "1: ALTEX5 output is high in idle phase"]
    HIGH,
    #[doc = "2: ALTEX5 output is low in idle phase"]
    LOW,
}
impl From<IDLECONF5_A> for u8 {
    #[inline(always)]
    fn from(variant: IDLECONF5_A) -> Self {
        match variant {
            IDLECONF5_A::DISABLE => 0,
            IDLECONF5_A::HIGH => 1,
            IDLECONF5_A::LOW => 2,
        }
    }
}
#[doc = "Reader of field `IDLECONF5`"]
pub type IDLECONF5_R = crate::R<u8, IDLECONF5_A>;
impl IDLECONF5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, IDLECONF5_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(IDLECONF5_A::DISABLE),
            1 => Val(IDLECONF5_A::HIGH),
            2 => Val(IDLECONF5_A::LOW),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IDLECONF5_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IDLECONF5_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IDLECONF5_A::LOW
    }
}
#[doc = "Write proxy for field `IDLECONF5`"]
pub struct IDLECONF5_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLECONF5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDLECONF5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ALTEX5 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IDLECONF5_A::DISABLE)
    }
    #[doc = "ALTEX5 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(IDLECONF5_A::HIGH)
    }
    #[doc = "ALTEX5 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(IDLECONF5_A::LOW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "ALTEX6 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLECONF6_A {
    #[doc = "0: ALTEX6 output is disabled in idle phase"]
    DISABLE,
    #[doc = "1: ALTEX6 output is high in idle phase"]
    HIGH,
    #[doc = "2: ALTEX6 output is low in idle phase"]
    LOW,
}
impl From<IDLECONF6_A> for u8 {
    #[inline(always)]
    fn from(variant: IDLECONF6_A) -> Self {
        match variant {
            IDLECONF6_A::DISABLE => 0,
            IDLECONF6_A::HIGH => 1,
            IDLECONF6_A::LOW => 2,
        }
    }
}
#[doc = "Reader of field `IDLECONF6`"]
pub type IDLECONF6_R = crate::R<u8, IDLECONF6_A>;
impl IDLECONF6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, IDLECONF6_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(IDLECONF6_A::DISABLE),
            1 => Val(IDLECONF6_A::HIGH),
            2 => Val(IDLECONF6_A::LOW),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IDLECONF6_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IDLECONF6_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IDLECONF6_A::LOW
    }
}
#[doc = "Write proxy for field `IDLECONF6`"]
pub struct IDLECONF6_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLECONF6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDLECONF6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ALTEX6 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IDLECONF6_A::DISABLE)
    }
    #[doc = "ALTEX6 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(IDLECONF6_A::HIGH)
    }
    #[doc = "ALTEX6 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(IDLECONF6_A::LOW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "ALTEX7 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IDLECONF7_A {
    #[doc = "0: ALTEX7 output is disabled in idle phase"]
    DISABLE,
    #[doc = "1: ALTEX7 output is high in idle phase"]
    HIGH,
    #[doc = "2: ALTEX7 output is low in idle phase"]
    LOW,
}
impl From<IDLECONF7_A> for u8 {
    #[inline(always)]
    fn from(variant: IDLECONF7_A) -> Self {
        match variant {
            IDLECONF7_A::DISABLE => 0,
            IDLECONF7_A::HIGH => 1,
            IDLECONF7_A::LOW => 2,
        }
    }
}
#[doc = "Reader of field `IDLECONF7`"]
pub type IDLECONF7_R = crate::R<u8, IDLECONF7_A>;
impl IDLECONF7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, IDLECONF7_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(IDLECONF7_A::DISABLE),
            1 => Val(IDLECONF7_A::HIGH),
            2 => Val(IDLECONF7_A::LOW),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == IDLECONF7_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == IDLECONF7_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == IDLECONF7_A::LOW
    }
}
#[doc = "Write proxy for field `IDLECONF7`"]
pub struct IDLECONF7_W<'a> {
    w: &'a mut W,
}
impl<'a> IDLECONF7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IDLECONF7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "ALTEX7 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(IDLECONF7_A::DISABLE)
    }
    #[doc = "ALTEX7 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(IDLECONF7_A::HIGH)
    }
    #[doc = "ALTEX7 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(IDLECONF7_A::LOW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `AEX0`"]
pub type AEX0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AEX0`"]
pub struct AEX0_W<'a> {
    w: &'a mut W,
}
impl<'a> AEX0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `AEX1`"]
pub type AEX1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AEX1`"]
pub struct AEX1_W<'a> {
    w: &'a mut W,
}
impl<'a> AEX1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `AEX2`"]
pub type AEX2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AEX2`"]
pub struct AEX2_W<'a> {
    w: &'a mut W,
}
impl<'a> AEX2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `AEX3`"]
pub type AEX3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AEX3`"]
pub struct AEX3_W<'a> {
    w: &'a mut W,
}
impl<'a> AEX3_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `AEX4`"]
pub type AEX4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AEX4`"]
pub struct AEX4_W<'a> {
    w: &'a mut W,
}
impl<'a> AEX4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `AEX5`"]
pub type AEX5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AEX5`"]
pub struct AEX5_W<'a> {
    w: &'a mut W,
}
impl<'a> AEX5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `AEX6`"]
pub type AEX6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AEX6`"]
pub struct AEX6_W<'a> {
    w: &'a mut W,
}
impl<'a> AEX6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `AEX7`"]
pub type AEX7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AEX7`"]
pub struct AEX7_W<'a> {
    w: &'a mut W,
}
impl<'a> AEX7_W<'a> {
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
    #[doc = "Bits 0:1 - ALTEX0 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf0(&self) -> IDLECONF0_R {
        IDLECONF0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - ALTEX1 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf1(&self) -> IDLECONF1_R {
        IDLECONF1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - ALTEX2 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf2(&self) -> IDLECONF2_R {
        IDLECONF2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - ALTEX3 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf3(&self) -> IDLECONF3_R {
        IDLECONF3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - ALTEX4 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf4(&self) -> IDLECONF4_R {
        IDLECONF4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - ALTEX5 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf5(&self) -> IDLECONF5_R {
        IDLECONF5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - ALTEX6 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf6(&self) -> IDLECONF6_R {
        IDLECONF6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - ALTEX7 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf7(&self) -> IDLECONF7_R {
        IDLECONF7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - ALTEX0 Always Excite Enable"]
    #[inline(always)]
    pub fn aex0(&self) -> AEX0_R {
        AEX0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ALTEX1 Always Excite Enable"]
    #[inline(always)]
    pub fn aex1(&self) -> AEX1_R {
        AEX1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ALTEX2 Always Excite Enable"]
    #[inline(always)]
    pub fn aex2(&self) -> AEX2_R {
        AEX2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - ALTEX3 Always Excite Enable"]
    #[inline(always)]
    pub fn aex3(&self) -> AEX3_R {
        AEX3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - ALTEX4 Always Excite Enable"]
    #[inline(always)]
    pub fn aex4(&self) -> AEX4_R {
        AEX4_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - ALTEX5 Always Excite Enable"]
    #[inline(always)]
    pub fn aex5(&self) -> AEX5_R {
        AEX5_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - ALTEX6 Always Excite Enable"]
    #[inline(always)]
    pub fn aex6(&self) -> AEX6_R {
        AEX6_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - ALTEX7 Always Excite Enable"]
    #[inline(always)]
    pub fn aex7(&self) -> AEX7_R {
        AEX7_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - ALTEX0 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf0(&mut self) -> IDLECONF0_W {
        IDLECONF0_W { w: self }
    }
    #[doc = "Bits 2:3 - ALTEX1 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf1(&mut self) -> IDLECONF1_W {
        IDLECONF1_W { w: self }
    }
    #[doc = "Bits 4:5 - ALTEX2 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf2(&mut self) -> IDLECONF2_W {
        IDLECONF2_W { w: self }
    }
    #[doc = "Bits 6:7 - ALTEX3 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf3(&mut self) -> IDLECONF3_W {
        IDLECONF3_W { w: self }
    }
    #[doc = "Bits 8:9 - ALTEX4 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf4(&mut self) -> IDLECONF4_W {
        IDLECONF4_W { w: self }
    }
    #[doc = "Bits 10:11 - ALTEX5 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf5(&mut self) -> IDLECONF5_W {
        IDLECONF5_W { w: self }
    }
    #[doc = "Bits 12:13 - ALTEX6 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf6(&mut self) -> IDLECONF6_W {
        IDLECONF6_W { w: self }
    }
    #[doc = "Bits 14:15 - ALTEX7 Idle Phase Configuration"]
    #[inline(always)]
    pub fn idleconf7(&mut self) -> IDLECONF7_W {
        IDLECONF7_W { w: self }
    }
    #[doc = "Bit 16 - ALTEX0 Always Excite Enable"]
    #[inline(always)]
    pub fn aex0(&mut self) -> AEX0_W {
        AEX0_W { w: self }
    }
    #[doc = "Bit 17 - ALTEX1 Always Excite Enable"]
    #[inline(always)]
    pub fn aex1(&mut self) -> AEX1_W {
        AEX1_W { w: self }
    }
    #[doc = "Bit 18 - ALTEX2 Always Excite Enable"]
    #[inline(always)]
    pub fn aex2(&mut self) -> AEX2_W {
        AEX2_W { w: self }
    }
    #[doc = "Bit 19 - ALTEX3 Always Excite Enable"]
    #[inline(always)]
    pub fn aex3(&mut self) -> AEX3_W {
        AEX3_W { w: self }
    }
    #[doc = "Bit 20 - ALTEX4 Always Excite Enable"]
    #[inline(always)]
    pub fn aex4(&mut self) -> AEX4_W {
        AEX4_W { w: self }
    }
    #[doc = "Bit 21 - ALTEX5 Always Excite Enable"]
    #[inline(always)]
    pub fn aex5(&mut self) -> AEX5_W {
        AEX5_W { w: self }
    }
    #[doc = "Bit 22 - ALTEX6 Always Excite Enable"]
    #[inline(always)]
    pub fn aex6(&mut self) -> AEX6_W {
        AEX6_W { w: self }
    }
    #[doc = "Bit 23 - ALTEX7 Always Excite Enable"]
    #[inline(always)]
    pub fn aex7(&mut self) -> AEX7_W {
        AEX7_W { w: self }
    }
}
