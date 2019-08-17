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
#[doc = "Possible values of the field `SWVLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWVLOC_A {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = "Location 3"]
    LOC3,
}
impl crate::ToBits<u8> for SWVLOC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SWVLOC_A::LOC0 => 0,
            SWVLOC_A::LOC1 => 1,
            SWVLOC_A::LOC2 => 2,
            SWVLOC_A::LOC3 => 3,
        }
    }
}
#[doc = "Reader of field `SWVLOC`"]
pub type SWVLOC_R = crate::R<u8, SWVLOC_A>;
impl SWVLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SWVLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SWVLOC_A::LOC0),
            1 => Val(SWVLOC_A::LOC1),
            2 => Val(SWVLOC_A::LOC2),
            3 => Val(SWVLOC_A::LOC3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == SWVLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == SWVLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == SWVLOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == SWVLOC_A::LOC3
    }
}
#[doc = "Write proxy for field `SWVLOC`"]
pub struct SWVLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> SWVLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWVLOC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(SWVLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(SWVLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(SWVLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(SWVLOC_A::LOC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Possible values of the field `ETMLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ETMLOC_A {
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
impl crate::ToBits<u8> for ETMLOC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            ETMLOC_A::LOC0 => 0,
            ETMLOC_A::LOC1 => 1,
            ETMLOC_A::LOC2 => 2,
            ETMLOC_A::LOC3 => 3,
            ETMLOC_A::LOC4 => 4,
            ETMLOC_A::LOC5 => 5,
        }
    }
}
#[doc = "Reader of field `ETMLOC`"]
pub type ETMLOC_R = crate::R<u8, ETMLOC_A>;
impl ETMLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ETMLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ETMLOC_A::LOC0),
            1 => Val(ETMLOC_A::LOC1),
            2 => Val(ETMLOC_A::LOC2),
            3 => Val(ETMLOC_A::LOC3),
            4 => Val(ETMLOC_A::LOC4),
            5 => Val(ETMLOC_A::LOC5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == ETMLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == ETMLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == ETMLOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == ETMLOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == ETMLOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == ETMLOC_A::LOC5
    }
}
#[doc = "Write proxy for field `ETMLOC`"]
pub struct ETMLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> ETMLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ETMLOC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(ETMLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(ETMLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(ETMLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(ETMLOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(ETMLOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(ETMLOC_A::LOC5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn swvloc(&self) -> SWVLOC_R {
        SWVLOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - I/O Location"]
    #[inline(always)]
    pub fn etmloc(&self) -> ETMLOC_R {
        ETMLOC_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn swvloc(&mut self) -> SWVLOC_W {
        SWVLOC_W { w: self }
    }
    #[doc = "Bits 6:11 - I/O Location"]
    #[inline(always)]
    pub fn etmloc(&mut self) -> ETMLOC_W {
        ETMLOC_W { w: self }
    }
}
