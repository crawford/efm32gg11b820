#[doc = "Reader of register ROUTELOC2"]
pub type R = crate::R<u32, super::ROUTELOC2>;
#[doc = "Writer for register ROUTELOC2"]
pub type W = crate::W<u32, super::ROUTELOC2>;
#[doc = "Register ROUTELOC2 `reset()`'s with value 0"]
impl crate::ResetValue for super::ROUTELOC2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH8LOC_A {
    #[doc = "0: Location 0"]
    LOC0,
    #[doc = "1: Location 1"]
    LOC1,
    #[doc = "2: Location 2"]
    LOC2,
}
impl From<CH8LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH8LOC_A) -> Self {
        match variant {
            CH8LOC_A::LOC0 => 0,
            CH8LOC_A::LOC1 => 1,
            CH8LOC_A::LOC2 => 2,
        }
    }
}
#[doc = "Reader of field `CH8LOC`"]
pub type CH8LOC_R = crate::R<u8, CH8LOC_A>;
impl CH8LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CH8LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CH8LOC_A::LOC0),
            1 => Val(CH8LOC_A::LOC1),
            2 => Val(CH8LOC_A::LOC2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH8LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH8LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH8LOC_A::LOC2
    }
}
#[doc = "Write proxy for field `CH8LOC`"]
pub struct CH8LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH8LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH8LOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH8LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH8LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH8LOC_A::LOC2)
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
pub enum CH9LOC_A {
    #[doc = "0: Location 0"]
    LOC0,
    #[doc = "1: Location 1"]
    LOC1,
    #[doc = "2: Location 2"]
    LOC2,
}
impl From<CH9LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH9LOC_A) -> Self {
        match variant {
            CH9LOC_A::LOC0 => 0,
            CH9LOC_A::LOC1 => 1,
            CH9LOC_A::LOC2 => 2,
        }
    }
}
#[doc = "Reader of field `CH9LOC`"]
pub type CH9LOC_R = crate::R<u8, CH9LOC_A>;
impl CH9LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CH9LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CH9LOC_A::LOC0),
            1 => Val(CH9LOC_A::LOC1),
            2 => Val(CH9LOC_A::LOC2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH9LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH9LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH9LOC_A::LOC2
    }
}
#[doc = "Write proxy for field `CH9LOC`"]
pub struct CH9LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH9LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH9LOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH9LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH9LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH9LOC_A::LOC2)
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
pub enum CH10LOC_A {
    #[doc = "0: Location 0"]
    LOC0,
    #[doc = "1: Location 1"]
    LOC1,
    #[doc = "2: Location 2"]
    LOC2,
}
impl From<CH10LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH10LOC_A) -> Self {
        match variant {
            CH10LOC_A::LOC0 => 0,
            CH10LOC_A::LOC1 => 1,
            CH10LOC_A::LOC2 => 2,
        }
    }
}
#[doc = "Reader of field `CH10LOC`"]
pub type CH10LOC_R = crate::R<u8, CH10LOC_A>;
impl CH10LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CH10LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CH10LOC_A::LOC0),
            1 => Val(CH10LOC_A::LOC1),
            2 => Val(CH10LOC_A::LOC2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH10LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH10LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH10LOC_A::LOC2
    }
}
#[doc = "Write proxy for field `CH10LOC`"]
pub struct CH10LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH10LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH10LOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH10LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH10LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH10LOC_A::LOC2)
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
pub enum CH11LOC_A {
    #[doc = "0: Location 0"]
    LOC0,
    #[doc = "1: Location 1"]
    LOC1,
    #[doc = "2: Location 2"]
    LOC2,
}
impl From<CH11LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH11LOC_A) -> Self {
        match variant {
            CH11LOC_A::LOC0 => 0,
            CH11LOC_A::LOC1 => 1,
            CH11LOC_A::LOC2 => 2,
        }
    }
}
#[doc = "Reader of field `CH11LOC`"]
pub type CH11LOC_R = crate::R<u8, CH11LOC_A>;
impl CH11LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CH11LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CH11LOC_A::LOC0),
            1 => Val(CH11LOC_A::LOC1),
            2 => Val(CH11LOC_A::LOC2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH11LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH11LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH11LOC_A::LOC2
    }
}
#[doc = "Write proxy for field `CH11LOC`"]
pub struct CH11LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH11LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH11LOC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH11LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH11LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH11LOC_A::LOC2)
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
    pub fn ch8loc(&self) -> CH8LOC_R {
        CH8LOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch9loc(&self) -> CH9LOC_R {
        CH9LOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch10loc(&self) -> CH10LOC_R {
        CH10LOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch11loc(&self) -> CH11LOC_R {
        CH11LOC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch8loc(&mut self) -> CH8LOC_W {
        CH8LOC_W { w: self }
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch9loc(&mut self) -> CH9LOC_W {
        CH9LOC_W { w: self }
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch10loc(&mut self) -> CH10LOC_W {
        CH10LOC_W { w: self }
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch11loc(&mut self) -> CH11LOC_W {
        CH11LOC_W { w: self }
    }
}
