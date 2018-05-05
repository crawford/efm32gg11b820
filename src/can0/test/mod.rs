#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TEST {
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
pub struct BASICR {
    bits: bool,
}
impl BASICR {
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
pub struct SILENTR {
    bits: bool,
}
impl SILENTR {
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
pub struct LBACKR {
    bits: bool,
}
impl LBACKR {
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
#[doc = "Possible values of the field `TX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TXR {
    #[doc = "Reset value, CAN_TX is controlled by the CAN Core."]
    CORE,
    #[doc = "Sample Point can be monitored at CAN_TX pin."]
    SAMPT,
    #[doc = "CAN_TX pin drives a dominant bit (0) value."]
    LOW,
    #[doc = "CAN_TX pin drives a recessive bit (1) value."]
    HIGH,
}
impl TXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            TXR::CORE => 0,
            TXR::SAMPT => 1,
            TXR::LOW => 2,
            TXR::HIGH => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> TXR {
        match value {
            0 => TXR::CORE,
            1 => TXR::SAMPT,
            2 => TXR::LOW,
            3 => TXR::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CORE`"]
    #[inline]
    pub fn is_core(&self) -> bool {
        *self == TXR::CORE
    }
    #[doc = "Checks if the value of the field is `SAMPT`"]
    #[inline]
    pub fn is_sampt(&self) -> bool {
        *self == TXR::SAMPT
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline]
    pub fn is_low(&self) -> bool {
        *self == TXR::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline]
    pub fn is_high(&self) -> bool {
        *self == TXR::HIGH
    }
}
#[doc = r" Value of the field"]
pub struct RXR {
    bits: bool,
}
impl RXR {
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
pub struct _BASICW<'a> {
    w: &'a mut W,
}
impl<'a> _BASICW<'a> {
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
pub struct _SILENTW<'a> {
    w: &'a mut W,
}
impl<'a> _SILENTW<'a> {
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
pub struct _LBACKW<'a> {
    w: &'a mut W,
}
impl<'a> _LBACKW<'a> {
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
#[doc = "Values that can be written to the field `TX`"]
pub enum TXW {
    #[doc = "Reset value, CAN_TX is controlled by the CAN Core."]
    CORE,
    #[doc = "Sample Point can be monitored at CAN_TX pin."]
    SAMPT,
    #[doc = "CAN_TX pin drives a dominant bit (0) value."]
    LOW,
    #[doc = "CAN_TX pin drives a recessive bit (1) value."]
    HIGH,
}
impl TXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TXW::CORE => 0,
            TXW::SAMPT => 1,
            TXW::LOW => 2,
            TXW::HIGH => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TXW<'a> {
    w: &'a mut W,
}
impl<'a> _TXW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TXW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Reset value, CAN_TX is controlled by the CAN Core."]
    #[inline]
    pub fn core(self) -> &'a mut W {
        self.variant(TXW::CORE)
    }
    #[doc = "Sample Point can be monitored at CAN_TX pin."]
    #[inline]
    pub fn sampt(self) -> &'a mut W {
        self.variant(TXW::SAMPT)
    }
    #[doc = "CAN_TX pin drives a dominant bit (0) value."]
    #[inline]
    pub fn low(self) -> &'a mut W {
        self.variant(TXW::LOW)
    }
    #[doc = "CAN_TX pin drives a recessive bit (1) value."]
    #[inline]
    pub fn high(self) -> &'a mut W {
        self.variant(TXW::HIGH)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 5;
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
    #[doc = "Bit 2 - Basic Mode"]
    #[inline]
    pub fn basic(&self) -> BASICR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BASICR { bits }
    }
    #[doc = "Bit 3 - Silent Mode"]
    #[inline]
    pub fn silent(&self) -> SILENTR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SILENTR { bits }
    }
    #[doc = "Bit 4 - Loopback Mode"]
    #[inline]
    pub fn lback(&self) -> LBACKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LBACKR { bits }
    }
    #[doc = "Bits 5:6 - Control of CAN_TX Pin"]
    #[inline]
    pub fn tx(&self) -> TXR {
        TXR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Monitors the Actual Value of CAN_RX Pin"]
    #[inline]
    pub fn rx(&self) -> RXR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        RXR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 2 - Basic Mode"]
    #[inline]
    pub fn basic(&mut self) -> _BASICW {
        _BASICW { w: self }
    }
    #[doc = "Bit 3 - Silent Mode"]
    #[inline]
    pub fn silent(&mut self) -> _SILENTW {
        _SILENTW { w: self }
    }
    #[doc = "Bit 4 - Loopback Mode"]
    #[inline]
    pub fn lback(&mut self) -> _LBACKW {
        _LBACKW { w: self }
    }
    #[doc = "Bits 5:6 - Control of CAN_TX Pin"]
    #[inline]
    pub fn tx(&mut self) -> _TXW {
        _TXW { w: self }
    }
}
