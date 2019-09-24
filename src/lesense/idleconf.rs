#[doc = "Reader of register IDLECONF"]
pub type R = crate::R<u32, super::IDLECONF>;
#[doc = "Writer for register IDLECONF"]
pub type W = crate::W<u32, super::IDLECONF>;
#[doc = "Register IDLECONF `reset()`'s with value 0"]
impl crate::ResetValue for super::IDLECONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Channel 0 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH0_A {
    #[doc = "0: CH0 output is disabled in idle phase"]
    DISABLE,
    #[doc = "1: CH0 output is high in idle phase"]
    HIGH,
    #[doc = "2: CH0 output is low in idle phase"]
    LOW,
    #[doc = "3: CH0 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl From<CH0_A> for u8 {
    #[inline(always)]
    fn from(variant: CH0_A) -> Self {
        match variant {
            CH0_A::DISABLE => 0,
            CH0_A::HIGH => 1,
            CH0_A::LOW => 2,
            CH0_A::DAC => 3,
        }
    }
}
#[doc = "Reader of field `CH0`"]
pub type CH0_R = crate::R<u8, CH0_A>;
impl CH0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH0_A {
        match self.bits {
            0 => CH0_A::DISABLE,
            1 => CH0_A::HIGH,
            2 => CH0_A::LOW,
            3 => CH0_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH0_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH0_A::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CH0_A::DAC
    }
}
#[doc = "Write proxy for field `CH0`"]
pub struct CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CH0 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH0_A::DISABLE)
    }
    #[doc = "CH0 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CH0_A::HIGH)
    }
    #[doc = "CH0 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CH0_A::LOW)
    }
    #[doc = "CH0 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(CH0_A::DAC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Channel 1 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH1_A {
    #[doc = "0: CH1 output is disabled in idle phase"]
    DISABLE,
    #[doc = "1: CH1 output is high in idle phase"]
    HIGH,
    #[doc = "2: CH1 output is low in idle phase"]
    LOW,
    #[doc = "3: CH1 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl From<CH1_A> for u8 {
    #[inline(always)]
    fn from(variant: CH1_A) -> Self {
        match variant {
            CH1_A::DISABLE => 0,
            CH1_A::HIGH => 1,
            CH1_A::LOW => 2,
            CH1_A::DAC => 3,
        }
    }
}
#[doc = "Reader of field `CH1`"]
pub type CH1_R = crate::R<u8, CH1_A>;
impl CH1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH1_A {
        match self.bits {
            0 => CH1_A::DISABLE,
            1 => CH1_A::HIGH,
            2 => CH1_A::LOW,
            3 => CH1_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH1_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH1_A::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CH1_A::DAC
    }
}
#[doc = "Write proxy for field `CH1`"]
pub struct CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CH1 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH1_A::DISABLE)
    }
    #[doc = "CH1 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CH1_A::HIGH)
    }
    #[doc = "CH1 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CH1_A::LOW)
    }
    #[doc = "CH1 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(CH1_A::DAC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Channel 2 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH2_A {
    #[doc = "0: CH2 output is disabled in idle phase"]
    DISABLE,
    #[doc = "1: CH2 output is high in idle phase"]
    HIGH,
    #[doc = "2: CH2 output is low in idle phase"]
    LOW,
    #[doc = "3: CH2 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl From<CH2_A> for u8 {
    #[inline(always)]
    fn from(variant: CH2_A) -> Self {
        match variant {
            CH2_A::DISABLE => 0,
            CH2_A::HIGH => 1,
            CH2_A::LOW => 2,
            CH2_A::DAC => 3,
        }
    }
}
#[doc = "Reader of field `CH2`"]
pub type CH2_R = crate::R<u8, CH2_A>;
impl CH2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH2_A {
        match self.bits {
            0 => CH2_A::DISABLE,
            1 => CH2_A::HIGH,
            2 => CH2_A::LOW,
            3 => CH2_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH2_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH2_A::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CH2_A::DAC
    }
}
#[doc = "Write proxy for field `CH2`"]
pub struct CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CH2 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH2_A::DISABLE)
    }
    #[doc = "CH2 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CH2_A::HIGH)
    }
    #[doc = "CH2 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CH2_A::LOW)
    }
    #[doc = "CH2 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(CH2_A::DAC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Channel 3 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH3_A {
    #[doc = "0: CH3 output is disabled in idle phase"]
    DISABLE,
    #[doc = "1: CH3 output is high in idle phase"]
    HIGH,
    #[doc = "2: CH3 output is low in idle phase"]
    LOW,
    #[doc = "3: CH3 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl From<CH3_A> for u8 {
    #[inline(always)]
    fn from(variant: CH3_A) -> Self {
        match variant {
            CH3_A::DISABLE => 0,
            CH3_A::HIGH => 1,
            CH3_A::LOW => 2,
            CH3_A::DAC => 3,
        }
    }
}
#[doc = "Reader of field `CH3`"]
pub type CH3_R = crate::R<u8, CH3_A>;
impl CH3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH3_A {
        match self.bits {
            0 => CH3_A::DISABLE,
            1 => CH3_A::HIGH,
            2 => CH3_A::LOW,
            3 => CH3_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH3_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH3_A::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CH3_A::DAC
    }
}
#[doc = "Write proxy for field `CH3`"]
pub struct CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH3_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CH3 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH3_A::DISABLE)
    }
    #[doc = "CH3 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CH3_A::HIGH)
    }
    #[doc = "CH3 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CH3_A::LOW)
    }
    #[doc = "CH3 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(CH3_A::DAC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Channel 4 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4_A {
    #[doc = "0: CH4 output is disabled in idle phase"]
    DISABLE,
    #[doc = "1: CH4 output is high in idle phase"]
    HIGH,
    #[doc = "2: CH4 output is low in idle phase"]
    LOW,
    #[doc = "3: CH4 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl From<CH4_A> for u8 {
    #[inline(always)]
    fn from(variant: CH4_A) -> Self {
        match variant {
            CH4_A::DISABLE => 0,
            CH4_A::HIGH => 1,
            CH4_A::LOW => 2,
            CH4_A::DAC => 3,
        }
    }
}
#[doc = "Reader of field `CH4`"]
pub type CH4_R = crate::R<u8, CH4_A>;
impl CH4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH4_A {
        match self.bits {
            0 => CH4_A::DISABLE,
            1 => CH4_A::HIGH,
            2 => CH4_A::LOW,
            3 => CH4_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH4_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH4_A::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CH4_A::DAC
    }
}
#[doc = "Write proxy for field `CH4`"]
pub struct CH4_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH4_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CH4 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH4_A::DISABLE)
    }
    #[doc = "CH4 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CH4_A::HIGH)
    }
    #[doc = "CH4 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CH4_A::LOW)
    }
    #[doc = "CH4 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(CH4_A::DAC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Channel 5 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5_A {
    #[doc = "0: CH5 output is disabled in idle phase"]
    DISABLE,
    #[doc = "1: CH5 output is high in idle phase"]
    HIGH,
    #[doc = "2: CH5 output is low in idle phase"]
    LOW,
    #[doc = "3: CH5 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl From<CH5_A> for u8 {
    #[inline(always)]
    fn from(variant: CH5_A) -> Self {
        match variant {
            CH5_A::DISABLE => 0,
            CH5_A::HIGH => 1,
            CH5_A::LOW => 2,
            CH5_A::DAC => 3,
        }
    }
}
#[doc = "Reader of field `CH5`"]
pub type CH5_R = crate::R<u8, CH5_A>;
impl CH5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH5_A {
        match self.bits {
            0 => CH5_A::DISABLE,
            1 => CH5_A::HIGH,
            2 => CH5_A::LOW,
            3 => CH5_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH5_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH5_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH5_A::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CH5_A::DAC
    }
}
#[doc = "Write proxy for field `CH5`"]
pub struct CH5_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH5_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CH5 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH5_A::DISABLE)
    }
    #[doc = "CH5 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CH5_A::HIGH)
    }
    #[doc = "CH5 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CH5_A::LOW)
    }
    #[doc = "CH5 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(CH5_A::DAC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Channel 6 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6_A {
    #[doc = "0: CH6 output is disabled in idle phase"]
    DISABLE,
    #[doc = "1: CH6 output is high in idle phase"]
    HIGH,
    #[doc = "2: CH6 output is low in idle phase"]
    LOW,
    #[doc = "3: CH6 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl From<CH6_A> for u8 {
    #[inline(always)]
    fn from(variant: CH6_A) -> Self {
        match variant {
            CH6_A::DISABLE => 0,
            CH6_A::HIGH => 1,
            CH6_A::LOW => 2,
            CH6_A::DAC => 3,
        }
    }
}
#[doc = "Reader of field `CH6`"]
pub type CH6_R = crate::R<u8, CH6_A>;
impl CH6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH6_A {
        match self.bits {
            0 => CH6_A::DISABLE,
            1 => CH6_A::HIGH,
            2 => CH6_A::LOW,
            3 => CH6_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH6_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH6_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH6_A::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CH6_A::DAC
    }
}
#[doc = "Write proxy for field `CH6`"]
pub struct CH6_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH6_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CH6 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH6_A::DISABLE)
    }
    #[doc = "CH6 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CH6_A::HIGH)
    }
    #[doc = "CH6 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CH6_A::LOW)
    }
    #[doc = "CH6 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(CH6_A::DAC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Channel 7 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7_A {
    #[doc = "0: CH7 output is disabled in idle phase"]
    DISABLE,
    #[doc = "1: CH7 output is high in idle phase"]
    HIGH,
    #[doc = "2: CH7 output is low in idle phase"]
    LOW,
    #[doc = "3: CH7 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl From<CH7_A> for u8 {
    #[inline(always)]
    fn from(variant: CH7_A) -> Self {
        match variant {
            CH7_A::DISABLE => 0,
            CH7_A::HIGH => 1,
            CH7_A::LOW => 2,
            CH7_A::DAC => 3,
        }
    }
}
#[doc = "Reader of field `CH7`"]
pub type CH7_R = crate::R<u8, CH7_A>;
impl CH7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH7_A {
        match self.bits {
            0 => CH7_A::DISABLE,
            1 => CH7_A::HIGH,
            2 => CH7_A::LOW,
            3 => CH7_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH7_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH7_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH7_A::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CH7_A::DAC
    }
}
#[doc = "Write proxy for field `CH7`"]
pub struct CH7_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH7_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CH7 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH7_A::DISABLE)
    }
    #[doc = "CH7 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CH7_A::HIGH)
    }
    #[doc = "CH7 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CH7_A::LOW)
    }
    #[doc = "CH7 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(CH7_A::DAC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Channel 8 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH8_A {
    #[doc = "0: CH8 output is disabled in idle phase"]
    DISABLE,
    #[doc = "1: CH8 output is high in idle phase"]
    HIGH,
    #[doc = "2: CH8 output is low in idle phase"]
    LOW,
    #[doc = "3: CH8 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl From<CH8_A> for u8 {
    #[inline(always)]
    fn from(variant: CH8_A) -> Self {
        match variant {
            CH8_A::DISABLE => 0,
            CH8_A::HIGH => 1,
            CH8_A::LOW => 2,
            CH8_A::DAC => 3,
        }
    }
}
#[doc = "Reader of field `CH8`"]
pub type CH8_R = crate::R<u8, CH8_A>;
impl CH8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH8_A {
        match self.bits {
            0 => CH8_A::DISABLE,
            1 => CH8_A::HIGH,
            2 => CH8_A::LOW,
            3 => CH8_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH8_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH8_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH8_A::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CH8_A::DAC
    }
}
#[doc = "Write proxy for field `CH8`"]
pub struct CH8_W<'a> {
    w: &'a mut W,
}
impl<'a> CH8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH8_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CH8 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH8_A::DISABLE)
    }
    #[doc = "CH8 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CH8_A::HIGH)
    }
    #[doc = "CH8 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CH8_A::LOW)
    }
    #[doc = "CH8 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(CH8_A::DAC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Channel 9 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH9_A {
    #[doc = "0: CH9 output is disabled in idle phase"]
    DISABLE,
    #[doc = "1: CH9 output is high in idle phase"]
    HIGH,
    #[doc = "2: CH9 output is low in idle phase"]
    LOW,
    #[doc = "3: CH9 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl From<CH9_A> for u8 {
    #[inline(always)]
    fn from(variant: CH9_A) -> Self {
        match variant {
            CH9_A::DISABLE => 0,
            CH9_A::HIGH => 1,
            CH9_A::LOW => 2,
            CH9_A::DAC => 3,
        }
    }
}
#[doc = "Reader of field `CH9`"]
pub type CH9_R = crate::R<u8, CH9_A>;
impl CH9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH9_A {
        match self.bits {
            0 => CH9_A::DISABLE,
            1 => CH9_A::HIGH,
            2 => CH9_A::LOW,
            3 => CH9_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH9_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH9_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH9_A::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CH9_A::DAC
    }
}
#[doc = "Write proxy for field `CH9`"]
pub struct CH9_W<'a> {
    w: &'a mut W,
}
impl<'a> CH9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH9_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CH9 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH9_A::DISABLE)
    }
    #[doc = "CH9 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CH9_A::HIGH)
    }
    #[doc = "CH9 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CH9_A::LOW)
    }
    #[doc = "CH9 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(CH9_A::DAC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Channel 10 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH10_A {
    #[doc = "0: CH10 output is disabled in idle phase"]
    DISABLE,
    #[doc = "1: CH10 output is high in idle phase"]
    HIGH,
    #[doc = "2: CH10 output is low in idle phase"]
    LOW,
    #[doc = "3: CH10 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl From<CH10_A> for u8 {
    #[inline(always)]
    fn from(variant: CH10_A) -> Self {
        match variant {
            CH10_A::DISABLE => 0,
            CH10_A::HIGH => 1,
            CH10_A::LOW => 2,
            CH10_A::DAC => 3,
        }
    }
}
#[doc = "Reader of field `CH10`"]
pub type CH10_R = crate::R<u8, CH10_A>;
impl CH10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH10_A {
        match self.bits {
            0 => CH10_A::DISABLE,
            1 => CH10_A::HIGH,
            2 => CH10_A::LOW,
            3 => CH10_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH10_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH10_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH10_A::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CH10_A::DAC
    }
}
#[doc = "Write proxy for field `CH10`"]
pub struct CH10_W<'a> {
    w: &'a mut W,
}
impl<'a> CH10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH10_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CH10 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH10_A::DISABLE)
    }
    #[doc = "CH10 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CH10_A::HIGH)
    }
    #[doc = "CH10 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CH10_A::LOW)
    }
    #[doc = "CH10 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(CH10_A::DAC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "Channel 11 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH11_A {
    #[doc = "0: CH11 output is disabled in idle phase"]
    DISABLE,
    #[doc = "1: CH11 output is high in idle phase"]
    HIGH,
    #[doc = "2: CH11 output is low in idle phase"]
    LOW,
    #[doc = "3: CH11 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl From<CH11_A> for u8 {
    #[inline(always)]
    fn from(variant: CH11_A) -> Self {
        match variant {
            CH11_A::DISABLE => 0,
            CH11_A::HIGH => 1,
            CH11_A::LOW => 2,
            CH11_A::DAC => 3,
        }
    }
}
#[doc = "Reader of field `CH11`"]
pub type CH11_R = crate::R<u8, CH11_A>;
impl CH11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH11_A {
        match self.bits {
            0 => CH11_A::DISABLE,
            1 => CH11_A::HIGH,
            2 => CH11_A::LOW,
            3 => CH11_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH11_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH11_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH11_A::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CH11_A::DAC
    }
}
#[doc = "Write proxy for field `CH11`"]
pub struct CH11_W<'a> {
    w: &'a mut W,
}
impl<'a> CH11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH11_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CH11 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH11_A::DISABLE)
    }
    #[doc = "CH11 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CH11_A::HIGH)
    }
    #[doc = "CH11 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CH11_A::LOW)
    }
    #[doc = "CH11 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(CH11_A::DAC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Channel 12 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH12_A {
    #[doc = "0: CH12 output is disabled in idle phase"]
    DISABLE,
    #[doc = "1: CH12 output is high in idle phase"]
    HIGH,
    #[doc = "2: CH12 output is low in idle phase"]
    LOW,
    #[doc = "3: CH12 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl From<CH12_A> for u8 {
    #[inline(always)]
    fn from(variant: CH12_A) -> Self {
        match variant {
            CH12_A::DISABLE => 0,
            CH12_A::HIGH => 1,
            CH12_A::LOW => 2,
            CH12_A::DAC => 3,
        }
    }
}
#[doc = "Reader of field `CH12`"]
pub type CH12_R = crate::R<u8, CH12_A>;
impl CH12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH12_A {
        match self.bits {
            0 => CH12_A::DISABLE,
            1 => CH12_A::HIGH,
            2 => CH12_A::LOW,
            3 => CH12_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH12_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH12_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH12_A::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CH12_A::DAC
    }
}
#[doc = "Write proxy for field `CH12`"]
pub struct CH12_W<'a> {
    w: &'a mut W,
}
impl<'a> CH12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH12_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CH12 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH12_A::DISABLE)
    }
    #[doc = "CH12 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CH12_A::HIGH)
    }
    #[doc = "CH12 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CH12_A::LOW)
    }
    #[doc = "CH12 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(CH12_A::DAC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Channel 13 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH13_A {
    #[doc = "0: CH13 output is disabled in idle phase"]
    DISABLE,
    #[doc = "1: CH13 output is high in idle phase"]
    HIGH,
    #[doc = "2: CH13 output is low in idle phase"]
    LOW,
    #[doc = "3: CH13 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl From<CH13_A> for u8 {
    #[inline(always)]
    fn from(variant: CH13_A) -> Self {
        match variant {
            CH13_A::DISABLE => 0,
            CH13_A::HIGH => 1,
            CH13_A::LOW => 2,
            CH13_A::DAC => 3,
        }
    }
}
#[doc = "Reader of field `CH13`"]
pub type CH13_R = crate::R<u8, CH13_A>;
impl CH13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH13_A {
        match self.bits {
            0 => CH13_A::DISABLE,
            1 => CH13_A::HIGH,
            2 => CH13_A::LOW,
            3 => CH13_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH13_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH13_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH13_A::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CH13_A::DAC
    }
}
#[doc = "Write proxy for field `CH13`"]
pub struct CH13_W<'a> {
    w: &'a mut W,
}
impl<'a> CH13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH13_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CH13 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH13_A::DISABLE)
    }
    #[doc = "CH13 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CH13_A::HIGH)
    }
    #[doc = "CH13 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CH13_A::LOW)
    }
    #[doc = "CH13 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(CH13_A::DAC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Channel 14 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH14_A {
    #[doc = "0: CH14 output is disabled in idle phase"]
    DISABLE,
    #[doc = "1: CH14 output is high in idle phase"]
    HIGH,
    #[doc = "2: CH14 output is low in idle phase"]
    LOW,
    #[doc = "3: CH14 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl From<CH14_A> for u8 {
    #[inline(always)]
    fn from(variant: CH14_A) -> Self {
        match variant {
            CH14_A::DISABLE => 0,
            CH14_A::HIGH => 1,
            CH14_A::LOW => 2,
            CH14_A::DAC => 3,
        }
    }
}
#[doc = "Reader of field `CH14`"]
pub type CH14_R = crate::R<u8, CH14_A>;
impl CH14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH14_A {
        match self.bits {
            0 => CH14_A::DISABLE,
            1 => CH14_A::HIGH,
            2 => CH14_A::LOW,
            3 => CH14_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH14_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH14_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH14_A::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CH14_A::DAC
    }
}
#[doc = "Write proxy for field `CH14`"]
pub struct CH14_W<'a> {
    w: &'a mut W,
}
impl<'a> CH14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH14_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CH14 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH14_A::DISABLE)
    }
    #[doc = "CH14 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CH14_A::HIGH)
    }
    #[doc = "CH14 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CH14_A::LOW)
    }
    #[doc = "CH14 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(CH14_A::DAC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Channel 15 Idle Phase Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH15_A {
    #[doc = "0: CH15 output is disabled in idle phase"]
    DISABLE,
    #[doc = "1: CH15 output is high in idle phase"]
    HIGH,
    #[doc = "2: CH15 output is low in idle phase"]
    LOW,
    #[doc = "3: CH15 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    DAC,
}
impl From<CH15_A> for u8 {
    #[inline(always)]
    fn from(variant: CH15_A) -> Self {
        match variant {
            CH15_A::DISABLE => 0,
            CH15_A::HIGH => 1,
            CH15_A::LOW => 2,
            CH15_A::DAC => 3,
        }
    }
}
#[doc = "Reader of field `CH15`"]
pub type CH15_R = crate::R<u8, CH15_A>;
impl CH15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CH15_A {
        match self.bits {
            0 => CH15_A::DISABLE,
            1 => CH15_A::HIGH,
            2 => CH15_A::LOW,
            3 => CH15_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CH15_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == CH15_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == CH15_A::LOW
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        *self == CH15_A::DAC
    }
}
#[doc = "Write proxy for field `CH15`"]
pub struct CH15_W<'a> {
    w: &'a mut W,
}
impl<'a> CH15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH15_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "CH15 output is disabled in idle phase"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CH15_A::DISABLE)
    }
    #[doc = "CH15 output is high in idle phase"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CH15_A::HIGH)
    }
    #[doc = "CH15 output is low in idle phase"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(CH15_A::LOW)
    }
    #[doc = "CH15 output is connected to VDAC output in idle phase. Note that this mode is only available on channels 0, 1, 2, 3, 12, 13, 14, 15"]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(CH15_A::DAC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Channel 0 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Channel 1 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - Channel 2 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Channel 3 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - Channel 4 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Channel 5 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch5(&self) -> CH5_R {
        CH5_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - Channel 6 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch6(&self) -> CH6_R {
        CH6_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - Channel 7 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch7(&self) -> CH7_R {
        CH7_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - Channel 8 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch8(&self) -> CH8_R {
        CH8_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Channel 9 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch9(&self) -> CH9_R {
        CH9_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Channel 10 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch10(&self) -> CH10_R {
        CH10_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Channel 11 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch11(&self) -> CH11_R {
        CH11_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Channel 12 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch12(&self) -> CH12_R {
        CH12_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Channel 13 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch13(&self) -> CH13_R {
        CH13_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Channel 14 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch14(&self) -> CH14_R {
        CH14_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 30:31 - Channel 15 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch15(&self) -> CH15_R {
        CH15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 0 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch0(&mut self) -> CH0_W {
        CH0_W { w: self }
    }
    #[doc = "Bits 2:3 - Channel 1 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch1(&mut self) -> CH1_W {
        CH1_W { w: self }
    }
    #[doc = "Bits 4:5 - Channel 2 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch2(&mut self) -> CH2_W {
        CH2_W { w: self }
    }
    #[doc = "Bits 6:7 - Channel 3 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch3(&mut self) -> CH3_W {
        CH3_W { w: self }
    }
    #[doc = "Bits 8:9 - Channel 4 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch4(&mut self) -> CH4_W {
        CH4_W { w: self }
    }
    #[doc = "Bits 10:11 - Channel 5 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch5(&mut self) -> CH5_W {
        CH5_W { w: self }
    }
    #[doc = "Bits 12:13 - Channel 6 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch6(&mut self) -> CH6_W {
        CH6_W { w: self }
    }
    #[doc = "Bits 14:15 - Channel 7 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch7(&mut self) -> CH7_W {
        CH7_W { w: self }
    }
    #[doc = "Bits 16:17 - Channel 8 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch8(&mut self) -> CH8_W {
        CH8_W { w: self }
    }
    #[doc = "Bits 18:19 - Channel 9 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch9(&mut self) -> CH9_W {
        CH9_W { w: self }
    }
    #[doc = "Bits 20:21 - Channel 10 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch10(&mut self) -> CH10_W {
        CH10_W { w: self }
    }
    #[doc = "Bits 22:23 - Channel 11 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch11(&mut self) -> CH11_W {
        CH11_W { w: self }
    }
    #[doc = "Bits 24:25 - Channel 12 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch12(&mut self) -> CH12_W {
        CH12_W { w: self }
    }
    #[doc = "Bits 26:27 - Channel 13 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch13(&mut self) -> CH13_W {
        CH13_W { w: self }
    }
    #[doc = "Bits 28:29 - Channel 14 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch14(&mut self) -> CH14_W {
        CH14_W { w: self }
    }
    #[doc = "Bits 30:31 - Channel 15 Idle Phase Configuration"]
    #[inline(always)]
    pub fn ch15(&mut self) -> CH15_W {
        CH15_W { w: self }
    }
}
