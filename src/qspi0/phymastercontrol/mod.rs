#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PHYMASTERCONTROL {
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
pub struct PHYMASTERINITIALDELAYR {
    bits: u8,
}
impl PHYMASTERINITIALDELAYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PHYMASTERNBINDICATIONSR {
    bits: u8,
}
impl PHYMASTERNBINDICATIONSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PHYMASTERPHASEDETECTSELECTORR {
    bits: u8,
}
impl PHYMASTERPHASEDETECTSELECTORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct PHYMASTERBYPASSMODER {
    bits: bool,
}
impl PHYMASTERBYPASSMODER {
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
pub struct PHYMASTERLOCKMODER {
    bits: bool,
}
impl PHYMASTERLOCKMODER {
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
pub struct _PHYMASTERINITIALDELAYW<'a> {
    w: &'a mut W,
}
impl<'a> _PHYMASTERINITIALDELAYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PHYMASTERNBINDICATIONSW<'a> {
    w: &'a mut W,
}
impl<'a> _PHYMASTERNBINDICATIONSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PHYMASTERPHASEDETECTSELECTORW<'a> {
    w: &'a mut W,
}
impl<'a> _PHYMASTERPHASEDETECTSELECTORW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PHYMASTERBYPASSMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _PHYMASTERBYPASSMODEW<'a> {
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
pub struct _PHYMASTERLOCKMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _PHYMASTERLOCKMODEW<'a> {
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
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:6 - PHY Master Initial Delay"]
    #[inline]
    pub fn phymasterinitialdelay(&self) -> PHYMASTERINITIALDELAYR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PHYMASTERINITIALDELAYR { bits }
    }
    #[doc = "Bits 16:18 - Holds the Number of Consecutive Increment or Decrement Indications"]
    #[inline]
    pub fn phymasternbindications(&self) -> PHYMASTERNBINDICATIONSR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PHYMASTERNBINDICATIONSR { bits }
    }
    #[doc = "Bits 20:22 - PHY Master Phase Detect Selector"]
    #[inline]
    pub fn phymasterphasedetectselector(&self) -> PHYMASTERPHASEDETECTSELECTORR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PHYMASTERPHASEDETECTSELECTORR { bits }
    }
    #[doc = "Bit 23 - PHY Master Bypass Mode"]
    #[inline]
    pub fn phymasterbypassmode(&self) -> PHYMASTERBYPASSMODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PHYMASTERBYPASSMODER { bits }
    }
    #[doc = "Bit 24 - PHY Master Lock Mode"]
    #[inline]
    pub fn phymasterlockmode(&self) -> PHYMASTERLOCKMODER {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        PHYMASTERLOCKMODER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 8388608 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - PHY Master Initial Delay"]
    #[inline]
    pub fn phymasterinitialdelay(&mut self) -> _PHYMASTERINITIALDELAYW {
        _PHYMASTERINITIALDELAYW { w: self }
    }
    #[doc = "Bits 16:18 - Holds the Number of Consecutive Increment or Decrement Indications"]
    #[inline]
    pub fn phymasternbindications(&mut self) -> _PHYMASTERNBINDICATIONSW {
        _PHYMASTERNBINDICATIONSW { w: self }
    }
    #[doc = "Bits 20:22 - PHY Master Phase Detect Selector"]
    #[inline]
    pub fn phymasterphasedetectselector(&mut self) -> _PHYMASTERPHASEDETECTSELECTORW {
        _PHYMASTERPHASEDETECTSELECTORW { w: self }
    }
    #[doc = "Bit 23 - PHY Master Bypass Mode"]
    #[inline]
    pub fn phymasterbypassmode(&mut self) -> _PHYMASTERBYPASSMODEW {
        _PHYMASTERBYPASSMODEW { w: self }
    }
    #[doc = "Bit 24 - PHY Master Lock Mode"]
    #[inline]
    pub fn phymasterlockmode(&mut self) -> _PHYMASTERLOCKMODEW {
        _PHYMASTERLOCKMODEW { w: self }
    }
}
