#[doc = "Reader of register EXTIPINSELL"]
pub type R = crate::R<u32, super::EXTIPINSELL>;
#[doc = "Writer for register EXTIPINSELL"]
pub type W = crate::W<u32, super::EXTIPINSELL>;
#[doc = "Register EXTIPINSELL `reset()`'s with value 0x3210_3210"]
impl crate::ResetValue for super::EXTIPINSELL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x3210_3210
    }
}
#[doc = "External Interrupt 0 Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPINSEL0_A {
    #[doc = "0: Pin 0"]
    PIN0,
    #[doc = "1: Pin 1"]
    PIN1,
    #[doc = "2: Pin 2"]
    PIN2,
    #[doc = "3: Pin 3"]
    PIN3,
}
impl From<EXTIPINSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL0_A) -> Self {
        match variant {
            EXTIPINSEL0_A::PIN0 => 0,
            EXTIPINSEL0_A::PIN1 => 1,
            EXTIPINSEL0_A::PIN2 => 2,
            EXTIPINSEL0_A::PIN3 => 3,
        }
    }
}
#[doc = "Reader of field `EXTIPINSEL0`"]
pub type EXTIPINSEL0_R = crate::R<u8, EXTIPINSEL0_A>;
impl EXTIPINSEL0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL0_A {
        match self.bits {
            0 => EXTIPINSEL0_A::PIN0,
            1 => EXTIPINSEL0_A::PIN1,
            2 => EXTIPINSEL0_A::PIN2,
            3 => EXTIPINSEL0_A::PIN3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == EXTIPINSEL0_A::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == EXTIPINSEL0_A::PIN1
    }
    #[doc = "Checks if the value of the field is `PIN2`"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == EXTIPINSEL0_A::PIN2
    }
    #[doc = "Checks if the value of the field is `PIN3`"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == EXTIPINSEL0_A::PIN3
    }
}
#[doc = "Write proxy for field `EXTIPINSEL0`"]
pub struct EXTIPINSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPINSEL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPINSEL0_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pin 0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut W {
        self.variant(EXTIPINSEL0_A::PIN0)
    }
    #[doc = "Pin 1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut W {
        self.variant(EXTIPINSEL0_A::PIN1)
    }
    #[doc = "Pin 2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut W {
        self.variant(EXTIPINSEL0_A::PIN2)
    }
    #[doc = "Pin 3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut W {
        self.variant(EXTIPINSEL0_A::PIN3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "External Interrupt 1 Pin Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPINSEL1_A {
    #[doc = "0: Pin 0"]
    PIN0,
    #[doc = "1: Pin 1"]
    PIN1,
    #[doc = "2: Pin 2"]
    PIN2,
    #[doc = "3: Pin 3"]
    PIN3,
}
impl From<EXTIPINSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL1_A) -> Self {
        match variant {
            EXTIPINSEL1_A::PIN0 => 0,
            EXTIPINSEL1_A::PIN1 => 1,
            EXTIPINSEL1_A::PIN2 => 2,
            EXTIPINSEL1_A::PIN3 => 3,
        }
    }
}
#[doc = "Reader of field `EXTIPINSEL1`"]
pub type EXTIPINSEL1_R = crate::R<u8, EXTIPINSEL1_A>;
impl EXTIPINSEL1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL1_A {
        match self.bits {
            0 => EXTIPINSEL1_A::PIN0,
            1 => EXTIPINSEL1_A::PIN1,
            2 => EXTIPINSEL1_A::PIN2,
            3 => EXTIPINSEL1_A::PIN3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == EXTIPINSEL1_A::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == EXTIPINSEL1_A::PIN1
    }
    #[doc = "Checks if the value of the field is `PIN2`"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == EXTIPINSEL1_A::PIN2
    }
    #[doc = "Checks if the value of the field is `PIN3`"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == EXTIPINSEL1_A::PIN3
    }
}
#[doc = "Write proxy for field `EXTIPINSEL1`"]
pub struct EXTIPINSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPINSEL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPINSEL1_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pin 0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut W {
        self.variant(EXTIPINSEL1_A::PIN0)
    }
    #[doc = "Pin 1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut W {
        self.variant(EXTIPINSEL1_A::PIN1)
    }
    #[doc = "Pin 2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut W {
        self.variant(EXTIPINSEL1_A::PIN2)
    }
    #[doc = "Pin 3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut W {
        self.variant(EXTIPINSEL1_A::PIN3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "External Interrupt 2 Pin Select\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPINSEL2_A {
    #[doc = "0: Pin 0"]
    PIN0,
    #[doc = "1: Pin 1"]
    PIN1,
    #[doc = "2: Pin 2"]
    PIN2,
    #[doc = "3: Pin 3"]
    PIN3,
}
impl From<EXTIPINSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL2_A) -> Self {
        match variant {
            EXTIPINSEL2_A::PIN0 => 0,
            EXTIPINSEL2_A::PIN1 => 1,
            EXTIPINSEL2_A::PIN2 => 2,
            EXTIPINSEL2_A::PIN3 => 3,
        }
    }
}
#[doc = "Reader of field `EXTIPINSEL2`"]
pub type EXTIPINSEL2_R = crate::R<u8, EXTIPINSEL2_A>;
impl EXTIPINSEL2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL2_A {
        match self.bits {
            0 => EXTIPINSEL2_A::PIN0,
            1 => EXTIPINSEL2_A::PIN1,
            2 => EXTIPINSEL2_A::PIN2,
            3 => EXTIPINSEL2_A::PIN3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == EXTIPINSEL2_A::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == EXTIPINSEL2_A::PIN1
    }
    #[doc = "Checks if the value of the field is `PIN2`"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == EXTIPINSEL2_A::PIN2
    }
    #[doc = "Checks if the value of the field is `PIN3`"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == EXTIPINSEL2_A::PIN3
    }
}
#[doc = "Write proxy for field `EXTIPINSEL2`"]
pub struct EXTIPINSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPINSEL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPINSEL2_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pin 0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut W {
        self.variant(EXTIPINSEL2_A::PIN0)
    }
    #[doc = "Pin 1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut W {
        self.variant(EXTIPINSEL2_A::PIN1)
    }
    #[doc = "Pin 2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut W {
        self.variant(EXTIPINSEL2_A::PIN2)
    }
    #[doc = "Pin 3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut W {
        self.variant(EXTIPINSEL2_A::PIN3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "External Interrupt 3 Pin Select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPINSEL3_A {
    #[doc = "0: Pin 0"]
    PIN0,
    #[doc = "1: Pin 1"]
    PIN1,
    #[doc = "2: Pin 2"]
    PIN2,
    #[doc = "3: Pin 3"]
    PIN3,
}
impl From<EXTIPINSEL3_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL3_A) -> Self {
        match variant {
            EXTIPINSEL3_A::PIN0 => 0,
            EXTIPINSEL3_A::PIN1 => 1,
            EXTIPINSEL3_A::PIN2 => 2,
            EXTIPINSEL3_A::PIN3 => 3,
        }
    }
}
#[doc = "Reader of field `EXTIPINSEL3`"]
pub type EXTIPINSEL3_R = crate::R<u8, EXTIPINSEL3_A>;
impl EXTIPINSEL3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL3_A {
        match self.bits {
            0 => EXTIPINSEL3_A::PIN0,
            1 => EXTIPINSEL3_A::PIN1,
            2 => EXTIPINSEL3_A::PIN2,
            3 => EXTIPINSEL3_A::PIN3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN0`"]
    #[inline(always)]
    pub fn is_pin0(&self) -> bool {
        *self == EXTIPINSEL3_A::PIN0
    }
    #[doc = "Checks if the value of the field is `PIN1`"]
    #[inline(always)]
    pub fn is_pin1(&self) -> bool {
        *self == EXTIPINSEL3_A::PIN1
    }
    #[doc = "Checks if the value of the field is `PIN2`"]
    #[inline(always)]
    pub fn is_pin2(&self) -> bool {
        *self == EXTIPINSEL3_A::PIN2
    }
    #[doc = "Checks if the value of the field is `PIN3`"]
    #[inline(always)]
    pub fn is_pin3(&self) -> bool {
        *self == EXTIPINSEL3_A::PIN3
    }
}
#[doc = "Write proxy for field `EXTIPINSEL3`"]
pub struct EXTIPINSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPINSEL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPINSEL3_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pin 0"]
    #[inline(always)]
    pub fn pin0(self) -> &'a mut W {
        self.variant(EXTIPINSEL3_A::PIN0)
    }
    #[doc = "Pin 1"]
    #[inline(always)]
    pub fn pin1(self) -> &'a mut W {
        self.variant(EXTIPINSEL3_A::PIN1)
    }
    #[doc = "Pin 2"]
    #[inline(always)]
    pub fn pin2(self) -> &'a mut W {
        self.variant(EXTIPINSEL3_A::PIN2)
    }
    #[doc = "Pin 3"]
    #[inline(always)]
    pub fn pin3(self) -> &'a mut W {
        self.variant(EXTIPINSEL3_A::PIN3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "External Interrupt 4 Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPINSEL4_A {
    #[doc = "0: Pin 4"]
    PIN4,
    #[doc = "1: Pin 5"]
    PIN5,
    #[doc = "2: Pin 6"]
    PIN6,
    #[doc = "3: Pin 7"]
    PIN7,
}
impl From<EXTIPINSEL4_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL4_A) -> Self {
        match variant {
            EXTIPINSEL4_A::PIN4 => 0,
            EXTIPINSEL4_A::PIN5 => 1,
            EXTIPINSEL4_A::PIN6 => 2,
            EXTIPINSEL4_A::PIN7 => 3,
        }
    }
}
#[doc = "Reader of field `EXTIPINSEL4`"]
pub type EXTIPINSEL4_R = crate::R<u8, EXTIPINSEL4_A>;
impl EXTIPINSEL4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL4_A {
        match self.bits {
            0 => EXTIPINSEL4_A::PIN4,
            1 => EXTIPINSEL4_A::PIN5,
            2 => EXTIPINSEL4_A::PIN6,
            3 => EXTIPINSEL4_A::PIN7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN4`"]
    #[inline(always)]
    pub fn is_pin4(&self) -> bool {
        *self == EXTIPINSEL4_A::PIN4
    }
    #[doc = "Checks if the value of the field is `PIN5`"]
    #[inline(always)]
    pub fn is_pin5(&self) -> bool {
        *self == EXTIPINSEL4_A::PIN5
    }
    #[doc = "Checks if the value of the field is `PIN6`"]
    #[inline(always)]
    pub fn is_pin6(&self) -> bool {
        *self == EXTIPINSEL4_A::PIN6
    }
    #[doc = "Checks if the value of the field is `PIN7`"]
    #[inline(always)]
    pub fn is_pin7(&self) -> bool {
        *self == EXTIPINSEL4_A::PIN7
    }
}
#[doc = "Write proxy for field `EXTIPINSEL4`"]
pub struct EXTIPINSEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPINSEL4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPINSEL4_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pin 4"]
    #[inline(always)]
    pub fn pin4(self) -> &'a mut W {
        self.variant(EXTIPINSEL4_A::PIN4)
    }
    #[doc = "Pin 5"]
    #[inline(always)]
    pub fn pin5(self) -> &'a mut W {
        self.variant(EXTIPINSEL4_A::PIN5)
    }
    #[doc = "Pin 6"]
    #[inline(always)]
    pub fn pin6(self) -> &'a mut W {
        self.variant(EXTIPINSEL4_A::PIN6)
    }
    #[doc = "Pin 7"]
    #[inline(always)]
    pub fn pin7(self) -> &'a mut W {
        self.variant(EXTIPINSEL4_A::PIN7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "External Interrupt 5 Pin Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPINSEL5_A {
    #[doc = "0: Pin 4"]
    PIN4,
    #[doc = "1: Pin 5"]
    PIN5,
    #[doc = "2: Pin 6"]
    PIN6,
    #[doc = "3: Pin 7"]
    PIN7,
}
impl From<EXTIPINSEL5_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL5_A) -> Self {
        match variant {
            EXTIPINSEL5_A::PIN4 => 0,
            EXTIPINSEL5_A::PIN5 => 1,
            EXTIPINSEL5_A::PIN6 => 2,
            EXTIPINSEL5_A::PIN7 => 3,
        }
    }
}
#[doc = "Reader of field `EXTIPINSEL5`"]
pub type EXTIPINSEL5_R = crate::R<u8, EXTIPINSEL5_A>;
impl EXTIPINSEL5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL5_A {
        match self.bits {
            0 => EXTIPINSEL5_A::PIN4,
            1 => EXTIPINSEL5_A::PIN5,
            2 => EXTIPINSEL5_A::PIN6,
            3 => EXTIPINSEL5_A::PIN7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN4`"]
    #[inline(always)]
    pub fn is_pin4(&self) -> bool {
        *self == EXTIPINSEL5_A::PIN4
    }
    #[doc = "Checks if the value of the field is `PIN5`"]
    #[inline(always)]
    pub fn is_pin5(&self) -> bool {
        *self == EXTIPINSEL5_A::PIN5
    }
    #[doc = "Checks if the value of the field is `PIN6`"]
    #[inline(always)]
    pub fn is_pin6(&self) -> bool {
        *self == EXTIPINSEL5_A::PIN6
    }
    #[doc = "Checks if the value of the field is `PIN7`"]
    #[inline(always)]
    pub fn is_pin7(&self) -> bool {
        *self == EXTIPINSEL5_A::PIN7
    }
}
#[doc = "Write proxy for field `EXTIPINSEL5`"]
pub struct EXTIPINSEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPINSEL5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPINSEL5_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pin 4"]
    #[inline(always)]
    pub fn pin4(self) -> &'a mut W {
        self.variant(EXTIPINSEL5_A::PIN4)
    }
    #[doc = "Pin 5"]
    #[inline(always)]
    pub fn pin5(self) -> &'a mut W {
        self.variant(EXTIPINSEL5_A::PIN5)
    }
    #[doc = "Pin 6"]
    #[inline(always)]
    pub fn pin6(self) -> &'a mut W {
        self.variant(EXTIPINSEL5_A::PIN6)
    }
    #[doc = "Pin 7"]
    #[inline(always)]
    pub fn pin7(self) -> &'a mut W {
        self.variant(EXTIPINSEL5_A::PIN7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "External Interrupt 6 Pin Select\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPINSEL6_A {
    #[doc = "0: Pin 4"]
    PIN4,
    #[doc = "1: Pin 5"]
    PIN5,
    #[doc = "2: Pin 6"]
    PIN6,
    #[doc = "3: Pin 7"]
    PIN7,
}
impl From<EXTIPINSEL6_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL6_A) -> Self {
        match variant {
            EXTIPINSEL6_A::PIN4 => 0,
            EXTIPINSEL6_A::PIN5 => 1,
            EXTIPINSEL6_A::PIN6 => 2,
            EXTIPINSEL6_A::PIN7 => 3,
        }
    }
}
#[doc = "Reader of field `EXTIPINSEL6`"]
pub type EXTIPINSEL6_R = crate::R<u8, EXTIPINSEL6_A>;
impl EXTIPINSEL6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL6_A {
        match self.bits {
            0 => EXTIPINSEL6_A::PIN4,
            1 => EXTIPINSEL6_A::PIN5,
            2 => EXTIPINSEL6_A::PIN6,
            3 => EXTIPINSEL6_A::PIN7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN4`"]
    #[inline(always)]
    pub fn is_pin4(&self) -> bool {
        *self == EXTIPINSEL6_A::PIN4
    }
    #[doc = "Checks if the value of the field is `PIN5`"]
    #[inline(always)]
    pub fn is_pin5(&self) -> bool {
        *self == EXTIPINSEL6_A::PIN5
    }
    #[doc = "Checks if the value of the field is `PIN6`"]
    #[inline(always)]
    pub fn is_pin6(&self) -> bool {
        *self == EXTIPINSEL6_A::PIN6
    }
    #[doc = "Checks if the value of the field is `PIN7`"]
    #[inline(always)]
    pub fn is_pin7(&self) -> bool {
        *self == EXTIPINSEL6_A::PIN7
    }
}
#[doc = "Write proxy for field `EXTIPINSEL6`"]
pub struct EXTIPINSEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPINSEL6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPINSEL6_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pin 4"]
    #[inline(always)]
    pub fn pin4(self) -> &'a mut W {
        self.variant(EXTIPINSEL6_A::PIN4)
    }
    #[doc = "Pin 5"]
    #[inline(always)]
    pub fn pin5(self) -> &'a mut W {
        self.variant(EXTIPINSEL6_A::PIN5)
    }
    #[doc = "Pin 6"]
    #[inline(always)]
    pub fn pin6(self) -> &'a mut W {
        self.variant(EXTIPINSEL6_A::PIN6)
    }
    #[doc = "Pin 7"]
    #[inline(always)]
    pub fn pin7(self) -> &'a mut W {
        self.variant(EXTIPINSEL6_A::PIN7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "External Interrupt 7 Pin Select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTIPINSEL7_A {
    #[doc = "0: Pin 4"]
    PIN4,
    #[doc = "1: Pin 5"]
    PIN5,
    #[doc = "2: Pin 6"]
    PIN6,
    #[doc = "3: Pin 7"]
    PIN7,
}
impl From<EXTIPINSEL7_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPINSEL7_A) -> Self {
        match variant {
            EXTIPINSEL7_A::PIN4 => 0,
            EXTIPINSEL7_A::PIN5 => 1,
            EXTIPINSEL7_A::PIN6 => 2,
            EXTIPINSEL7_A::PIN7 => 3,
        }
    }
}
#[doc = "Reader of field `EXTIPINSEL7`"]
pub type EXTIPINSEL7_R = crate::R<u8, EXTIPINSEL7_A>;
impl EXTIPINSEL7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTIPINSEL7_A {
        match self.bits {
            0 => EXTIPINSEL7_A::PIN4,
            1 => EXTIPINSEL7_A::PIN5,
            2 => EXTIPINSEL7_A::PIN6,
            3 => EXTIPINSEL7_A::PIN7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PIN4`"]
    #[inline(always)]
    pub fn is_pin4(&self) -> bool {
        *self == EXTIPINSEL7_A::PIN4
    }
    #[doc = "Checks if the value of the field is `PIN5`"]
    #[inline(always)]
    pub fn is_pin5(&self) -> bool {
        *self == EXTIPINSEL7_A::PIN5
    }
    #[doc = "Checks if the value of the field is `PIN6`"]
    #[inline(always)]
    pub fn is_pin6(&self) -> bool {
        *self == EXTIPINSEL7_A::PIN6
    }
    #[doc = "Checks if the value of the field is `PIN7`"]
    #[inline(always)]
    pub fn is_pin7(&self) -> bool {
        *self == EXTIPINSEL7_A::PIN7
    }
}
#[doc = "Write proxy for field `EXTIPINSEL7`"]
pub struct EXTIPINSEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPINSEL7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPINSEL7_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pin 4"]
    #[inline(always)]
    pub fn pin4(self) -> &'a mut W {
        self.variant(EXTIPINSEL7_A::PIN4)
    }
    #[doc = "Pin 5"]
    #[inline(always)]
    pub fn pin5(self) -> &'a mut W {
        self.variant(EXTIPINSEL7_A::PIN5)
    }
    #[doc = "Pin 6"]
    #[inline(always)]
    pub fn pin6(self) -> &'a mut W {
        self.variant(EXTIPINSEL7_A::PIN6)
    }
    #[doc = "Pin 7"]
    #[inline(always)]
    pub fn pin7(self) -> &'a mut W {
        self.variant(EXTIPINSEL7_A::PIN7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - External Interrupt 0 Pin Select"]
    #[inline(always)]
    pub fn extipinsel0(&self) -> EXTIPINSEL0_R {
        EXTIPINSEL0_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - External Interrupt 1 Pin Select"]
    #[inline(always)]
    pub fn extipinsel1(&self) -> EXTIPINSEL1_R {
        EXTIPINSEL1_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - External Interrupt 2 Pin Select"]
    #[inline(always)]
    pub fn extipinsel2(&self) -> EXTIPINSEL2_R {
        EXTIPINSEL2_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - External Interrupt 3 Pin Select"]
    #[inline(always)]
    pub fn extipinsel3(&self) -> EXTIPINSEL3_R {
        EXTIPINSEL3_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - External Interrupt 4 Pin Select"]
    #[inline(always)]
    pub fn extipinsel4(&self) -> EXTIPINSEL4_R {
        EXTIPINSEL4_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - External Interrupt 5 Pin Select"]
    #[inline(always)]
    pub fn extipinsel5(&self) -> EXTIPINSEL5_R {
        EXTIPINSEL5_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - External Interrupt 6 Pin Select"]
    #[inline(always)]
    pub fn extipinsel6(&self) -> EXTIPINSEL6_R {
        EXTIPINSEL6_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - External Interrupt 7 Pin Select"]
    #[inline(always)]
    pub fn extipinsel7(&self) -> EXTIPINSEL7_R {
        EXTIPINSEL7_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Interrupt 0 Pin Select"]
    #[inline(always)]
    pub fn extipinsel0(&mut self) -> EXTIPINSEL0_W {
        EXTIPINSEL0_W { w: self }
    }
    #[doc = "Bits 4:5 - External Interrupt 1 Pin Select"]
    #[inline(always)]
    pub fn extipinsel1(&mut self) -> EXTIPINSEL1_W {
        EXTIPINSEL1_W { w: self }
    }
    #[doc = "Bits 8:9 - External Interrupt 2 Pin Select"]
    #[inline(always)]
    pub fn extipinsel2(&mut self) -> EXTIPINSEL2_W {
        EXTIPINSEL2_W { w: self }
    }
    #[doc = "Bits 12:13 - External Interrupt 3 Pin Select"]
    #[inline(always)]
    pub fn extipinsel3(&mut self) -> EXTIPINSEL3_W {
        EXTIPINSEL3_W { w: self }
    }
    #[doc = "Bits 16:17 - External Interrupt 4 Pin Select"]
    #[inline(always)]
    pub fn extipinsel4(&mut self) -> EXTIPINSEL4_W {
        EXTIPINSEL4_W { w: self }
    }
    #[doc = "Bits 20:21 - External Interrupt 5 Pin Select"]
    #[inline(always)]
    pub fn extipinsel5(&mut self) -> EXTIPINSEL5_W {
        EXTIPINSEL5_W { w: self }
    }
    #[doc = "Bits 24:25 - External Interrupt 6 Pin Select"]
    #[inline(always)]
    pub fn extipinsel6(&mut self) -> EXTIPINSEL6_W {
        EXTIPINSEL6_W { w: self }
    }
    #[doc = "Bits 28:29 - External Interrupt 7 Pin Select"]
    #[inline(always)]
    pub fn extipinsel7(&mut self) -> EXTIPINSEL7_W {
        EXTIPINSEL7_W { w: self }
    }
}
