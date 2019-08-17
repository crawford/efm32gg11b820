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
#[doc = "Possible values of the field `CC0LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC0LOC_A {
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
impl crate::ToBits<u8> for CC0LOC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CC0LOC_A::LOC0 => 0,
            CC0LOC_A::LOC1 => 1,
            CC0LOC_A::LOC2 => 2,
            CC0LOC_A::LOC3 => 3,
            CC0LOC_A::LOC4 => 4,
            CC0LOC_A::LOC5 => 5,
            CC0LOC_A::LOC6 => 6,
            CC0LOC_A::LOC7 => 7,
        }
    }
}
#[doc = "Reader of field `CC0LOC`"]
pub type CC0LOC_R = crate::R<u8, CC0LOC_A>;
impl CC0LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CC0LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CC0LOC_A::LOC0),
            1 => Val(CC0LOC_A::LOC1),
            2 => Val(CC0LOC_A::LOC2),
            3 => Val(CC0LOC_A::LOC3),
            4 => Val(CC0LOC_A::LOC4),
            5 => Val(CC0LOC_A::LOC5),
            6 => Val(CC0LOC_A::LOC6),
            7 => Val(CC0LOC_A::LOC7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CC0LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CC0LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CC0LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CC0LOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CC0LOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CC0LOC_A::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CC0LOC_A::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == CC0LOC_A::LOC7
    }
}
#[doc = "Write proxy for field `CC0LOC`"]
pub struct CC0LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> CC0LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC0LOC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CC0LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CC0LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CC0LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CC0LOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CC0LOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CC0LOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut W {
        self.variant(CC0LOC_A::LOC6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut W {
        self.variant(CC0LOC_A::LOC7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Possible values of the field `CC1LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC1LOC_A {
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
impl crate::ToBits<u8> for CC1LOC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CC1LOC_A::LOC0 => 0,
            CC1LOC_A::LOC1 => 1,
            CC1LOC_A::LOC2 => 2,
            CC1LOC_A::LOC3 => 3,
            CC1LOC_A::LOC4 => 4,
            CC1LOC_A::LOC5 => 5,
            CC1LOC_A::LOC6 => 6,
            CC1LOC_A::LOC7 => 7,
        }
    }
}
#[doc = "Reader of field `CC1LOC`"]
pub type CC1LOC_R = crate::R<u8, CC1LOC_A>;
impl CC1LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CC1LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CC1LOC_A::LOC0),
            1 => Val(CC1LOC_A::LOC1),
            2 => Val(CC1LOC_A::LOC2),
            3 => Val(CC1LOC_A::LOC3),
            4 => Val(CC1LOC_A::LOC4),
            5 => Val(CC1LOC_A::LOC5),
            6 => Val(CC1LOC_A::LOC6),
            7 => Val(CC1LOC_A::LOC7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CC1LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CC1LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CC1LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CC1LOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CC1LOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CC1LOC_A::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CC1LOC_A::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == CC1LOC_A::LOC7
    }
}
#[doc = "Write proxy for field `CC1LOC`"]
pub struct CC1LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> CC1LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC1LOC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CC1LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CC1LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CC1LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CC1LOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CC1LOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CC1LOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut W {
        self.variant(CC1LOC_A::LOC6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut W {
        self.variant(CC1LOC_A::LOC7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `CC2LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC2LOC_A {
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
impl crate::ToBits<u8> for CC2LOC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CC2LOC_A::LOC0 => 0,
            CC2LOC_A::LOC1 => 1,
            CC2LOC_A::LOC2 => 2,
            CC2LOC_A::LOC3 => 3,
            CC2LOC_A::LOC4 => 4,
            CC2LOC_A::LOC5 => 5,
            CC2LOC_A::LOC6 => 6,
            CC2LOC_A::LOC7 => 7,
        }
    }
}
#[doc = "Reader of field `CC2LOC`"]
pub type CC2LOC_R = crate::R<u8, CC2LOC_A>;
impl CC2LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CC2LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CC2LOC_A::LOC0),
            1 => Val(CC2LOC_A::LOC1),
            2 => Val(CC2LOC_A::LOC2),
            3 => Val(CC2LOC_A::LOC3),
            4 => Val(CC2LOC_A::LOC4),
            5 => Val(CC2LOC_A::LOC5),
            6 => Val(CC2LOC_A::LOC6),
            7 => Val(CC2LOC_A::LOC7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CC2LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CC2LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CC2LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CC2LOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CC2LOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CC2LOC_A::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CC2LOC_A::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == CC2LOC_A::LOC7
    }
}
#[doc = "Write proxy for field `CC2LOC`"]
pub struct CC2LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> CC2LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC2LOC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CC2LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CC2LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CC2LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CC2LOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CC2LOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CC2LOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut W {
        self.variant(CC2LOC_A::LOC6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut W {
        self.variant(CC2LOC_A::LOC7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `CC3LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CC3LOC_A {
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
impl crate::ToBits<u8> for CC3LOC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CC3LOC_A::LOC0 => 0,
            CC3LOC_A::LOC1 => 1,
            CC3LOC_A::LOC2 => 2,
            CC3LOC_A::LOC3 => 3,
            CC3LOC_A::LOC4 => 4,
            CC3LOC_A::LOC5 => 5,
            CC3LOC_A::LOC6 => 6,
            CC3LOC_A::LOC7 => 7,
        }
    }
}
#[doc = "Reader of field `CC3LOC`"]
pub type CC3LOC_R = crate::R<u8, CC3LOC_A>;
impl CC3LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CC3LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CC3LOC_A::LOC0),
            1 => Val(CC3LOC_A::LOC1),
            2 => Val(CC3LOC_A::LOC2),
            3 => Val(CC3LOC_A::LOC3),
            4 => Val(CC3LOC_A::LOC4),
            5 => Val(CC3LOC_A::LOC5),
            6 => Val(CC3LOC_A::LOC6),
            7 => Val(CC3LOC_A::LOC7),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CC3LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CC3LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CC3LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CC3LOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CC3LOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CC3LOC_A::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == CC3LOC_A::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == CC3LOC_A::LOC7
    }
}
#[doc = "Write proxy for field `CC3LOC`"]
pub struct CC3LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> CC3LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CC3LOC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CC3LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CC3LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CC3LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CC3LOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CC3LOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CC3LOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut W {
        self.variant(CC3LOC_A::LOC6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut W {
        self.variant(CC3LOC_A::LOC7)
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
    pub fn cc0loc(&self) -> CC0LOC_R {
        CC0LOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn cc1loc(&self) -> CC1LOC_R {
        CC1LOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn cc2loc(&self) -> CC2LOC_R {
        CC2LOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn cc3loc(&self) -> CC3LOC_R {
        CC3LOC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn cc0loc(&mut self) -> CC0LOC_W {
        CC0LOC_W { w: self }
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn cc1loc(&mut self) -> CC1LOC_W {
        CC1LOC_W { w: self }
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn cc2loc(&mut self) -> CC2LOC_W {
        CC2LOC_W { w: self }
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn cc3loc(&mut self) -> CC3LOC_W {
        CC3LOC_W { w: self }
    }
}
