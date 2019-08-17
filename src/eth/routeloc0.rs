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
#[doc = "Possible values of the field `MIITXLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIITXLOC_A {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
}
impl crate::ToBits<u8> for MIITXLOC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            MIITXLOC_A::LOC0 => 0,
            MIITXLOC_A::LOC1 => 1,
        }
    }
}
#[doc = "Reader of field `MIITXLOC`"]
pub type MIITXLOC_R = crate::R<u8, MIITXLOC_A>;
impl MIITXLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MIITXLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MIITXLOC_A::LOC0),
            1 => Val(MIITXLOC_A::LOC1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == MIITXLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == MIITXLOC_A::LOC1
    }
}
#[doc = "Write proxy for field `MIITXLOC`"]
pub struct MIITXLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> MIITXLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MIITXLOC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(MIITXLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(MIITXLOC_A::LOC1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Possible values of the field `MIIRXLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIIRXLOC_A {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
}
impl crate::ToBits<u8> for MIIRXLOC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            MIIRXLOC_A::LOC0 => 0,
            MIIRXLOC_A::LOC1 => 1,
            MIIRXLOC_A::LOC2 => 2,
        }
    }
}
#[doc = "Reader of field `MIIRXLOC`"]
pub type MIIRXLOC_R = crate::R<u8, MIIRXLOC_A>;
impl MIIRXLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MIIRXLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MIIRXLOC_A::LOC0),
            1 => Val(MIIRXLOC_A::LOC1),
            2 => Val(MIIRXLOC_A::LOC2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == MIIRXLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == MIIRXLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == MIIRXLOC_A::LOC2
    }
}
#[doc = "Write proxy for field `MIIRXLOC`"]
pub struct MIIRXLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> MIIRXLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MIIRXLOC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(MIIRXLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(MIIRXLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(MIIRXLOC_A::LOC2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `MIICRSLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIICRSLOC_A {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
}
impl crate::ToBits<u8> for MIICRSLOC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            MIICRSLOC_A::LOC0 => 0,
            MIICRSLOC_A::LOC1 => 1,
            MIICRSLOC_A::LOC2 => 2,
        }
    }
}
#[doc = "Reader of field `MIICRSLOC`"]
pub type MIICRSLOC_R = crate::R<u8, MIICRSLOC_A>;
impl MIICRSLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MIICRSLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MIICRSLOC_A::LOC0),
            1 => Val(MIICRSLOC_A::LOC1),
            2 => Val(MIICRSLOC_A::LOC2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == MIICRSLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == MIICRSLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == MIICRSLOC_A::LOC2
    }
}
#[doc = "Write proxy for field `MIICRSLOC`"]
pub struct MIICRSLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> MIICRSLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MIICRSLOC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(MIICRSLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(MIICRSLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(MIICRSLOC_A::LOC2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `MIICOLLOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MIICOLLOC_A {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
}
impl crate::ToBits<u8> for MIICOLLOC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            MIICOLLOC_A::LOC0 => 0,
            MIICOLLOC_A::LOC1 => 1,
            MIICOLLOC_A::LOC2 => 2,
        }
    }
}
#[doc = "Reader of field `MIICOLLOC`"]
pub type MIICOLLOC_R = crate::R<u8, MIICOLLOC_A>;
impl MIICOLLOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MIICOLLOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MIICOLLOC_A::LOC0),
            1 => Val(MIICOLLOC_A::LOC1),
            2 => Val(MIICOLLOC_A::LOC2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == MIICOLLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == MIICOLLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == MIICOLLOC_A::LOC2
    }
}
#[doc = "Write proxy for field `MIICOLLOC`"]
pub struct MIICOLLOC_W<'a> {
    w: &'a mut W,
}
impl<'a> MIICOLLOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MIICOLLOC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(MIICOLLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(MIICOLLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(MIICOLLOC_A::LOC2)
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
    pub fn miitxloc(&self) -> MIITXLOC_R {
        MIITXLOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn miirxloc(&self) -> MIIRXLOC_R {
        MIIRXLOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn miicrsloc(&self) -> MIICRSLOC_R {
        MIICRSLOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn miicolloc(&self) -> MIICOLLOC_R {
        MIICOLLOC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn miitxloc(&mut self) -> MIITXLOC_W {
        MIITXLOC_W { w: self }
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn miirxloc(&mut self) -> MIIRXLOC_W {
        MIIRXLOC_W { w: self }
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn miicrsloc(&mut self) -> MIICRSLOC_W {
        MIICRSLOC_W { w: self }
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn miicolloc(&mut self) -> MIICOLLOC_W {
        MIICOLLOC_W { w: self }
    }
}
