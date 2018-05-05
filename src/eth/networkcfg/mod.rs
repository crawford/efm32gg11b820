#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::NETWORKCFG {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct SPEEDR {
    bits: bool,
}
impl SPEEDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct FULLDUPLEXR {
    bits: bool,
}
impl FULLDUPLEXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct DISCRDNONVLANFRAMESR {
    bits: bool,
}
impl DISCRDNONVLANFRAMESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct JUMBOFRAMESR {
    bits: bool,
}
impl JUMBOFRAMESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct COPYALLFRAMESR {
    bits: bool,
}
impl COPYALLFRAMESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct NOBROADCASTR {
    bits: bool,
}
impl NOBROADCASTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct MULTICASTHASHENR {
    bits: bool,
}
impl MULTICASTHASHENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct UNICASTHASHENR {
    bits: bool,
}
impl UNICASTHASHENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct RX1536BYTEFRAMESR {
    bits: bool,
}
impl RX1536BYTEFRAMESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct RETRYTESTR {
    bits: bool,
}
impl RETRYTESTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct PAUSEENR {
    bits: bool,
}
impl PAUSEENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct RXBUFFOFFSETR {
    bits: u8,
}
impl RXBUFFOFFSETR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct LENFIELDERRFRMDISCRDR {
    bits: bool,
}
impl LENFIELDERRFRMDISCRDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct FCSREMOVER {
    bits: bool,
}
impl FCSREMOVER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = "Possible values of the field `MDCCLKDIV`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MDCCLKDIVR {
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
impl MDCCLKDIVR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MDCCLKDIVR::DIVBY8 => 0,
            MDCCLKDIVR::DIVBY16 => 1,
            MDCCLKDIVR::DIVBY32 => 2,
            MDCCLKDIVR::DIVBY48 => 3,
            MDCCLKDIVR::DIVBY64 => 4,
            MDCCLKDIVR::DIVBY96 => 5,
            MDCCLKDIVR::DIVBY128 => 6,
            MDCCLKDIVR::DIVBY224 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MDCCLKDIVR {
        match value {
            0 => MDCCLKDIVR::DIVBY8,
            1 => MDCCLKDIVR::DIVBY16,
            2 => MDCCLKDIVR::DIVBY32,
            3 => MDCCLKDIVR::DIVBY48,
            4 => MDCCLKDIVR::DIVBY64,
            5 => MDCCLKDIVR::DIVBY96,
            6 => MDCCLKDIVR::DIVBY128,
            7 => MDCCLKDIVR::DIVBY224,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVBY8`"]
    #[inline]
    pub fn is_divby8(&self) -> bool {
        *self == MDCCLKDIVR::DIVBY8
    }
    #[doc = "Checks if the value of the field is `DIVBY16`"]
    #[inline]
    pub fn is_divby16(&self) -> bool {
        *self == MDCCLKDIVR::DIVBY16
    }
    #[doc = "Checks if the value of the field is `DIVBY32`"]
    #[inline]
    pub fn is_divby32(&self) -> bool {
        *self == MDCCLKDIVR::DIVBY32
    }
    #[doc = "Checks if the value of the field is `DIVBY48`"]
    #[inline]
    pub fn is_divby48(&self) -> bool {
        *self == MDCCLKDIVR::DIVBY48
    }
    #[doc = "Checks if the value of the field is `DIVBY64`"]
    #[inline]
    pub fn is_divby64(&self) -> bool {
        *self == MDCCLKDIVR::DIVBY64
    }
    #[doc = "Checks if the value of the field is `DIVBY96`"]
    #[inline]
    pub fn is_divby96(&self) -> bool {
        *self == MDCCLKDIVR::DIVBY96
    }
    #[doc = "Checks if the value of the field is `DIVBY128`"]
    #[inline]
    pub fn is_divby128(&self) -> bool {
        *self == MDCCLKDIVR::DIVBY128
    }
    #[doc = "Checks if the value of the field is `DIVBY224`"]
    #[inline]
    pub fn is_divby224(&self) -> bool {
        *self == MDCCLKDIVR::DIVBY224
    }
}
#[doc = r" Value of the field"]
pub struct DISCOPYOFPFRAMESR {
    bits: bool,
}
impl DISCOPYOFPFRAMESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct RXCHKSUMOFFLOADENR {
    bits: bool,
}
impl RXCHKSUMOFFLOADENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct ENHALFDUPLEXRXR {
    bits: bool,
}
impl ENHALFDUPLEXRXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct IGNORERXFCSR {
    bits: bool,
}
impl IGNORERXFCSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct IPGSTRTCHENR {
    bits: bool,
}
impl IPGSTRTCHENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct NSPCHANGER {
    bits: bool,
}
impl NSPCHANGER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct IGNOREIPGRXERR {
    bits: bool,
}
impl IGNOREIPGRXERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Proxy"]
pub struct _SPEEDW<'a> {
    w: &'a mut W,
}
impl<'a> _SPEEDW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FULLDUPLEXW<'a> {
    w: &'a mut W,
}
impl<'a> _FULLDUPLEXW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DISCRDNONVLANFRAMESW<'a> {
    w: &'a mut W,
}
impl<'a> _DISCRDNONVLANFRAMESW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _JUMBOFRAMESW<'a> {
    w: &'a mut W,
}
impl<'a> _JUMBOFRAMESW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _COPYALLFRAMESW<'a> {
    w: &'a mut W,
}
impl<'a> _COPYALLFRAMESW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NOBROADCASTW<'a> {
    w: &'a mut W,
}
impl<'a> _NOBROADCASTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MULTICASTHASHENW<'a> {
    w: &'a mut W,
}
impl<'a> _MULTICASTHASHENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _UNICASTHASHENW<'a> {
    w: &'a mut W,
}
impl<'a> _UNICASTHASHENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RX1536BYTEFRAMESW<'a> {
    w: &'a mut W,
}
impl<'a> _RX1536BYTEFRAMESW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RETRYTESTW<'a> {
    w: &'a mut W,
}
impl<'a> _RETRYTESTW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PAUSEENW<'a> {
    w: &'a mut W,
}
impl<'a> _PAUSEENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RXBUFFOFFSETW<'a> {
    w: &'a mut W,
}
impl<'a> _RXBUFFOFFSETW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LENFIELDERRFRMDISCRDW<'a> {
    w: &'a mut W,
}
impl<'a> _LENFIELDERRFRMDISCRDW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _FCSREMOVEW<'a> {
    w: &'a mut W,
}
impl<'a> _FCSREMOVEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MDCCLKDIV`"]
pub enum MDCCLKDIVW {
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
impl MDCCLKDIVW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MDCCLKDIVW::DIVBY8 => 0,
            MDCCLKDIVW::DIVBY16 => 1,
            MDCCLKDIVW::DIVBY32 => 2,
            MDCCLKDIVW::DIVBY48 => 3,
            MDCCLKDIVW::DIVBY64 => 4,
            MDCCLKDIVW::DIVBY96 => 5,
            MDCCLKDIVW::DIVBY128 => 6,
            MDCCLKDIVW::DIVBY224 => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MDCCLKDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _MDCCLKDIVW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MDCCLKDIVW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "divide HFBUSCLKETH by 8 (HFBUSCLKETH up to 20 MHz)"]
    #[inline]
    pub fn divby8(self) -> &'a mut W {
        self.variant(MDCCLKDIVW::DIVBY8)
    }
    #[doc = "divide HFBUSCLKETH by 16 (HFBUSCLKETH up to 40 MHz)"]
    #[inline]
    pub fn divby16(self) -> &'a mut W {
        self.variant(MDCCLKDIVW::DIVBY16)
    }
    #[doc = "divide HFBUSCLKETH by 32 (HFBUSCLKETH up to 80 MHz)"]
    #[inline]
    pub fn divby32(self) -> &'a mut W {
        self.variant(MDCCLKDIVW::DIVBY32)
    }
    #[doc = "divide HFBUSCLKETH by 48 (HFBUSCLKETH up to 120 MHz)"]
    #[inline]
    pub fn divby48(self) -> &'a mut W {
        self.variant(MDCCLKDIVW::DIVBY48)
    }
    #[doc = "divide HFBUSCLKETH by 64 (HFBUSCLKETH up to 160 MHz)"]
    #[inline]
    pub fn divby64(self) -> &'a mut W {
        self.variant(MDCCLKDIVW::DIVBY64)
    }
    #[doc = "divide HFBUSCLKETH by 96 (HFBUSCLKETH up to 240 MHz)"]
    #[inline]
    pub fn divby96(self) -> &'a mut W {
        self.variant(MDCCLKDIVW::DIVBY96)
    }
    #[doc = "divide HFBUSCLKETH by 128 (HFBUSCLKETH up to 320 MHz)"]
    #[inline]
    pub fn divby128(self) -> &'a mut W {
        self.variant(MDCCLKDIVW::DIVBY128)
    }
    #[doc = "divide HFBUSCLKETH by 224 (HFBUSCLKETH up to 540 MHz)"]
    #[inline]
    pub fn divby224(self) -> &'a mut W {
        self.variant(MDCCLKDIVW::DIVBY224)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DISCOPYOFPFRAMESW<'a> {
    w: &'a mut W,
}
impl<'a> _DISCOPYOFPFRAMESW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _RXCHKSUMOFFLOADENW<'a> {
    w: &'a mut W,
}
impl<'a> _RXCHKSUMOFFLOADENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENHALFDUPLEXRXW<'a> {
    w: &'a mut W,
}
impl<'a> _ENHALFDUPLEXRXW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IGNORERXFCSW<'a> {
    w: &'a mut W,
}
impl<'a> _IGNORERXFCSW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IPGSTRTCHENW<'a> {
    w: &'a mut W,
}
impl<'a> _IPGSTRTCHENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NSPCHANGEW<'a> {
    w: &'a mut W,
}
impl<'a> _NSPCHANGEW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IGNOREIPGRXERW<'a> {
    w: &'a mut W,
}
impl<'a> _IGNOREIPGRXERW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Speed"]
    #[inline]
    pub fn speed(&self) -> SPEEDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SPEEDR { bits }
    }
    #[doc = "Bit 1 - Full duplex"]
    #[inline]
    pub fn fullduplex(&self) -> FULLDUPLEXR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FULLDUPLEXR { bits }
    }
    #[doc = "Bit 2 - Discard non-VLAN frames"]
    #[inline]
    pub fn discrdnonvlanframes(&self) -> DISCRDNONVLANFRAMESR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DISCRDNONVLANFRAMESR { bits }
    }
    #[doc = "Bit 3 - Jumbo frames enable"]
    #[inline]
    pub fn jumboframes(&self) -> JUMBOFRAMESR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        JUMBOFRAMESR { bits }
    }
    #[doc = "Bit 4 - Copy all frames"]
    #[inline]
    pub fn copyallframes(&self) -> COPYALLFRAMESR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        COPYALLFRAMESR { bits }
    }
    #[doc = "Bit 5 - No broadcast"]
    #[inline]
    pub fn nobroadcast(&self) -> NOBROADCASTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NOBROADCASTR { bits }
    }
    #[doc = "Bit 6 - Multicast hash enable"]
    #[inline]
    pub fn multicasthashen(&self) -> MULTICASTHASHENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MULTICASTHASHENR { bits }
    }
    #[doc = "Bit 7 - Unicast hash enable"]
    #[inline]
    pub fn unicasthashen(&self) -> UNICASTHASHENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        UNICASTHASHENR { bits }
    }
    #[doc = "Bit 8 - Receive 1536 byte frames"]
    #[inline]
    pub fn rx1536byteframes(&self) -> RX1536BYTEFRAMESR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RX1536BYTEFRAMESR { bits }
    }
    #[doc = "Bit 12 - Retry test"]
    #[inline]
    pub fn retrytest(&self) -> RETRYTESTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RETRYTESTR { bits }
    }
    #[doc = "Bit 13 - Pause enable"]
    #[inline]
    pub fn pauseen(&self) -> PAUSEENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PAUSEENR { bits }
    }
    #[doc = "Bits 14:15 - Receive buffer offset"]
    #[inline]
    pub fn rxbuffoffset(&self) -> RXBUFFOFFSETR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RXBUFFOFFSETR { bits }
    }
    #[doc = "Bit 16 - Length field error frame discard"]
    #[inline]
    pub fn lenfielderrfrmdiscrd(&self) -> LENFIELDERRFRMDISCRDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LENFIELDERRFRMDISCRDR { bits }
    }
    #[doc = "Bit 17 - FCS remove"]
    #[inline]
    pub fn fcsremove(&self) -> FCSREMOVER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        FCSREMOVER { bits }
    }
    #[doc = "Bits 18:20 - MDC clock division"]
    #[inline]
    pub fn mdcclkdiv(&self) -> MDCCLKDIVR {
        MDCCLKDIVR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 23 - Disable copy of pause frames"]
    #[inline]
    pub fn discopyofpframes(&self) -> DISCOPYOFPFRAMESR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DISCOPYOFPFRAMESR { bits }
    }
    #[doc = "Bit 24 - Receive checksum offload enable"]
    #[inline]
    pub fn rxchksumoffloaden(&self) -> RXCHKSUMOFFLOADENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXCHKSUMOFFLOADENR { bits }
    }
    #[doc = "Bit 25 - Enable frames to be received in half-duplex mode while transmitting."]
    #[inline]
    pub fn enhalfduplexrx(&self) -> ENHALFDUPLEXRXR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENHALFDUPLEXRXR { bits }
    }
    #[doc = "Bit 26 - Ignore RX FCS"]
    #[inline]
    pub fn ignorerxfcs(&self) -> IGNORERXFCSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IGNORERXFCSR { bits }
    }
    #[doc = "Bit 28 - IPG stretch enable"]
    #[inline]
    pub fn ipgstrtchen(&self) -> IPGSTRTCHENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IPGSTRTCHENR { bits }
    }
    #[doc = "Bit 29 - Receive bad preamble."]
    #[inline]
    pub fn nspchange(&self) -> NSPCHANGER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 29;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        NSPCHANGER { bits }
    }
    #[doc = "Bit 30 - Ignore IPG rx_er."]
    #[inline]
    pub fn ignoreipgrxer(&self) -> IGNOREIPGRXERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IGNOREIPGRXERR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 524288 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Speed"]
    #[inline]
    pub fn speed(&mut self) -> _SPEEDW {
        _SPEEDW { w: self }
    }
    #[doc = "Bit 1 - Full duplex"]
    #[inline]
    pub fn fullduplex(&mut self) -> _FULLDUPLEXW {
        _FULLDUPLEXW { w: self }
    }
    #[doc = "Bit 2 - Discard non-VLAN frames"]
    #[inline]
    pub fn discrdnonvlanframes(&mut self) -> _DISCRDNONVLANFRAMESW {
        _DISCRDNONVLANFRAMESW { w: self }
    }
    #[doc = "Bit 3 - Jumbo frames enable"]
    #[inline]
    pub fn jumboframes(&mut self) -> _JUMBOFRAMESW {
        _JUMBOFRAMESW { w: self }
    }
    #[doc = "Bit 4 - Copy all frames"]
    #[inline]
    pub fn copyallframes(&mut self) -> _COPYALLFRAMESW {
        _COPYALLFRAMESW { w: self }
    }
    #[doc = "Bit 5 - No broadcast"]
    #[inline]
    pub fn nobroadcast(&mut self) -> _NOBROADCASTW {
        _NOBROADCASTW { w: self }
    }
    #[doc = "Bit 6 - Multicast hash enable"]
    #[inline]
    pub fn multicasthashen(&mut self) -> _MULTICASTHASHENW {
        _MULTICASTHASHENW { w: self }
    }
    #[doc = "Bit 7 - Unicast hash enable"]
    #[inline]
    pub fn unicasthashen(&mut self) -> _UNICASTHASHENW {
        _UNICASTHASHENW { w: self }
    }
    #[doc = "Bit 8 - Receive 1536 byte frames"]
    #[inline]
    pub fn rx1536byteframes(&mut self) -> _RX1536BYTEFRAMESW {
        _RX1536BYTEFRAMESW { w: self }
    }
    #[doc = "Bit 12 - Retry test"]
    #[inline]
    pub fn retrytest(&mut self) -> _RETRYTESTW {
        _RETRYTESTW { w: self }
    }
    #[doc = "Bit 13 - Pause enable"]
    #[inline]
    pub fn pauseen(&mut self) -> _PAUSEENW {
        _PAUSEENW { w: self }
    }
    #[doc = "Bits 14:15 - Receive buffer offset"]
    #[inline]
    pub fn rxbuffoffset(&mut self) -> _RXBUFFOFFSETW {
        _RXBUFFOFFSETW { w: self }
    }
    #[doc = "Bit 16 - Length field error frame discard"]
    #[inline]
    pub fn lenfielderrfrmdiscrd(&mut self) -> _LENFIELDERRFRMDISCRDW {
        _LENFIELDERRFRMDISCRDW { w: self }
    }
    #[doc = "Bit 17 - FCS remove"]
    #[inline]
    pub fn fcsremove(&mut self) -> _FCSREMOVEW {
        _FCSREMOVEW { w: self }
    }
    #[doc = "Bits 18:20 - MDC clock division"]
    #[inline]
    pub fn mdcclkdiv(&mut self) -> _MDCCLKDIVW {
        _MDCCLKDIVW { w: self }
    }
    #[doc = "Bit 23 - Disable copy of pause frames"]
    #[inline]
    pub fn discopyofpframes(&mut self) -> _DISCOPYOFPFRAMESW {
        _DISCOPYOFPFRAMESW { w: self }
    }
    #[doc = "Bit 24 - Receive checksum offload enable"]
    #[inline]
    pub fn rxchksumoffloaden(&mut self) -> _RXCHKSUMOFFLOADENW {
        _RXCHKSUMOFFLOADENW { w: self }
    }
    #[doc = "Bit 25 - Enable frames to be received in half-duplex mode while transmitting."]
    #[inline]
    pub fn enhalfduplexrx(&mut self) -> _ENHALFDUPLEXRXW {
        _ENHALFDUPLEXRXW { w: self }
    }
    #[doc = "Bit 26 - Ignore RX FCS"]
    #[inline]
    pub fn ignorerxfcs(&mut self) -> _IGNORERXFCSW {
        _IGNORERXFCSW { w: self }
    }
    #[doc = "Bit 28 - IPG stretch enable"]
    #[inline]
    pub fn ipgstrtchen(&mut self) -> _IPGSTRTCHENW {
        _IPGSTRTCHENW { w: self }
    }
    #[doc = "Bit 29 - Receive bad preamble."]
    #[inline]
    pub fn nspchange(&mut self) -> _NSPCHANGEW {
        _NSPCHANGEW { w: self }
    }
    #[doc = "Bit 30 - Ignore IPG rx_er."]
    #[inline]
    pub fn ignoreipgrxer(&mut self) -> _IGNOREIPGRXERW {
        _IGNOREIPGRXERW { w: self }
    }
}
