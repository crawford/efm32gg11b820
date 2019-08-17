#[doc = "Reader of register ROUTE"]
pub type R = crate::R<u32, super::ROUTE>;
#[doc = "Writer for register ROUTE"]
pub type W = crate::W<u32, super::ROUTE>;
#[doc = "Register ROUTE `reset()`'s with value 0"]
impl crate::ResetValue for super::ROUTE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXPEN`"]
pub type TXPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXPEN`"]
pub struct TXPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
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
    #[doc = "Location 7"]
    LOC7,
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
            RXLOC_A::LOC7 => 7,
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
            7 => Val(RXLOC_A::LOC7),
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
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == RXLOC_A::LOC7
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
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut W {
        self.variant(RXLOC_A::LOC7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 2)) | (((value as u32) & 0x3f) << 2);
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
    #[doc = "Location 7"]
    LOC7,
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
            TXLOC_A::LOC7 => 7,
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
            7 => Val(TXLOC_A::LOC7),
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
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == TXLOC_A::LOC7
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
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut W {
        self.variant(TXLOC_A::LOC7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TX Pin Enable"]
    #[inline(always)]
    pub fn txpen(&self) -> TXPEN_R {
        TXPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 2:7 - RX Pin Location"]
    #[inline(always)]
    pub fn rxloc(&self) -> RXLOC_R {
        RXLOC_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - TX Pin Location"]
    #[inline(always)]
    pub fn txloc(&self) -> TXLOC_R {
        TXLOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TX Pin Enable"]
    #[inline(always)]
    pub fn txpen(&mut self) -> TXPEN_W {
        TXPEN_W { w: self }
    }
    #[doc = "Bits 2:7 - RX Pin Location"]
    #[inline(always)]
    pub fn rxloc(&mut self) -> RXLOC_W {
        RXLOC_W { w: self }
    }
    #[doc = "Bits 8:13 - TX Pin Location"]
    #[inline(always)]
    pub fn txloc(&mut self) -> TXLOC_W {
        TXLOC_W { w: self }
    }
}
