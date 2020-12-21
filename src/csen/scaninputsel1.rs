#[doc = "Reader of register SCANINPUTSEL1"]
pub type R = crate::R<u32, super::SCANINPUTSEL1>;
#[doc = "Writer for register SCANINPUTSEL1"]
pub type W = crate::W<u32, super::SCANINPUTSEL1>;
#[doc = "Register SCANINPUTSEL1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SCANINPUTSEL1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "CSEN_INPUT32-39 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPUT32TO39SEL_A {
    #[doc = "4: `100`"]
    APORT1CH0TO7 = 4,
    #[doc = "5: `101`"]
    APORT1CH8TO15 = 5,
    #[doc = "6: `110`"]
    APORT1CH16TO23 = 6,
    #[doc = "7: `111`"]
    APORT1CH24TO31 = 7,
    #[doc = "12: `1100`"]
    APORT3CH0TO7 = 12,
    #[doc = "13: `1101`"]
    APORT3CH8TO15 = 13,
    #[doc = "14: `1110`"]
    APORT3CH16TO23 = 14,
    #[doc = "15: `1111`"]
    APORT3CH24TO31 = 15,
}
impl From<INPUT32TO39SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT32TO39SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `INPUT32TO39SEL`"]
pub type INPUT32TO39SEL_R = crate::R<u8, INPUT32TO39SEL_A>;
impl INPUT32TO39SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, INPUT32TO39SEL_A> {
        use crate::Variant::*;
        match self.bits {
            4 => Val(INPUT32TO39SEL_A::APORT1CH0TO7),
            5 => Val(INPUT32TO39SEL_A::APORT1CH8TO15),
            6 => Val(INPUT32TO39SEL_A::APORT1CH16TO23),
            7 => Val(INPUT32TO39SEL_A::APORT1CH24TO31),
            12 => Val(INPUT32TO39SEL_A::APORT3CH0TO7),
            13 => Val(INPUT32TO39SEL_A::APORT3CH8TO15),
            14 => Val(INPUT32TO39SEL_A::APORT3CH16TO23),
            15 => Val(INPUT32TO39SEL_A::APORT3CH24TO31),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `APORT1CH0TO7`"]
    #[inline(always)]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == INPUT32TO39SEL_A::APORT1CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT1CH8TO15`"]
    #[inline(always)]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == INPUT32TO39SEL_A::APORT1CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH16TO23`"]
    #[inline(always)]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == INPUT32TO39SEL_A::APORT1CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT1CH24TO31`"]
    #[inline(always)]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == INPUT32TO39SEL_A::APORT1CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT3CH0TO7`"]
    #[inline(always)]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == INPUT32TO39SEL_A::APORT3CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT3CH8TO15`"]
    #[inline(always)]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == INPUT32TO39SEL_A::APORT3CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT3CH16TO23`"]
    #[inline(always)]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == INPUT32TO39SEL_A::APORT3CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT3CH24TO31`"]
    #[inline(always)]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == INPUT32TO39SEL_A::APORT3CH24TO31
    }
}
#[doc = "Write proxy for field `INPUT32TO39SEL`"]
pub struct INPUT32TO39SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT32TO39SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INPUT32TO39SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn aport1ch0to7(self) -> &'a mut W {
        self.variant(INPUT32TO39SEL_A::APORT1CH0TO7)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn aport1ch8to15(self) -> &'a mut W {
        self.variant(INPUT32TO39SEL_A::APORT1CH8TO15)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn aport1ch16to23(self) -> &'a mut W {
        self.variant(INPUT32TO39SEL_A::APORT1CH16TO23)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn aport1ch24to31(self) -> &'a mut W {
        self.variant(INPUT32TO39SEL_A::APORT1CH24TO31)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn aport3ch0to7(self) -> &'a mut W {
        self.variant(INPUT32TO39SEL_A::APORT3CH0TO7)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn aport3ch8to15(self) -> &'a mut W {
        self.variant(INPUT32TO39SEL_A::APORT3CH8TO15)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn aport3ch16to23(self) -> &'a mut W {
        self.variant(INPUT32TO39SEL_A::APORT3CH16TO23)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn aport3ch24to31(self) -> &'a mut W {
        self.variant(INPUT32TO39SEL_A::APORT3CH24TO31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "CSEN_INPUT40-47 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPUT40TO47SEL_A {
    #[doc = "4: `100`"]
    APORT1CH0TO7 = 4,
    #[doc = "5: `101`"]
    APORT1CH8TO15 = 5,
    #[doc = "6: `110`"]
    APORT1CH16TO23 = 6,
    #[doc = "7: `111`"]
    APORT1CH24TO31 = 7,
    #[doc = "12: `1100`"]
    APORT3CH0TO7 = 12,
    #[doc = "13: `1101`"]
    APORT3CH8TO15 = 13,
    #[doc = "14: `1110`"]
    APORT3CH16TO23 = 14,
    #[doc = "15: `1111`"]
    APORT3CH24TO31 = 15,
}
impl From<INPUT40TO47SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT40TO47SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `INPUT40TO47SEL`"]
pub type INPUT40TO47SEL_R = crate::R<u8, INPUT40TO47SEL_A>;
impl INPUT40TO47SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, INPUT40TO47SEL_A> {
        use crate::Variant::*;
        match self.bits {
            4 => Val(INPUT40TO47SEL_A::APORT1CH0TO7),
            5 => Val(INPUT40TO47SEL_A::APORT1CH8TO15),
            6 => Val(INPUT40TO47SEL_A::APORT1CH16TO23),
            7 => Val(INPUT40TO47SEL_A::APORT1CH24TO31),
            12 => Val(INPUT40TO47SEL_A::APORT3CH0TO7),
            13 => Val(INPUT40TO47SEL_A::APORT3CH8TO15),
            14 => Val(INPUT40TO47SEL_A::APORT3CH16TO23),
            15 => Val(INPUT40TO47SEL_A::APORT3CH24TO31),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `APORT1CH0TO7`"]
    #[inline(always)]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == INPUT40TO47SEL_A::APORT1CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT1CH8TO15`"]
    #[inline(always)]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == INPUT40TO47SEL_A::APORT1CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH16TO23`"]
    #[inline(always)]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == INPUT40TO47SEL_A::APORT1CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT1CH24TO31`"]
    #[inline(always)]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == INPUT40TO47SEL_A::APORT1CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT3CH0TO7`"]
    #[inline(always)]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == INPUT40TO47SEL_A::APORT3CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT3CH8TO15`"]
    #[inline(always)]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == INPUT40TO47SEL_A::APORT3CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT3CH16TO23`"]
    #[inline(always)]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == INPUT40TO47SEL_A::APORT3CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT3CH24TO31`"]
    #[inline(always)]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == INPUT40TO47SEL_A::APORT3CH24TO31
    }
}
#[doc = "Write proxy for field `INPUT40TO47SEL`"]
pub struct INPUT40TO47SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT40TO47SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INPUT40TO47SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn aport1ch0to7(self) -> &'a mut W {
        self.variant(INPUT40TO47SEL_A::APORT1CH0TO7)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn aport1ch8to15(self) -> &'a mut W {
        self.variant(INPUT40TO47SEL_A::APORT1CH8TO15)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn aport1ch16to23(self) -> &'a mut W {
        self.variant(INPUT40TO47SEL_A::APORT1CH16TO23)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn aport1ch24to31(self) -> &'a mut W {
        self.variant(INPUT40TO47SEL_A::APORT1CH24TO31)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn aport3ch0to7(self) -> &'a mut W {
        self.variant(INPUT40TO47SEL_A::APORT3CH0TO7)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn aport3ch8to15(self) -> &'a mut W {
        self.variant(INPUT40TO47SEL_A::APORT3CH8TO15)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn aport3ch16to23(self) -> &'a mut W {
        self.variant(INPUT40TO47SEL_A::APORT3CH16TO23)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn aport3ch24to31(self) -> &'a mut W {
        self.variant(INPUT40TO47SEL_A::APORT3CH24TO31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "CSEN_INPUT48-55 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPUT48TO55SEL_A {
    #[doc = "4: `100`"]
    APORT1CH0TO7 = 4,
    #[doc = "5: `101`"]
    APORT1CH8TO15 = 5,
    #[doc = "6: `110`"]
    APORT1CH16TO23 = 6,
    #[doc = "7: `111`"]
    APORT1CH24TO31 = 7,
    #[doc = "12: `1100`"]
    APORT3CH0TO7 = 12,
    #[doc = "13: `1101`"]
    APORT3CH8TO15 = 13,
    #[doc = "14: `1110`"]
    APORT3CH16TO23 = 14,
    #[doc = "15: `1111`"]
    APORT3CH24TO31 = 15,
}
impl From<INPUT48TO55SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT48TO55SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `INPUT48TO55SEL`"]
pub type INPUT48TO55SEL_R = crate::R<u8, INPUT48TO55SEL_A>;
impl INPUT48TO55SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, INPUT48TO55SEL_A> {
        use crate::Variant::*;
        match self.bits {
            4 => Val(INPUT48TO55SEL_A::APORT1CH0TO7),
            5 => Val(INPUT48TO55SEL_A::APORT1CH8TO15),
            6 => Val(INPUT48TO55SEL_A::APORT1CH16TO23),
            7 => Val(INPUT48TO55SEL_A::APORT1CH24TO31),
            12 => Val(INPUT48TO55SEL_A::APORT3CH0TO7),
            13 => Val(INPUT48TO55SEL_A::APORT3CH8TO15),
            14 => Val(INPUT48TO55SEL_A::APORT3CH16TO23),
            15 => Val(INPUT48TO55SEL_A::APORT3CH24TO31),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `APORT1CH0TO7`"]
    #[inline(always)]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == INPUT48TO55SEL_A::APORT1CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT1CH8TO15`"]
    #[inline(always)]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == INPUT48TO55SEL_A::APORT1CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH16TO23`"]
    #[inline(always)]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == INPUT48TO55SEL_A::APORT1CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT1CH24TO31`"]
    #[inline(always)]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == INPUT48TO55SEL_A::APORT1CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT3CH0TO7`"]
    #[inline(always)]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == INPUT48TO55SEL_A::APORT3CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT3CH8TO15`"]
    #[inline(always)]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == INPUT48TO55SEL_A::APORT3CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT3CH16TO23`"]
    #[inline(always)]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == INPUT48TO55SEL_A::APORT3CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT3CH24TO31`"]
    #[inline(always)]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == INPUT48TO55SEL_A::APORT3CH24TO31
    }
}
#[doc = "Write proxy for field `INPUT48TO55SEL`"]
pub struct INPUT48TO55SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT48TO55SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INPUT48TO55SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn aport1ch0to7(self) -> &'a mut W {
        self.variant(INPUT48TO55SEL_A::APORT1CH0TO7)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn aport1ch8to15(self) -> &'a mut W {
        self.variant(INPUT48TO55SEL_A::APORT1CH8TO15)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn aport1ch16to23(self) -> &'a mut W {
        self.variant(INPUT48TO55SEL_A::APORT1CH16TO23)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn aport1ch24to31(self) -> &'a mut W {
        self.variant(INPUT48TO55SEL_A::APORT1CH24TO31)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn aport3ch0to7(self) -> &'a mut W {
        self.variant(INPUT48TO55SEL_A::APORT3CH0TO7)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn aport3ch8to15(self) -> &'a mut W {
        self.variant(INPUT48TO55SEL_A::APORT3CH8TO15)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn aport3ch16to23(self) -> &'a mut W {
        self.variant(INPUT48TO55SEL_A::APORT3CH16TO23)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn aport3ch24to31(self) -> &'a mut W {
        self.variant(INPUT48TO55SEL_A::APORT3CH24TO31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "CSEN_INPUT56-63 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPUT56TO63SEL_A {
    #[doc = "4: `100`"]
    APORT1CH0TO7 = 4,
    #[doc = "5: `101`"]
    APORT1CH8TO15 = 5,
    #[doc = "6: `110`"]
    APORT1CH16TO23 = 6,
    #[doc = "7: `111`"]
    APORT1CH24TO31 = 7,
    #[doc = "12: `1100`"]
    APORT3CH0TO7 = 12,
    #[doc = "13: `1101`"]
    APORT3CH8TO15 = 13,
    #[doc = "14: `1110`"]
    APORT3CH16TO23 = 14,
    #[doc = "15: `1111`"]
    APORT3CH24TO31 = 15,
}
impl From<INPUT56TO63SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT56TO63SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `INPUT56TO63SEL`"]
pub type INPUT56TO63SEL_R = crate::R<u8, INPUT56TO63SEL_A>;
impl INPUT56TO63SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, INPUT56TO63SEL_A> {
        use crate::Variant::*;
        match self.bits {
            4 => Val(INPUT56TO63SEL_A::APORT1CH0TO7),
            5 => Val(INPUT56TO63SEL_A::APORT1CH8TO15),
            6 => Val(INPUT56TO63SEL_A::APORT1CH16TO23),
            7 => Val(INPUT56TO63SEL_A::APORT1CH24TO31),
            12 => Val(INPUT56TO63SEL_A::APORT3CH0TO7),
            13 => Val(INPUT56TO63SEL_A::APORT3CH8TO15),
            14 => Val(INPUT56TO63SEL_A::APORT3CH16TO23),
            15 => Val(INPUT56TO63SEL_A::APORT3CH24TO31),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `APORT1CH0TO7`"]
    #[inline(always)]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == INPUT56TO63SEL_A::APORT1CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT1CH8TO15`"]
    #[inline(always)]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == INPUT56TO63SEL_A::APORT1CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH16TO23`"]
    #[inline(always)]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == INPUT56TO63SEL_A::APORT1CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT1CH24TO31`"]
    #[inline(always)]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == INPUT56TO63SEL_A::APORT1CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT3CH0TO7`"]
    #[inline(always)]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == INPUT56TO63SEL_A::APORT3CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT3CH8TO15`"]
    #[inline(always)]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == INPUT56TO63SEL_A::APORT3CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT3CH16TO23`"]
    #[inline(always)]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == INPUT56TO63SEL_A::APORT3CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT3CH24TO31`"]
    #[inline(always)]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == INPUT56TO63SEL_A::APORT3CH24TO31
    }
}
#[doc = "Write proxy for field `INPUT56TO63SEL`"]
pub struct INPUT56TO63SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT56TO63SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INPUT56TO63SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn aport1ch0to7(self) -> &'a mut W {
        self.variant(INPUT56TO63SEL_A::APORT1CH0TO7)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn aport1ch8to15(self) -> &'a mut W {
        self.variant(INPUT56TO63SEL_A::APORT1CH8TO15)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn aport1ch16to23(self) -> &'a mut W {
        self.variant(INPUT56TO63SEL_A::APORT1CH16TO23)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn aport1ch24to31(self) -> &'a mut W {
        self.variant(INPUT56TO63SEL_A::APORT1CH24TO31)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn aport3ch0to7(self) -> &'a mut W {
        self.variant(INPUT56TO63SEL_A::APORT3CH0TO7)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn aport3ch8to15(self) -> &'a mut W {
        self.variant(INPUT56TO63SEL_A::APORT3CH8TO15)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn aport3ch16to23(self) -> &'a mut W {
        self.variant(INPUT56TO63SEL_A::APORT3CH16TO23)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn aport3ch24to31(self) -> &'a mut W {
        self.variant(INPUT56TO63SEL_A::APORT3CH24TO31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - CSEN_INPUT32-39 Select"]
    #[inline(always)]
    pub fn input32to39sel(&self) -> INPUT32TO39SEL_R {
        INPUT32TO39SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - CSEN_INPUT40-47 Select"]
    #[inline(always)]
    pub fn input40to47sel(&self) -> INPUT40TO47SEL_R {
        INPUT40TO47SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - CSEN_INPUT48-55 Select"]
    #[inline(always)]
    pub fn input48to55sel(&self) -> INPUT48TO55SEL_R {
        INPUT48TO55SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - CSEN_INPUT56-63 Select"]
    #[inline(always)]
    pub fn input56to63sel(&self) -> INPUT56TO63SEL_R {
        INPUT56TO63SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CSEN_INPUT32-39 Select"]
    #[inline(always)]
    pub fn input32to39sel(&mut self) -> INPUT32TO39SEL_W {
        INPUT32TO39SEL_W { w: self }
    }
    #[doc = "Bits 8:11 - CSEN_INPUT40-47 Select"]
    #[inline(always)]
    pub fn input40to47sel(&mut self) -> INPUT40TO47SEL_W {
        INPUT40TO47SEL_W { w: self }
    }
    #[doc = "Bits 16:19 - CSEN_INPUT48-55 Select"]
    #[inline(always)]
    pub fn input48to55sel(&mut self) -> INPUT48TO55SEL_W {
        INPUT48TO55SEL_W { w: self }
    }
    #[doc = "Bits 24:27 - CSEN_INPUT56-63 Select"]
    #[inline(always)]
    pub fn input56to63sel(&mut self) -> INPUT56TO63SEL_W {
        INPUT56TO63SEL_W { w: self }
    }
}
