#[doc = "Reader of register ROUTELOC4"]
pub type R = crate::R<u32, super::ROUTELOC4>;
#[doc = "Writer for register ROUTELOC4"]
pub type W = crate::W<u32, super::ROUTELOC4>;
#[doc = "Register ROUTELOC4 `reset()`'s with value 0"]
impl crate::ResetValue for super::ROUTELOC4 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Possible values of the field `CH16LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH16LOC_A {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
}
impl crate::ToBits<u8> for CH16LOC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CH16LOC_A::LOC0 => 0,
            CH16LOC_A::LOC1 => 1,
            CH16LOC_A::LOC2 => 2,
        }
    }
}
#[doc = "Reader of field `CH16LOC`"]
pub type CH16LOC_R = crate::R<u8, CH16LOC_A>;
impl CH16LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CH16LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CH16LOC_A::LOC0),
            1 => Val(CH16LOC_A::LOC1),
            2 => Val(CH16LOC_A::LOC2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH16LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH16LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH16LOC_A::LOC2
    }
}
#[doc = "Write proxy for field `CH16LOC`"]
pub struct CH16LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH16LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH16LOC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH16LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH16LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH16LOC_A::LOC2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Possible values of the field `CH17LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH17LOC_A {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
}
impl crate::ToBits<u8> for CH17LOC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CH17LOC_A::LOC0 => 0,
            CH17LOC_A::LOC1 => 1,
            CH17LOC_A::LOC2 => 2,
        }
    }
}
#[doc = "Reader of field `CH17LOC`"]
pub type CH17LOC_R = crate::R<u8, CH17LOC_A>;
impl CH17LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CH17LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CH17LOC_A::LOC0),
            1 => Val(CH17LOC_A::LOC1),
            2 => Val(CH17LOC_A::LOC2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH17LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH17LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH17LOC_A::LOC2
    }
}
#[doc = "Write proxy for field `CH17LOC`"]
pub struct CH17LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH17LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH17LOC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH17LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH17LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH17LOC_A::LOC2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `CH18LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH18LOC_A {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
}
impl crate::ToBits<u8> for CH18LOC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CH18LOC_A::LOC0 => 0,
            CH18LOC_A::LOC1 => 1,
            CH18LOC_A::LOC2 => 2,
        }
    }
}
#[doc = "Reader of field `CH18LOC`"]
pub type CH18LOC_R = crate::R<u8, CH18LOC_A>;
impl CH18LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CH18LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CH18LOC_A::LOC0),
            1 => Val(CH18LOC_A::LOC1),
            2 => Val(CH18LOC_A::LOC2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH18LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH18LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH18LOC_A::LOC2
    }
}
#[doc = "Write proxy for field `CH18LOC`"]
pub struct CH18LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH18LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH18LOC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH18LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH18LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH18LOC_A::LOC2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `CH19LOC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CH19LOC_A {
    #[doc = "Location 0"]
    LOC0,
    #[doc = "Location 1"]
    LOC1,
    #[doc = "Location 2"]
    LOC2,
}
impl crate::ToBits<u8> for CH19LOC_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CH19LOC_A::LOC0 => 0,
            CH19LOC_A::LOC1 => 1,
            CH19LOC_A::LOC2 => 2,
        }
    }
}
#[doc = "Reader of field `CH19LOC`"]
pub type CH19LOC_R = crate::R<u8, CH19LOC_A>;
impl CH19LOC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CH19LOC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CH19LOC_A::LOC0),
            1 => Val(CH19LOC_A::LOC1),
            2 => Val(CH19LOC_A::LOC2),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH19LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH19LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH19LOC_A::LOC2
    }
}
#[doc = "Write proxy for field `CH19LOC`"]
pub struct CH19LOC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH19LOC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CH19LOC_A) -> &'a mut W {
        use crate::ToBits;
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH19LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH19LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH19LOC_A::LOC2)
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
    pub fn ch16loc(&self) -> CH16LOC_R {
        CH16LOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch17loc(&self) -> CH17LOC_R {
        CH17LOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch18loc(&self) -> CH18LOC_R {
        CH18LOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch19loc(&self) -> CH19LOC_R {
        CH19LOC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch16loc(&mut self) -> CH16LOC_W {
        CH16LOC_W { w: self }
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch17loc(&mut self) -> CH17LOC_W {
        CH17LOC_W { w: self }
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch18loc(&mut self) -> CH18LOC_W {
        CH18LOC_W { w: self }
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch19loc(&mut self) -> CH19LOC_W {
        CH19LOC_W { w: self }
    }
}
