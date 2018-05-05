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
pub struct EM2BLOCKR {
    bits: bool,
}
impl EM2BLOCKR {
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
pub struct EM2BODDISR {
    bits: bool,
}
impl EM2BODDISR {
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
pub struct EM01LDR {
    bits: bool,
}
impl EM01LDR {
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
pub struct EM23VSCALEAUTOWSENR {
    bits: bool,
}
impl EM23VSCALEAUTOWSENR {
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
#[doc = "Possible values of the field `EM23VSCALE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EM23VSCALER {
    #[doc = "Voltage Scale Level 2"]
    VSCALE2,
    #[doc = "Voltage Scale Level 0"]
    VSCALE0,
    #[doc = "RESV"]
    RESV,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EM23VSCALER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EM23VSCALER::VSCALE2 => 0,
            EM23VSCALER::VSCALE0 => 2,
            EM23VSCALER::RESV => 3,
            EM23VSCALER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EM23VSCALER {
        match value {
            0 => EM23VSCALER::VSCALE2,
            2 => EM23VSCALER::VSCALE0,
            3 => EM23VSCALER::RESV,
            i => EM23VSCALER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VSCALE2`"]
    #[inline]
    pub fn is_vscale2(&self) -> bool {
        *self == EM23VSCALER::VSCALE2
    }
    #[doc = "Checks if the value of the field is `VSCALE0`"]
    #[inline]
    pub fn is_vscale0(&self) -> bool {
        *self == EM23VSCALER::VSCALE0
    }
    #[doc = "Checks if the value of the field is `RESV`"]
    #[inline]
    pub fn is_resv(&self) -> bool {
        *self == EM23VSCALER::RESV
    }
}
#[doc = "Possible values of the field `EM4HVSCALE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EM4HVSCALER {
    #[doc = "Voltage Scale Level 2"]
    VSCALE2,
    #[doc = "Voltage Scale Level 0"]
    VSCALE0,
    #[doc = "RESV"]
    RESV,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl EM4HVSCALER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            EM4HVSCALER::VSCALE2 => 0,
            EM4HVSCALER::VSCALE0 => 2,
            EM4HVSCALER::RESV => 3,
            EM4HVSCALER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> EM4HVSCALER {
        match value {
            0 => EM4HVSCALER::VSCALE2,
            2 => EM4HVSCALER::VSCALE0,
            3 => EM4HVSCALER::RESV,
            i => EM4HVSCALER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `VSCALE2`"]
    #[inline]
    pub fn is_vscale2(&self) -> bool {
        *self == EM4HVSCALER::VSCALE2
    }
    #[doc = "Checks if the value of the field is `VSCALE0`"]
    #[inline]
    pub fn is_vscale0(&self) -> bool {
        *self == EM4HVSCALER::VSCALE0
    }
    #[doc = "Checks if the value of the field is `RESV`"]
    #[inline]
    pub fn is_resv(&self) -> bool {
        *self == EM4HVSCALER::RESV
    }
}
#[doc = r" Proxy"]
pub struct _EM2BLOCKW<'a> {
    w: &'a mut W,
}
impl<'a> _EM2BLOCKW<'a> {
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
pub struct _EM2BODDISW<'a> {
    w: &'a mut W,
}
impl<'a> _EM2BODDISW<'a> {
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
pub struct _EM01LDW<'a> {
    w: &'a mut W,
}
impl<'a> _EM01LDW<'a> {
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
pub struct _EM23VSCALEAUTOWSENW<'a> {
    w: &'a mut W,
}
impl<'a> _EM23VSCALEAUTOWSENW<'a> {
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
#[doc = "Values that can be written to the field `EM23VSCALE`"]
pub enum EM23VSCALEW {
    #[doc = "Voltage Scale Level 2"]
    VSCALE2,
    #[doc = "Voltage Scale Level 0"]
    VSCALE0,
    #[doc = "RESV"]
    RESV,
}
impl EM23VSCALEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EM23VSCALEW::VSCALE2 => 0,
            EM23VSCALEW::VSCALE0 => 2,
            EM23VSCALEW::RESV => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EM23VSCALEW<'a> {
    w: &'a mut W,
}
impl<'a> _EM23VSCALEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EM23VSCALEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Voltage Scale Level 2"]
    #[inline]
    pub fn vscale2(self) -> &'a mut W {
        self.variant(EM23VSCALEW::VSCALE2)
    }
    #[doc = "Voltage Scale Level 0"]
    #[inline]
    pub fn vscale0(self) -> &'a mut W {
        self.variant(EM23VSCALEW::VSCALE0)
    }
    #[doc = "RESV"]
    #[inline]
    pub fn resv(self) -> &'a mut W {
        self.variant(EM23VSCALEW::RESV)
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
#[doc = "Values that can be written to the field `EM4HVSCALE`"]
pub enum EM4HVSCALEW {
    #[doc = "Voltage Scale Level 2"]
    VSCALE2,
    #[doc = "Voltage Scale Level 0"]
    VSCALE0,
    #[doc = "RESV"]
    RESV,
}
impl EM4HVSCALEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            EM4HVSCALEW::VSCALE2 => 0,
            EM4HVSCALEW::VSCALE0 => 2,
            EM4HVSCALEW::RESV => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _EM4HVSCALEW<'a> {
    w: &'a mut W,
}
impl<'a> _EM4HVSCALEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: EM4HVSCALEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Voltage Scale Level 2"]
    #[inline]
    pub fn vscale2(self) -> &'a mut W {
        self.variant(EM4HVSCALEW::VSCALE2)
    }
    #[doc = "Voltage Scale Level 0"]
    #[inline]
    pub fn vscale0(self) -> &'a mut W {
        self.variant(EM4HVSCALEW::VSCALE0)
    }
    #[doc = "RESV"]
    #[inline]
    pub fn resv(self) -> &'a mut W {
        self.variant(EM4HVSCALEW::RESV)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
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
    #[doc = "Bit 1 - Energy Mode 2 Block"]
    #[inline]
    pub fn em2block(&self) -> EM2BLOCKR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EM2BLOCKR { bits }
    }
    #[doc = "Bit 2 - Disable BOD in EM2"]
    #[inline]
    pub fn em2boddis(&self) -> EM2BODDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EM2BODDISR { bits }
    }
    #[doc = "Bit 3 - Reserved for internal use. Do not change."]
    #[inline]
    pub fn em01ld(&self) -> EM01LDR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EM01LDR { bits }
    }
    #[doc = "Bit 4 - Automatically Configures Flash, Ram and Frequency to Wakeup From EM2 or EM3 at Low Voltage"]
    #[inline]
    pub fn em23vscaleautowsen(&self) -> EM23VSCALEAUTOWSENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        EM23VSCALEAUTOWSENR { bits }
    }
    #[doc = "Bits 8:9 - EM23 Voltage Scale"]
    #[inline]
    pub fn em23vscale(&self) -> EM23VSCALER {
        EM23VSCALER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:17 - EM4H Voltage Scale"]
    #[inline]
    pub fn em4hvscale(&self) -> EM4HVSCALER {
        EM4HVSCALER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
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
    #[doc = "Bit 1 - Energy Mode 2 Block"]
    #[inline]
    pub fn em2block(&mut self) -> _EM2BLOCKW {
        _EM2BLOCKW { w: self }
    }
    #[doc = "Bit 2 - Disable BOD in EM2"]
    #[inline]
    pub fn em2boddis(&mut self) -> _EM2BODDISW {
        _EM2BODDISW { w: self }
    }
    #[doc = "Bit 3 - Reserved for internal use. Do not change."]
    #[inline]
    pub fn em01ld(&mut self) -> _EM01LDW {
        _EM01LDW { w: self }
    }
    #[doc = "Bit 4 - Automatically Configures Flash, Ram and Frequency to Wakeup From EM2 or EM3 at Low Voltage"]
    #[inline]
    pub fn em23vscaleautowsen(&mut self) -> _EM23VSCALEAUTOWSENW {
        _EM23VSCALEAUTOWSENW { w: self }
    }
    #[doc = "Bits 8:9 - EM23 Voltage Scale"]
    #[inline]
    pub fn em23vscale(&mut self) -> _EM23VSCALEW {
        _EM23VSCALEW { w: self }
    }
    #[doc = "Bits 16:17 - EM4H Voltage Scale"]
    #[inline]
    pub fn em4hvscale(&mut self) -> _EM4HVSCALEW {
        _EM4HVSCALEW { w: self }
    }
}
