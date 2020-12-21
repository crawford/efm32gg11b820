#[doc = "Reader of register CH3_CTRL"]
pub type R = crate::R<u32, super::CH3_CTRL>;
#[doc = "Writer for register CH3_CTRL"]
pub type W = crate::W<u32, super::CH3_CTRL>;
#[doc = "Register CH3_CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CH3_CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "DMA Structure Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STRUCTTYPE_A {
    #[doc = "0: DMA transfer structure type selected."]
    TRANSFER = 0,
    #[doc = "1: Synchronization structure type selected."]
    SYNCHRONIZE = 1,
    #[doc = "2: Write immediate value structure type selected."]
    WRITE = 2,
}
impl From<STRUCTTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: STRUCTTYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `STRUCTTYPE`"]
pub type STRUCTTYPE_R = crate::R<u8, STRUCTTYPE_A>;
impl STRUCTTYPE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, STRUCTTYPE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(STRUCTTYPE_A::TRANSFER),
            1 => Val(STRUCTTYPE_A::SYNCHRONIZE),
            2 => Val(STRUCTTYPE_A::WRITE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `TRANSFER`"]
    #[inline(always)]
    pub fn is_transfer(&self) -> bool {
        *self == STRUCTTYPE_A::TRANSFER
    }
    #[doc = "Checks if the value of the field is `SYNCHRONIZE`"]
    #[inline(always)]
    pub fn is_synchronize(&self) -> bool {
        *self == STRUCTTYPE_A::SYNCHRONIZE
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == STRUCTTYPE_A::WRITE
    }
}
#[doc = "Write proxy for field `STRUCTREQ`"]
pub struct STRUCTREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> STRUCTREQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `XFERCNT`"]
pub type XFERCNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `XFERCNT`"]
pub struct XFERCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> XFERCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 4)) | (((value as u32) & 0x07ff) << 4);
        self.w
    }
}
#[doc = "Reader of field `BYTESWAP`"]
pub type BYTESWAP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BYTESWAP`"]
pub struct BYTESWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTESWAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Block Transfer Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BLOCKSIZE_A {
    #[doc = "0: One unit transfer per arbitration"]
    UNIT1 = 0,
    #[doc = "1: Two unit transfers per arbitration"]
    UNIT2 = 1,
    #[doc = "2: Three unit transfers per arbitration"]
    UNIT3 = 2,
    #[doc = "3: Four unit transfers per arbitration"]
    UNIT4 = 3,
    #[doc = "4: Six unit transfers per arbitration"]
    UNIT6 = 4,
    #[doc = "5: Eight unit transfers per arbitration"]
    UNIT8 = 5,
    #[doc = "7: Sixteen unit transfers per arbitration"]
    UNIT16 = 7,
    #[doc = "9: 32 unit transfers per arbitration"]
    UNIT32 = 9,
    #[doc = "10: 64 unit transfers per arbitration"]
    UNIT64 = 10,
    #[doc = "11: 128 unit transfers per arbitration"]
    UNIT128 = 11,
    #[doc = "12: 256 unit transfers per arbitration"]
    UNIT256 = 12,
    #[doc = "13: 512 unit transfers per arbitration"]
    UNIT512 = 13,
    #[doc = "14: 1024 unit transfers per arbitration"]
    UNIT1024 = 14,
    #[doc = "15: Transfer all units as specified by the XFRCNT field"]
    ALL = 15,
}
impl From<BLOCKSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: BLOCKSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `BLOCKSIZE`"]
pub type BLOCKSIZE_R = crate::R<u8, BLOCKSIZE_A>;
impl BLOCKSIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, BLOCKSIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(BLOCKSIZE_A::UNIT1),
            1 => Val(BLOCKSIZE_A::UNIT2),
            2 => Val(BLOCKSIZE_A::UNIT3),
            3 => Val(BLOCKSIZE_A::UNIT4),
            4 => Val(BLOCKSIZE_A::UNIT6),
            5 => Val(BLOCKSIZE_A::UNIT8),
            7 => Val(BLOCKSIZE_A::UNIT16),
            9 => Val(BLOCKSIZE_A::UNIT32),
            10 => Val(BLOCKSIZE_A::UNIT64),
            11 => Val(BLOCKSIZE_A::UNIT128),
            12 => Val(BLOCKSIZE_A::UNIT256),
            13 => Val(BLOCKSIZE_A::UNIT512),
            14 => Val(BLOCKSIZE_A::UNIT1024),
            15 => Val(BLOCKSIZE_A::ALL),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `UNIT1`"]
    #[inline(always)]
    pub fn is_unit1(&self) -> bool {
        *self == BLOCKSIZE_A::UNIT1
    }
    #[doc = "Checks if the value of the field is `UNIT2`"]
    #[inline(always)]
    pub fn is_unit2(&self) -> bool {
        *self == BLOCKSIZE_A::UNIT2
    }
    #[doc = "Checks if the value of the field is `UNIT3`"]
    #[inline(always)]
    pub fn is_unit3(&self) -> bool {
        *self == BLOCKSIZE_A::UNIT3
    }
    #[doc = "Checks if the value of the field is `UNIT4`"]
    #[inline(always)]
    pub fn is_unit4(&self) -> bool {
        *self == BLOCKSIZE_A::UNIT4
    }
    #[doc = "Checks if the value of the field is `UNIT6`"]
    #[inline(always)]
    pub fn is_unit6(&self) -> bool {
        *self == BLOCKSIZE_A::UNIT6
    }
    #[doc = "Checks if the value of the field is `UNIT8`"]
    #[inline(always)]
    pub fn is_unit8(&self) -> bool {
        *self == BLOCKSIZE_A::UNIT8
    }
    #[doc = "Checks if the value of the field is `UNIT16`"]
    #[inline(always)]
    pub fn is_unit16(&self) -> bool {
        *self == BLOCKSIZE_A::UNIT16
    }
    #[doc = "Checks if the value of the field is `UNIT32`"]
    #[inline(always)]
    pub fn is_unit32(&self) -> bool {
        *self == BLOCKSIZE_A::UNIT32
    }
    #[doc = "Checks if the value of the field is `UNIT64`"]
    #[inline(always)]
    pub fn is_unit64(&self) -> bool {
        *self == BLOCKSIZE_A::UNIT64
    }
    #[doc = "Checks if the value of the field is `UNIT128`"]
    #[inline(always)]
    pub fn is_unit128(&self) -> bool {
        *self == BLOCKSIZE_A::UNIT128
    }
    #[doc = "Checks if the value of the field is `UNIT256`"]
    #[inline(always)]
    pub fn is_unit256(&self) -> bool {
        *self == BLOCKSIZE_A::UNIT256
    }
    #[doc = "Checks if the value of the field is `UNIT512`"]
    #[inline(always)]
    pub fn is_unit512(&self) -> bool {
        *self == BLOCKSIZE_A::UNIT512
    }
    #[doc = "Checks if the value of the field is `UNIT1024`"]
    #[inline(always)]
    pub fn is_unit1024(&self) -> bool {
        *self == BLOCKSIZE_A::UNIT1024
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == BLOCKSIZE_A::ALL
    }
}
#[doc = "Write proxy for field `BLOCKSIZE`"]
pub struct BLOCKSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> BLOCKSIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BLOCKSIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "One unit transfer per arbitration"]
    #[inline(always)]
    pub fn unit1(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::UNIT1)
    }
    #[doc = "Two unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit2(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::UNIT2)
    }
    #[doc = "Three unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit3(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::UNIT3)
    }
    #[doc = "Four unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit4(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::UNIT4)
    }
    #[doc = "Six unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit6(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::UNIT6)
    }
    #[doc = "Eight unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit8(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::UNIT8)
    }
    #[doc = "Sixteen unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit16(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::UNIT16)
    }
    #[doc = "32 unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit32(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::UNIT32)
    }
    #[doc = "64 unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit64(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::UNIT64)
    }
    #[doc = "128 unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit128(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::UNIT128)
    }
    #[doc = "256 unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit256(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::UNIT256)
    }
    #[doc = "512 unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit512(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::UNIT512)
    }
    #[doc = "1024 unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit1024(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::UNIT1024)
    }
    #[doc = "Transfer all units as specified by the XFRCNT field"]
    #[inline(always)]
    pub fn all(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::ALL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `DONEIFSEN`"]
pub type DONEIFSEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DONEIFSEN`"]
pub struct DONEIFSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DONEIFSEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `REQMODE`"]
pub type REQMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REQMODE`"]
pub struct REQMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> REQMODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `DECLOOPCNT`"]
pub type DECLOOPCNT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DECLOOPCNT`"]
pub struct DECLOOPCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DECLOOPCNT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `IGNORESREQ`"]
pub type IGNORESREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IGNORESREQ`"]
pub struct IGNORESREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> IGNORESREQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Source Address Increment Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRCINC_A {
    #[doc = "0: Increment source address by one unit data size after each read"]
    ONE = 0,
    #[doc = "1: Increment source address by two unit data sizes after each read"]
    TWO = 1,
    #[doc = "2: Increment source address by four unit data sizes after each read"]
    FOUR = 2,
    #[doc = "3: Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    NONE = 3,
}
impl From<SRCINC_A> for u8 {
    #[inline(always)]
    fn from(variant: SRCINC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SRCINC`"]
pub type SRCINC_R = crate::R<u8, SRCINC_A>;
impl SRCINC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRCINC_A {
        match self.bits {
            0 => SRCINC_A::ONE,
            1 => SRCINC_A::TWO,
            2 => SRCINC_A::FOUR,
            3 => SRCINC_A::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == SRCINC_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == SRCINC_A::TWO
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == SRCINC_A::FOUR
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SRCINC_A::NONE
    }
}
#[doc = "Write proxy for field `SRCINC`"]
pub struct SRCINC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRCINC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRCINC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Increment source address by one unit data size after each read"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(SRCINC_A::ONE)
    }
    #[doc = "Increment source address by two unit data sizes after each read"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(SRCINC_A::TWO)
    }
    #[doc = "Increment source address by four unit data sizes after each read"]
    #[inline(always)]
    pub fn four(self) -> &'a mut W {
        self.variant(SRCINC_A::FOUR)
    }
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SRCINC_A::NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "Unit Data Transfer Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SIZE_A {
    #[doc = "0: Each unit transfer is a byte"]
    BYTE = 0,
    #[doc = "1: Each unit transfer is a half-word"]
    HALFWORD = 1,
    #[doc = "2: Each unit transfer is a word"]
    WORD = 2,
}
impl From<SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SIZE`"]
pub type SIZE_R = crate::R<u8, SIZE_A>;
impl SIZE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SIZE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SIZE_A::BYTE),
            1 => Val(SIZE_A::HALFWORD),
            2 => Val(SIZE_A::WORD),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == SIZE_A::BYTE
    }
    #[doc = "Checks if the value of the field is `HALFWORD`"]
    #[inline(always)]
    pub fn is_halfword(&self) -> bool {
        *self == SIZE_A::HALFWORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == SIZE_A::WORD
    }
}
#[doc = "Write proxy for field `SIZE`"]
pub struct SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> SIZE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIZE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Each unit transfer is a byte"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(SIZE_A::BYTE)
    }
    #[doc = "Each unit transfer is a half-word"]
    #[inline(always)]
    pub fn halfword(self) -> &'a mut W {
        self.variant(SIZE_A::HALFWORD)
    }
    #[doc = "Each unit transfer is a word"]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(SIZE_A::WORD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Destination Address Increment Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DSTINC_A {
    #[doc = "0: Increment destination address by one unit data size after each write"]
    ONE = 0,
    #[doc = "1: Increment destination address by two unit data sizes after each write"]
    TWO = 1,
    #[doc = "2: Increment destination address by four unit data sizes after each write"]
    FOUR = 2,
    #[doc = "3: Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    NONE = 3,
}
impl From<DSTINC_A> for u8 {
    #[inline(always)]
    fn from(variant: DSTINC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DSTINC`"]
pub type DSTINC_R = crate::R<u8, DSTINC_A>;
impl DSTINC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSTINC_A {
        match self.bits {
            0 => DSTINC_A::ONE,
            1 => DSTINC_A::TWO,
            2 => DSTINC_A::FOUR,
            3 => DSTINC_A::NONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == DSTINC_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == DSTINC_A::TWO
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == DSTINC_A::FOUR
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == DSTINC_A::NONE
    }
}
#[doc = "Write proxy for field `DSTINC`"]
pub struct DSTINC_W<'a> {
    w: &'a mut W,
}
impl<'a> DSTINC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSTINC_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Increment destination address by one unit data size after each write"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(DSTINC_A::ONE)
    }
    #[doc = "Increment destination address by two unit data sizes after each write"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(DSTINC_A::TWO)
    }
    #[doc = "Increment destination address by four unit data sizes after each write"]
    #[inline(always)]
    pub fn four(self) -> &'a mut W {
        self.variant(DSTINC_A::FOUR)
    }
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(DSTINC_A::NONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `SRCMODE`"]
pub type SRCMODE_R = crate::R<bool, bool>;
#[doc = "Reader of field `DSTMODE`"]
pub type DSTMODE_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:1 - DMA Structure Type"]
    #[inline(always)]
    pub fn structtype(&self) -> STRUCTTYPE_R {
        STRUCTTYPE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:14 - DMA Unit Data Transfer Count"]
    #[inline(always)]
    pub fn xfercnt(&self) -> XFERCNT_R {
        XFERCNT_R::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - Endian Byte Swap"]
    #[inline(always)]
    pub fn byteswap(&self) -> BYTESWAP_R {
        BYTESWAP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - Block Transfer Size"]
    #[inline(always)]
    pub fn blocksize(&self) -> BLOCKSIZE_R {
        BLOCKSIZE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - DMA Operation Done Interrupt Flag Set Enable"]
    #[inline(always)]
    pub fn doneifsen(&self) -> DONEIFSEN_R {
        DONEIFSEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - DMA Request Transfer Mode Select"]
    #[inline(always)]
    pub fn reqmode(&self) -> REQMODE_R {
        REQMODE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Decrement Loop Count"]
    #[inline(always)]
    pub fn decloopcnt(&self) -> DECLOOPCNT_R {
        DECLOOPCNT_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Ignore Sreq"]
    #[inline(always)]
    pub fn ignoresreq(&self) -> IGNORESREQ_R {
        IGNORESREQ_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - Source Address Increment Size"]
    #[inline(always)]
    pub fn srcinc(&self) -> SRCINC_R {
        SRCINC_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Unit Data Transfer Size"]
    #[inline(always)]
    pub fn size(&self) -> SIZE_R {
        SIZE_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - Destination Address Increment Size"]
    #[inline(always)]
    pub fn dstinc(&self) -> DSTINC_R {
        DSTINC_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 30 - Source Addressing Mode"]
    #[inline(always)]
    pub fn srcmode(&self) -> SRCMODE_R {
        SRCMODE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Destination Addressing Mode"]
    #[inline(always)]
    pub fn dstmode(&self) -> DSTMODE_R {
        DSTMODE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Structure DMA Transfer Request"]
    #[inline(always)]
    pub fn structreq(&mut self) -> STRUCTREQ_W {
        STRUCTREQ_W { w: self }
    }
    #[doc = "Bits 4:14 - DMA Unit Data Transfer Count"]
    #[inline(always)]
    pub fn xfercnt(&mut self) -> XFERCNT_W {
        XFERCNT_W { w: self }
    }
    #[doc = "Bit 15 - Endian Byte Swap"]
    #[inline(always)]
    pub fn byteswap(&mut self) -> BYTESWAP_W {
        BYTESWAP_W { w: self }
    }
    #[doc = "Bits 16:19 - Block Transfer Size"]
    #[inline(always)]
    pub fn blocksize(&mut self) -> BLOCKSIZE_W {
        BLOCKSIZE_W { w: self }
    }
    #[doc = "Bit 20 - DMA Operation Done Interrupt Flag Set Enable"]
    #[inline(always)]
    pub fn doneifsen(&mut self) -> DONEIFSEN_W {
        DONEIFSEN_W { w: self }
    }
    #[doc = "Bit 21 - DMA Request Transfer Mode Select"]
    #[inline(always)]
    pub fn reqmode(&mut self) -> REQMODE_W {
        REQMODE_W { w: self }
    }
    #[doc = "Bit 22 - Decrement Loop Count"]
    #[inline(always)]
    pub fn decloopcnt(&mut self) -> DECLOOPCNT_W {
        DECLOOPCNT_W { w: self }
    }
    #[doc = "Bit 23 - Ignore Sreq"]
    #[inline(always)]
    pub fn ignoresreq(&mut self) -> IGNORESREQ_W {
        IGNORESREQ_W { w: self }
    }
    #[doc = "Bits 24:25 - Source Address Increment Size"]
    #[inline(always)]
    pub fn srcinc(&mut self) -> SRCINC_W {
        SRCINC_W { w: self }
    }
    #[doc = "Bits 26:27 - Unit Data Transfer Size"]
    #[inline(always)]
    pub fn size(&mut self) -> SIZE_W {
        SIZE_W { w: self }
    }
    #[doc = "Bits 28:29 - Destination Address Increment Size"]
    #[inline(always)]
    pub fn dstinc(&mut self) -> DSTINC_W {
        DSTINC_W { w: self }
    }
}
