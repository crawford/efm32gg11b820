#[doc = "Reader of register SCANINPUTSEL"]
pub type R = crate::R<u32, super::SCANINPUTSEL>;
#[doc = "Writer for register SCANINPUTSEL"]
pub type W = crate::W<u32, super::SCANINPUTSEL>;
#[doc = "Register SCANINPUTSEL `reset()`'s with value 0"]
impl crate::ResetValue for super::SCANINPUTSEL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Inputs Chosen for ADCn_INPUT7-ADCn_INPUT0 as Referred in SCANMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPUT0TO7SEL_A {
    #[doc = "0: `0`"]
    APORT0CH0TO7 = 0,
    #[doc = "1: `1`"]
    APORT0CH8TO15 = 1,
    #[doc = "4: `100`"]
    APORT1CH0TO7 = 4,
    #[doc = "5: `101`"]
    APORT1CH8TO15 = 5,
    #[doc = "6: `110`"]
    APORT1CH16TO23 = 6,
    #[doc = "7: `111`"]
    APORT1CH24TO31 = 7,
    #[doc = "8: `1000`"]
    APORT2CH0TO7 = 8,
    #[doc = "9: `1001`"]
    APORT2CH8TO15 = 9,
    #[doc = "10: `1010`"]
    APORT2CH16TO23 = 10,
    #[doc = "11: `1011`"]
    APORT2CH24TO31 = 11,
    #[doc = "12: `1100`"]
    APORT3CH0TO7 = 12,
    #[doc = "13: `1101`"]
    APORT3CH8TO15 = 13,
    #[doc = "14: `1110`"]
    APORT3CH16TO23 = 14,
    #[doc = "15: `1111`"]
    APORT3CH24TO31 = 15,
    #[doc = "16: `10000`"]
    APORT4CH0TO7 = 16,
    #[doc = "17: `10001`"]
    APORT4CH8TO15 = 17,
    #[doc = "18: `10010`"]
    APORT4CH16TO23 = 18,
    #[doc = "19: `10011`"]
    APORT4CH24TO31 = 19,
}
impl From<INPUT0TO7SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT0TO7SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `INPUT0TO7SEL`"]
pub type INPUT0TO7SEL_R = crate::R<u8, INPUT0TO7SEL_A>;
impl INPUT0TO7SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, INPUT0TO7SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(INPUT0TO7SEL_A::APORT0CH0TO7),
            1 => Val(INPUT0TO7SEL_A::APORT0CH8TO15),
            4 => Val(INPUT0TO7SEL_A::APORT1CH0TO7),
            5 => Val(INPUT0TO7SEL_A::APORT1CH8TO15),
            6 => Val(INPUT0TO7SEL_A::APORT1CH16TO23),
            7 => Val(INPUT0TO7SEL_A::APORT1CH24TO31),
            8 => Val(INPUT0TO7SEL_A::APORT2CH0TO7),
            9 => Val(INPUT0TO7SEL_A::APORT2CH8TO15),
            10 => Val(INPUT0TO7SEL_A::APORT2CH16TO23),
            11 => Val(INPUT0TO7SEL_A::APORT2CH24TO31),
            12 => Val(INPUT0TO7SEL_A::APORT3CH0TO7),
            13 => Val(INPUT0TO7SEL_A::APORT3CH8TO15),
            14 => Val(INPUT0TO7SEL_A::APORT3CH16TO23),
            15 => Val(INPUT0TO7SEL_A::APORT3CH24TO31),
            16 => Val(INPUT0TO7SEL_A::APORT4CH0TO7),
            17 => Val(INPUT0TO7SEL_A::APORT4CH8TO15),
            18 => Val(INPUT0TO7SEL_A::APORT4CH16TO23),
            19 => Val(INPUT0TO7SEL_A::APORT4CH24TO31),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `APORT0CH0TO7`"]
    #[inline(always)]
    pub fn is_aport0ch0to7(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT0CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT0CH8TO15`"]
    #[inline(always)]
    pub fn is_aport0ch8to15(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT0CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH0TO7`"]
    #[inline(always)]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT1CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT1CH8TO15`"]
    #[inline(always)]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT1CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH16TO23`"]
    #[inline(always)]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT1CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT1CH24TO31`"]
    #[inline(always)]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT1CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT2CH0TO7`"]
    #[inline(always)]
    pub fn is_aport2ch0to7(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT2CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT2CH8TO15`"]
    #[inline(always)]
    pub fn is_aport2ch8to15(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT2CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT2CH16TO23`"]
    #[inline(always)]
    pub fn is_aport2ch16to23(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT2CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT2CH24TO31`"]
    #[inline(always)]
    pub fn is_aport2ch24to31(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT2CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT3CH0TO7`"]
    #[inline(always)]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT3CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT3CH8TO15`"]
    #[inline(always)]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT3CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT3CH16TO23`"]
    #[inline(always)]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT3CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT3CH24TO31`"]
    #[inline(always)]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT3CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT4CH0TO7`"]
    #[inline(always)]
    pub fn is_aport4ch0to7(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT4CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT4CH8TO15`"]
    #[inline(always)]
    pub fn is_aport4ch8to15(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT4CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT4CH16TO23`"]
    #[inline(always)]
    pub fn is_aport4ch16to23(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT4CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT4CH24TO31`"]
    #[inline(always)]
    pub fn is_aport4ch24to31(&self) -> bool {
        *self == INPUT0TO7SEL_A::APORT4CH24TO31
    }
}
#[doc = "Write proxy for field `INPUT0TO7SEL`"]
pub struct INPUT0TO7SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT0TO7SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INPUT0TO7SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn aport0ch0to7(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT0CH0TO7)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn aport0ch8to15(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT0CH8TO15)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn aport1ch0to7(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT1CH0TO7)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn aport1ch8to15(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT1CH8TO15)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn aport1ch16to23(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT1CH16TO23)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn aport1ch24to31(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT1CH24TO31)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn aport2ch0to7(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT2CH0TO7)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn aport2ch8to15(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT2CH8TO15)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn aport2ch16to23(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT2CH16TO23)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn aport2ch24to31(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT2CH24TO31)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn aport3ch0to7(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT3CH0TO7)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn aport3ch8to15(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT3CH8TO15)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn aport3ch16to23(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT3CH16TO23)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn aport3ch24to31(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT3CH24TO31)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn aport4ch0to7(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT4CH0TO7)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn aport4ch8to15(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT4CH8TO15)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn aport4ch16to23(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT4CH16TO23)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn aport4ch24to31(self) -> &'a mut W {
        self.variant(INPUT0TO7SEL_A::APORT4CH24TO31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Inputs Chosen for ADCn_INPUT8-ADCn_INPUT15 as Referred in SCANMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPUT8TO15SEL_A {
    #[doc = "0: `0`"]
    APORT0CH0TO7 = 0,
    #[doc = "1: `1`"]
    APORT0CH8TO15 = 1,
    #[doc = "4: `100`"]
    APORT1CH0TO7 = 4,
    #[doc = "5: `101`"]
    APORT1CH8TO15 = 5,
    #[doc = "6: `110`"]
    APORT1CH16TO23 = 6,
    #[doc = "7: `111`"]
    APORT1CH24TO31 = 7,
    #[doc = "8: `1000`"]
    APORT2CH0TO7 = 8,
    #[doc = "9: `1001`"]
    APORT2CH8TO15 = 9,
    #[doc = "10: `1010`"]
    APORT2CH16TO23 = 10,
    #[doc = "11: `1011`"]
    APORT2CH24TO31 = 11,
    #[doc = "12: `1100`"]
    APORT3CH0TO7 = 12,
    #[doc = "13: `1101`"]
    APORT3CH8TO15 = 13,
    #[doc = "14: `1110`"]
    APORT3CH16TO23 = 14,
    #[doc = "15: `1111`"]
    APORT3CH24TO31 = 15,
    #[doc = "16: `10000`"]
    APORT4CH0TO7 = 16,
    #[doc = "17: `10001`"]
    APORT4CH8TO15 = 17,
    #[doc = "18: `10010`"]
    APORT4CH16TO23 = 18,
    #[doc = "19: `10011`"]
    APORT4CH24TO31 = 19,
}
impl From<INPUT8TO15SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT8TO15SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `INPUT8TO15SEL`"]
pub type INPUT8TO15SEL_R = crate::R<u8, INPUT8TO15SEL_A>;
impl INPUT8TO15SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, INPUT8TO15SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(INPUT8TO15SEL_A::APORT0CH0TO7),
            1 => Val(INPUT8TO15SEL_A::APORT0CH8TO15),
            4 => Val(INPUT8TO15SEL_A::APORT1CH0TO7),
            5 => Val(INPUT8TO15SEL_A::APORT1CH8TO15),
            6 => Val(INPUT8TO15SEL_A::APORT1CH16TO23),
            7 => Val(INPUT8TO15SEL_A::APORT1CH24TO31),
            8 => Val(INPUT8TO15SEL_A::APORT2CH0TO7),
            9 => Val(INPUT8TO15SEL_A::APORT2CH8TO15),
            10 => Val(INPUT8TO15SEL_A::APORT2CH16TO23),
            11 => Val(INPUT8TO15SEL_A::APORT2CH24TO31),
            12 => Val(INPUT8TO15SEL_A::APORT3CH0TO7),
            13 => Val(INPUT8TO15SEL_A::APORT3CH8TO15),
            14 => Val(INPUT8TO15SEL_A::APORT3CH16TO23),
            15 => Val(INPUT8TO15SEL_A::APORT3CH24TO31),
            16 => Val(INPUT8TO15SEL_A::APORT4CH0TO7),
            17 => Val(INPUT8TO15SEL_A::APORT4CH8TO15),
            18 => Val(INPUT8TO15SEL_A::APORT4CH16TO23),
            19 => Val(INPUT8TO15SEL_A::APORT4CH24TO31),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `APORT0CH0TO7`"]
    #[inline(always)]
    pub fn is_aport0ch0to7(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT0CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT0CH8TO15`"]
    #[inline(always)]
    pub fn is_aport0ch8to15(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT0CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH0TO7`"]
    #[inline(always)]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT1CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT1CH8TO15`"]
    #[inline(always)]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT1CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH16TO23`"]
    #[inline(always)]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT1CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT1CH24TO31`"]
    #[inline(always)]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT1CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT2CH0TO7`"]
    #[inline(always)]
    pub fn is_aport2ch0to7(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT2CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT2CH8TO15`"]
    #[inline(always)]
    pub fn is_aport2ch8to15(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT2CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT2CH16TO23`"]
    #[inline(always)]
    pub fn is_aport2ch16to23(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT2CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT2CH24TO31`"]
    #[inline(always)]
    pub fn is_aport2ch24to31(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT2CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT3CH0TO7`"]
    #[inline(always)]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT3CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT3CH8TO15`"]
    #[inline(always)]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT3CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT3CH16TO23`"]
    #[inline(always)]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT3CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT3CH24TO31`"]
    #[inline(always)]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT3CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT4CH0TO7`"]
    #[inline(always)]
    pub fn is_aport4ch0to7(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT4CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT4CH8TO15`"]
    #[inline(always)]
    pub fn is_aport4ch8to15(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT4CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT4CH16TO23`"]
    #[inline(always)]
    pub fn is_aport4ch16to23(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT4CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT4CH24TO31`"]
    #[inline(always)]
    pub fn is_aport4ch24to31(&self) -> bool {
        *self == INPUT8TO15SEL_A::APORT4CH24TO31
    }
}
#[doc = "Write proxy for field `INPUT8TO15SEL`"]
pub struct INPUT8TO15SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT8TO15SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INPUT8TO15SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn aport0ch0to7(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT0CH0TO7)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn aport0ch8to15(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT0CH8TO15)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn aport1ch0to7(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT1CH0TO7)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn aport1ch8to15(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT1CH8TO15)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn aport1ch16to23(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT1CH16TO23)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn aport1ch24to31(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT1CH24TO31)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn aport2ch0to7(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT2CH0TO7)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn aport2ch8to15(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT2CH8TO15)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn aport2ch16to23(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT2CH16TO23)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn aport2ch24to31(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT2CH24TO31)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn aport3ch0to7(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT3CH0TO7)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn aport3ch8to15(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT3CH8TO15)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn aport3ch16to23(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT3CH16TO23)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn aport3ch24to31(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT3CH24TO31)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn aport4ch0to7(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT4CH0TO7)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn aport4ch8to15(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT4CH8TO15)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn aport4ch16to23(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT4CH16TO23)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn aport4ch24to31(self) -> &'a mut W {
        self.variant(INPUT8TO15SEL_A::APORT4CH24TO31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Inputs Chosen for ADCn_INPUT16-ADCn_INPUT23 as Referred in SCANMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPUT16TO23SEL_A {
    #[doc = "0: `0`"]
    APORT0CH0TO7 = 0,
    #[doc = "1: `1`"]
    APORT0CH8TO15 = 1,
    #[doc = "4: `100`"]
    APORT1CH0TO7 = 4,
    #[doc = "5: `101`"]
    APORT1CH8TO15 = 5,
    #[doc = "6: `110`"]
    APORT1CH16TO23 = 6,
    #[doc = "7: `111`"]
    APORT1CH24TO31 = 7,
    #[doc = "8: `1000`"]
    APORT2CH0TO7 = 8,
    #[doc = "9: `1001`"]
    APORT2CH8TO15 = 9,
    #[doc = "10: `1010`"]
    APORT2CH16TO23 = 10,
    #[doc = "11: `1011`"]
    APORT2CH24TO31 = 11,
    #[doc = "12: `1100`"]
    APORT3CH0TO7 = 12,
    #[doc = "13: `1101`"]
    APORT3CH8TO15 = 13,
    #[doc = "14: `1110`"]
    APORT3CH16TO23 = 14,
    #[doc = "15: `1111`"]
    APORT3CH24TO31 = 15,
    #[doc = "16: `10000`"]
    APORT4CH0TO7 = 16,
    #[doc = "17: `10001`"]
    APORT4CH8TO15 = 17,
    #[doc = "18: `10010`"]
    APORT4CH16TO23 = 18,
    #[doc = "19: `10011`"]
    APORT4CH24TO31 = 19,
}
impl From<INPUT16TO23SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT16TO23SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `INPUT16TO23SEL`"]
pub type INPUT16TO23SEL_R = crate::R<u8, INPUT16TO23SEL_A>;
impl INPUT16TO23SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, INPUT16TO23SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(INPUT16TO23SEL_A::APORT0CH0TO7),
            1 => Val(INPUT16TO23SEL_A::APORT0CH8TO15),
            4 => Val(INPUT16TO23SEL_A::APORT1CH0TO7),
            5 => Val(INPUT16TO23SEL_A::APORT1CH8TO15),
            6 => Val(INPUT16TO23SEL_A::APORT1CH16TO23),
            7 => Val(INPUT16TO23SEL_A::APORT1CH24TO31),
            8 => Val(INPUT16TO23SEL_A::APORT2CH0TO7),
            9 => Val(INPUT16TO23SEL_A::APORT2CH8TO15),
            10 => Val(INPUT16TO23SEL_A::APORT2CH16TO23),
            11 => Val(INPUT16TO23SEL_A::APORT2CH24TO31),
            12 => Val(INPUT16TO23SEL_A::APORT3CH0TO7),
            13 => Val(INPUT16TO23SEL_A::APORT3CH8TO15),
            14 => Val(INPUT16TO23SEL_A::APORT3CH16TO23),
            15 => Val(INPUT16TO23SEL_A::APORT3CH24TO31),
            16 => Val(INPUT16TO23SEL_A::APORT4CH0TO7),
            17 => Val(INPUT16TO23SEL_A::APORT4CH8TO15),
            18 => Val(INPUT16TO23SEL_A::APORT4CH16TO23),
            19 => Val(INPUT16TO23SEL_A::APORT4CH24TO31),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `APORT0CH0TO7`"]
    #[inline(always)]
    pub fn is_aport0ch0to7(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT0CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT0CH8TO15`"]
    #[inline(always)]
    pub fn is_aport0ch8to15(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT0CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH0TO7`"]
    #[inline(always)]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT1CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT1CH8TO15`"]
    #[inline(always)]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT1CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH16TO23`"]
    #[inline(always)]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT1CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT1CH24TO31`"]
    #[inline(always)]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT1CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT2CH0TO7`"]
    #[inline(always)]
    pub fn is_aport2ch0to7(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT2CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT2CH8TO15`"]
    #[inline(always)]
    pub fn is_aport2ch8to15(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT2CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT2CH16TO23`"]
    #[inline(always)]
    pub fn is_aport2ch16to23(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT2CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT2CH24TO31`"]
    #[inline(always)]
    pub fn is_aport2ch24to31(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT2CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT3CH0TO7`"]
    #[inline(always)]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT3CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT3CH8TO15`"]
    #[inline(always)]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT3CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT3CH16TO23`"]
    #[inline(always)]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT3CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT3CH24TO31`"]
    #[inline(always)]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT3CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT4CH0TO7`"]
    #[inline(always)]
    pub fn is_aport4ch0to7(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT4CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT4CH8TO15`"]
    #[inline(always)]
    pub fn is_aport4ch8to15(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT4CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT4CH16TO23`"]
    #[inline(always)]
    pub fn is_aport4ch16to23(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT4CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT4CH24TO31`"]
    #[inline(always)]
    pub fn is_aport4ch24to31(&self) -> bool {
        *self == INPUT16TO23SEL_A::APORT4CH24TO31
    }
}
#[doc = "Write proxy for field `INPUT16TO23SEL`"]
pub struct INPUT16TO23SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT16TO23SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INPUT16TO23SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn aport0ch0to7(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT0CH0TO7)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn aport0ch8to15(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT0CH8TO15)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn aport1ch0to7(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT1CH0TO7)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn aport1ch8to15(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT1CH8TO15)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn aport1ch16to23(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT1CH16TO23)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn aport1ch24to31(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT1CH24TO31)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn aport2ch0to7(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT2CH0TO7)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn aport2ch8to15(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT2CH8TO15)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn aport2ch16to23(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT2CH16TO23)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn aport2ch24to31(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT2CH24TO31)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn aport3ch0to7(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT3CH0TO7)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn aport3ch8to15(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT3CH8TO15)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn aport3ch16to23(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT3CH16TO23)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn aport3ch24to31(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT3CH24TO31)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn aport4ch0to7(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT4CH0TO7)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn aport4ch8to15(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT4CH8TO15)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn aport4ch16to23(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT4CH16TO23)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn aport4ch24to31(self) -> &'a mut W {
        self.variant(INPUT16TO23SEL_A::APORT4CH24TO31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
#[doc = "Inputs Chosen for ADCn_INPUT24-ADCn_INPUT31 as Referred in SCANMASK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INPUT24TO31SEL_A {
    #[doc = "0: `0`"]
    APORT0CH0TO7 = 0,
    #[doc = "1: `1`"]
    APORT0CH8TO15 = 1,
    #[doc = "4: `100`"]
    APORT1CH0TO7 = 4,
    #[doc = "5: `101`"]
    APORT1CH8TO15 = 5,
    #[doc = "6: `110`"]
    APORT1CH16TO23 = 6,
    #[doc = "7: `111`"]
    APORT1CH24TO31 = 7,
    #[doc = "8: `1000`"]
    APORT2CH0TO7 = 8,
    #[doc = "9: `1001`"]
    APORT2CH8TO15 = 9,
    #[doc = "10: `1010`"]
    APORT2CH16TO23 = 10,
    #[doc = "11: `1011`"]
    APORT2CH24TO31 = 11,
    #[doc = "12: `1100`"]
    APORT3CH0TO7 = 12,
    #[doc = "13: `1101`"]
    APORT3CH8TO15 = 13,
    #[doc = "14: `1110`"]
    APORT3CH16TO23 = 14,
    #[doc = "15: `1111`"]
    APORT3CH24TO31 = 15,
    #[doc = "16: `10000`"]
    APORT4CH0TO7 = 16,
    #[doc = "17: `10001`"]
    APORT4CH8TO15 = 17,
    #[doc = "18: `10010`"]
    APORT4CH16TO23 = 18,
    #[doc = "19: `10011`"]
    APORT4CH24TO31 = 19,
}
impl From<INPUT24TO31SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: INPUT24TO31SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `INPUT24TO31SEL`"]
pub type INPUT24TO31SEL_R = crate::R<u8, INPUT24TO31SEL_A>;
impl INPUT24TO31SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, INPUT24TO31SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(INPUT24TO31SEL_A::APORT0CH0TO7),
            1 => Val(INPUT24TO31SEL_A::APORT0CH8TO15),
            4 => Val(INPUT24TO31SEL_A::APORT1CH0TO7),
            5 => Val(INPUT24TO31SEL_A::APORT1CH8TO15),
            6 => Val(INPUT24TO31SEL_A::APORT1CH16TO23),
            7 => Val(INPUT24TO31SEL_A::APORT1CH24TO31),
            8 => Val(INPUT24TO31SEL_A::APORT2CH0TO7),
            9 => Val(INPUT24TO31SEL_A::APORT2CH8TO15),
            10 => Val(INPUT24TO31SEL_A::APORT2CH16TO23),
            11 => Val(INPUT24TO31SEL_A::APORT2CH24TO31),
            12 => Val(INPUT24TO31SEL_A::APORT3CH0TO7),
            13 => Val(INPUT24TO31SEL_A::APORT3CH8TO15),
            14 => Val(INPUT24TO31SEL_A::APORT3CH16TO23),
            15 => Val(INPUT24TO31SEL_A::APORT3CH24TO31),
            16 => Val(INPUT24TO31SEL_A::APORT4CH0TO7),
            17 => Val(INPUT24TO31SEL_A::APORT4CH8TO15),
            18 => Val(INPUT24TO31SEL_A::APORT4CH16TO23),
            19 => Val(INPUT24TO31SEL_A::APORT4CH24TO31),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `APORT0CH0TO7`"]
    #[inline(always)]
    pub fn is_aport0ch0to7(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT0CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT0CH8TO15`"]
    #[inline(always)]
    pub fn is_aport0ch8to15(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT0CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH0TO7`"]
    #[inline(always)]
    pub fn is_aport1ch0to7(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT1CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT1CH8TO15`"]
    #[inline(always)]
    pub fn is_aport1ch8to15(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT1CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT1CH16TO23`"]
    #[inline(always)]
    pub fn is_aport1ch16to23(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT1CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT1CH24TO31`"]
    #[inline(always)]
    pub fn is_aport1ch24to31(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT1CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT2CH0TO7`"]
    #[inline(always)]
    pub fn is_aport2ch0to7(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT2CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT2CH8TO15`"]
    #[inline(always)]
    pub fn is_aport2ch8to15(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT2CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT2CH16TO23`"]
    #[inline(always)]
    pub fn is_aport2ch16to23(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT2CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT2CH24TO31`"]
    #[inline(always)]
    pub fn is_aport2ch24to31(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT2CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT3CH0TO7`"]
    #[inline(always)]
    pub fn is_aport3ch0to7(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT3CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT3CH8TO15`"]
    #[inline(always)]
    pub fn is_aport3ch8to15(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT3CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT3CH16TO23`"]
    #[inline(always)]
    pub fn is_aport3ch16to23(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT3CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT3CH24TO31`"]
    #[inline(always)]
    pub fn is_aport3ch24to31(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT3CH24TO31
    }
    #[doc = "Checks if the value of the field is `APORT4CH0TO7`"]
    #[inline(always)]
    pub fn is_aport4ch0to7(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT4CH0TO7
    }
    #[doc = "Checks if the value of the field is `APORT4CH8TO15`"]
    #[inline(always)]
    pub fn is_aport4ch8to15(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT4CH8TO15
    }
    #[doc = "Checks if the value of the field is `APORT4CH16TO23`"]
    #[inline(always)]
    pub fn is_aport4ch16to23(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT4CH16TO23
    }
    #[doc = "Checks if the value of the field is `APORT4CH24TO31`"]
    #[inline(always)]
    pub fn is_aport4ch24to31(&self) -> bool {
        *self == INPUT24TO31SEL_A::APORT4CH24TO31
    }
}
#[doc = "Write proxy for field `INPUT24TO31SEL`"]
pub struct INPUT24TO31SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> INPUT24TO31SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INPUT24TO31SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn aport0ch0to7(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT0CH0TO7)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn aport0ch8to15(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT0CH8TO15)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn aport1ch0to7(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT1CH0TO7)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn aport1ch8to15(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT1CH8TO15)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn aport1ch16to23(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT1CH16TO23)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn aport1ch24to31(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT1CH24TO31)
    }
    #[doc = "`1000`"]
    #[inline(always)]
    pub fn aport2ch0to7(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT2CH0TO7)
    }
    #[doc = "`1001`"]
    #[inline(always)]
    pub fn aport2ch8to15(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT2CH8TO15)
    }
    #[doc = "`1010`"]
    #[inline(always)]
    pub fn aport2ch16to23(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT2CH16TO23)
    }
    #[doc = "`1011`"]
    #[inline(always)]
    pub fn aport2ch24to31(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT2CH24TO31)
    }
    #[doc = "`1100`"]
    #[inline(always)]
    pub fn aport3ch0to7(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT3CH0TO7)
    }
    #[doc = "`1101`"]
    #[inline(always)]
    pub fn aport3ch8to15(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT3CH8TO15)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn aport3ch16to23(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT3CH16TO23)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn aport3ch24to31(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT3CH24TO31)
    }
    #[doc = "`10000`"]
    #[inline(always)]
    pub fn aport4ch0to7(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT4CH0TO7)
    }
    #[doc = "`10001`"]
    #[inline(always)]
    pub fn aport4ch8to15(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT4CH8TO15)
    }
    #[doc = "`10010`"]
    #[inline(always)]
    pub fn aport4ch16to23(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT4CH16TO23)
    }
    #[doc = "`10011`"]
    #[inline(always)]
    pub fn aport4ch24to31(self) -> &'a mut W {
        self.variant(INPUT24TO31SEL_A::APORT4CH24TO31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Inputs Chosen for ADCn_INPUT7-ADCn_INPUT0 as Referred in SCANMASK"]
    #[inline(always)]
    pub fn input0to7sel(&self) -> INPUT0TO7SEL_R {
        INPUT0TO7SEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Inputs Chosen for ADCn_INPUT8-ADCn_INPUT15 as Referred in SCANMASK"]
    #[inline(always)]
    pub fn input8to15sel(&self) -> INPUT8TO15SEL_R {
        INPUT8TO15SEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Inputs Chosen for ADCn_INPUT16-ADCn_INPUT23 as Referred in SCANMASK"]
    #[inline(always)]
    pub fn input16to23sel(&self) -> INPUT16TO23SEL_R {
        INPUT16TO23SEL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Inputs Chosen for ADCn_INPUT24-ADCn_INPUT31 as Referred in SCANMASK"]
    #[inline(always)]
    pub fn input24to31sel(&self) -> INPUT24TO31SEL_R {
        INPUT24TO31SEL_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Inputs Chosen for ADCn_INPUT7-ADCn_INPUT0 as Referred in SCANMASK"]
    #[inline(always)]
    pub fn input0to7sel(&mut self) -> INPUT0TO7SEL_W {
        INPUT0TO7SEL_W { w: self }
    }
    #[doc = "Bits 8:12 - Inputs Chosen for ADCn_INPUT8-ADCn_INPUT15 as Referred in SCANMASK"]
    #[inline(always)]
    pub fn input8to15sel(&mut self) -> INPUT8TO15SEL_W {
        INPUT8TO15SEL_W { w: self }
    }
    #[doc = "Bits 16:20 - Inputs Chosen for ADCn_INPUT16-ADCn_INPUT23 as Referred in SCANMASK"]
    #[inline(always)]
    pub fn input16to23sel(&mut self) -> INPUT16TO23SEL_W {
        INPUT16TO23SEL_W { w: self }
    }
    #[doc = "Bits 24:28 - Inputs Chosen for ADCn_INPUT24-ADCn_INPUT31 as Referred in SCANMASK"]
    #[inline(always)]
    pub fn input24to31sel(&mut self) -> INPUT24TO31SEL_W {
        INPUT24TO31SEL_W { w: self }
    }
}
