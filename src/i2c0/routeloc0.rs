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
#[doc = "Possible values of the field `SDALOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDALOC_A {
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
    #[doc = "Location 7"]
    LOC7,
}
impl crate::ToBits<u8> for SDALOC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SDALOC_A::LOC0 => 0,
            SDALOC_A::LOC1 => 1,
            SDALOC_A::LOC2 => 2,
            SDALOC_A::LOC3 => 3,
            SDALOC_A::LOC4 => 4,
            SDALOC_A::LOC5 => 5,
            SDALOC_A::LOC6 => 6,
            SDALOC_A::LOC7 => 7,
        }
    }
}
#[doc = "Reader of field `SDALOC`"]
pub type SDALOC_R = crate::R<u8, SDALOC_A>;
impl SDALOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SDALOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SDALOC_A::LOC0),
            1 => Val(SDALOC_A::LOC1),
            2 => Val(SDALOC_A::LOC2),
            3 => Val(SDALOC_A::LOC3),
            4 => Val(SDALOC_A::LOC4),
            5 => Val(SDALOC_A::LOC5),
            6 => Val(SDALOC_A::LOC6),
            7 => Val(SDALOC_A::LOC7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == SDALOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == SDALOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == SDALOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == SDALOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == SDALOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == SDALOC_A::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == SDALOC_A::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == SDALOC_A::LOC7
    }
}
#[doc = "Write proxy for field `SDALOC`"]
pub struct SDALOC_W<'a> {
    w: &'a mut W,
}
impl<'a> SDALOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDALOC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(SDALOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(SDALOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(SDALOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(SDALOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(SDALOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(SDALOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut W {
        self.variant(SDALOC_A::LOC6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut W {
        self.variant(SDALOC_A::LOC7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Possible values of the field `SCLLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SCLLOC_A {
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
    #[doc = "Location 7"]
    LOC7,
}
impl crate::ToBits<u8> for SCLLOC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SCLLOC_A::LOC0 => 0,
            SCLLOC_A::LOC1 => 1,
            SCLLOC_A::LOC2 => 2,
            SCLLOC_A::LOC3 => 3,
            SCLLOC_A::LOC4 => 4,
            SCLLOC_A::LOC5 => 5,
            SCLLOC_A::LOC6 => 6,
            SCLLOC_A::LOC7 => 7,
        }
    }
}
#[doc = "Reader of field `SCLLOC`"]
pub type SCLLOC_R = crate::R<u8, SCLLOC_A>;
impl SCLLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SCLLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SCLLOC_A::LOC0),
            1 => Val(SCLLOC_A::LOC1),
            2 => Val(SCLLOC_A::LOC2),
            3 => Val(SCLLOC_A::LOC3),
            4 => Val(SCLLOC_A::LOC4),
            5 => Val(SCLLOC_A::LOC5),
            6 => Val(SCLLOC_A::LOC6),
            7 => Val(SCLLOC_A::LOC7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == SCLLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == SCLLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == SCLLOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == SCLLOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == SCLLOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == SCLLOC_A::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == SCLLOC_A::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == SCLLOC_A::LOC7
    }
}
#[doc = "Write proxy for field `SCLLOC`"]
pub struct SCLLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SCLLOC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(SCLLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(SCLLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(SCLLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(SCLLOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(SCLLOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(SCLLOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut W {
        self.variant(SCLLOC_A::LOC6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut W {
        self.variant(SCLLOC_A::LOC7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn sdaloc(&self) -> SDALOC_R {
        SDALOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn sclloc(&self) -> SCLLOC_R {
        SCLLOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn sdaloc(&mut self) -> SDALOC_W {
        SDALOC_W { w: self }
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn sclloc(&mut self) -> SCLLOC_W {
        SCLLOC_W { w: self }
    }
}
