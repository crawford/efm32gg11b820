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
#[doc = "Reader of field `DEBUGRUN`"]
pub type DEBUGRUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEBUGRUN`"]
pub struct DEBUGRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUGRUN_W<'a> {
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
#[doc = "Select Low Frequency Oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OSCSEL_A {
    #[doc = "0: Output is driven low"]
    DISABLED = 0,
    #[doc = "1: Select Low Frequency RC Oscillator"]
    LFRCO = 1,
    #[doc = "2: Select Low Frequency Crystal Oscillator"]
    LFXO = 2,
    #[doc = "3: Select Ultra Low Frequency RC Oscillator"]
    ULFRCO = 3,
}
impl From<OSCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: OSCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OSCSEL`"]
pub type OSCSEL_R = crate::R<u8, OSCSEL_A>;
impl OSCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCSEL_A {
        match self.bits {
            0 => OSCSEL_A::DISABLED,
            1 => OSCSEL_A::LFRCO,
            2 => OSCSEL_A::LFXO,
            3 => OSCSEL_A::ULFRCO,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSCSEL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == OSCSEL_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == OSCSEL_A::LFXO
    }
    #[doc = "Checks if the value of the field is `ULFRCO`"]
    #[inline(always)]
    pub fn is_ulfrco(&self) -> bool {
        *self == OSCSEL_A::ULFRCO
    }
}
#[doc = "Write proxy for field `OSCSEL`"]
pub struct OSCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OSCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Output is driven low"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OSCSEL_A::DISABLED)
    }
    #[doc = "Select Low Frequency RC Oscillator"]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(OSCSEL_A::LFRCO)
    }
    #[doc = "Select Low Frequency Crystal Oscillator"]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(OSCSEL_A::LFXO)
    }
    #[doc = "Select Ultra Low Frequency RC Oscillator"]
    #[inline(always)]
    pub fn ulfrco(self) -> &'a mut W {
        self.variant(OSCSEL_A::ULFRCO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Prescaler Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESC_A {
    #[doc = "0: LF Oscillator frequency undivided"]
    DIV1 = 0,
    #[doc = "1: LF Oscillator frequency divided by 2"]
    DIV2 = 1,
    #[doc = "2: LF Oscillator frequency divided by 4"]
    DIV4 = 2,
    #[doc = "3: LF Oscillator frequency divided by 8"]
    DIV8 = 3,
    #[doc = "4: LF Oscillator frequency divided by 16"]
    DIV16 = 4,
    #[doc = "5: LF Oscillator frequency divided by 32"]
    DIV32 = 5,
    #[doc = "6: LF Oscillator frequency divided by 64"]
    DIV64 = 6,
    #[doc = "7: LF Oscillator frequency divided by 128"]
    DIV128 = 7,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRESC`"]
pub type PRESC_R = crate::R<u8, PRESC_A>;
impl PRESC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESC_A {
        match self.bits {
            0 => PRESC_A::DIV1,
            1 => PRESC_A::DIV2,
            2 => PRESC_A::DIV4,
            3 => PRESC_A::DIV8,
            4 => PRESC_A::DIV16,
            5 => PRESC_A::DIV32,
            6 => PRESC_A::DIV64,
            7 => PRESC_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESC_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESC_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESC_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESC_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESC_A::DIV128
    }
}
#[doc = "Write proxy for field `PRESC`"]
pub struct PRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "LF Oscillator frequency undivided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PRESC_A::DIV1)
    }
    #[doc = "LF Oscillator frequency divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESC_A::DIV2)
    }
    #[doc = "LF Oscillator frequency divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESC_A::DIV4)
    }
    #[doc = "LF Oscillator frequency divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESC_A::DIV8)
    }
    #[doc = "LF Oscillator frequency divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESC_A::DIV16)
    }
    #[doc = "LF Oscillator frequency divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRESC_A::DIV32)
    }
    #[doc = "LF Oscillator frequency divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESC_A::DIV64)
    }
    #[doc = "LF Oscillator frequency divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PRESC_A::DIV128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable CRYOTIMER"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DEBUGRUN_R {
        DEBUGRUN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Select Low Frequency Oscillator"]
    #[inline(always)]
    pub fn oscsel(&self) -> OSCSEL_R {
        OSCSEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 5:7 - Prescaler Setting"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 5) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable CRYOTIMER"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&mut self) -> DEBUGRUN_W {
        DEBUGRUN_W { w: self }
    }
    #[doc = "Bits 2:3 - Select Low Frequency Oscillator"]
    #[inline(always)]
    pub fn oscsel(&mut self) -> OSCSEL_W {
        OSCSEL_W { w: self }
    }
    #[doc = "Bits 5:7 - Prescaler Setting"]
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W {
        PRESC_W { w: self }
    }
}
