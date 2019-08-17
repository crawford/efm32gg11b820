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
#[doc = "Possible values of the field `EBILOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EBILOC_A {
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
}
impl crate::ToBits<u8> for EBILOC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            EBILOC_A::LOC0 => 0,
            EBILOC_A::LOC1 => 1,
            EBILOC_A::LOC2 => 2,
            EBILOC_A::LOC3 => 3,
            EBILOC_A::LOC4 => 4,
            EBILOC_A::LOC5 => 5,
        }
    }
}
#[doc = "Reader of field `EBILOC`"]
pub type EBILOC_R = crate::R<u8, EBILOC_A>;
impl EBILOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, EBILOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(EBILOC_A::LOC0),
            1 => Val(EBILOC_A::LOC1),
            2 => Val(EBILOC_A::LOC2),
            3 => Val(EBILOC_A::LOC3),
            4 => Val(EBILOC_A::LOC4),
            5 => Val(EBILOC_A::LOC5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == EBILOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == EBILOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == EBILOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == EBILOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == EBILOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == EBILOC_A::LOC5
    }
}
#[doc = "Write proxy for field `EBILOC`"]
pub struct EBILOC_W<'a> {
    w: &'a mut W,
}
impl<'a> EBILOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EBILOC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(EBILOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(EBILOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(EBILOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(EBILOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(EBILOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(EBILOC_A::LOC5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `NANDLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NANDLOC_A {
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
}
impl crate::ToBits<u8> for NANDLOC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            NANDLOC_A::LOC0 => 0,
            NANDLOC_A::LOC1 => 1,
            NANDLOC_A::LOC2 => 2,
            NANDLOC_A::LOC3 => 3,
            NANDLOC_A::LOC4 => 4,
            NANDLOC_A::LOC5 => 5,
        }
    }
}
#[doc = "Reader of field `NANDLOC`"]
pub type NANDLOC_R = crate::R<u8, NANDLOC_A>;
impl NANDLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, NANDLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(NANDLOC_A::LOC0),
            1 => Val(NANDLOC_A::LOC1),
            2 => Val(NANDLOC_A::LOC2),
            3 => Val(NANDLOC_A::LOC3),
            4 => Val(NANDLOC_A::LOC4),
            5 => Val(NANDLOC_A::LOC5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == NANDLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == NANDLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == NANDLOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == NANDLOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == NANDLOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == NANDLOC_A::LOC5
    }
}
#[doc = "Write proxy for field `NANDLOC`"]
pub struct NANDLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> NANDLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NANDLOC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(NANDLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(NANDLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(NANDLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(NANDLOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(NANDLOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(NANDLOC_A::LOC5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `TFTLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TFTLOC_A {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = "Location 3"]
    LOC3,
}
impl crate::ToBits<u8> for TFTLOC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            TFTLOC_A::LOC0 => 0,
            TFTLOC_A::LOC1 => 1,
            TFTLOC_A::LOC2 => 2,
            TFTLOC_A::LOC3 => 3,
        }
    }
}
#[doc = "Reader of field `TFTLOC`"]
pub type TFTLOC_R = crate::R<u8, TFTLOC_A>;
impl TFTLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, TFTLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(TFTLOC_A::LOC0),
            1 => Val(TFTLOC_A::LOC1),
            2 => Val(TFTLOC_A::LOC2),
            3 => Val(TFTLOC_A::LOC3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == TFTLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == TFTLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == TFTLOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == TFTLOC_A::LOC3
    }
}
#[doc = "Write proxy for field `TFTLOC`"]
pub struct TFTLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> TFTLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TFTLOC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(TFTLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(TFTLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(TFTLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(TFTLOC_A::LOC3)
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
    pub fn ebiloc(&self) -> EBILOC_R {
        EBILOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn csloc(&self) -> CSLOC_R {
        CSLOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn nandloc(&self) -> NANDLOC_R {
        NANDLOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn tftloc(&self) -> TFTLOC_R {
        TFTLOC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ebiloc(&mut self) -> EBILOC_W {
        EBILOC_W { w: self }
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn csloc(&mut self) -> CSLOC_W {
        CSLOC_W { w: self }
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn nandloc(&mut self) -> NANDLOC_W {
        NANDLOC_W { w: self }
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn tftloc(&mut self) -> TFTLOC_W {
        TFTLOC_W { w: self }
    }
}
