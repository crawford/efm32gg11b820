#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::R5VCTRL {
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
pub struct BYPASSR {
    bits: bool,
}
impl BYPASSR {
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
pub struct EM4WUENR {
    bits: bool,
}
impl EM4WUENR {
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
pub struct IMONENR {
    bits: bool,
}
impl IMONENR {
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
#[doc = "Possible values of the field `INPUTMODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INPUTMODER {
    #[doc = "Regulator input supply switched automatically to the highest voltage of either VBUS or VREGI"]
    AUTO,
    #[doc = "Force VBUS pin as the regulator input"]
    VBUS,
    #[doc = "Force VREGI pin as the regulator input"]
    VREGI,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl INPUTMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            INPUTMODER::AUTO => 0,
            INPUTMODER::VBUS => 1,
            INPUTMODER::VREGI => 2,
            INPUTMODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> INPUTMODER {
        match value {
            0 => INPUTMODER::AUTO,
            1 => INPUTMODER::VBUS,
            2 => INPUTMODER::VREGI,
            i => INPUTMODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AUTO`"]
    #[inline]
    pub fn is_auto(&self) -> bool {
        *self == INPUTMODER::AUTO
    }
    #[doc = "Checks if the value of the field is `VBUS`"]
    #[inline]
    pub fn is_vbus(&self) -> bool {
        *self == INPUTMODER::VBUS
    }
    #[doc = "Checks if the value of the field is `VREGI`"]
    #[inline]
    pub fn is_vregi(&self) -> bool {
        *self == INPUTMODER::VREGI
    }
}
#[doc = r" Proxy"]
pub struct _BYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _BYPASSW<'a> {
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
pub struct _EM4WUENW<'a> {
    w: &'a mut W,
}
impl<'a> _EM4WUENW<'a> {
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
pub struct _IMONENW<'a> {
    w: &'a mut W,
}
impl<'a> _IMONENW<'a> {
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
#[doc = "Values that can be written to the field `INPUTMODE`"]
pub enum INPUTMODEW {
    #[doc = "Regulator input supply switched automatically to the highest voltage of either VBUS or VREGI"]
    AUTO,
    #[doc = "Force VBUS pin as the regulator input"]
    VBUS,
    #[doc = "Force VREGI pin as the regulator input"]
    VREGI,
}
impl INPUTMODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            INPUTMODEW::AUTO => 0,
            INPUTMODEW::VBUS => 1,
            INPUTMODEW::VREGI => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INPUTMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _INPUTMODEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INPUTMODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Regulator input supply switched automatically to the highest voltage of either VBUS or VREGI"]
    #[inline]
    pub fn auto(self) -> &'a mut W {
        self.variant(INPUTMODEW::AUTO)
    }
    #[doc = "Force VBUS pin as the regulator input"]
    #[inline]
    pub fn vbus(self) -> &'a mut W {
        self.variant(INPUTMODEW::VBUS)
    }
    #[doc = "Force VREGI pin as the regulator input"]
    #[inline]
    pub fn vregi(self) -> &'a mut W {
        self.variant(INPUTMODEW::VREGI)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
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
    #[doc = "Bit 0 - 5V Regulator Bypass"]
    #[inline]
    pub fn bypass(&self) -> BYPASSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BYPASSR { bits }
    }
    #[doc = "Bit 1 - Enable EM4 Wakeup Due to VBUS Detection"]
    #[inline]
    pub fn em4wuen(&self) -> EM4WUENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EM4WUENR { bits }
    }
    #[doc = "Bit 2 - Enable the Regulator Current Monitor for Selected Current Path to Either VREGI or VBUS"]
    #[inline]
    pub fn imonen(&self) -> IMONENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        IMONENR { bits }
    }
    #[doc = "Bits 8:9 - 5V Input Mode"]
    #[inline]
    pub fn inputmode(&self) -> INPUTMODER {
        INPUTMODER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
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
    #[doc = "Bit 0 - 5V Regulator Bypass"]
    #[inline]
    pub fn bypass(&mut self) -> _BYPASSW {
        _BYPASSW { w: self }
    }
    #[doc = "Bit 1 - Enable EM4 Wakeup Due to VBUS Detection"]
    #[inline]
    pub fn em4wuen(&mut self) -> _EM4WUENW {
        _EM4WUENW { w: self }
    }
    #[doc = "Bit 2 - Enable the Regulator Current Monitor for Selected Current Path to Either VREGI or VBUS"]
    #[inline]
    pub fn imonen(&mut self) -> _IMONENW {
        _IMONENW { w: self }
    }
    #[doc = "Bits 8:9 - 5V Input Mode"]
    #[inline]
    pub fn inputmode(&mut self) -> _INPUTMODEW {
        _INPUTMODEW { w: self }
    }
}
