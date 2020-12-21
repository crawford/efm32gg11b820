#[doc = "Reader of register EXTIPINSELH"]
pub type R = crate::R<u32, super::EXTIPINSELH>;
#[doc = "Writer for register EXTIPINSELH"]
pub type W = crate::W<u32, super::EXTIPINSELH>;
#[doc = "Register EXTIPINSELH `reset()`'s with value 0x3210_3210"]
impl crate::ResetValue for super::EXTIPINSELH {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3210_3210
    }
}
#[doc = "External Interrupt 8 Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPINSEL8_A {
    #[doc = "0: Pin 8"]
    PIN8 = 0,
    #[doc = "1: Pin 9"]
    PIN9 = 1,
    #[doc = "2: Pin 10"]
    PIN10 = 2,
    #[doc = "3: Pin 11"]
    PIN11 = 3,
}
impl From<EXTIPINSEL8_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL8_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTIPINSEL8`"]
pub type EXTIPINSEL8_R = crate::R<u8, EXTIPINSEL8_A>;
impl EXTIPINSEL8_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL8_A {
        match self.bits {
            0 => EXTIPINSEL8_A::PIN8,
            1 => EXTIPINSEL8_A::PIN9,
            2 => EXTIPINSEL8_A::PIN10,
            3 => EXTIPINSEL8_A::PIN11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN8`"]
    #[inline(always)]
    pub fn is_pin8(&self) -> bool {
        *self == EXTIPINSEL8_A::PIN8
    }
    #[doc = "Checks if the value of the field is `PIN9`"]
    #[inline(always)]
    pub fn is_pin9(&self) -> bool {
        *self == EXTIPINSEL8_A::PIN9
    }
    #[doc = "Checks if the value of the field is `PIN10`"]
    #[inline(always)]
    pub fn is_pin10(&self) -> bool {
        *self == EXTIPINSEL8_A::PIN10
    }
    #[doc = "Checks if the value of the field is `PIN11`"]
    #[inline(always)]
    pub fn is_pin11(&self) -> bool {
        *self == EXTIPINSEL8_A::PIN11
    }
}
#[doc = "Write proxy for field `EXTIPINSEL8`"]
pub struct EXTIPINSEL8_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPINSEL8_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPINSEL8_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pin 8"]
    #[inline(always)]
    pub fn pin8(self) -> &'a mut W {
        self.variant(EXTIPINSEL8_A::PIN8)
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub fn pin9(self) -> &'a mut W {
        self.variant(EXTIPINSEL8_A::PIN9)
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub fn pin10(self) -> &'a mut W {
        self.variant(EXTIPINSEL8_A::PIN10)
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub fn pin11(self) -> &'a mut W {
        self.variant(EXTIPINSEL8_A::PIN11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "External Interrupt 9 Pin Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPINSEL9_A {
    #[doc = "0: Pin 8"]
    PIN8 = 0,
    #[doc = "1: Pin 9"]
    PIN9 = 1,
    #[doc = "2: Pin 10"]
    PIN10 = 2,
    #[doc = "3: Pin 11"]
    PIN11 = 3,
}
impl From<EXTIPINSEL9_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL9_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTIPINSEL9`"]
pub type EXTIPINSEL9_R = crate::R<u8, EXTIPINSEL9_A>;
impl EXTIPINSEL9_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL9_A {
        match self.bits {
            0 => EXTIPINSEL9_A::PIN8,
            1 => EXTIPINSEL9_A::PIN9,
            2 => EXTIPINSEL9_A::PIN10,
            3 => EXTIPINSEL9_A::PIN11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN8`"]
    #[inline(always)]
    pub fn is_pin8(&self) -> bool {
        *self == EXTIPINSEL9_A::PIN8
    }
    #[doc = "Checks if the value of the field is `PIN9`"]
    #[inline(always)]
    pub fn is_pin9(&self) -> bool {
        *self == EXTIPINSEL9_A::PIN9
    }
    #[doc = "Checks if the value of the field is `PIN10`"]
    #[inline(always)]
    pub fn is_pin10(&self) -> bool {
        *self == EXTIPINSEL9_A::PIN10
    }
    #[doc = "Checks if the value of the field is `PIN11`"]
    #[inline(always)]
    pub fn is_pin11(&self) -> bool {
        *self == EXTIPINSEL9_A::PIN11
    }
}
#[doc = "Write proxy for field `EXTIPINSEL9`"]
pub struct EXTIPINSEL9_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPINSEL9_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPINSEL9_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pin 8"]
    #[inline(always)]
    pub fn pin8(self) -> &'a mut W {
        self.variant(EXTIPINSEL9_A::PIN8)
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub fn pin9(self) -> &'a mut W {
        self.variant(EXTIPINSEL9_A::PIN9)
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub fn pin10(self) -> &'a mut W {
        self.variant(EXTIPINSEL9_A::PIN10)
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub fn pin11(self) -> &'a mut W {
        self.variant(EXTIPINSEL9_A::PIN11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "External Interrupt 10 Pin Select\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPINSEL10_A {
    #[doc = "0: Pin 8"]
    PIN8 = 0,
    #[doc = "1: Pin 9"]
    PIN9 = 1,
    #[doc = "2: Pin 10"]
    PIN10 = 2,
    #[doc = "3: Pin 11"]
    PIN11 = 3,
}
impl From<EXTIPINSEL10_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL10_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTIPINSEL10`"]
pub type EXTIPINSEL10_R = crate::R<u8, EXTIPINSEL10_A>;
impl EXTIPINSEL10_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL10_A {
        match self.bits {
            0 => EXTIPINSEL10_A::PIN8,
            1 => EXTIPINSEL10_A::PIN9,
            2 => EXTIPINSEL10_A::PIN10,
            3 => EXTIPINSEL10_A::PIN11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN8`"]
    #[inline(always)]
    pub fn is_pin8(&self) -> bool {
        *self == EXTIPINSEL10_A::PIN8
    }
    #[doc = "Checks if the value of the field is `PIN9`"]
    #[inline(always)]
    pub fn is_pin9(&self) -> bool {
        *self == EXTIPINSEL10_A::PIN9
    }
    #[doc = "Checks if the value of the field is `PIN10`"]
    #[inline(always)]
    pub fn is_pin10(&self) -> bool {
        *self == EXTIPINSEL10_A::PIN10
    }
    #[doc = "Checks if the value of the field is `PIN11`"]
    #[inline(always)]
    pub fn is_pin11(&self) -> bool {
        *self == EXTIPINSEL10_A::PIN11
    }
}
#[doc = "Write proxy for field `EXTIPINSEL10`"]
pub struct EXTIPINSEL10_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPINSEL10_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPINSEL10_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pin 8"]
    #[inline(always)]
    pub fn pin8(self) -> &'a mut W {
        self.variant(EXTIPINSEL10_A::PIN8)
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub fn pin9(self) -> &'a mut W {
        self.variant(EXTIPINSEL10_A::PIN9)
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub fn pin10(self) -> &'a mut W {
        self.variant(EXTIPINSEL10_A::PIN10)
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub fn pin11(self) -> &'a mut W {
        self.variant(EXTIPINSEL10_A::PIN11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "External Interrupt 11 Pin Select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPINSEL11_A {
    #[doc = "0: Pin 8"]
    PIN8 = 0,
    #[doc = "1: Pin 9"]
    PIN9 = 1,
    #[doc = "2: Pin 10"]
    PIN10 = 2,
    #[doc = "3: Pin 11"]
    PIN11 = 3,
}
impl From<EXTIPINSEL11_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL11_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTIPINSEL11`"]
pub type EXTIPINSEL11_R = crate::R<u8, EXTIPINSEL11_A>;
impl EXTIPINSEL11_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL11_A {
        match self.bits {
            0 => EXTIPINSEL11_A::PIN8,
            1 => EXTIPINSEL11_A::PIN9,
            2 => EXTIPINSEL11_A::PIN10,
            3 => EXTIPINSEL11_A::PIN11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN8`"]
    #[inline(always)]
    pub fn is_pin8(&self) -> bool {
        *self == EXTIPINSEL11_A::PIN8
    }
    #[doc = "Checks if the value of the field is `PIN9`"]
    #[inline(always)]
    pub fn is_pin9(&self) -> bool {
        *self == EXTIPINSEL11_A::PIN9
    }
    #[doc = "Checks if the value of the field is `PIN10`"]
    #[inline(always)]
    pub fn is_pin10(&self) -> bool {
        *self == EXTIPINSEL11_A::PIN10
    }
    #[doc = "Checks if the value of the field is `PIN11`"]
    #[inline(always)]
    pub fn is_pin11(&self) -> bool {
        *self == EXTIPINSEL11_A::PIN11
    }
}
#[doc = "Write proxy for field `EXTIPINSEL11`"]
pub struct EXTIPINSEL11_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPINSEL11_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPINSEL11_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pin 8"]
    #[inline(always)]
    pub fn pin8(self) -> &'a mut W {
        self.variant(EXTIPINSEL11_A::PIN8)
    }
    #[doc = "Pin 9"]
    #[inline(always)]
    pub fn pin9(self) -> &'a mut W {
        self.variant(EXTIPINSEL11_A::PIN9)
    }
    #[doc = "Pin 10"]
    #[inline(always)]
    pub fn pin10(self) -> &'a mut W {
        self.variant(EXTIPINSEL11_A::PIN10)
    }
    #[doc = "Pin 11"]
    #[inline(always)]
    pub fn pin11(self) -> &'a mut W {
        self.variant(EXTIPINSEL11_A::PIN11)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "External Interrupt 12 Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPINSEL12_A {
    #[doc = "0: Pin 12"]
    PIN12 = 0,
    #[doc = "1: Pin 13"]
    PIN13 = 1,
    #[doc = "2: Pin 14"]
    PIN14 = 2,
    #[doc = "3: Pin 15"]
    PIN15 = 3,
}
impl From<EXTIPINSEL12_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL12_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTIPINSEL12`"]
pub type EXTIPINSEL12_R = crate::R<u8, EXTIPINSEL12_A>;
impl EXTIPINSEL12_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL12_A {
        match self.bits {
            0 => EXTIPINSEL12_A::PIN12,
            1 => EXTIPINSEL12_A::PIN13,
            2 => EXTIPINSEL12_A::PIN14,
            3 => EXTIPINSEL12_A::PIN15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN12`"]
    #[inline(always)]
    pub fn is_pin12(&self) -> bool {
        *self == EXTIPINSEL12_A::PIN12
    }
    #[doc = "Checks if the value of the field is `PIN13`"]
    #[inline(always)]
    pub fn is_pin13(&self) -> bool {
        *self == EXTIPINSEL12_A::PIN13
    }
    #[doc = "Checks if the value of the field is `PIN14`"]
    #[inline(always)]
    pub fn is_pin14(&self) -> bool {
        *self == EXTIPINSEL12_A::PIN14
    }
    #[doc = "Checks if the value of the field is `PIN15`"]
    #[inline(always)]
    pub fn is_pin15(&self) -> bool {
        *self == EXTIPINSEL12_A::PIN15
    }
}
#[doc = "Write proxy for field `EXTIPINSEL12`"]
pub struct EXTIPINSEL12_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPINSEL12_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPINSEL12_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pin 12"]
    #[inline(always)]
    pub fn pin12(self) -> &'a mut W {
        self.variant(EXTIPINSEL12_A::PIN12)
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub fn pin13(self) -> &'a mut W {
        self.variant(EXTIPINSEL12_A::PIN13)
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub fn pin14(self) -> &'a mut W {
        self.variant(EXTIPINSEL12_A::PIN14)
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub fn pin15(self) -> &'a mut W {
        self.variant(EXTIPINSEL12_A::PIN15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "External Interrupt 13 Pin Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPINSEL13_A {
    #[doc = "0: Pin 12"]
    PIN12 = 0,
    #[doc = "1: Pin 13"]
    PIN13 = 1,
    #[doc = "2: Pin 14"]
    PIN14 = 2,
    #[doc = "3: Pin 15"]
    PIN15 = 3,
}
impl From<EXTIPINSEL13_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL13_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTIPINSEL13`"]
pub type EXTIPINSEL13_R = crate::R<u8, EXTIPINSEL13_A>;
impl EXTIPINSEL13_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL13_A {
        match self.bits {
            0 => EXTIPINSEL13_A::PIN12,
            1 => EXTIPINSEL13_A::PIN13,
            2 => EXTIPINSEL13_A::PIN14,
            3 => EXTIPINSEL13_A::PIN15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN12`"]
    #[inline(always)]
    pub fn is_pin12(&self) -> bool {
        *self == EXTIPINSEL13_A::PIN12
    }
    #[doc = "Checks if the value of the field is `PIN13`"]
    #[inline(always)]
    pub fn is_pin13(&self) -> bool {
        *self == EXTIPINSEL13_A::PIN13
    }
    #[doc = "Checks if the value of the field is `PIN14`"]
    #[inline(always)]
    pub fn is_pin14(&self) -> bool {
        *self == EXTIPINSEL13_A::PIN14
    }
    #[doc = "Checks if the value of the field is `PIN15`"]
    #[inline(always)]
    pub fn is_pin15(&self) -> bool {
        *self == EXTIPINSEL13_A::PIN15
    }
}
#[doc = "Write proxy for field `EXTIPINSEL13`"]
pub struct EXTIPINSEL13_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPINSEL13_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPINSEL13_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pin 12"]
    #[inline(always)]
    pub fn pin12(self) -> &'a mut W {
        self.variant(EXTIPINSEL13_A::PIN12)
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub fn pin13(self) -> &'a mut W {
        self.variant(EXTIPINSEL13_A::PIN13)
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub fn pin14(self) -> &'a mut W {
        self.variant(EXTIPINSEL13_A::PIN14)
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub fn pin15(self) -> &'a mut W {
        self.variant(EXTIPINSEL13_A::PIN15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "External Interrupt 14 Pin Select\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPINSEL14_A {
    #[doc = "0: Pin 12"]
    PIN12 = 0,
    #[doc = "1: Pin 13"]
    PIN13 = 1,
    #[doc = "2: Pin 14"]
    PIN14 = 2,
    #[doc = "3: Pin 15"]
    PIN15 = 3,
}
impl From<EXTIPINSEL14_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL14_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTIPINSEL14`"]
pub type EXTIPINSEL14_R = crate::R<u8, EXTIPINSEL14_A>;
impl EXTIPINSEL14_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL14_A {
        match self.bits {
            0 => EXTIPINSEL14_A::PIN12,
            1 => EXTIPINSEL14_A::PIN13,
            2 => EXTIPINSEL14_A::PIN14,
            3 => EXTIPINSEL14_A::PIN15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN12`"]
    #[inline(always)]
    pub fn is_pin12(&self) -> bool {
        *self == EXTIPINSEL14_A::PIN12
    }
    #[doc = "Checks if the value of the field is `PIN13`"]
    #[inline(always)]
    pub fn is_pin13(&self) -> bool {
        *self == EXTIPINSEL14_A::PIN13
    }
    #[doc = "Checks if the value of the field is `PIN14`"]
    #[inline(always)]
    pub fn is_pin14(&self) -> bool {
        *self == EXTIPINSEL14_A::PIN14
    }
    #[doc = "Checks if the value of the field is `PIN15`"]
    #[inline(always)]
    pub fn is_pin15(&self) -> bool {
        *self == EXTIPINSEL14_A::PIN15
    }
}
#[doc = "Write proxy for field `EXTIPINSEL14`"]
pub struct EXTIPINSEL14_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPINSEL14_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPINSEL14_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pin 12"]
    #[inline(always)]
    pub fn pin12(self) -> &'a mut W {
        self.variant(EXTIPINSEL14_A::PIN12)
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub fn pin13(self) -> &'a mut W {
        self.variant(EXTIPINSEL14_A::PIN13)
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub fn pin14(self) -> &'a mut W {
        self.variant(EXTIPINSEL14_A::PIN14)
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub fn pin15(self) -> &'a mut W {
        self.variant(EXTIPINSEL14_A::PIN15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "External Interrupt 15 Pin Select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPINSEL15_A {
    #[doc = "0: Pin 12"]
    PIN12 = 0,
    #[doc = "1: Pin 13"]
    PIN13 = 1,
    #[doc = "2: Pin 14"]
    PIN14 = 2,
    #[doc = "3: Pin 15"]
    PIN15 = 3,
}
impl From<EXTIPINSEL15_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL15_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `EXTIPINSEL15`"]
pub type EXTIPINSEL15_R = crate::R<u8, EXTIPINSEL15_A>;
impl EXTIPINSEL15_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL15_A {
        match self.bits {
            0 => EXTIPINSEL15_A::PIN12,
            1 => EXTIPINSEL15_A::PIN13,
            2 => EXTIPINSEL15_A::PIN14,
            3 => EXTIPINSEL15_A::PIN15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN12`"]
    #[inline(always)]
    pub fn is_pin12(&self) -> bool {
        *self == EXTIPINSEL15_A::PIN12
    }
    #[doc = "Checks if the value of the field is `PIN13`"]
    #[inline(always)]
    pub fn is_pin13(&self) -> bool {
        *self == EXTIPINSEL15_A::PIN13
    }
    #[doc = "Checks if the value of the field is `PIN14`"]
    #[inline(always)]
    pub fn is_pin14(&self) -> bool {
        *self == EXTIPINSEL15_A::PIN14
    }
    #[doc = "Checks if the value of the field is `PIN15`"]
    #[inline(always)]
    pub fn is_pin15(&self) -> bool {
        *self == EXTIPINSEL15_A::PIN15
    }
}
#[doc = "Write proxy for field `EXTIPINSEL15`"]
pub struct EXTIPINSEL15_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPINSEL15_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPINSEL15_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pin 12"]
    #[inline(always)]
    pub fn pin12(self) -> &'a mut W {
        self.variant(EXTIPINSEL15_A::PIN12)
    }
    #[doc = "Pin 13"]
    #[inline(always)]
    pub fn pin13(self) -> &'a mut W {
        self.variant(EXTIPINSEL15_A::PIN13)
    }
    #[doc = "Pin 14"]
    #[inline(always)]
    pub fn pin14(self) -> &'a mut W {
        self.variant(EXTIPINSEL15_A::PIN14)
    }
    #[doc = "Pin 15"]
    #[inline(always)]
    pub fn pin15(self) -> &'a mut W {
        self.variant(EXTIPINSEL15_A::PIN15)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - External Interrupt 8 Pin Select"]
    #[inline(always)]
    pub fn extipinsel8(&self) -> EXTIPINSEL8_R {
        EXTIPINSEL8_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - External Interrupt 9 Pin Select"]
    #[inline(always)]
    pub fn extipinsel9(&self) -> EXTIPINSEL9_R {
        EXTIPINSEL9_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - External Interrupt 10 Pin Select"]
    #[inline(always)]
    pub fn extipinsel10(&self) -> EXTIPINSEL10_R {
        EXTIPINSEL10_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - External Interrupt 11 Pin Select"]
    #[inline(always)]
    pub fn extipinsel11(&self) -> EXTIPINSEL11_R {
        EXTIPINSEL11_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - External Interrupt 12 Pin Select"]
    #[inline(always)]
    pub fn extipinsel12(&self) -> EXTIPINSEL12_R {
        EXTIPINSEL12_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - External Interrupt 13 Pin Select"]
    #[inline(always)]
    pub fn extipinsel13(&self) -> EXTIPINSEL13_R {
        EXTIPINSEL13_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - External Interrupt 14 Pin Select"]
    #[inline(always)]
    pub fn extipinsel14(&self) -> EXTIPINSEL14_R {
        EXTIPINSEL14_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - External Interrupt 15 Pin Select"]
    #[inline(always)]
    pub fn extipinsel15(&self) -> EXTIPINSEL15_R {
        EXTIPINSEL15_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Interrupt 8 Pin Select"]
    #[inline(always)]
    pub fn extipinsel8(&mut self) -> EXTIPINSEL8_W {
        EXTIPINSEL8_W { w: self }
    }
    #[doc = "Bits 4:5 - External Interrupt 9 Pin Select"]
    #[inline(always)]
    pub fn extipinsel9(&mut self) -> EXTIPINSEL9_W {
        EXTIPINSEL9_W { w: self }
    }
    #[doc = "Bits 8:9 - External Interrupt 10 Pin Select"]
    #[inline(always)]
    pub fn extipinsel10(&mut self) -> EXTIPINSEL10_W {
        EXTIPINSEL10_W { w: self }
    }
    #[doc = "Bits 12:13 - External Interrupt 11 Pin Select"]
    #[inline(always)]
    pub fn extipinsel11(&mut self) -> EXTIPINSEL11_W {
        EXTIPINSEL11_W { w: self }
    }
    #[doc = "Bits 16:17 - External Interrupt 12 Pin Select"]
    #[inline(always)]
    pub fn extipinsel12(&mut self) -> EXTIPINSEL12_W {
        EXTIPINSEL12_W { w: self }
    }
    #[doc = "Bits 20:21 - External Interrupt 13 Pin Select"]
    #[inline(always)]
    pub fn extipinsel13(&mut self) -> EXTIPINSEL13_W {
        EXTIPINSEL13_W { w: self }
    }
    #[doc = "Bits 24:25 - External Interrupt 14 Pin Select"]
    #[inline(always)]
    pub fn extipinsel14(&mut self) -> EXTIPINSEL14_W {
        EXTIPINSEL14_W { w: self }
    }
    #[doc = "Bits 28:29 - External Interrupt 15 Pin Select"]
    #[inline(always)]
    pub fn extipinsel15(&mut self) -> EXTIPINSEL15_W {
        EXTIPINSEL15_W { w: self }
    }
}
