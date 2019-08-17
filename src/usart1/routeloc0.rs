#[doc = "Reader of register ROUTELOC0"]
pub type R = crate::R<u32, super::ROUTELOC0>;
#[doc = "Writer for register ROUTELOC0"]
pub type W = crate::W<u32, super::ROUTELOC0>;
#[doc = "Register ROUTELOC0 `reset()`'s with value 0"]
impl crate::ResetValue for super::ROUTELOC0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `RXLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RXLOC_A {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = "Location 3"]
    LOC3,
    #[doc = "Location 4"]
    LOC4,
    #[doc = "Location 5"]
    LOC5,
    #[doc = "Location 6"]
    LOC6,
}
impl crate::ToBits<u8> for RXLOC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            RXLOC_A::LOC0 => 0,
            RXLOC_A::LOC1 => 1,
            RXLOC_A::LOC2 => 2,
            RXLOC_A::LOC3 => 3,
            RXLOC_A::LOC4 => 4,
            RXLOC_A::LOC5 => 5,
            RXLOC_A::LOC6 => 6,
        }
    }
}
#[doc = "Reader of field `RXLOC`"]
pub type RXLOC_R = crate::R<u8, RXLOC_A>;
impl RXLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RXLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RXLOC_A::LOC0),
            1 => Val(RXLOC_A::LOC1),
            2 => Val(RXLOC_A::LOC2),
            3 => Val(RXLOC_A::LOC3),
            4 => Val(RXLOC_A::LOC4),
            5 => Val(RXLOC_A::LOC5),
            6 => Val(RXLOC_A::LOC6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == RXLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == RXLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == RXLOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == RXLOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == RXLOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == RXLOC_A::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == RXLOC_A::LOC6
    }
}
#[doc = "Write proxy for field `RXLOC`"]
pub struct RXLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> RXLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RXLOC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(RXLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(RXLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(RXLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(RXLOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(RXLOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(RXLOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut W {
        self.variant(RXLOC_A::LOC6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Possible values of the field `TXLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXLOC_A {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = "Location 3"]
    LOC3,
    #[doc = "Location 4"]
    LOC4,
    #[doc = "Location 5"]
    LOC5,
    #[doc = "Location 6"]
    LOC6,
}
impl crate::ToBits<u8> for TXLOC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            TXLOC_A::LOC0 => 0,
            TXLOC_A::LOC1 => 1,
            TXLOC_A::LOC2 => 2,
            TXLOC_A::LOC3 => 3,
            TXLOC_A::LOC4 => 4,
            TXLOC_A::LOC5 => 5,
            TXLOC_A::LOC6 => 6,
        }
    }
}
#[doc = "Reader of field `TXLOC`"]
pub type TXLOC_R = crate::R<u8, TXLOC_A>;
impl TXLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TXLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TXLOC_A::LOC0),
            1 => Val(TXLOC_A::LOC1),
            2 => Val(TXLOC_A::LOC2),
            3 => Val(TXLOC_A::LOC3),
            4 => Val(TXLOC_A::LOC4),
            5 => Val(TXLOC_A::LOC5),
            6 => Val(TXLOC_A::LOC6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == TXLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == TXLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == TXLOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == TXLOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == TXLOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == TXLOC_A::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == TXLOC_A::LOC6
    }
}
#[doc = "Write proxy for field `TXLOC`"]
pub struct TXLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> TXLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TXLOC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(TXLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(TXLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(TXLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(TXLOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(TXLOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(TXLOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut W {
        self.variant(TXLOC_A::LOC6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `CSLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSLOC_A {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = "Location 3"]
    LOC3,
    #[doc = "Location 4"]
    LOC4,
    #[doc = "Location 5"]
    LOC5,
    #[doc = "Location 6"]
    LOC6,
}
impl crate::ToBits<u8> for CSLOC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CSLOC_A::LOC0 => 0,
            CSLOC_A::LOC1 => 1,
            CSLOC_A::LOC2 => 2,
            CSLOC_A::LOC3 => 3,
            CSLOC_A::LOC4 => 4,
            CSLOC_A::LOC5 => 5,
            CSLOC_A::LOC6 => 6,
        }
    }
}
#[doc = "Reader of field `CSLOC`"]
pub type CSLOC_R = crate::R<u8, CSLOC_A>;
impl CSLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CSLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CSLOC_A::LOC0),
            1 => Val(CSLOC_A::LOC1),
            2 => Val(CSLOC_A::LOC2),
            3 => Val(CSLOC_A::LOC3),
            4 => Val(CSLOC_A::LOC4),
            5 => Val(CSLOC_A::LOC5),
            6 => Val(CSLOC_A::LOC6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CSLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CSLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CSLOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CSLOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CSLOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CSLOC_A::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CSLOC_A::LOC6
    }
}
#[doc = "Write proxy for field `CSLOC`"]
pub struct CSLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> CSLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSLOC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CSLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CSLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CSLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CSLOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CSLOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CSLOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut W {
        self.variant(CSLOC_A::LOC6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `CLKLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CLKLOC_A {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = "Location 3"]
    LOC3,
    #[doc = "Location 4"]
    LOC4,
    #[doc = "Location 5"]
    LOC5,
    #[doc = "Location 6"]
    LOC6,
}
impl crate::ToBits<u8> for CLKLOC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CLKLOC_A::LOC0 => 0,
            CLKLOC_A::LOC1 => 1,
            CLKLOC_A::LOC2 => 2,
            CLKLOC_A::LOC3 => 3,
            CLKLOC_A::LOC4 => 4,
            CLKLOC_A::LOC5 => 5,
            CLKLOC_A::LOC6 => 6,
        }
    }
}
#[doc = "Reader of field `CLKLOC`"]
pub type CLKLOC_R = crate::R<u8, CLKLOC_A>;
impl CLKLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CLKLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CLKLOC_A::LOC0),
            1 => Val(CLKLOC_A::LOC1),
            2 => Val(CLKLOC_A::LOC2),
            3 => Val(CLKLOC_A::LOC3),
            4 => Val(CLKLOC_A::LOC4),
            5 => Val(CLKLOC_A::LOC5),
            6 => Val(CLKLOC_A::LOC6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CLKLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CLKLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CLKLOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CLKLOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CLKLOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CLKLOC_A::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CLKLOC_A::LOC6
    }
}
#[doc = "Write proxy for field `CLKLOC`"]
pub struct CLKLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLKLOC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CLKLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CLKLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CLKLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CLKLOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CLKLOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CLKLOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut W {
        self.variant(CLKLOC_A::LOC6)
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
    pub fn rxloc(&self) -> RXLOC_R {
        RXLOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn txloc(&self) -> TXLOC_R {
        TXLOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn csloc(&self) -> CSLOC_R {
        CSLOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn clkloc(&self) -> CLKLOC_R {
        CLKLOC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn rxloc(&mut self) -> RXLOC_W {
        RXLOC_W { w: self }
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn txloc(&mut self) -> TXLOC_W {
        TXLOC_W { w: self }
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn csloc(&mut self) -> CSLOC_W {
        CSLOC_W { w: self }
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn clkloc(&mut self) -> CLKLOC_W {
        CLKLOC_W { w: self }
    }
}
