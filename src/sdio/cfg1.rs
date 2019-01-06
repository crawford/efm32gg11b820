#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG1 {
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
pub struct ASYNCINTRSUPR {
    bits: bool,
}
impl ASYNCINTRSUPR {
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
#[doc = "Possible values of the field `SLOTTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLOTTYPER {
    #[doc = "Removable SD Card Slot"]
    RMSDSLOT,
    #[doc = "Embedded SD Card Slot"]
    EMSDSLOT,
    #[doc = "Shared SD Card Slot"]
    SHBUSSLOT,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SLOTTYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SLOTTYPER::RMSDSLOT => 0,
            SLOTTYPER::EMSDSLOT => 1,
            SLOTTYPER::SHBUSSLOT => 2,
            SLOTTYPER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SLOTTYPER {
        match value {
            0 => SLOTTYPER::RMSDSLOT,
            1 => SLOTTYPER::EMSDSLOT,
            2 => SLOTTYPER::SHBUSSLOT,
            i => SLOTTYPER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `RMSDSLOT`"]
    #[inline]
    pub fn is_rmsdslot(&self) -> bool {
        *self == SLOTTYPER::RMSDSLOT
    }
    #[doc = "Checks if the value of the field is `EMSDSLOT`"]
    #[inline]
    pub fn is_emsdslot(&self) -> bool {
        *self == SLOTTYPER::EMSDSLOT
    }
    #[doc = "Checks if the value of the field is `SHBUSSLOT`"]
    #[inline]
    pub fn is_shbusslot(&self) -> bool {
        *self == SLOTTYPER::SHBUSSLOT
    }
}
#[doc = r" Value of the field"]
pub struct CSDR50SUPR {
    bits: bool,
}
impl CSDR50SUPR {
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
pub struct CSDR104SUPR {
    bits: bool,
}
impl CSDR104SUPR {
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
pub struct CDDR50SUPR {
    bits: bool,
}
impl CDDR50SUPR {
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
pub struct CDRVASUPR {
    bits: bool,
}
impl CDRVASUPR {
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
pub struct CDRVCSUPR {
    bits: bool,
}
impl CDRVCSUPR {
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
pub struct CDRVDSUPR {
    bits: bool,
}
impl CDRVDSUPR {
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
pub struct RETUNTMRCTLR {
    bits: u8,
}
impl RETUNTMRCTLR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TUNSDR50R {
    bits: bool,
}
impl TUNSDR50R {
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
pub struct RETUNMODESR {
    bits: u8,
}
impl RETUNMODESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct SPISUPR {
    bits: bool,
}
impl SPISUPR {
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
pub struct ASYNCWKUPENR {
    bits: bool,
}
impl ASYNCWKUPENR {
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
pub struct _ASYNCINTRSUPW<'a> {
    w: &'a mut W,
}
impl<'a> _ASYNCINTRSUPW<'a> {
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
#[doc = "Values that can be written to the field `SLOTTYPE`"]
pub enum SLOTTYPEW {
    #[doc = "Removable SD Card Slot"]
    RMSDSLOT,
    #[doc = "Embedded SD Card Slot"]
    EMSDSLOT,
    #[doc = "Shared SD Card Slot"]
    SHBUSSLOT,
}
impl SLOTTYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SLOTTYPEW::RMSDSLOT => 0,
            SLOTTYPEW::EMSDSLOT => 1,
            SLOTTYPEW::SHBUSSLOT => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SLOTTYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _SLOTTYPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SLOTTYPEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Removable SD Card Slot"]
    #[inline]
    pub fn rmsdslot(self) -> &'a mut W {
        self.variant(SLOTTYPEW::RMSDSLOT)
    }
    #[doc = "Embedded SD Card Slot"]
    #[inline]
    pub fn emsdslot(self) -> &'a mut W {
        self.variant(SLOTTYPEW::EMSDSLOT)
    }
    #[doc = "Shared SD Card Slot"]
    #[inline]
    pub fn shbusslot(self) -> &'a mut W {
        self.variant(SLOTTYPEW::SHBUSSLOT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CSDR50SUPW<'a> {
    w: &'a mut W,
}
impl<'a> _CSDR50SUPW<'a> {
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
pub struct _CSDR104SUPW<'a> {
    w: &'a mut W,
}
impl<'a> _CSDR104SUPW<'a> {
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
pub struct _CDDR50SUPW<'a> {
    w: &'a mut W,
}
impl<'a> _CDDR50SUPW<'a> {
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
pub struct _CDRVASUPW<'a> {
    w: &'a mut W,
}
impl<'a> _CDRVASUPW<'a> {
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
pub struct _CDRVCSUPW<'a> {
    w: &'a mut W,
}
impl<'a> _CDRVCSUPW<'a> {
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
pub struct _CDRVDSUPW<'a> {
    w: &'a mut W,
}
impl<'a> _CDRVDSUPW<'a> {
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
pub struct _RETUNTMRCTLW<'a> {
    w: &'a mut W,
}
impl<'a> _RETUNTMRCTLW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TUNSDR50W<'a> {
    w: &'a mut W,
}
impl<'a> _TUNSDR50W<'a> {
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
pub struct _RETUNMODESW<'a> {
    w: &'a mut W,
}
impl<'a> _RETUNMODESW<'a> {
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
pub struct _SPISUPW<'a> {
    w: &'a mut W,
}
impl<'a> _SPISUPW<'a> {
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
pub struct _ASYNCWKUPENW<'a> {
    w: &'a mut W,
}
impl<'a> _ASYNCWKUPENW<'a> {
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
        const OFFSET: u8 = 18;
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
    #[doc = "Bit 0 - Asynchronous Interrupt Support"]
    #[inline]
    pub fn asyncintrsup(&self) -> ASYNCINTRSUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ASYNCINTRSUPR { bits }
    }
    #[doc = "Bits 1:2 - Slot Type"]
    #[inline]
    pub fn slottype(&self) -> SLOTTYPER {
        SLOTTYPER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 3 - Core Support SDR50"]
    #[inline]
    pub fn csdr50sup(&self) -> CSDR50SUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CSDR50SUPR { bits }
    }
    #[doc = "Bit 4 - Support SDR104"]
    #[inline]
    pub fn csdr104sup(&self) -> CSDR104SUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CSDR104SUPR { bits }
    }
    #[doc = "Bit 5 - Support DDR50"]
    #[inline]
    pub fn cddr50sup(&self) -> CDDR50SUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CDDR50SUPR { bits }
    }
    #[doc = "Bit 6 - Support Type a Driver"]
    #[inline]
    pub fn cdrvasup(&self) -> CDRVASUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CDRVASUPR { bits }
    }
    #[doc = "Bit 7 - Support Type C Driver"]
    #[inline]
    pub fn cdrvcsup(&self) -> CDRVCSUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CDRVCSUPR { bits }
    }
    #[doc = "Bit 8 - Support Type D Driver"]
    #[inline]
    pub fn cdrvdsup(&self) -> CDRVDSUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CDRVDSUPR { bits }
    }
    #[doc = "Bits 9:12 - Retuning Timer Control"]
    #[inline]
    pub fn retuntmrctl(&self) -> RETUNTMRCTLR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 9;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RETUNTMRCTLR { bits }
    }
    #[doc = "Bit 13 - Tuning for SDR50"]
    #[inline]
    pub fn tunsdr50(&self) -> TUNSDR50R {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        TUNSDR50R { bits }
    }
    #[doc = "Bits 14:15 - Retuning Modes"]
    #[inline]
    pub fn retunmodes(&self) -> RETUNMODESR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RETUNMODESR { bits }
    }
    #[doc = "Bit 16 - SPI Support"]
    #[inline]
    pub fn spisup(&self) -> SPISUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        SPISUPR { bits }
    }
    #[doc = "Bit 18 - Asynchronous Wakeup Enable"]
    #[inline]
    pub fn asyncwkupen(&self) -> ASYNCWKUPENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ASYNCWKUPENR { bits }
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
    #[doc = "Bit 0 - Asynchronous Interrupt Support"]
    #[inline]
    pub fn asyncintrsup(&mut self) -> _ASYNCINTRSUPW {
        _ASYNCINTRSUPW { w: self }
    }
    #[doc = "Bits 1:2 - Slot Type"]
    #[inline]
    pub fn slottype(&mut self) -> _SLOTTYPEW {
        _SLOTTYPEW { w: self }
    }
    #[doc = "Bit 3 - Core Support SDR50"]
    #[inline]
    pub fn csdr50sup(&mut self) -> _CSDR50SUPW {
        _CSDR50SUPW { w: self }
    }
    #[doc = "Bit 4 - Support SDR104"]
    #[inline]
    pub fn csdr104sup(&mut self) -> _CSDR104SUPW {
        _CSDR104SUPW { w: self }
    }
    #[doc = "Bit 5 - Support DDR50"]
    #[inline]
    pub fn cddr50sup(&mut self) -> _CDDR50SUPW {
        _CDDR50SUPW { w: self }
    }
    #[doc = "Bit 6 - Support Type a Driver"]
    #[inline]
    pub fn cdrvasup(&mut self) -> _CDRVASUPW {
        _CDRVASUPW { w: self }
    }
    #[doc = "Bit 7 - Support Type C Driver"]
    #[inline]
    pub fn cdrvcsup(&mut self) -> _CDRVCSUPW {
        _CDRVCSUPW { w: self }
    }
    #[doc = "Bit 8 - Support Type D Driver"]
    #[inline]
    pub fn cdrvdsup(&mut self) -> _CDRVDSUPW {
        _CDRVDSUPW { w: self }
    }
    #[doc = "Bits 9:12 - Retuning Timer Control"]
    #[inline]
    pub fn retuntmrctl(&mut self) -> _RETUNTMRCTLW {
        _RETUNTMRCTLW { w: self }
    }
    #[doc = "Bit 13 - Tuning for SDR50"]
    #[inline]
    pub fn tunsdr50(&mut self) -> _TUNSDR50W {
        _TUNSDR50W { w: self }
    }
    #[doc = "Bits 14:15 - Retuning Modes"]
    #[inline]
    pub fn retunmodes(&mut self) -> _RETUNMODESW {
        _RETUNMODESW { w: self }
    }
    #[doc = "Bit 16 - SPI Support"]
    #[inline]
    pub fn spisup(&mut self) -> _SPISUPW {
        _SPISUPW { w: self }
    }
    #[doc = "Bit 18 - Asynchronous Wakeup Enable"]
    #[inline]
    pub fn asyncwkupen(&mut self) -> _ASYNCWKUPENW {
        _ASYNCWKUPENW { w: self }
    }
}
