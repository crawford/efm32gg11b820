#[doc = "Reader of register TIMCTRL"]
pub type R = crate::R<u32, super::TIMCTRL>;
#[doc = "Writer for register TIMCTRL"]
pub type W = crate::W<u32, super::TIMCTRL>;
#[doc = "Register TIMCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::TIMCTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Prescaling Factor for High Frequency Timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum AUXPRESC_A {
    #[doc = "0: High frequency timer is clocked with AUXHFRCO/1"]
    DIV1 = 0,
    #[doc = "1: High frequency timer is clocked with AUXHFRCO/2"]
    DIV2 = 1,
    #[doc = "2: High frequency timer is clocked with AUXHFRCO/4"]
    DIV4 = 2,
    #[doc = "3: High frequency timer is clocked with AUXHFRCO/8"]
    DIV8 = 3,
}
impl From<AUXPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: AUXPRESC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `AUXPRESC`"]
pub type AUXPRESC_R = crate::R<u8, AUXPRESC_A>;
impl AUXPRESC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUXPRESC_A {
        match self.bits {
            0 => AUXPRESC_A::DIV1,
            1 => AUXPRESC_A::DIV2,
            2 => AUXPRESC_A::DIV4,
            3 => AUXPRESC_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == AUXPRESC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == AUXPRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == AUXPRESC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == AUXPRESC_A::DIV8
    }
}
#[doc = "Write proxy for field `AUXPRESC`"]
pub struct AUXPRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXPRESC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AUXPRESC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "High frequency timer is clocked with AUXHFRCO/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(AUXPRESC_A::DIV1)
    }
    #[doc = "High frequency timer is clocked with AUXHFRCO/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(AUXPRESC_A::DIV2)
    }
    #[doc = "High frequency timer is clocked with AUXHFRCO/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(AUXPRESC_A::DIV4)
    }
    #[doc = "High frequency timer is clocked with AUXHFRCO/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(AUXPRESC_A::DIV8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Prescaling Factor for Low Frequency Timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LFPRESC_A {
    #[doc = "0: Low frequency timer is clocked with LFACLKLESENSE/1"]
    DIV1 = 0,
    #[doc = "1: Low frequency timer is clocked with LFACLKLESENSE/2"]
    DIV2 = 1,
    #[doc = "2: Low frequency timer is clocked with LFACLKLESENSE/4"]
    DIV4 = 2,
    #[doc = "3: Low frequency timer is clocked with LFACLKLESENSE/8"]
    DIV8 = 3,
    #[doc = "4: Low frequency timer is clocked with LFACLKLESENSE/16"]
    DIV16 = 4,
    #[doc = "5: Low frequency timer is clocked with LFACLKLESENSE/32"]
    DIV32 = 5,
    #[doc = "6: Low frequency timer is clocked with LFACLKLESENSE/64"]
    DIV64 = 6,
    #[doc = "7: Low frequency timer is clocked with LFACLKLESENSE/128"]
    DIV128 = 7,
}
impl From<LFPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: LFPRESC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `LFPRESC`"]
pub type LFPRESC_R = crate::R<u8, LFPRESC_A>;
impl LFPRESC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LFPRESC_A {
        match self.bits {
            0 => LFPRESC_A::DIV1,
            1 => LFPRESC_A::DIV2,
            2 => LFPRESC_A::DIV4,
            3 => LFPRESC_A::DIV8,
            4 => LFPRESC_A::DIV16,
            5 => LFPRESC_A::DIV32,
            6 => LFPRESC_A::DIV64,
            7 => LFPRESC_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == LFPRESC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == LFPRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == LFPRESC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == LFPRESC_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == LFPRESC_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == LFPRESC_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == LFPRESC_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == LFPRESC_A::DIV128
    }
}
#[doc = "Write proxy for field `LFPRESC`"]
pub struct LFPRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> LFPRESC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LFPRESC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(LFPRESC_A::DIV1)
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(LFPRESC_A::DIV2)
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(LFPRESC_A::DIV4)
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(LFPRESC_A::DIV8)
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(LFPRESC_A::DIV16)
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(LFPRESC_A::DIV32)
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(LFPRESC_A::DIV64)
    }
    #[doc = "Low frequency timer is clocked with LFACLKLESENSE/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(LFPRESC_A::DIV128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | (((value as u32) & 0x07) << 4);
        self.w
    }
}
#[doc = "Period Counter Prescaling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PCPRESC_A {
    #[doc = "0: The period counter clock frequency is LFACLKLESENSE/1"]
    DIV1 = 0,
    #[doc = "1: The period counter clock frequency is LFACLKLESENSE/2"]
    DIV2 = 1,
    #[doc = "2: The period counter clock frequency is LFACLKLESENSE/4"]
    DIV4 = 2,
    #[doc = "3: The period counter clock frequency is LFACLKLESENSE/8"]
    DIV8 = 3,
    #[doc = "4: The period counter clock frequency is LFACLKLESENSE/16"]
    DIV16 = 4,
    #[doc = "5: The period counter clock frequency is LFACLKLESENSE/32"]
    DIV32 = 5,
    #[doc = "6: The period counter clock frequency is LFACLKLESENSE/64"]
    DIV64 = 6,
    #[doc = "7: The period counter clock frequency is LFACLKLESENSE/128"]
    DIV128 = 7,
}
impl From<PCPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PCPRESC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PCPRESC`"]
pub type PCPRESC_R = crate::R<u8, PCPRESC_A>;
impl PCPRESC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCPRESC_A {
        match self.bits {
            0 => PCPRESC_A::DIV1,
            1 => PCPRESC_A::DIV2,
            2 => PCPRESC_A::DIV4,
            3 => PCPRESC_A::DIV8,
            4 => PCPRESC_A::DIV16,
            5 => PCPRESC_A::DIV32,
            6 => PCPRESC_A::DIV64,
            7 => PCPRESC_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PCPRESC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PCPRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PCPRESC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PCPRESC_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PCPRESC_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PCPRESC_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PCPRESC_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PCPRESC_A::DIV128
    }
}
#[doc = "Write proxy for field `PCPRESC`"]
pub struct PCPRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> PCPRESC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PCPRESC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PCPRESC_A::DIV1)
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PCPRESC_A::DIV2)
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PCPRESC_A::DIV4)
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PCPRESC_A::DIV8)
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PCPRESC_A::DIV16)
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PCPRESC_A::DIV32)
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PCPRESC_A::DIV64)
    }
    #[doc = "The period counter clock frequency is LFACLKLESENSE/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PCPRESC_A::DIV128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Reader of field `PCTOP`"]
pub type PCTOP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PCTOP`"]
pub struct PCTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> PCTOP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 12)) | (((value as u32) & 0xff) << 12);
        self.w
    }
}
#[doc = "Reader of field `STARTDLY`"]
pub type STARTDLY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `STARTDLY`"]
pub struct STARTDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
        self.w
    }
}
#[doc = "Reader of field `AUXSTARTUP`"]
pub type AUXSTARTUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AUXSTARTUP`"]
pub struct AUXSTARTUP_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXSTARTUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Prescaling Factor for High Frequency Timer"]
    #[inline(always)]
    pub fn auxpresc(&self) -> AUXPRESC_R {
        AUXPRESC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:6 - Prescaling Factor for Low Frequency Timer"]
    #[inline(always)]
    pub fn lfpresc(&self) -> LFPRESC_R {
        LFPRESC_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Period Counter Prescaling"]
    #[inline(always)]
    pub fn pcpresc(&self) -> PCPRESC_R {
        PCPRESC_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:19 - Period Counter Top Value"]
    #[inline(always)]
    pub fn pctop(&self) -> PCTOP_R {
        PCTOP_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    #[doc = "Bits 22:23 - Start Delay Configuration"]
    #[inline(always)]
    pub fn startdly(&self) -> STARTDLY_R {
        STARTDLY_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bit 28 - AUXHFRCO Startup Configuration"]
    #[inline(always)]
    pub fn auxstartup(&self) -> AUXSTARTUP_R {
        AUXSTARTUP_R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Prescaling Factor for High Frequency Timer"]
    #[inline(always)]
    pub fn auxpresc(&mut self) -> AUXPRESC_W {
        AUXPRESC_W { w: self }
    }
    #[doc = "Bits 4:6 - Prescaling Factor for Low Frequency Timer"]
    #[inline(always)]
    pub fn lfpresc(&mut self) -> LFPRESC_W {
        LFPRESC_W { w: self }
    }
    #[doc = "Bits 8:10 - Period Counter Prescaling"]
    #[inline(always)]
    pub fn pcpresc(&mut self) -> PCPRESC_W {
        PCPRESC_W { w: self }
    }
    #[doc = "Bits 12:19 - Period Counter Top Value"]
    #[inline(always)]
    pub fn pctop(&mut self) -> PCTOP_W {
        PCTOP_W { w: self }
    }
    #[doc = "Bits 22:23 - Start Delay Configuration"]
    #[inline(always)]
    pub fn startdly(&mut self) -> STARTDLY_W {
        STARTDLY_W { w: self }
    }
    #[doc = "Bit 28 - AUXHFRCO Startup Configuration"]
    #[inline(always)]
    pub fn auxstartup(&mut self) -> AUXSTARTUP_W {
        AUXSTARTUP_W { w: self }
    }
}
