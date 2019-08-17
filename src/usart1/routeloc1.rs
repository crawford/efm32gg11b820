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
#[doc = "Possible values of the field `CTSLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTSLOC_A {
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
impl crate::ToBits<u8> for CTSLOC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CTSLOC_A::LOC0 => 0,
            CTSLOC_A::LOC1 => 1,
            CTSLOC_A::LOC2 => 2,
            CTSLOC_A::LOC3 => 3,
            CTSLOC_A::LOC4 => 4,
            CTSLOC_A::LOC5 => 5,
            CTSLOC_A::LOC6 => 6,
        }
    }
}
#[doc = "Reader of field `CTSLOC`"]
pub type CTSLOC_R = crate::R<u8, CTSLOC_A>;
impl CTSLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CTSLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CTSLOC_A::LOC0),
            1 => Val(CTSLOC_A::LOC1),
            2 => Val(CTSLOC_A::LOC2),
            3 => Val(CTSLOC_A::LOC3),
            4 => Val(CTSLOC_A::LOC4),
            5 => Val(CTSLOC_A::LOC5),
            6 => Val(CTSLOC_A::LOC6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CTSLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CTSLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CTSLOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CTSLOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CTSLOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CTSLOC_A::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CTSLOC_A::LOC6
    }
}
#[doc = "Write proxy for field `CTSLOC`"]
pub struct CTSLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTSLOC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CTSLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CTSLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CTSLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CTSLOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CTSLOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CTSLOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut W {
        self.variant(CTSLOC_A::LOC6)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Possible values of the field `RTSLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RTSLOC_A {
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
impl crate::ToBits<u8> for RTSLOC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            RTSLOC_A::LOC0 => 0,
            RTSLOC_A::LOC1 => 1,
            RTSLOC_A::LOC2 => 2,
            RTSLOC_A::LOC3 => 3,
            RTSLOC_A::LOC4 => 4,
            RTSLOC_A::LOC5 => 5,
            RTSLOC_A::LOC6 => 6,
        }
    }
}
#[doc = "Reader of field `RTSLOC`"]
pub type RTSLOC_R = crate::R<u8, RTSLOC_A>;
impl RTSLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RTSLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RTSLOC_A::LOC0),
            1 => Val(RTSLOC_A::LOC1),
            2 => Val(RTSLOC_A::LOC2),
            3 => Val(RTSLOC_A::LOC3),
            4 => Val(RTSLOC_A::LOC4),
            5 => Val(RTSLOC_A::LOC5),
            6 => Val(RTSLOC_A::LOC6),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == RTSLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == RTSLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == RTSLOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == RTSLOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == RTSLOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == RTSLOC_A::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == RTSLOC_A::LOC6
    }
}
#[doc = "Write proxy for field `RTSLOC`"]
pub struct RTSLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTSLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RTSLOC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(RTSLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(RTSLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(RTSLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(RTSLOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(RTSLOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(RTSLOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut W {
        self.variant(RTSLOC_A::LOC6)
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
    pub fn ctsloc(&self) -> CTSLOC_R {
        CTSLOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn rtsloc(&self) -> RTSLOC_R {
        RTSLOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ctsloc(&mut self) -> CTSLOC_W {
        CTSLOC_W { w: self }
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn rtsloc(&mut self) -> RTSLOC_W {
        RTSLOC_W { w: self }
    }
}
