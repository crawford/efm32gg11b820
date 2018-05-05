#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DEVINSTRWRCONFIG {
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
pub struct WROPCODER {
    bits: u8,
}
impl WROPCODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WELDISR {
    bits: bool,
}
impl WELDISR {
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
pub struct ADDRXFERTYPESTDMODER {
    bits: u8,
}
impl ADDRXFERTYPESTDMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DATAXFERTYPEEXTMODER {
    bits: u8,
}
impl DATAXFERTYPEEXTMODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DUMMYWRCLKCYCLESR {
    bits: u8,
}
impl DUMMYWRCLKCYCLESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _WROPCODEW<'a> {
    w: &'a mut W,
}
impl<'a> _WROPCODEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WELDISW<'a> {
    w: &'a mut W,
}
impl<'a> _WELDISW<'a> {
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
pub struct _ADDRXFERTYPESTDMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _ADDRXFERTYPESTDMODEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DATAXFERTYPEEXTMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _DATAXFERTYPEEXTMODEW<'a> {
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
#[doc = r" Proxy"]
pub struct _DUMMYWRCLKCYCLESW<'a> {
    w: &'a mut W,
}
impl<'a> _DUMMYWRCLKCYCLESW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 24;
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
    #[doc = "Bits 0:7 - Write Opcode"]
    #[inline]
    pub fn wropcode(&self) -> WROPCODER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WROPCODER { bits }
    }
    #[doc = "Bit 8 - WEL Disable"]
    #[inline]
    pub fn weldis(&self) -> WELDISR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        WELDISR { bits }
    }
    #[doc = "Bits 12:13 - Address Transfer Type for Standard SPI Modes"]
    #[inline]
    pub fn addrxfertypestdmode(&self) -> ADDRXFERTYPESTDMODER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ADDRXFERTYPESTDMODER { bits }
    }
    #[doc = "Bits 16:17 - Data Transfer Type for Standard SPI Modes"]
    #[inline]
    pub fn dataxfertypeextmode(&self) -> DATAXFERTYPEEXTMODER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DATAXFERTYPEEXTMODER { bits }
    }
    #[doc = "Bits 24:28 - Dummy Write Clock Cycles"]
    #[inline]
    pub fn dummywrclkcycles(&self) -> DUMMYWRCLKCYCLESR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DUMMYWRCLKCYCLESR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 2 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Write Opcode"]
    #[inline]
    pub fn wropcode(&mut self) -> _WROPCODEW {
        _WROPCODEW { w: self }
    }
    #[doc = "Bit 8 - WEL Disable"]
    #[inline]
    pub fn weldis(&mut self) -> _WELDISW {
        _WELDISW { w: self }
    }
    #[doc = "Bits 12:13 - Address Transfer Type for Standard SPI Modes"]
    #[inline]
    pub fn addrxfertypestdmode(&mut self) -> _ADDRXFERTYPESTDMODEW {
        _ADDRXFERTYPESTDMODEW { w: self }
    }
    #[doc = "Bits 16:17 - Data Transfer Type for Standard SPI Modes"]
    #[inline]
    pub fn dataxfertypeextmode(&mut self) -> _DATAXFERTYPEEXTMODEW {
        _DATAXFERTYPEEXTMODEW { w: self }
    }
    #[doc = "Bits 24:28 - Dummy Write Clock Cycles"]
    #[inline]
    pub fn dummywrclkcycles(&mut self) -> _DUMMYWRCLKCYCLESW {
        _DUMMYWRCLKCYCLESW { w: self }
    }
}
