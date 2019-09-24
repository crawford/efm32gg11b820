#[doc = "Reader of register DSTATUS"]
pub type R = crate::R<u32, super::DSTATUS>;
#[doc = "Data 0 Zero\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DATA0ZERO_A {
    #[doc = "1: In DATA0 bits 0 to 31 are all zero."]
    ZERO0TO31,
    #[doc = "2: In DATA0 bits 32 to 63 are all zero."]
    ZERO32TO63,
    #[doc = "4: In DATA0 bits 64 to 95 are all zero."]
    ZERO64TO95,
    #[doc = "8: In DATA0 bits 96 to 127 are all zero."]
    ZERO96TO127,
}
impl From<DATA0ZERO_A> for u8 {
    #[inline(always)]
    fn from(variant: DATA0ZERO_A) -> Self {
        match variant {
            DATA0ZERO_A::ZERO0TO31 => 1,
            DATA0ZERO_A::ZERO32TO63 => 2,
            DATA0ZERO_A::ZERO64TO95 => 4,
            DATA0ZERO_A::ZERO96TO127 => 8,
        }
    }
}
#[doc = "Reader of field `DATA0ZERO`"]
pub type DATA0ZERO_R = crate::R<u8, DATA0ZERO_A>;
impl DATA0ZERO_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DATA0ZERO_A> {
        use crate::Variant::*;
        match self.bits {
            1 => Val(DATA0ZERO_A::ZERO0TO31),
            2 => Val(DATA0ZERO_A::ZERO32TO63),
            4 => Val(DATA0ZERO_A::ZERO64TO95),
            8 => Val(DATA0ZERO_A::ZERO96TO127),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `ZERO0TO31`"]
    #[inline(always)]
    pub fn is_zero0to31(&self) -> bool {
        *self == DATA0ZERO_A::ZERO0TO31
    }
    #[doc = "Checks if the value of the field is `ZERO32TO63`"]
    #[inline(always)]
    pub fn is_zero32to63(&self) -> bool {
        *self == DATA0ZERO_A::ZERO32TO63
    }
    #[doc = "Checks if the value of the field is `ZERO64TO95`"]
    #[inline(always)]
    pub fn is_zero64to95(&self) -> bool {
        *self == DATA0ZERO_A::ZERO64TO95
    }
    #[doc = "Checks if the value of the field is `ZERO96TO127`"]
    #[inline(always)]
    pub fn is_zero96to127(&self) -> bool {
        *self == DATA0ZERO_A::ZERO96TO127
    }
}
#[doc = "Reader of field `DDATA0LSBS`"]
pub type DDATA0LSBS_R = crate::R<u8, u8>;
#[doc = "Reader of field `DDATA0MSBS`"]
pub type DDATA0MSBS_R = crate::R<u8, u8>;
#[doc = "Reader of field `DDATA1MSB`"]
pub type DDATA1MSB_R = crate::R<bool, bool>;
#[doc = "Reader of field `CARRY`"]
pub type CARRY_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:3 - Data 0 Zero"]
    #[inline(always)]
    pub fn data0zero(&self) -> DATA0ZERO_R {
        DATA0ZERO_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - LSBs in DDATA0"]
    #[inline(always)]
    pub fn ddata0lsbs(&self) -> DDATA0LSBS_R {
        DDATA0LSBS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - MSB in DDATA0"]
    #[inline(always)]
    pub fn ddata0msbs(&self) -> DDATA0MSBS_R {
        DDATA0MSBS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - MSB in DDATA1"]
    #[inline(always)]
    pub fn ddata1msb(&self) -> DDATA1MSB_R {
        DDATA1MSB_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Carry From Arithmetic Operation"]
    #[inline(always)]
    pub fn carry(&self) -> CARRY_R {
        CARRY_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
