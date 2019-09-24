#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `AES`"]
pub type AES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AES`"]
pub struct AES_W<'a> {
    w: &'a mut W,
}
impl<'a> AES_W<'a> {
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
#[doc = "Reader of field `KEYBUFDIS`"]
pub type KEYBUFDIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `KEYBUFDIS`"]
pub struct KEYBUFDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> KEYBUFDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `SHA`"]
pub type SHA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SHA`"]
pub struct SHA_W<'a> {
    w: &'a mut W,
}
impl<'a> SHA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `NOBUSYSTALL`"]
pub type NOBUSYSTALL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOBUSYSTALL`"]
pub struct NOBUSYSTALL_W<'a> {
    w: &'a mut W,
}
impl<'a> NOBUSYSTALL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Increment Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INCWIDTH_A {
    #[doc = "0: Byte 15 in DATA1 is used for the increment function."]
    INCWIDTH1,
    #[doc = "1: Bytes 14 and 15 in DATA1 are used for the increment function."]
    INCWIDTH2,
    #[doc = "2: Bytes 13 to 15 in DATA1 are used for the increment function."]
    INCWIDTH3,
    #[doc = "3: Bytes 12 to 15 in DATA1 are used for the increment function."]
    INCWIDTH4,
}
impl From<INCWIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: INCWIDTH_A) -> Self {
        match variant {
            INCWIDTH_A::INCWIDTH1 => 0,
            INCWIDTH_A::INCWIDTH2 => 1,
            INCWIDTH_A::INCWIDTH3 => 2,
            INCWIDTH_A::INCWIDTH4 => 3,
        }
    }
}
#[doc = "Reader of field `INCWIDTH`"]
pub type INCWIDTH_R = crate::R<u8, INCWIDTH_A>;
impl INCWIDTH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INCWIDTH_A {
        match self.bits {
            0 => INCWIDTH_A::INCWIDTH1,
            1 => INCWIDTH_A::INCWIDTH2,
            2 => INCWIDTH_A::INCWIDTH3,
            3 => INCWIDTH_A::INCWIDTH4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INCWIDTH1`"]
    #[inline(always)]
    pub fn is_incwidth1(&self) -> bool {
        *self == INCWIDTH_A::INCWIDTH1
    }
    #[doc = "Checks if the value of the field is `INCWIDTH2`"]
    #[inline(always)]
    pub fn is_incwidth2(&self) -> bool {
        *self == INCWIDTH_A::INCWIDTH2
    }
    #[doc = "Checks if the value of the field is `INCWIDTH3`"]
    #[inline(always)]
    pub fn is_incwidth3(&self) -> bool {
        *self == INCWIDTH_A::INCWIDTH3
    }
    #[doc = "Checks if the value of the field is `INCWIDTH4`"]
    #[inline(always)]
    pub fn is_incwidth4(&self) -> bool {
        *self == INCWIDTH_A::INCWIDTH4
    }
}
#[doc = "Write proxy for field `INCWIDTH`"]
pub struct INCWIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> INCWIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INCWIDTH_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Byte 15 in DATA1 is used for the increment function."]
    #[inline(always)]
    pub fn incwidth1(self) -> &'a mut W {
        self.variant(INCWIDTH_A::INCWIDTH1)
    }
    #[doc = "Bytes 14 and 15 in DATA1 are used for the increment function."]
    #[inline(always)]
    pub fn incwidth2(self) -> &'a mut W {
        self.variant(INCWIDTH_A::INCWIDTH2)
    }
    #[doc = "Bytes 13 to 15 in DATA1 are used for the increment function."]
    #[inline(always)]
    pub fn incwidth3(self) -> &'a mut W {
        self.variant(INCWIDTH_A::INCWIDTH3)
    }
    #[doc = "Bytes 12 to 15 in DATA1 are used for the increment function."]
    #[inline(always)]
    pub fn incwidth4(self) -> &'a mut W {
        self.variant(INCWIDTH_A::INCWIDTH4)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "DMA0 Read Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA0MODE_A {
    #[doc = "0: Target register is fully read/written during every DMA transaction"]
    FULL,
    #[doc = "1: Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Zero padding is automatically added when writing."]
    LENLIMIT,
    #[doc = "2: Target register is fully read/written during every DMA transaction. Bytewise DMA."]
    FULLBYTE,
    #[doc = "3: Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing."]
    LENLIMITBYTE,
}
impl From<DMA0MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA0MODE_A) -> Self {
        match variant {
            DMA0MODE_A::FULL => 0,
            DMA0MODE_A::LENLIMIT => 1,
            DMA0MODE_A::FULLBYTE => 2,
            DMA0MODE_A::LENLIMITBYTE => 3,
        }
    }
}
#[doc = "Reader of field `DMA0MODE`"]
pub type DMA0MODE_R = crate::R<u8, DMA0MODE_A>;
impl DMA0MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA0MODE_A {
        match self.bits {
            0 => DMA0MODE_A::FULL,
            1 => DMA0MODE_A::LENLIMIT,
            2 => DMA0MODE_A::FULLBYTE,
            3 => DMA0MODE_A::LENLIMITBYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == DMA0MODE_A::FULL
    }
    #[doc = "Checks if the value of the field is `LENLIMIT`"]
    #[inline(always)]
    pub fn is_lenlimit(&self) -> bool {
        *self == DMA0MODE_A::LENLIMIT
    }
    #[doc = "Checks if the value of the field is `FULLBYTE`"]
    #[inline(always)]
    pub fn is_fullbyte(&self) -> bool {
        *self == DMA0MODE_A::FULLBYTE
    }
    #[doc = "Checks if the value of the field is `LENLIMITBYTE`"]
    #[inline(always)]
    pub fn is_lenlimitbyte(&self) -> bool {
        *self == DMA0MODE_A::LENLIMITBYTE
    }
}
#[doc = "Write proxy for field `DMA0MODE`"]
pub struct DMA0MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA0MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA0MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Target register is fully read/written during every DMA transaction"]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(DMA0MODE_A::FULL)
    }
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Zero padding is automatically added when writing."]
    #[inline(always)]
    pub fn lenlimit(self) -> &'a mut W {
        self.variant(DMA0MODE_A::LENLIMIT)
    }
    #[doc = "Target register is fully read/written during every DMA transaction. Bytewise DMA."]
    #[inline(always)]
    pub fn fullbyte(self) -> &'a mut W {
        self.variant(DMA0MODE_A::FULLBYTE)
    }
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing."]
    #[inline(always)]
    pub fn lenlimitbyte(self) -> &'a mut W {
        self.variant(DMA0MODE_A::LENLIMITBYTE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "DMA0 Read Register Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA0RSEL_A {
    #[doc = "0: `0`"]
    DATA0,
    #[doc = "1: `1`"]
    DDATA0,
    #[doc = "2: `10`"]
    DDATA0BIG,
    #[doc = "3: `11`"]
    QDATA0,
}
impl From<DMA0RSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA0RSEL_A) -> Self {
        match variant {
            DMA0RSEL_A::DATA0 => 0,
            DMA0RSEL_A::DDATA0 => 1,
            DMA0RSEL_A::DDATA0BIG => 2,
            DMA0RSEL_A::QDATA0 => 3,
        }
    }
}
#[doc = "Reader of field `DMA0RSEL`"]
pub type DMA0RSEL_R = crate::R<u8, DMA0RSEL_A>;
impl DMA0RSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA0RSEL_A {
        match self.bits {
            0 => DMA0RSEL_A::DATA0,
            1 => DMA0RSEL_A::DDATA0,
            2 => DMA0RSEL_A::DDATA0BIG,
            3 => DMA0RSEL_A::QDATA0,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DATA0`"]
    #[inline(always)]
    pub fn is_data0(&self) -> bool {
        *self == DMA0RSEL_A::DATA0
    }
    #[doc = "Checks if the value of the field is `DDATA0`"]
    #[inline(always)]
    pub fn is_ddata0(&self) -> bool {
        *self == DMA0RSEL_A::DDATA0
    }
    #[doc = "Checks if the value of the field is `DDATA0BIG`"]
    #[inline(always)]
    pub fn is_ddata0big(&self) -> bool {
        *self == DMA0RSEL_A::DDATA0BIG
    }
    #[doc = "Checks if the value of the field is `QDATA0`"]
    #[inline(always)]
    pub fn is_qdata0(&self) -> bool {
        *self == DMA0RSEL_A::QDATA0
    }
}
#[doc = "Write proxy for field `DMA0RSEL`"]
pub struct DMA0RSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA0RSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA0RSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn data0(self) -> &'a mut W {
        self.variant(DMA0RSEL_A::DATA0)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ddata0(self) -> &'a mut W {
        self.variant(DMA0RSEL_A::DDATA0)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn ddata0big(self) -> &'a mut W {
        self.variant(DMA0RSEL_A::DDATA0BIG)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn qdata0(self) -> &'a mut W {
        self.variant(DMA0RSEL_A::QDATA0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "DMA1 Read Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA1MODE_A {
    #[doc = "0: Target register is fully read/written during every DMA transaction"]
    FULL,
    #[doc = "1: Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Zero padding is automatically added when writing."]
    LENLIMIT,
    #[doc = "2: Target register is fully read/written during every DMA transaction. Bytewise DMA."]
    FULLBYTE,
    #[doc = "3: Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing."]
    LENLIMITBYTE,
}
impl From<DMA1MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA1MODE_A) -> Self {
        match variant {
            DMA1MODE_A::FULL => 0,
            DMA1MODE_A::LENLIMIT => 1,
            DMA1MODE_A::FULLBYTE => 2,
            DMA1MODE_A::LENLIMITBYTE => 3,
        }
    }
}
#[doc = "Reader of field `DMA1MODE`"]
pub type DMA1MODE_R = crate::R<u8, DMA1MODE_A>;
impl DMA1MODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA1MODE_A {
        match self.bits {
            0 => DMA1MODE_A::FULL,
            1 => DMA1MODE_A::LENLIMIT,
            2 => DMA1MODE_A::FULLBYTE,
            3 => DMA1MODE_A::LENLIMITBYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == DMA1MODE_A::FULL
    }
    #[doc = "Checks if the value of the field is `LENLIMIT`"]
    #[inline(always)]
    pub fn is_lenlimit(&self) -> bool {
        *self == DMA1MODE_A::LENLIMIT
    }
    #[doc = "Checks if the value of the field is `FULLBYTE`"]
    #[inline(always)]
    pub fn is_fullbyte(&self) -> bool {
        *self == DMA1MODE_A::FULLBYTE
    }
    #[doc = "Checks if the value of the field is `LENLIMITBYTE`"]
    #[inline(always)]
    pub fn is_lenlimitbyte(&self) -> bool {
        *self == DMA1MODE_A::LENLIMITBYTE
    }
}
#[doc = "Write proxy for field `DMA1MODE`"]
pub struct DMA1MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA1MODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Target register is fully read/written during every DMA transaction"]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(DMA1MODE_A::FULL)
    }
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Zero padding is automatically added when writing."]
    #[inline(always)]
    pub fn lenlimit(self) -> &'a mut W {
        self.variant(DMA1MODE_A::LENLIMIT)
    }
    #[doc = "Target register is fully read/written during every DMA transaction. Bytewise DMA."]
    #[inline(always)]
    pub fn fullbyte(self) -> &'a mut W {
        self.variant(DMA1MODE_A::FULLBYTE)
    }
    #[doc = "Length Limited. When the current length, i.e. LENGTHA or LENGTHB indicates that there are less bytes available than the register size, only length + 1 bytes + necessary zero padding is read. Bytewise DMA. Zero padding is automatically added when writing."]
    #[inline(always)]
    pub fn lenlimitbyte(self) -> &'a mut W {
        self.variant(DMA1MODE_A::LENLIMITBYTE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | (((value as u32) & 0x03) << 24);
        self.w
    }
}
#[doc = "DATA0 DMA Unaligned Read Register Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DMA1RSEL_A {
    #[doc = "0: `0`"]
    DATA1,
    #[doc = "1: `1`"]
    DDATA1,
    #[doc = "2: `10`"]
    QDATA1,
    #[doc = "3: `11`"]
    QDATA1BIG,
}
impl From<DMA1RSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DMA1RSEL_A) -> Self {
        match variant {
            DMA1RSEL_A::DATA1 => 0,
            DMA1RSEL_A::DDATA1 => 1,
            DMA1RSEL_A::QDATA1 => 2,
            DMA1RSEL_A::QDATA1BIG => 3,
        }
    }
}
#[doc = "Reader of field `DMA1RSEL`"]
pub type DMA1RSEL_R = crate::R<u8, DMA1RSEL_A>;
impl DMA1RSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA1RSEL_A {
        match self.bits {
            0 => DMA1RSEL_A::DATA1,
            1 => DMA1RSEL_A::DDATA1,
            2 => DMA1RSEL_A::QDATA1,
            3 => DMA1RSEL_A::QDATA1BIG,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DATA1`"]
    #[inline(always)]
    pub fn is_data1(&self) -> bool {
        *self == DMA1RSEL_A::DATA1
    }
    #[doc = "Checks if the value of the field is `DDATA1`"]
    #[inline(always)]
    pub fn is_ddata1(&self) -> bool {
        *self == DMA1RSEL_A::DDATA1
    }
    #[doc = "Checks if the value of the field is `QDATA1`"]
    #[inline(always)]
    pub fn is_qdata1(&self) -> bool {
        *self == DMA1RSEL_A::QDATA1
    }
    #[doc = "Checks if the value of the field is `QDATA1BIG`"]
    #[inline(always)]
    pub fn is_qdata1big(&self) -> bool {
        *self == DMA1RSEL_A::QDATA1BIG
    }
}
#[doc = "Write proxy for field `DMA1RSEL`"]
pub struct DMA1RSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1RSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DMA1RSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn data1(self) -> &'a mut W {
        self.variant(DMA1RSEL_A::DATA1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn ddata1(self) -> &'a mut W {
        self.variant(DMA1RSEL_A::DDATA1)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn qdata1(self) -> &'a mut W {
        self.variant(DMA1RSEL_A::QDATA1)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn qdata1big(self) -> &'a mut W {
        self.variant(DMA1RSEL_A::QDATA1BIG)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "Reader of field `COMBDMA0WEREQ`"]
pub type COMBDMA0WEREQ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMBDMA0WEREQ`"]
pub struct COMBDMA0WEREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> COMBDMA0WEREQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - AES Mode"]
    #[inline(always)]
    pub fn aes(&self) -> AES_R {
        AES_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Key Buffer Disable"]
    #[inline(always)]
    pub fn keybufdis(&self) -> KEYBUFDIS_R {
        KEYBUFDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SHA Mode"]
    #[inline(always)]
    pub fn sha(&self) -> SHA_R {
        SHA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 10 - No Stalling of Bus When Busy"]
    #[inline(always)]
    pub fn nobusystall(&self) -> NOBUSYSTALL_R {
        NOBUSYSTALL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Increment Width"]
    #[inline(always)]
    pub fn incwidth(&self) -> INCWIDTH_R {
        INCWIDTH_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - DMA0 Read Mode"]
    #[inline(always)]
    pub fn dma0mode(&self) -> DMA0MODE_R {
        DMA0MODE_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - DMA0 Read Register Select"]
    #[inline(always)]
    pub fn dma0rsel(&self) -> DMA0RSEL_R {
        DMA0RSEL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - DMA1 Read Mode"]
    #[inline(always)]
    pub fn dma1mode(&self) -> DMA1MODE_R {
        DMA1MODE_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - DATA0 DMA Unaligned Read Register Select"]
    #[inline(always)]
    pub fn dma1rsel(&self) -> DMA1RSEL_R {
        DMA1RSEL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 31 - Combined Data0 Write DMA Request"]
    #[inline(always)]
    pub fn combdma0wereq(&self) -> COMBDMA0WEREQ_R {
        COMBDMA0WEREQ_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - AES Mode"]
    #[inline(always)]
    pub fn aes(&mut self) -> AES_W {
        AES_W { w: self }
    }
    #[doc = "Bit 1 - Key Buffer Disable"]
    #[inline(always)]
    pub fn keybufdis(&mut self) -> KEYBUFDIS_W {
        KEYBUFDIS_W { w: self }
    }
    #[doc = "Bit 2 - SHA Mode"]
    #[inline(always)]
    pub fn sha(&mut self) -> SHA_W {
        SHA_W { w: self }
    }
    #[doc = "Bit 10 - No Stalling of Bus When Busy"]
    #[inline(always)]
    pub fn nobusystall(&mut self) -> NOBUSYSTALL_W {
        NOBUSYSTALL_W { w: self }
    }
    #[doc = "Bits 14:15 - Increment Width"]
    #[inline(always)]
    pub fn incwidth(&mut self) -> INCWIDTH_W {
        INCWIDTH_W { w: self }
    }
    #[doc = "Bits 16:17 - DMA0 Read Mode"]
    #[inline(always)]
    pub fn dma0mode(&mut self) -> DMA0MODE_W {
        DMA0MODE_W { w: self }
    }
    #[doc = "Bits 20:21 - DMA0 Read Register Select"]
    #[inline(always)]
    pub fn dma0rsel(&mut self) -> DMA0RSEL_W {
        DMA0RSEL_W { w: self }
    }
    #[doc = "Bits 24:25 - DMA1 Read Mode"]
    #[inline(always)]
    pub fn dma1mode(&mut self) -> DMA1MODE_W {
        DMA1MODE_W { w: self }
    }
    #[doc = "Bits 28:29 - DATA0 DMA Unaligned Read Register Select"]
    #[inline(always)]
    pub fn dma1rsel(&mut self) -> DMA1RSEL_W {
        DMA1RSEL_W { w: self }
    }
    #[doc = "Bit 31 - Combined Data0 Write DMA Request"]
    #[inline(always)]
    pub fn combdma0wereq(&mut self) -> COMBDMA0WEREQ_W {
        COMBDMA0WEREQ_W { w: self }
    }
}
