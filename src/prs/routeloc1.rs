#[doc = "Reader of register ROUTELOC1"]
pub type R = crate::R<u32, super::ROUTELOC1>;
#[doc = "Writer for register ROUTELOC1"]
pub type W = crate::W<u32, super::ROUTELOC1>;
#[doc = "Register ROUTELOC1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ROUTELOC1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH4LOC_A {
    #[doc = "0: Location 0"]
    LOC0,
    #[doc = "1: Location 1"]
    LOC1,
    #[doc = "2: Location 2"]
    LOC2,
}
impl From<CH4LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH4LOC_A) -> Self {
        match variant {
            CH4LOC_A::LOC0 => 0,
            CH4LOC_A::LOC1 => 1,
            CH4LOC_A::LOC2 => 2,
        }
    }
}
#[doc = "Reader of field `CH4LOC`"]
pub type CH4LOC_R = crate::R<u8, CH4LOC_A>;
impl CH4LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CH4LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CH4LOC_A::LOC0),
            1 => Val(CH4LOC_A::LOC1),
            2 => Val(CH4LOC_A::LOC2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH4LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH4LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH4LOC_A::LOC2
    }
}
#[doc = "Write proxy for field `CH4LOC`"]
pub struct CH4LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH4LOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH4LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH4LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH4LOC_A::LOC2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH5LOC_A {
    #[doc = "0: Location 0"]
    LOC0,
    #[doc = "1: Location 1"]
    LOC1,
    #[doc = "2: Location 2"]
    LOC2,
}
impl From<CH5LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH5LOC_A) -> Self {
        match variant {
            CH5LOC_A::LOC0 => 0,
            CH5LOC_A::LOC1 => 1,
            CH5LOC_A::LOC2 => 2,
        }
    }
}
#[doc = "Reader of field `CH5LOC`"]
pub type CH5LOC_R = crate::R<u8, CH5LOC_A>;
impl CH5LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CH5LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CH5LOC_A::LOC0),
            1 => Val(CH5LOC_A::LOC1),
            2 => Val(CH5LOC_A::LOC2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH5LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH5LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH5LOC_A::LOC2
    }
}
#[doc = "Write proxy for field `CH5LOC`"]
pub struct CH5LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH5LOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH5LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH5LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH5LOC_A::LOC2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH6LOC_A {
    #[doc = "0: Location 0"]
    LOC0,
    #[doc = "1: Location 1"]
    LOC1,
    #[doc = "2: Location 2"]
    LOC2,
}
impl From<CH6LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH6LOC_A) -> Self {
        match variant {
            CH6LOC_A::LOC0 => 0,
            CH6LOC_A::LOC1 => 1,
            CH6LOC_A::LOC2 => 2,
        }
    }
}
#[doc = "Reader of field `CH6LOC`"]
pub type CH6LOC_R = crate::R<u8, CH6LOC_A>;
impl CH6LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CH6LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CH6LOC_A::LOC0),
            1 => Val(CH6LOC_A::LOC1),
            2 => Val(CH6LOC_A::LOC2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH6LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH6LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH6LOC_A::LOC2
    }
}
#[doc = "Write proxy for field `CH6LOC`"]
pub struct CH6LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH6LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH6LOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH6LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH6LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH6LOC_A::LOC2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH7LOC_A {
    #[doc = "0: Location 0"]
    LOC0,
    #[doc = "1: Location 1"]
    LOC1,
    #[doc = "2: Location 2"]
    LOC2,
}
impl From<CH7LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH7LOC_A) -> Self {
        match variant {
            CH7LOC_A::LOC0 => 0,
            CH7LOC_A::LOC1 => 1,
            CH7LOC_A::LOC2 => 2,
        }
    }
}
#[doc = "Reader of field `CH7LOC`"]
pub type CH7LOC_R = crate::R<u8, CH7LOC_A>;
impl CH7LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CH7LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CH7LOC_A::LOC0),
            1 => Val(CH7LOC_A::LOC1),
            2 => Val(CH7LOC_A::LOC2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH7LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH7LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH7LOC_A::LOC2
    }
}
#[doc = "Write proxy for field `CH7LOC`"]
pub struct CH7LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH7LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH7LOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH7LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH7LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH7LOC_A::LOC2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | (((value as u32) & 0x3f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch4loc(&self) -> CH4LOC_R {
        CH4LOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch5loc(&self) -> CH5LOC_R {
        CH5LOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch6loc(&self) -> CH6LOC_R {
        CH6LOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch7loc(&self) -> CH7LOC_R {
        CH7LOC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch4loc(&mut self) -> CH4LOC_W {
        CH4LOC_W { w: self }
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch5loc(&mut self) -> CH5LOC_W {
        CH5LOC_W { w: self }
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch6loc(&mut self) -> CH6LOC_W {
        CH6LOC_W { w: self }
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch7loc(&mut self) -> CH7LOC_W {
        CH7LOC_W { w: self }
    }
}
