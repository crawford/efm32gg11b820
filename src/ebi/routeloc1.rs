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
#[doc = "Possible values of the field `ADLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADLOC_A {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
}
impl crate::ToBits<u8> for ADLOC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            ADLOC_A::LOC0 => 0,
            ADLOC_A::LOC1 => 1,
            ADLOC_A::LOC2 => 2,
        }
    }
}
#[doc = "Reader of field `ADLOC`"]
pub type ADLOC_R = crate::R<u8, ADLOC_A>;
impl ADLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ADLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ADLOC_A::LOC0),
            1 => Val(ADLOC_A::LOC1),
            2 => Val(ADLOC_A::LOC2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == ADLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == ADLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == ADLOC_A::LOC2
    }
}
#[doc = "Write proxy for field `ADLOC`"]
pub struct ADLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADLOC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(ADLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(ADLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(ADLOC_A::LOC2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Possible values of the field `ALOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALOC_A {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
    #[doc = "Location 3"]
    LOC3,
}
impl crate::ToBits<u8> for ALOC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            ALOC_A::LOC0 => 0,
            ALOC_A::LOC1 => 1,
            ALOC_A::LOC2 => 2,
            ALOC_A::LOC3 => 3,
        }
    }
}
#[doc = "Reader of field `ALOC`"]
pub type ALOC_R = crate::R<u8, ALOC_A>;
impl ALOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, ALOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(ALOC_A::LOC0),
            1 => Val(ALOC_A::LOC1),
            2 => Val(ALOC_A::LOC2),
            3 => Val(ALOC_A::LOC3),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == ALOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == ALOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == ALOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == ALOC_A::LOC3
    }
}
#[doc = "Write proxy for field `ALOC`"]
pub struct ALOC_W<'a> {
    w: &'a mut W,
}
impl<'a> ALOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALOC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(ALOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(ALOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(ALOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(ALOC_A::LOC3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `RDYLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDYLOC_A {
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
impl crate::ToBits<u8> for RDYLOC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            RDYLOC_A::LOC0 => 0,
            RDYLOC_A::LOC1 => 1,
            RDYLOC_A::LOC2 => 2,
            RDYLOC_A::LOC3 => 3,
            RDYLOC_A::LOC4 => 4,
            RDYLOC_A::LOC5 => 5,
        }
    }
}
#[doc = "Reader of field `RDYLOC`"]
pub type RDYLOC_R = crate::R<u8, RDYLOC_A>;
impl RDYLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RDYLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RDYLOC_A::LOC0),
            1 => Val(RDYLOC_A::LOC1),
            2 => Val(RDYLOC_A::LOC2),
            3 => Val(RDYLOC_A::LOC3),
            4 => Val(RDYLOC_A::LOC4),
            5 => Val(RDYLOC_A::LOC5),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == RDYLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == RDYLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == RDYLOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == RDYLOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == RDYLOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == RDYLOC_A::LOC5
    }
}
#[doc = "Write proxy for field `RDYLOC`"]
pub struct RDYLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> RDYLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDYLOC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(RDYLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(RDYLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(RDYLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(RDYLOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(RDYLOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(RDYLOC_A::LOC5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn adloc(&self) -> ADLOC_R {
        ADLOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn aloc(&self) -> ALOC_R {
        ALOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn rdyloc(&self) -> RDYLOC_R {
        RDYLOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn adloc(&mut self) -> ADLOC_W {
        ADLOC_W { w: self }
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn aloc(&mut self) -> ALOC_W {
        ALOC_W { w: self }
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn rdyloc(&mut self) -> RDYLOC_W {
        RDYLOC_W { w: self }
    }
}
