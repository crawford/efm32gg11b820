#[doc = "Reader of register CH1_INTERACT"]
pub type R = crate::R<u32, super::CH1_INTERACT>;
#[doc = "Writer for register CH1_INTERACT"]
pub type W = crate::W<u32, super::CH1_INTERACT>;
#[doc = "Register CH1_INTERACT `reset()`'s with value 0"]
impl crate::ResetValue for super::CH1_INTERACT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `THRES`"]
pub type THRES_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `THRES`"]
pub struct THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | ((value as u32) & 0x0fff);
        self.w
    }
}
#[doc = "Select Sample Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SAMPLE_A {
    #[doc = "0: Counter output will be used in evaluation"]
    ACMPCOUNT = 0,
    #[doc = "1: ACMP output will be used in evaluation"]
    ACMP = 1,
    #[doc = "2: ADC output will be used in evaluation"]
    ADC = 2,
    #[doc = "3: Differential ADC output will be used in evaluation"]
    ADCDIFF = 3,
}
impl From<SAMPLE_A> for u8 {
    #[inline(always)]
    fn from(variant: SAMPLE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SAMPLE`"]
pub type SAMPLE_R = crate::R<u8, SAMPLE_A>;
impl SAMPLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMPLE_A {
        match self.bits {
            0 => SAMPLE_A::ACMPCOUNT,
            1 => SAMPLE_A::ACMP,
            2 => SAMPLE_A::ADC,
            3 => SAMPLE_A::ADCDIFF,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ACMPCOUNT`"]
    #[inline(always)]
    pub fn is_acmpcount(&self) -> bool {
        *self == SAMPLE_A::ACMPCOUNT
    }
    #[doc = "Checks if the value of the field is `ACMP`"]
    #[inline(always)]
    pub fn is_acmp(&self) -> bool {
        *self == SAMPLE_A::ACMP
    }
    #[doc = "Checks if the value of the field is `ADC`"]
    #[inline(always)]
    pub fn is_adc(&self) -> bool {
        *self == SAMPLE_A::ADC
    }
    #[doc = "Checks if the value of the field is `ADCDIFF`"]
    #[inline(always)]
    pub fn is_adcdiff(&self) -> bool {
        *self == SAMPLE_A::ADCDIFF
    }
}
#[doc = "Write proxy for field `SAMPLE`"]
pub struct SAMPLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMPLE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Counter output will be used in evaluation"]
    #[inline(always)]
    pub fn acmpcount(self) -> &'a mut W {
        self.variant(SAMPLE_A::ACMPCOUNT)
    }
    #[doc = "ACMP output will be used in evaluation"]
    #[inline(always)]
    pub fn acmp(self) -> &'a mut W {
        self.variant(SAMPLE_A::ACMP)
    }
    #[doc = "ADC output will be used in evaluation"]
    #[inline(always)]
    pub fn adc(self) -> &'a mut W {
        self.variant(SAMPLE_A::ADC)
    }
    #[doc = "Differential ADC output will be used in evaluation"]
    #[inline(always)]
    pub fn adcdiff(self) -> &'a mut W {
        self.variant(SAMPLE_A::ADCDIFF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Enable Interrupt Generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SETIF_A {
    #[doc = "0: No interrupt is generated"]
    NONE = 0,
    #[doc = "1: Set interrupt flag if the sensor triggers."]
    LEVEL = 1,
    #[doc = "2: Set interrupt flag on positive edge of the sensor state"]
    POSEDGE = 2,
    #[doc = "3: Set interrupt flag on negative edge of the sensor state"]
    NEGEDGE = 3,
    #[doc = "4: Set interrupt flag on both edges of the sensor state"]
    BOTHEDGES = 4,
}
impl From<SETIF_A> for u8 {
    #[inline(always)]
    fn from(variant: SETIF_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SETIF`"]
pub type SETIF_R = crate::R<u8, SETIF_A>;
impl SETIF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SETIF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SETIF_A::NONE),
            1 => Val(SETIF_A::LEVEL),
            2 => Val(SETIF_A::POSEDGE),
            3 => Val(SETIF_A::NEGEDGE),
            4 => Val(SETIF_A::BOTHEDGES),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SETIF_A::NONE
    }
    #[doc = "Checks if the value of the field is `LEVEL`"]
    #[inline(always)]
    pub fn is_level(&self) -> bool {
        *self == SETIF_A::LEVEL
    }
    #[doc = "Checks if the value of the field is `POSEDGE`"]
    #[inline(always)]
    pub fn is_posedge(&self) -> bool {
        *self == SETIF_A::POSEDGE
    }
    #[doc = "Checks if the value of the field is `NEGEDGE`"]
    #[inline(always)]
    pub fn is_negedge(&self) -> bool {
        *self == SETIF_A::NEGEDGE
    }
    #[doc = "Checks if the value of the field is `BOTHEDGES`"]
    #[inline(always)]
    pub fn is_bothedges(&self) -> bool {
        *self == SETIF_A::BOTHEDGES
    }
}
#[doc = "Write proxy for field `SETIF`"]
pub struct SETIF_W<'a> {
    w: &'a mut W,
}
impl<'a> SETIF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SETIF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No interrupt is generated"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SETIF_A::NONE)
    }
    #[doc = "Set interrupt flag if the sensor triggers."]
    #[inline(always)]
    pub fn level(self) -> &'a mut W {
        self.variant(SETIF_A::LEVEL)
    }
    #[doc = "Set interrupt flag on positive edge of the sensor state"]
    #[inline(always)]
    pub fn posedge(self) -> &'a mut W {
        self.variant(SETIF_A::POSEDGE)
    }
    #[doc = "Set interrupt flag on negative edge of the sensor state"]
    #[inline(always)]
    pub fn negedge(self) -> &'a mut W {
        self.variant(SETIF_A::NEGEDGE)
    }
    #[doc = "Set interrupt flag on both edges of the sensor state"]
    #[inline(always)]
    pub fn bothedges(self) -> &'a mut W {
        self.variant(SETIF_A::BOTHEDGES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 14)) | (((value as u32) & 0x07) << 14);
        self.w
    }
}
#[doc = "Set GPIO Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXMODE_A {
    #[doc = "0: Disabled"]
    DISABLE = 0,
    #[doc = "1: Push Pull, GPIO is driven high"]
    HIGH = 1,
    #[doc = "2: Push Pull, GPIO is driven low"]
    LOW = 2,
    #[doc = "3: VDAC output"]
    DACOUT = 3,
}
impl From<EXMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: EXMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXMODE`"]
pub type EXMODE_R = crate::R<u8, EXMODE_A>;
impl EXMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXMODE_A {
        match self.bits {
            0 => EXMODE_A::DISABLE,
            1 => EXMODE_A::HIGH,
            2 => EXMODE_A::LOW,
            3 => EXMODE_A::DACOUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EXMODE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == EXMODE_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == EXMODE_A::LOW
    }
    #[doc = "Checks if the value of the field is `DACOUT`"]
    #[inline(always)]
    pub fn is_dacout(&self) -> bool {
        *self == EXMODE_A::DACOUT
    }
}
#[doc = "Write proxy for field `EXMODE`"]
pub struct EXMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EXMODE_A::DISABLE)
    }
    #[doc = "Push Pull, GPIO is driven high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(EXMODE_A::HIGH)
    }
    #[doc = "Push Pull, GPIO is driven low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(EXMODE_A::LOW)
    }
    #[doc = "VDAC output"]
    #[inline(always)]
    pub fn dacout(self) -> &'a mut W {
        self.variant(EXMODE_A::DACOUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | (((value as u32) & 0x03) << 17);
        self.w
    }
}
#[doc = "Reader of field `EXCLK`"]
pub type EXCLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EXCLK`"]
pub struct EXCLK_W<'a> {
    w: &'a mut W,
}
impl<'a> EXCLK_W<'a> {
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
#[doc = "Reader of field `SAMPLECLK`"]
pub type SAMPLECLK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SAMPLECLK`"]
pub struct SAMPLECLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLECLK_W<'a> {
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
#[doc = "Reader of field `ALTEX`"]
pub type ALTEX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ALTEX`"]
pub struct ALTEX_W<'a> {
    w: &'a mut W,
}
impl<'a> ALTEX_W<'a> {
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
impl R {
    #[doc = "Bits 0:11 - ACMP Threshold or VDAC Data"]
    #[inline(always)]
    pub fn thres(&self) -> THRES_R {
        THRES_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:13 - Select Sample Mode"]
    #[inline(always)]
    pub fn sample(&self) -> SAMPLE_R {
        SAMPLE_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 14:16 - Enable Interrupt Generation"]
    #[inline(always)]
    pub fn setif(&self) -> SETIF_R {
        SETIF_R::new(((self.bits >> 14) & 0x07) as u8)
    }
    #[doc = "Bits 17:18 - Set GPIO Mode"]
    #[inline(always)]
    pub fn exmode(&self) -> EXMODE_R {
        EXMODE_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bit 19 - Select Clock Used for Excitation Timing"]
    #[inline(always)]
    pub fn exclk(&self) -> EXCLK_R {
        EXCLK_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Select Clock Used for Timing of Sample Delay"]
    #[inline(always)]
    pub fn sampleclk(&self) -> SAMPLECLK_R {
        SAMPLECLK_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Use Alternative Excite Pin"]
    #[inline(always)]
    pub fn altex(&self) -> ALTEX_R {
        ALTEX_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - ACMP Threshold or VDAC Data"]
    #[inline(always)]
    pub fn thres(&mut self) -> THRES_W {
        THRES_W { w: self }
    }
    #[doc = "Bits 12:13 - Select Sample Mode"]
    #[inline(always)]
    pub fn sample(&mut self) -> SAMPLE_W {
        SAMPLE_W { w: self }
    }
    #[doc = "Bits 14:16 - Enable Interrupt Generation"]
    #[inline(always)]
    pub fn setif(&mut self) -> SETIF_W {
        SETIF_W { w: self }
    }
    #[doc = "Bits 17:18 - Set GPIO Mode"]
    #[inline(always)]
    pub fn exmode(&mut self) -> EXMODE_W {
        EXMODE_W { w: self }
    }
    #[doc = "Bit 19 - Select Clock Used for Excitation Timing"]
    #[inline(always)]
    pub fn exclk(&mut self) -> EXCLK_W {
        EXCLK_W { w: self }
    }
    #[doc = "Bit 20 - Select Clock Used for Timing of Sample Delay"]
    #[inline(always)]
    pub fn sampleclk(&mut self) -> SAMPLECLK_W {
        SAMPLECLK_W { w: self }
    }
    #[doc = "Bit 21 - Use Alternative Excite Pin"]
    #[inline(always)]
    pub fn altex(&mut self) -> ALTEX_W {
        ALTEX_W { w: self }
    }
}
