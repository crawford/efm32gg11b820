#[doc = "Reader of register NETWORKCFG"]
pub type R = crate::R<u32, super::NETWORKCFG>;
#[doc = "Writer for register NETWORKCFG"]
pub type W = crate::W<u32, super::NETWORKCFG>;
#[doc = "Register NETWORKCFG `reset()`'s with value 0x0008_0000"]
impl crate::ResetValue for super::NETWORKCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0008_0000
    }
}
#[doc = "Reader of field `SPEED`"]
pub type SPEED_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPEED`"]
pub struct SPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> SPEED_W<'a> {
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
#[doc = "Reader of field `FULLDUPLEX`"]
pub type FULLDUPLEX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FULLDUPLEX`"]
pub struct FULLDUPLEX_W<'a> {
    w: &'a mut W,
}
impl<'a> FULLDUPLEX_W<'a> {
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
#[doc = "Reader of field `DISCRDNONVLANFRAMES`"]
pub type DISCRDNONVLANFRAMES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISCRDNONVLANFRAMES`"]
pub struct DISCRDNONVLANFRAMES_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCRDNONVLANFRAMES_W<'a> {
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
#[doc = "Reader of field `JUMBOFRAMES`"]
pub type JUMBOFRAMES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JUMBOFRAMES`"]
pub struct JUMBOFRAMES_W<'a> {
    w: &'a mut W,
}
impl<'a> JUMBOFRAMES_W<'a> {
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
#[doc = "Reader of field `COPYALLFRAMES`"]
pub type COPYALLFRAMES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COPYALLFRAMES`"]
pub struct COPYALLFRAMES_W<'a> {
    w: &'a mut W,
}
impl<'a> COPYALLFRAMES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `NOBROADCAST`"]
pub type NOBROADCAST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NOBROADCAST`"]
pub struct NOBROADCAST_W<'a> {
    w: &'a mut W,
}
impl<'a> NOBROADCAST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `MULTICASTHASHEN`"]
pub type MULTICASTHASHEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MULTICASTHASHEN`"]
pub struct MULTICASTHASHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MULTICASTHASHEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `UNICASTHASHEN`"]
pub type UNICASTHASHEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UNICASTHASHEN`"]
pub struct UNICASTHASHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UNICASTHASHEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `RX1536BYTEFRAMES`"]
pub type RX1536BYTEFRAMES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX1536BYTEFRAMES`"]
pub struct RX1536BYTEFRAMES_W<'a> {
    w: &'a mut W,
}
impl<'a> RX1536BYTEFRAMES_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `RETRYTEST`"]
pub type RETRYTEST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RETRYTEST`"]
pub struct RETRYTEST_W<'a> {
    w: &'a mut W,
}
impl<'a> RETRYTEST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `PAUSEEN`"]
pub type PAUSEEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAUSEEN`"]
pub struct PAUSEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PAUSEEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `RXBUFFOFFSET`"]
pub type RXBUFFOFFSET_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXBUFFOFFSET`"]
pub struct RXBUFFOFFSET_W<'a> {
    w: &'a mut W,
}
impl<'a> RXBUFFOFFSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `LENFIELDERRFRMDISCRD`"]
pub type LENFIELDERRFRMDISCRD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LENFIELDERRFRMDISCRD`"]
pub struct LENFIELDERRFRMDISCRD_W<'a> {
    w: &'a mut W,
}
impl<'a> LENFIELDERRFRMDISCRD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `FCSREMOVE`"]
pub type FCSREMOVE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FCSREMOVE`"]
pub struct FCSREMOVE_W<'a> {
    w: &'a mut W,
}
impl<'a> FCSREMOVE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Possible values of the field `MDCCLKDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDCCLKDIV_A {
    #[doc = "divide HFBUSCLKETH by 8 (HFBUSCLKETH up to 20 MHz)"]
    DIVBY8,
    #[doc = "divide HFBUSCLKETH by 16 (HFBUSCLKETH up to 40 MHz)"]
    DIVBY16,
    #[doc = "divide HFBUSCLKETH by 32 (HFBUSCLKETH up to 80 MHz)"]
    DIVBY32,
    #[doc = "divide HFBUSCLKETH by 48 (HFBUSCLKETH up to 120 MHz)"]
    DIVBY48,
    #[doc = "divide HFBUSCLKETH by 64 (HFBUSCLKETH up to 160 MHz)"]
    DIVBY64,
    #[doc = "divide HFBUSCLKETH by 96 (HFBUSCLKETH up to 240 MHz)"]
    DIVBY96,
    #[doc = "divide HFBUSCLKETH by 128 (HFBUSCLKETH up to 320 MHz)"]
    DIVBY128,
    #[doc = "divide HFBUSCLKETH by 224 (HFBUSCLKETH up to 540 MHz)"]
    DIVBY224,
}
impl crate::ToBits<u8> for MDCCLKDIV_A {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            MDCCLKDIV_A::DIVBY8 => 0,
            MDCCLKDIV_A::DIVBY16 => 1,
            MDCCLKDIV_A::DIVBY32 => 2,
            MDCCLKDIV_A::DIVBY48 => 3,
            MDCCLKDIV_A::DIVBY64 => 4,
            MDCCLKDIV_A::DIVBY96 => 5,
            MDCCLKDIV_A::DIVBY128 => 6,
            MDCCLKDIV_A::DIVBY224 => 7,
        }
    }
}
#[doc = "Reader of field `MDCCLKDIV`"]
pub type MDCCLKDIV_R = crate::R<u8, MDCCLKDIV_A>;
impl MDCCLKDIV_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDCCLKDIV_A {
        match self.bits {
            0 => MDCCLKDIV_A::DIVBY8,
            1 => MDCCLKDIV_A::DIVBY16,
            2 => MDCCLKDIV_A::DIVBY32,
            3 => MDCCLKDIV_A::DIVBY48,
            4 => MDCCLKDIV_A::DIVBY64,
            5 => MDCCLKDIV_A::DIVBY96,
            6 => MDCCLKDIV_A::DIVBY128,
            7 => MDCCLKDIV_A::DIVBY224,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVBY8`"]
    #[inline(always)]
    pub fn is_divby8(&self) -> bool {
        *self == MDCCLKDIV_A::DIVBY8
    }
    #[doc = "Checks if the value of the field is `DIVBY16`"]
    #[inline(always)]
    pub fn is_divby16(&self) -> bool {
        *self == MDCCLKDIV_A::DIVBY16
    }
    #[doc = "Checks if the value of the field is `DIVBY32`"]
    #[inline(always)]
    pub fn is_divby32(&self) -> bool {
        *self == MDCCLKDIV_A::DIVBY32
    }
    #[doc = "Checks if the value of the field is `DIVBY48`"]
    #[inline(always)]
    pub fn is_divby48(&self) -> bool {
        *self == MDCCLKDIV_A::DIVBY48
    }
    #[doc = "Checks if the value of the field is `DIVBY64`"]
    #[inline(always)]
    pub fn is_divby64(&self) -> bool {
        *self == MDCCLKDIV_A::DIVBY64
    }
    #[doc = "Checks if the value of the field is `DIVBY96`"]
    #[inline(always)]
    pub fn is_divby96(&self) -> bool {
        *self == MDCCLKDIV_A::DIVBY96
    }
    #[doc = "Checks if the value of the field is `DIVBY128`"]
    #[inline(always)]
    pub fn is_divby128(&self) -> bool {
        *self == MDCCLKDIV_A::DIVBY128
    }
    #[doc = "Checks if the value of the field is `DIVBY224`"]
    #[inline(always)]
    pub fn is_divby224(&self) -> bool {
        *self == MDCCLKDIV_A::DIVBY224
    }
}
#[doc = "Write proxy for field `MDCCLKDIV`"]
pub struct MDCCLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MDCCLKDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MDCCLKDIV_A) -> &'a mut W {
        use crate::ToBits;
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "divide HFBUSCLKETH by 8 (HFBUSCLKETH up to 20 MHz)"]
    #[inline(always)]
    pub fn divby8(self) -> &'a mut W {
        self.variant(MDCCLKDIV_A::DIVBY8)
    }
    #[doc = "divide HFBUSCLKETH by 16 (HFBUSCLKETH up to 40 MHz)"]
    #[inline(always)]
    pub fn divby16(self) -> &'a mut W {
        self.variant(MDCCLKDIV_A::DIVBY16)
    }
    #[doc = "divide HFBUSCLKETH by 32 (HFBUSCLKETH up to 80 MHz)"]
    #[inline(always)]
    pub fn divby32(self) -> &'a mut W {
        self.variant(MDCCLKDIV_A::DIVBY32)
    }
    #[doc = "divide HFBUSCLKETH by 48 (HFBUSCLKETH up to 120 MHz)"]
    #[inline(always)]
    pub fn divby48(self) -> &'a mut W {
        self.variant(MDCCLKDIV_A::DIVBY48)
    }
    #[doc = "divide HFBUSCLKETH by 64 (HFBUSCLKETH up to 160 MHz)"]
    #[inline(always)]
    pub fn divby64(self) -> &'a mut W {
        self.variant(MDCCLKDIV_A::DIVBY64)
    }
    #[doc = "divide HFBUSCLKETH by 96 (HFBUSCLKETH up to 240 MHz)"]
    #[inline(always)]
    pub fn divby96(self) -> &'a mut W {
        self.variant(MDCCLKDIV_A::DIVBY96)
    }
    #[doc = "divide HFBUSCLKETH by 128 (HFBUSCLKETH up to 320 MHz)"]
    #[inline(always)]
    pub fn divby128(self) -> &'a mut W {
        self.variant(MDCCLKDIV_A::DIVBY128)
    }
    #[doc = "divide HFBUSCLKETH by 224 (HFBUSCLKETH up to 540 MHz)"]
    #[inline(always)]
    pub fn divby224(self) -> &'a mut W {
        self.variant(MDCCLKDIV_A::DIVBY224)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | (((value as u32) & 0x07) << 18);
        self.w
    }
}
#[doc = "Reader of field `DISCOPYOFPFRAMES`"]
pub type DISCOPYOFPFRAMES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DISCOPYOFPFRAMES`"]
pub struct DISCOPYOFPFRAMES_W<'a> {
    w: &'a mut W,
}
impl<'a> DISCOPYOFPFRAMES_W<'a> {
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
#[doc = "Reader of field `RXCHKSUMOFFLOADEN`"]
pub type RXCHKSUMOFFLOADEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXCHKSUMOFFLOADEN`"]
pub struct RXCHKSUMOFFLOADEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCHKSUMOFFLOADEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `ENHALFDUPLEXRX`"]
pub type ENHALFDUPLEXRX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENHALFDUPLEXRX`"]
pub struct ENHALFDUPLEXRX_W<'a> {
    w: &'a mut W,
}
impl<'a> ENHALFDUPLEXRX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `IGNORERXFCS`"]
pub type IGNORERXFCS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IGNORERXFCS`"]
pub struct IGNORERXFCS_W<'a> {
    w: &'a mut W,
}
impl<'a> IGNORERXFCS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `IPGSTRTCHEN`"]
pub type IPGSTRTCHEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IPGSTRTCHEN`"]
pub struct IPGSTRTCHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IPGSTRTCHEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `NSPCHANGE`"]
pub type NSPCHANGE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NSPCHANGE`"]
pub struct NSPCHANGE_W<'a> {
    w: &'a mut W,
}
impl<'a> NSPCHANGE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `IGNOREIPGRXER`"]
pub type IGNOREIPGRXER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IGNOREIPGRXER`"]
pub struct IGNOREIPGRXER_W<'a> {
    w: &'a mut W,
}
impl<'a> IGNOREIPGRXER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Speed"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Full duplex"]
    #[inline(always)]
    pub fn fullduplex(&self) -> FULLDUPLEX_R {
        FULLDUPLEX_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Discard non-VLAN frames"]
    #[inline(always)]
    pub fn discrdnonvlanframes(&self) -> DISCRDNONVLANFRAMES_R {
        DISCRDNONVLANFRAMES_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Jumbo frames enable"]
    #[inline(always)]
    pub fn jumboframes(&self) -> JUMBOFRAMES_R {
        JUMBOFRAMES_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Copy all frames"]
    #[inline(always)]
    pub fn copyallframes(&self) -> COPYALLFRAMES_R {
        COPYALLFRAMES_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - No broadcast"]
    #[inline(always)]
    pub fn nobroadcast(&self) -> NOBROADCAST_R {
        NOBROADCAST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Multicast hash enable"]
    #[inline(always)]
    pub fn multicasthashen(&self) -> MULTICASTHASHEN_R {
        MULTICASTHASHEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Unicast hash enable"]
    #[inline(always)]
    pub fn unicasthashen(&self) -> UNICASTHASHEN_R {
        UNICASTHASHEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Receive 1536 byte frames"]
    #[inline(always)]
    pub fn rx1536byteframes(&self) -> RX1536BYTEFRAMES_R {
        RX1536BYTEFRAMES_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Retry test"]
    #[inline(always)]
    pub fn retrytest(&self) -> RETRYTEST_R {
        RETRYTEST_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Pause enable"]
    #[inline(always)]
    pub fn pauseen(&self) -> PAUSEEN_R {
        PAUSEEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Receive buffer offset"]
    #[inline(always)]
    pub fn rxbuffoffset(&self) -> RXBUFFOFFSET_R {
        RXBUFFOFFSET_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 16 - Length field error frame discard"]
    #[inline(always)]
    pub fn lenfielderrfrmdiscrd(&self) -> LENFIELDERRFRMDISCRD_R {
        LENFIELDERRFRMDISCRD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - FCS remove"]
    #[inline(always)]
    pub fn fcsremove(&self) -> FCSREMOVE_R {
        FCSREMOVE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:20 - MDC clock division"]
    #[inline(always)]
    pub fn mdcclkdiv(&self) -> MDCCLKDIV_R {
        MDCCLKDIV_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bit 23 - Disable copy of pause frames"]
    #[inline(always)]
    pub fn discopyofpframes(&self) -> DISCOPYOFPFRAMES_R {
        DISCOPYOFPFRAMES_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Receive checksum offload enable"]
    #[inline(always)]
    pub fn rxchksumoffloaden(&self) -> RXCHKSUMOFFLOADEN_R {
        RXCHKSUMOFFLOADEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable frames to be received in half-duplex mode while transmitting."]
    #[inline(always)]
    pub fn enhalfduplexrx(&self) -> ENHALFDUPLEXRX_R {
        ENHALFDUPLEXRX_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Ignore RX FCS"]
    #[inline(always)]
    pub fn ignorerxfcs(&self) -> IGNORERXFCS_R {
        IGNORERXFCS_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 28 - IPG stretch enable"]
    #[inline(always)]
    pub fn ipgstrtchen(&self) -> IPGSTRTCHEN_R {
        IPGSTRTCHEN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Receive bad preamble."]
    #[inline(always)]
    pub fn nspchange(&self) -> NSPCHANGE_R {
        NSPCHANGE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Ignore IPG rx_er."]
    #[inline(always)]
    pub fn ignoreipgrxer(&self) -> IGNOREIPGRXER_R {
        IGNOREIPGRXER_R::new(((self.bits >> 30) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Speed"]
    #[inline(always)]
    pub fn speed(&mut self) -> SPEED_W {
        SPEED_W { w: self }
    }
    #[doc = "Bit 1 - Full duplex"]
    #[inline(always)]
    pub fn fullduplex(&mut self) -> FULLDUPLEX_W {
        FULLDUPLEX_W { w: self }
    }
    #[doc = "Bit 2 - Discard non-VLAN frames"]
    #[inline(always)]
    pub fn discrdnonvlanframes(&mut self) -> DISCRDNONVLANFRAMES_W {
        DISCRDNONVLANFRAMES_W { w: self }
    }
    #[doc = "Bit 3 - Jumbo frames enable"]
    #[inline(always)]
    pub fn jumboframes(&mut self) -> JUMBOFRAMES_W {
        JUMBOFRAMES_W { w: self }
    }
    #[doc = "Bit 4 - Copy all frames"]
    #[inline(always)]
    pub fn copyallframes(&mut self) -> COPYALLFRAMES_W {
        COPYALLFRAMES_W { w: self }
    }
    #[doc = "Bit 5 - No broadcast"]
    #[inline(always)]
    pub fn nobroadcast(&mut self) -> NOBROADCAST_W {
        NOBROADCAST_W { w: self }
    }
    #[doc = "Bit 6 - Multicast hash enable"]
    #[inline(always)]
    pub fn multicasthashen(&mut self) -> MULTICASTHASHEN_W {
        MULTICASTHASHEN_W { w: self }
    }
    #[doc = "Bit 7 - Unicast hash enable"]
    #[inline(always)]
    pub fn unicasthashen(&mut self) -> UNICASTHASHEN_W {
        UNICASTHASHEN_W { w: self }
    }
    #[doc = "Bit 8 - Receive 1536 byte frames"]
    #[inline(always)]
    pub fn rx1536byteframes(&mut self) -> RX1536BYTEFRAMES_W {
        RX1536BYTEFRAMES_W { w: self }
    }
    #[doc = "Bit 12 - Retry test"]
    #[inline(always)]
    pub fn retrytest(&mut self) -> RETRYTEST_W {
        RETRYTEST_W { w: self }
    }
    #[doc = "Bit 13 - Pause enable"]
    #[inline(always)]
    pub fn pauseen(&mut self) -> PAUSEEN_W {
        PAUSEEN_W { w: self }
    }
    #[doc = "Bits 14:15 - Receive buffer offset"]
    #[inline(always)]
    pub fn rxbuffoffset(&mut self) -> RXBUFFOFFSET_W {
        RXBUFFOFFSET_W { w: self }
    }
    #[doc = "Bit 16 - Length field error frame discard"]
    #[inline(always)]
    pub fn lenfielderrfrmdiscrd(&mut self) -> LENFIELDERRFRMDISCRD_W {
        LENFIELDERRFRMDISCRD_W { w: self }
    }
    #[doc = "Bit 17 - FCS remove"]
    #[inline(always)]
    pub fn fcsremove(&mut self) -> FCSREMOVE_W {
        FCSREMOVE_W { w: self }
    }
    #[doc = "Bits 18:20 - MDC clock division"]
    #[inline(always)]
    pub fn mdcclkdiv(&mut self) -> MDCCLKDIV_W {
        MDCCLKDIV_W { w: self }
    }
    #[doc = "Bit 23 - Disable copy of pause frames"]
    #[inline(always)]
    pub fn discopyofpframes(&mut self) -> DISCOPYOFPFRAMES_W {
        DISCOPYOFPFRAMES_W { w: self }
    }
    #[doc = "Bit 24 - Receive checksum offload enable"]
    #[inline(always)]
    pub fn rxchksumoffloaden(&mut self) -> RXCHKSUMOFFLOADEN_W {
        RXCHKSUMOFFLOADEN_W { w: self }
    }
    #[doc = "Bit 25 - Enable frames to be received in half-duplex mode while transmitting."]
    #[inline(always)]
    pub fn enhalfduplexrx(&mut self) -> ENHALFDUPLEXRX_W {
        ENHALFDUPLEXRX_W { w: self }
    }
    #[doc = "Bit 26 - Ignore RX FCS"]
    #[inline(always)]
    pub fn ignorerxfcs(&mut self) -> IGNORERXFCS_W {
        IGNORERXFCS_W { w: self }
    }
    #[doc = "Bit 28 - IPG stretch enable"]
    #[inline(always)]
    pub fn ipgstrtchen(&mut self) -> IPGSTRTCHEN_W {
        IPGSTRTCHEN_W { w: self }
    }
    #[doc = "Bit 29 - Receive bad preamble."]
    #[inline(always)]
    pub fn nspchange(&mut self) -> NSPCHANGE_W {
        NSPCHANGE_W { w: self }
    }
    #[doc = "Bit 30 - Ignore IPG rx_er."]
    #[inline(always)]
    pub fn ignoreipgrxer(&mut self) -> IGNOREIPGRXER_W {
        IGNOREIPGRXER_W { w: self }
    }
}
