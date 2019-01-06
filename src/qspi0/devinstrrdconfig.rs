#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DEVINSTRRDCONFIG {
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
pub struct RDOPCODENONXIPR {
    bits: u8,
}
impl RDOPCODENONXIPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct INSTRTYPER {
    bits: u8,
}
impl INSTRTYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct DDRENR {
    bits: bool,
}
impl DDRENR {
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
pub struct MODEBITENABLER {
    bits: bool,
}
impl MODEBITENABLER {
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
pub struct DUMMYRDCLKCYCLESR {
    bits: u8,
}
impl DUMMYRDCLKCYCLESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _RDOPCODENONXIPW<'a> {
    w: &'a mut W,
}
impl<'a> _RDOPCODENONXIPW<'a> {
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
pub struct _INSTRTYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _INSTRTYPEW<'a> {
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
#[doc = r" Proxy"]
pub struct _DDRENW<'a> {
    w: &'a mut W,
}
impl<'a> _DDRENW<'a> {
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
        const OFFSET: u8 = 10;
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
pub struct _MODEBITENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _MODEBITENABLEW<'a> {
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
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DUMMYRDCLKCYCLESW<'a> {
    w: &'a mut W,
}
impl<'a> _DUMMYRDCLKCYCLESW<'a> {
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
    #[doc = "Bits 0:7 - Read Opcode in Non-XIP Mode"]
    #[inline]
    pub fn rdopcodenonxip(&self) -> RDOPCODENONXIPR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        RDOPCODENONXIPR { bits }
    }
    #[doc = "Bits 8:9 - Instruction Type"]
    #[inline]
    pub fn instrtype(&self) -> INSTRTYPER {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        INSTRTYPER { bits }
    }
    #[doc = "Bit 10 - DDR Enable"]
    #[inline]
    pub fn ddren(&self) -> DDRENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DDRENR { bits }
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
    #[doc = "Bit 20 - Mode Bit Enable"]
    #[inline]
    pub fn modebitenable(&self) -> MODEBITENABLER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MODEBITENABLER { bits }
    }
    #[doc = "Bits 24:28 - Dummy Read Clock Cycles"]
    #[inline]
    pub fn dummyrdclkcycles(&self) -> DUMMYRDCLKCYCLESR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DUMMYRDCLKCYCLESR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 3 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Read Opcode in Non-XIP Mode"]
    #[inline]
    pub fn rdopcodenonxip(&mut self) -> _RDOPCODENONXIPW {
        _RDOPCODENONXIPW { w: self }
    }
    #[doc = "Bits 8:9 - Instruction Type"]
    #[inline]
    pub fn instrtype(&mut self) -> _INSTRTYPEW {
        _INSTRTYPEW { w: self }
    }
    #[doc = "Bit 10 - DDR Enable"]
    #[inline]
    pub fn ddren(&mut self) -> _DDRENW {
        _DDRENW { w: self }
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
    #[doc = "Bit 20 - Mode Bit Enable"]
    #[inline]
    pub fn modebitenable(&mut self) -> _MODEBITENABLEW {
        _MODEBITENABLEW { w: self }
    }
    #[doc = "Bits 24:28 - Dummy Read Clock Cycles"]
    #[inline]
    pub fn dummyrdclkcycles(&mut self) -> _DUMMYRDCLKCYCLESW {
        _DUMMYRDCLKCYCLESW { w: self }
    }
}
