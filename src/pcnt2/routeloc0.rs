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
#[doc = "Possible values of the field `S0INLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S0INLOC_A {
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
impl crate::ToBits<u8> for S0INLOC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            S0INLOC_A::LOC0 => 0,
            S0INLOC_A::LOC1 => 1,
            S0INLOC_A::LOC2 => 2,
            S0INLOC_A::LOC3 => 3,
            S0INLOC_A::LOC4 => 4,
            S0INLOC_A::LOC5 => 5,
            S0INLOC_A::LOC6 => 6,
            S0INLOC_A::LOC7 => 7,
        }
    }
}
#[doc = "Reader of field `S0INLOC`"]
pub type S0INLOC_R = crate::R<u8, S0INLOC_A>;
impl S0INLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, S0INLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(S0INLOC_A::LOC0),
            1 => Val(S0INLOC_A::LOC1),
            2 => Val(S0INLOC_A::LOC2),
            3 => Val(S0INLOC_A::LOC3),
            4 => Val(S0INLOC_A::LOC4),
            5 => Val(S0INLOC_A::LOC5),
            6 => Val(S0INLOC_A::LOC6),
            7 => Val(S0INLOC_A::LOC7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == S0INLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == S0INLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == S0INLOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == S0INLOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == S0INLOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == S0INLOC_A::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == S0INLOC_A::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == S0INLOC_A::LOC7
    }
}
#[doc = "Write proxy for field `S0INLOC`"]
pub struct S0INLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> S0INLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S0INLOC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut W {
        self.variant(S0INLOC_A::LOC7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Possible values of the field `S1INLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum S1INLOC_A {
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
impl crate::ToBits<u8> for S1INLOC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            S1INLOC_A::LOC0 => 0,
            S1INLOC_A::LOC1 => 1,
            S1INLOC_A::LOC2 => 2,
            S1INLOC_A::LOC3 => 3,
            S1INLOC_A::LOC4 => 4,
            S1INLOC_A::LOC5 => 5,
            S1INLOC_A::LOC6 => 6,
            S1INLOC_A::LOC7 => 7,
        }
    }
}
#[doc = "Reader of field `S1INLOC`"]
pub type S1INLOC_R = crate::R<u8, S1INLOC_A>;
impl S1INLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, S1INLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(S1INLOC_A::LOC0),
            1 => Val(S1INLOC_A::LOC1),
            2 => Val(S1INLOC_A::LOC2),
            3 => Val(S1INLOC_A::LOC3),
            4 => Val(S1INLOC_A::LOC4),
            5 => Val(S1INLOC_A::LOC5),
            6 => Val(S1INLOC_A::LOC6),
            7 => Val(S1INLOC_A::LOC7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == S1INLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == S1INLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == S1INLOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == S1INLOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == S1INLOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == S1INLOC_A::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == S1INLOC_A::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == S1INLOC_A::LOC7
    }
}
#[doc = "Write proxy for field `S1INLOC`"]
pub struct S1INLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> S1INLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: S1INLOC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut W {
        self.variant(S1INLOC_A::LOC7)
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
    pub fn s0inloc(&self) -> S0INLOC_R {
        S0INLOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn s1inloc(&self) -> S1INLOC_R {
        S1INLOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn s0inloc(&mut self) -> S0INLOC_W {
        S0INLOC_W { w: self }
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn s1inloc(&mut self) -> S1INLOC_W {
        S1INLOC_W { w: self }
    }
}
