#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
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
        R { bits: self.register.get() }
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
pub struct VBUSENAPR {
    bits: bool,
}
impl VBUSENAPR {
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
pub struct SELFPOWEREDR {
    bits: bool,
}
impl SELFPOWEREDR {
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
#[doc = "Possible values of the field `LEMOSCCTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LEMOSCCTRLR {
    #[doc = "Low Energy Mode has no effect on neither USBC or USHFRCO."]
    NONE,
    #[doc = "The USBC clock is gated when Low Energy Mode is active."]
    GATE,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl LEMOSCCTRLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            LEMOSCCTRLR::NONE => 0,
            LEMOSCCTRLR::GATE => 1,
            LEMOSCCTRLR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> LEMOSCCTRLR {
        match value {
            0 => LEMOSCCTRLR::NONE,
            1 => LEMOSCCTRLR::GATE,
            i => LEMOSCCTRLR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline]
    pub fn is_none(&self) -> bool {
        *self == LEMOSCCTRLR::NONE
    }
    #[doc = "Checks if the value of the field is `GATE`"]
    #[inline]
    pub fn is_gate(&self) -> bool {
        *self == LEMOSCCTRLR::GATE
    }
}
#[doc = r" Value of the field"]
pub struct LEMPHYCTRLR {
    bits: bool,
}
impl LEMPHYCTRLR {
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
pub struct LEMIDLEENR {
    bits: bool,
}
impl LEMIDLEENR {
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
pub struct IDCDENR {
    bits: bool,
}
impl IDCDENR {
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
pub struct OTGCLKCDISR {
    bits: bool,
}
impl OTGCLKCDISR {
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
pub struct OTGIDINDISR {
    bits: bool,
}
impl OTGIDINDISR {
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
pub struct OTGPHYCTRLDISR {
    bits: bool,
}
impl OTGPHYCTRLDISR {
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
#[doc = "Possible values of the field `DCDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCDENR {
    #[doc = "DCD is disabled."]
    DISABLED,
    #[doc = "Only DCD timeout will be initiated."]
    TIMEOUT,
    #[doc = "Full DCD operation (physical contact and timeout) will be initiated."]
    ENABLED,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DCDENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DCDENR::DISABLED => 0,
            DCDENR::TIMEOUT => 2,
            DCDENR::ENABLED => 3,
            DCDENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DCDENR {
        match value {
            0 => DCDENR::DISABLED,
            2 => DCDENR::TIMEOUT,
            3 => DCDENR::ENABLED,
            i => DCDENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline]
    pub fn is_disabled(&self) -> bool {
        *self == DCDENR::DISABLED
    }
    #[doc = "Checks if the value of the field is `TIMEOUT`"]
    #[inline]
    pub fn is_timeout(&self) -> bool {
        *self == DCDENR::TIMEOUT
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline]
    pub fn is_enabled(&self) -> bool {
        *self == DCDENR::ENABLED
    }
}
#[doc = r" Value of the field"]
pub struct PDENR {
    bits: bool,
}
impl PDENR {
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
pub struct SDENR {
    bits: bool,
}
impl SDENR {
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
pub struct _VBUSENAPW<'a> {
    w: &'a mut W,
}
impl<'a> _VBUSENAPW<'a> {
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
pub struct _SELFPOWEREDW<'a> {
    w: &'a mut W,
}
impl<'a> _SELFPOWEREDW<'a> {
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
#[doc = "Values that can be written to the field `LEMOSCCTRL`"]
pub enum LEMOSCCTRLW {
    #[doc = "Low Energy Mode has no effect on neither USBC or USHFRCO."]
    NONE,
    #[doc = "The USBC clock is gated when Low Energy Mode is active."]
    GATE,
}
impl LEMOSCCTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            LEMOSCCTRLW::NONE => 0,
            LEMOSCCTRLW::GATE => 1,
        }
    }
}
#[doc = r" Proxy"]
pub struct _LEMOSCCTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _LEMOSCCTRLW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: LEMOSCCTRLW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Low Energy Mode has no effect on neither USBC or USHFRCO."]
    #[inline]
    pub fn none(self) -> &'a mut W {
        self.variant(LEMOSCCTRLW::NONE)
    }
    #[doc = "The USBC clock is gated when Low Energy Mode is active."]
    #[inline]
    pub fn gate(self) -> &'a mut W {
        self.variant(LEMOSCCTRLW::GATE)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LEMPHYCTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _LEMPHYCTRLW<'a> {
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
pub struct _LEMIDLEENW<'a> {
    w: &'a mut W,
}
impl<'a> _LEMIDLEENW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _IDCDENW<'a> {
    w: &'a mut W,
}
impl<'a> _IDCDENW<'a> {
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
pub struct _OTGCLKCDISW<'a> {
    w: &'a mut W,
}
impl<'a> _OTGCLKCDISW<'a> {
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
pub struct _OTGIDINDISW<'a> {
    w: &'a mut W,
}
impl<'a> _OTGIDINDISW<'a> {
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
pub struct _OTGPHYCTRLDISW<'a> {
    w: &'a mut W,
}
impl<'a> _OTGPHYCTRLDISW<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DCDEN`"]
pub enum DCDENW {
    #[doc = "DCD is disabled."]
    DISABLED,
    #[doc = "Only DCD timeout will be initiated."]
    TIMEOUT,
    #[doc = "Full DCD operation (physical contact and timeout) will be initiated."]
    ENABLED,
}
impl DCDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DCDENW::DISABLED => 0,
            DCDENW::TIMEOUT => 2,
            DCDENW::ENABLED => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DCDENW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DCDENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "DCD is disabled."]
    #[inline]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DCDENW::DISABLED)
    }
    #[doc = "Only DCD timeout will be initiated."]
    #[inline]
    pub fn timeout(self) -> &'a mut W {
        self.variant(DCDENW::TIMEOUT)
    }
    #[doc = "Full DCD operation (physical contact and timeout) will be initiated."]
    #[inline]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DCDENW::ENABLED)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PDENW<'a> {
    w: &'a mut W,
}
impl<'a> _PDENW<'a> {
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
#[doc = r" Proxy"]
pub struct _SDENW<'a> {
    w: &'a mut W,
}
impl<'a> _SDENW<'a> {
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
        const OFFSET: u8 = 31;
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
    #[doc = "Bit 0 - VBUSEN Active Polarity"]
    #[inline]
    pub fn vbusenap(&self) -> VBUSENAPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        VBUSENAPR { bits }
    }
    #[doc = "Bit 3 - PHY Power"]
    #[inline]
    pub fn selfpowered(&self) -> SELFPOWEREDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SELFPOWEREDR { bits }
    }
    #[doc = "Bits 4:5 - Low Energy Mode Oscillator Control"]
    #[inline]
    pub fn lemoscctrl(&self) -> LEMOSCCTRLR {
        LEMOSCCTRLR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 7 - Low Energy Mode USB PHY Control"]
    #[inline]
    pub fn lemphyctrl(&self) -> LEMPHYCTRLR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LEMPHYCTRLR { bits }
    }
    #[doc = "Bit 9 - Low Energy Mode on Bus Idle Enable"]
    #[inline]
    pub fn lemidleen(&self) -> LEMIDLEENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LEMIDLEENR { bits }
    }
    #[doc = "Bit 12 - ID Pull-up Enable"]
    #[inline]
    pub fn idcden(&self) -> IDCDENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IDCDENR { bits }
    }
    #[doc = "Bit 25 - OTG CLKC Disable"]
    #[inline]
    pub fn otgclkcdis(&self) -> OTGCLKCDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 25;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OTGCLKCDISR { bits }
    }
    #[doc = "Bit 26 - OTG ID Input Disable"]
    #[inline]
    pub fn otgidindis(&self) -> OTGIDINDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OTGIDINDISR { bits }
    }
    #[doc = "Bit 27 - OTG Control Signals to PHY Disable"]
    #[inline]
    pub fn otgphyctrldis(&self) -> OTGPHYCTRLDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 27;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        OTGPHYCTRLDISR { bits }
    }
    #[doc = "Bits 28:29 - Data Contact Detection Enable"]
    #[inline]
    pub fn dcden(&self) -> DCDENR {
        DCDENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 30 - Primary Detection Enable"]
    #[inline]
    pub fn pden(&self) -> PDENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 30;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PDENR { bits }
    }
    #[doc = "Bit 31 - Secondary Detection Enable"]
    #[inline]
    pub fn sden(&self) -> SDENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 31;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SDENR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 32 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - VBUSEN Active Polarity"]
    #[inline]
    pub fn vbusenap(&mut self) -> _VBUSENAPW {
        _VBUSENAPW { w: self }
    }
    #[doc = "Bit 3 - PHY Power"]
    #[inline]
    pub fn selfpowered(&mut self) -> _SELFPOWEREDW {
        _SELFPOWEREDW { w: self }
    }
    #[doc = "Bits 4:5 - Low Energy Mode Oscillator Control"]
    #[inline]
    pub fn lemoscctrl(&mut self) -> _LEMOSCCTRLW {
        _LEMOSCCTRLW { w: self }
    }
    #[doc = "Bit 7 - Low Energy Mode USB PHY Control"]
    #[inline]
    pub fn lemphyctrl(&mut self) -> _LEMPHYCTRLW {
        _LEMPHYCTRLW { w: self }
    }
    #[doc = "Bit 9 - Low Energy Mode on Bus Idle Enable"]
    #[inline]
    pub fn lemidleen(&mut self) -> _LEMIDLEENW {
        _LEMIDLEENW { w: self }
    }
    #[doc = "Bit 12 - ID Pull-up Enable"]
    #[inline]
    pub fn idcden(&mut self) -> _IDCDENW {
        _IDCDENW { w: self }
    }
    #[doc = "Bit 25 - OTG CLKC Disable"]
    #[inline]
    pub fn otgclkcdis(&mut self) -> _OTGCLKCDISW {
        _OTGCLKCDISW { w: self }
    }
    #[doc = "Bit 26 - OTG ID Input Disable"]
    #[inline]
    pub fn otgidindis(&mut self) -> _OTGIDINDISW {
        _OTGIDINDISW { w: self }
    }
    #[doc = "Bit 27 - OTG Control Signals to PHY Disable"]
    #[inline]
    pub fn otgphyctrldis(&mut self) -> _OTGPHYCTRLDISW {
        _OTGPHYCTRLDISW { w: self }
    }
    #[doc = "Bits 28:29 - Data Contact Detection Enable"]
    #[inline]
    pub fn dcden(&mut self) -> _DCDENW {
        _DCDENW { w: self }
    }
    #[doc = "Bit 30 - Primary Detection Enable"]
    #[inline]
    pub fn pden(&mut self) -> _PDENW {
        _PDENW { w: self }
    }
    #[doc = "Bit 31 - Secondary Detection Enable"]
    #[inline]
    pub fn sden(&mut self) -> _SDENW {
        _SDENW { w: self }
    }
}
