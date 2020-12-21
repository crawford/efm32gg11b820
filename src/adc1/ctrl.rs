#[doc = "Reader of register CTRL"]
pub type R = crate::R<u32, super::CTRL>;
#[doc = "Writer for register CTRL"]
pub type W = crate::W<u32, super::CTRL>;
#[doc = "Register CTRL `reset()`'s with value 0x001f_0000"]
impl crate::ResetValue for super::CTRL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x001f_0000
    }
}
#[doc = "Warm-up Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum WARMUPMODE_A {
    #[doc = "0: ADC is shut down after each conversion. 5us warmup time is used before each conversion."]
    NORMAL = 0,
    #[doc = "1: ADC is kept in standby mode between conversions. 1us warmup time is used before each conversion."]
    KEEPINSTANDBY = 1,
    #[doc = "2: ADC is kept in slow acquisition mode between conversions. 1us warmup time is used before each conversion."]
    KEEPINSLOWACC = 2,
    #[doc = "3: ADC is kept on after conversions, allowing for continuous conversion."]
    KEEPADCWARM = 3,
}
impl From<WARMUPMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: WARMUPMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `WARMUPMODE`"]
pub type WARMUPMODE_R = crate::R<u8, WARMUPMODE_A>;
impl WARMUPMODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WARMUPMODE_A {
        match self.bits {
            0 => WARMUPMODE_A::NORMAL,
            1 => WARMUPMODE_A::KEEPINSTANDBY,
            2 => WARMUPMODE_A::KEEPINSLOWACC,
            3 => WARMUPMODE_A::KEEPADCWARM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == WARMUPMODE_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `KEEPINSTANDBY`"]
    #[inline(always)]
    pub fn is_keepinstandby(&self) -> bool {
        *self == WARMUPMODE_A::KEEPINSTANDBY
    }
    #[doc = "Checks if the value of the field is `KEEPINSLOWACC`"]
    #[inline(always)]
    pub fn is_keepinslowacc(&self) -> bool {
        *self == WARMUPMODE_A::KEEPINSLOWACC
    }
    #[doc = "Checks if the value of the field is `KEEPADCWARM`"]
    #[inline(always)]
    pub fn is_keepadcwarm(&self) -> bool {
        *self == WARMUPMODE_A::KEEPADCWARM
    }
}
#[doc = "Write proxy for field `WARMUPMODE`"]
pub struct WARMUPMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> WARMUPMODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WARMUPMODE_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "ADC is shut down after each conversion. 5us warmup time is used before each conversion."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(WARMUPMODE_A::NORMAL)
    }
    #[doc = "ADC is kept in standby mode between conversions. 1us warmup time is used before each conversion."]
    #[inline(always)]
    pub fn keepinstandby(self) -> &'a mut W {
        self.variant(WARMUPMODE_A::KEEPINSTANDBY)
    }
    #[doc = "ADC is kept in slow acquisition mode between conversions. 1us warmup time is used before each conversion."]
    #[inline(always)]
    pub fn keepinslowacc(self) -> &'a mut W {
        self.variant(WARMUPMODE_A::KEEPINSLOWACC)
    }
    #[doc = "ADC is kept on after conversions, allowing for continuous conversion."]
    #[inline(always)]
    pub fn keepadcwarm(self) -> &'a mut W {
        self.variant(WARMUPMODE_A::KEEPADCWARM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `SINGLEDMAWU`"]
pub type SINGLEDMAWU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SINGLEDMAWU`"]
pub struct SINGLEDMAWU_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGLEDMAWU_W<'a> {
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
#[doc = "Reader of field `SCANDMAWU`"]
pub type SCANDMAWU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SCANDMAWU`"]
pub struct SCANDMAWU_W<'a> {
    w: &'a mut W,
}
impl<'a> SCANDMAWU_W<'a> {
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
#[doc = "Reader of field `TAILGATE`"]
pub type TAILGATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TAILGATE`"]
pub struct TAILGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAILGATE_W<'a> {
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
#[doc = "Reader of field `ASYNCCLKEN`"]
pub type ASYNCCLKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ASYNCCLKEN`"]
pub struct ASYNCCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ASYNCCLKEN_W<'a> {
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
#[doc = "Reader of field `ADCCLKMODE`"]
pub type ADCCLKMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADCCLKMODE`"]
pub struct ADCCLKMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCCLKMODE_W<'a> {
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
#[doc = "Prescalar Setting for ADC Sample and Conversion Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRESC_A {
    #[doc = "0: `0`"]
    NODIVISION = 0,
}
impl From<PRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `PRESC`"]
pub type PRESC_R = crate::R<u8, PRESC_A>;
impl PRESC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRESC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRESC_A::NODIVISION),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `NODIVISION`"]
    #[inline(always)]
    pub fn is_nodivision(&self) -> bool {
        *self == PRESC_A::NODIVISION
    }
}
#[doc = "Write proxy for field `PRESC`"]
pub struct PRESC_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut W {
        self.variant(PRESC_A::NODIVISION)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `TIMEBASE`"]
pub type TIMEBASE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TIMEBASE`"]
pub struct TIMEBASE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEBASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Oversample Rate Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OVSRSEL_A {
    #[doc = "0: 2 samples for each conversion result"]
    X2 = 0,
    #[doc = "1: 4 samples for each conversion result"]
    X4 = 1,
    #[doc = "2: 8 samples for each conversion result"]
    X8 = 2,
    #[doc = "3: 16 samples for each conversion result"]
    X16 = 3,
    #[doc = "4: 32 samples for each conversion result"]
    X32 = 4,
    #[doc = "5: 64 samples for each conversion result"]
    X64 = 5,
    #[doc = "6: 128 samples for each conversion result"]
    X128 = 6,
    #[doc = "7: 256 samples for each conversion result"]
    X256 = 7,
    #[doc = "8: 512 samples for each conversion result"]
    X512 = 8,
    #[doc = "9: 1024 samples for each conversion result"]
    X1024 = 9,
    #[doc = "10: 2048 samples for each conversion result"]
    X2048 = 10,
    #[doc = "11: 4096 samples for each conversion result"]
    X4096 = 11,
}
impl From<OVSRSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: OVSRSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `OVSRSEL`"]
pub type OVSRSEL_R = crate::R<u8, OVSRSEL_A>;
impl OVSRSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, OVSRSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(OVSRSEL_A::X2),
            1 => Val(OVSRSEL_A::X4),
            2 => Val(OVSRSEL_A::X8),
            3 => Val(OVSRSEL_A::X16),
            4 => Val(OVSRSEL_A::X32),
            5 => Val(OVSRSEL_A::X64),
            6 => Val(OVSRSEL_A::X128),
            7 => Val(OVSRSEL_A::X256),
            8 => Val(OVSRSEL_A::X512),
            9 => Val(OVSRSEL_A::X1024),
            10 => Val(OVSRSEL_A::X2048),
            11 => Val(OVSRSEL_A::X4096),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `X2`"]
    #[inline(always)]
    pub fn is_x2(&self) -> bool {
        *self == OVSRSEL_A::X2
    }
    #[doc = "Checks if the value of the field is `X4`"]
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        *self == OVSRSEL_A::X4
    }
    #[doc = "Checks if the value of the field is `X8`"]
    #[inline(always)]
    pub fn is_x8(&self) -> bool {
        *self == OVSRSEL_A::X8
    }
    #[doc = "Checks if the value of the field is `X16`"]
    #[inline(always)]
    pub fn is_x16(&self) -> bool {
        *self == OVSRSEL_A::X16
    }
    #[doc = "Checks if the value of the field is `X32`"]
    #[inline(always)]
    pub fn is_x32(&self) -> bool {
        *self == OVSRSEL_A::X32
    }
    #[doc = "Checks if the value of the field is `X64`"]
    #[inline(always)]
    pub fn is_x64(&self) -> bool {
        *self == OVSRSEL_A::X64
    }
    #[doc = "Checks if the value of the field is `X128`"]
    #[inline(always)]
    pub fn is_x128(&self) -> bool {
        *self == OVSRSEL_A::X128
    }
    #[doc = "Checks if the value of the field is `X256`"]
    #[inline(always)]
    pub fn is_x256(&self) -> bool {
        *self == OVSRSEL_A::X256
    }
    #[doc = "Checks if the value of the field is `X512`"]
    #[inline(always)]
    pub fn is_x512(&self) -> bool {
        *self == OVSRSEL_A::X512
    }
    #[doc = "Checks if the value of the field is `X1024`"]
    #[inline(always)]
    pub fn is_x1024(&self) -> bool {
        *self == OVSRSEL_A::X1024
    }
    #[doc = "Checks if the value of the field is `X2048`"]
    #[inline(always)]
    pub fn is_x2048(&self) -> bool {
        *self == OVSRSEL_A::X2048
    }
    #[doc = "Checks if the value of the field is `X4096`"]
    #[inline(always)]
    pub fn is_x4096(&self) -> bool {
        *self == OVSRSEL_A::X4096
    }
}
#[doc = "Write proxy for field `OVSRSEL`"]
pub struct OVSRSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OVSRSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OVSRSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "2 samples for each conversion result"]
    #[inline(always)]
    pub fn x2(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X2)
    }
    #[doc = "4 samples for each conversion result"]
    #[inline(always)]
    pub fn x4(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X4)
    }
    #[doc = "8 samples for each conversion result"]
    #[inline(always)]
    pub fn x8(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X8)
    }
    #[doc = "16 samples for each conversion result"]
    #[inline(always)]
    pub fn x16(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X16)
    }
    #[doc = "32 samples for each conversion result"]
    #[inline(always)]
    pub fn x32(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X32)
    }
    #[doc = "64 samples for each conversion result"]
    #[inline(always)]
    pub fn x64(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X64)
    }
    #[doc = "128 samples for each conversion result"]
    #[inline(always)]
    pub fn x128(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X128)
    }
    #[doc = "256 samples for each conversion result"]
    #[inline(always)]
    pub fn x256(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X256)
    }
    #[doc = "512 samples for each conversion result"]
    #[inline(always)]
    pub fn x512(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X512)
    }
    #[doc = "1024 samples for each conversion result"]
    #[inline(always)]
    pub fn x1024(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X1024)
    }
    #[doc = "2048 samples for each conversion result"]
    #[inline(always)]
    pub fn x2048(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X2048)
    }
    #[doc = "4096 samples for each conversion result"]
    #[inline(always)]
    pub fn x4096(self) -> &'a mut W {
        self.variant(OVSRSEL_A::X4096)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `DBGHALT`"]
pub type DBGHALT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBGHALT`"]
pub struct DBGHALT_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGHALT_W<'a> {
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
#[doc = "Reader of field `CHCONMODE`"]
pub type CHCONMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CHCONMODE`"]
pub struct CHCONMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHCONMODE_W<'a> {
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
#[doc = "Channel Connect and Reference Warm Sel When ADC is IDLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHCONREFWARMIDLE_A {
    #[doc = "0: Keep scan reference warm and APORT switches for first scan channel closed if WARMUPMODE is not NORMAL"]
    PREFSCAN = 0,
    #[doc = "1: Keep single reference warm and keep APORT switches for single channel closed if WARMUPMODE is not NORMAL"]
    PREFSINGLE = 1,
    #[doc = "2: Keep last used reference warm and keep APORT switches for corresponding channel closed if WARMUPMODE is not NORMAL"]
    KEEPPREV = 2,
}
impl From<CHCONREFWARMIDLE_A> for u8 {
    #[inline(always)]
    fn from(variant: CHCONREFWARMIDLE_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CHCONREFWARMIDLE`"]
pub type CHCONREFWARMIDLE_R = crate::R<u8, CHCONREFWARMIDLE_A>;
impl CHCONREFWARMIDLE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CHCONREFWARMIDLE_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CHCONREFWARMIDLE_A::PREFSCAN),
            1 => Val(CHCONREFWARMIDLE_A::PREFSINGLE),
            2 => Val(CHCONREFWARMIDLE_A::KEEPPREV),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PREFSCAN`"]
    #[inline(always)]
    pub fn is_prefscan(&self) -> bool {
        *self == CHCONREFWARMIDLE_A::PREFSCAN
    }
    #[doc = "Checks if the value of the field is `PREFSINGLE`"]
    #[inline(always)]
    pub fn is_prefsingle(&self) -> bool {
        *self == CHCONREFWARMIDLE_A::PREFSINGLE
    }
    #[doc = "Checks if the value of the field is `KEEPPREV`"]
    #[inline(always)]
    pub fn is_keepprev(&self) -> bool {
        *self == CHCONREFWARMIDLE_A::KEEPPREV
    }
}
#[doc = "Write proxy for field `CHCONREFWARMIDLE`"]
pub struct CHCONREFWARMIDLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CHCONREFWARMIDLE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHCONREFWARMIDLE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Keep scan reference warm and APORT switches for first scan channel closed if WARMUPMODE is not NORMAL"]
    #[inline(always)]
    pub fn prefscan(self) -> &'a mut W {
        self.variant(CHCONREFWARMIDLE_A::PREFSCAN)
    }
    #[doc = "Keep single reference warm and keep APORT switches for single channel closed if WARMUPMODE is not NORMAL"]
    #[inline(always)]
    pub fn prefsingle(self) -> &'a mut W {
        self.variant(CHCONREFWARMIDLE_A::PREFSINGLE)
    }
    #[doc = "Keep last used reference warm and keep APORT switches for corresponding channel closed if WARMUPMODE is not NORMAL"]
    #[inline(always)]
    pub fn keepprev(self) -> &'a mut W {
        self.variant(CHCONREFWARMIDLE_A::KEEPPREV)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | (((value as u32) & 0x03) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Warm-up Mode"]
    #[inline(always)]
    pub fn warmupmode(&self) -> WARMUPMODE_R {
        WARMUPMODE_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - SINGLEFIFO DMA Wakeup"]
    #[inline(always)]
    pub fn singledmawu(&self) -> SINGLEDMAWU_R {
        SINGLEDMAWU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SCANFIFO DMA Wakeup"]
    #[inline(always)]
    pub fn scandmawu(&self) -> SCANDMAWU_R {
        SCANDMAWU_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Conversion Tailgating"]
    #[inline(always)]
    pub fn tailgate(&self) -> TAILGATE_R {
        TAILGATE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Selects ASYNC CLK Enable Mode When ADCCLKMODE=1"]
    #[inline(always)]
    pub fn asyncclken(&self) -> ASYNCCLKEN_R {
        ASYNCCLKEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ADC Clock Mode"]
    #[inline(always)]
    pub fn adcclkmode(&self) -> ADCCLKMODE_R {
        ADCCLKMODE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - Prescalar Setting for ADC Sample and Conversion Clock"]
    #[inline(always)]
    pub fn presc(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - 1us Time Base"]
    #[inline(always)]
    pub fn timebase(&self) -> TIMEBASE_R {
        TIMEBASE_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:27 - Oversample Rate Select"]
    #[inline(always)]
    pub fn ovsrsel(&self) -> OVSRSEL_R {
        OVSRSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Debug Mode Halt Enable"]
    #[inline(always)]
    pub fn dbghalt(&self) -> DBGHALT_R {
        DBGHALT_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Channel Connect"]
    #[inline(always)]
    pub fn chconmode(&self) -> CHCONMODE_R {
        CHCONMODE_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bits 30:31 - Channel Connect and Reference Warm Sel When ADC is IDLE"]
    #[inline(always)]
    pub fn chconrefwarmidle(&self) -> CHCONREFWARMIDLE_R {
        CHCONREFWARMIDLE_R::new(((self.bits >> 30) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Warm-up Mode"]
    #[inline(always)]
    pub fn warmupmode(&mut self) -> WARMUPMODE_W {
        WARMUPMODE_W { w: self }
    }
    #[doc = "Bit 2 - SINGLEFIFO DMA Wakeup"]
    #[inline(always)]
    pub fn singledmawu(&mut self) -> SINGLEDMAWU_W {
        SINGLEDMAWU_W { w: self }
    }
    #[doc = "Bit 3 - SCANFIFO DMA Wakeup"]
    #[inline(always)]
    pub fn scandmawu(&mut self) -> SCANDMAWU_W {
        SCANDMAWU_W { w: self }
    }
    #[doc = "Bit 4 - Conversion Tailgating"]
    #[inline(always)]
    pub fn tailgate(&mut self) -> TAILGATE_W {
        TAILGATE_W { w: self }
    }
    #[doc = "Bit 6 - Selects ASYNC CLK Enable Mode When ADCCLKMODE=1"]
    #[inline(always)]
    pub fn asyncclken(&mut self) -> ASYNCCLKEN_W {
        ASYNCCLKEN_W { w: self }
    }
    #[doc = "Bit 7 - ADC Clock Mode"]
    #[inline(always)]
    pub fn adcclkmode(&mut self) -> ADCCLKMODE_W {
        ADCCLKMODE_W { w: self }
    }
    #[doc = "Bits 8:14 - Prescalar Setting for ADC Sample and Conversion Clock"]
    #[inline(always)]
    pub fn presc(&mut self) -> PRESC_W {
        PRESC_W { w: self }
    }
    #[doc = "Bits 16:22 - 1us Time Base"]
    #[inline(always)]
    pub fn timebase(&mut self) -> TIMEBASE_W {
        TIMEBASE_W { w: self }
    }
    #[doc = "Bits 24:27 - Oversample Rate Select"]
    #[inline(always)]
    pub fn ovsrsel(&mut self) -> OVSRSEL_W {
        OVSRSEL_W { w: self }
    }
    #[doc = "Bit 28 - Debug Mode Halt Enable"]
    #[inline(always)]
    pub fn dbghalt(&mut self) -> DBGHALT_W {
        DBGHALT_W { w: self }
    }
    #[doc = "Bit 29 - Channel Connect"]
    #[inline(always)]
    pub fn chconmode(&mut self) -> CHCONMODE_W {
        CHCONMODE_W { w: self }
    }
    #[doc = "Bits 30:31 - Channel Connect and Reference Warm Sel When ADC is IDLE"]
    #[inline(always)]
    pub fn chconrefwarmidle(&mut self) -> CHCONREFWARMIDLE_W {
        CHCONREFWARMIDLE_W { w: self }
    }
}
