#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DATTRIM1 {
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
pub struct ROUTR {
    bits: u8,
}
impl ROUTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ENDLYPULLUPR {
    bits: bool,
}
impl ENDLYPULLUPR {
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
pub struct DLYPULLUPFSR {
    bits: u8,
}
impl DLYPULLUPFSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct VCRSFSR {
    bits: u8,
}
impl VCRSFSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TFDMFSR {
    bits: u8,
}
impl TFDMFSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRDMFSR {
    bits: u8,
}
impl TRDMFSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TFDPFSR {
    bits: u8,
}
impl TFDPFSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct TRDPFSR {
    bits: u8,
}
impl TRDPFSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _ROUTW<'a> {
    w: &'a mut W,
}
impl<'a> _ROUTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENDLYPULLUPW<'a> {
    w: &'a mut W,
}
impl<'a> _ENDLYPULLUPW<'a> {
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
pub struct _DLYPULLUPFSW<'a> {
    w: &'a mut W,
}
impl<'a> _DLYPULLUPFSW<'a> {
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
pub struct _VCRSFSW<'a> {
    w: &'a mut W,
}
impl<'a> _VCRSFSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TFDMFSW<'a> {
    w: &'a mut W,
}
impl<'a> _TFDMFSW<'a> {
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
pub struct _TRDMFSW<'a> {
    w: &'a mut W,
}
impl<'a> _TRDMFSW<'a> {
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
pub struct _TFDPFSW<'a> {
    w: &'a mut W,
}
impl<'a> _TFDPFSW<'a> {
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
pub struct _TRDPFSW<'a> {
    w: &'a mut W,
}
impl<'a> _TRDPFSW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
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
    #[doc = "Bits 0:5 - Trim for DP and DM Output Impedance for Both FS and LS"]
    #[inline]
    pub fn rout(&self) -> ROUTR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        ROUTR { bits }
    }
    #[doc = "Bit 7 - Enables Delay of Pull in TX Mode for Both FS and LS"]
    #[inline]
    pub fn endlypullup(&self) -> ENDLYPULLUPR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENDLYPULLUPR { bits }
    }
    #[doc = "Bits 8:9 - Trim for Rising Crossover Voltage in FS"]
    #[inline]
    pub fn dlypullupfs(&self) -> DLYPULLUPFSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        DLYPULLUPFSR { bits }
    }
    #[doc = "Bits 10:11 - Trim for Falling Crossover Voltage in FS"]
    #[inline]
    pub fn vcrsfs(&self) -> VCRSFSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        VCRSFSR { bits }
    }
    #[doc = "Bits 12:13 - Trim for DM Fall Time in FS"]
    #[inline]
    pub fn tfdmfs(&self) -> TFDMFSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TFDMFSR { bits }
    }
    #[doc = "Bits 14:15 - Trim for DM Rise Time in FS"]
    #[inline]
    pub fn trdmfs(&self) -> TRDMFSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRDMFSR { bits }
    }
    #[doc = "Bits 16:17 - Trim for DP Fall Time in FS"]
    #[inline]
    pub fn tfdpfs(&self) -> TFDPFSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TFDPFSR { bits }
    }
    #[doc = "Bits 18:19 - Trim for DP Rise Time in FS"]
    #[inline]
    pub fn trdpfs(&self) -> TRDPFSR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        TRDPFSR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 36 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:5 - Trim for DP and DM Output Impedance for Both FS and LS"]
    #[inline]
    pub fn rout(&mut self) -> _ROUTW {
        _ROUTW { w: self }
    }
    #[doc = "Bit 7 - Enables Delay of Pull in TX Mode for Both FS and LS"]
    #[inline]
    pub fn endlypullup(&mut self) -> _ENDLYPULLUPW {
        _ENDLYPULLUPW { w: self }
    }
    #[doc = "Bits 8:9 - Trim for Rising Crossover Voltage in FS"]
    #[inline]
    pub fn dlypullupfs(&mut self) -> _DLYPULLUPFSW {
        _DLYPULLUPFSW { w: self }
    }
    #[doc = "Bits 10:11 - Trim for Falling Crossover Voltage in FS"]
    #[inline]
    pub fn vcrsfs(&mut self) -> _VCRSFSW {
        _VCRSFSW { w: self }
    }
    #[doc = "Bits 12:13 - Trim for DM Fall Time in FS"]
    #[inline]
    pub fn tfdmfs(&mut self) -> _TFDMFSW {
        _TFDMFSW { w: self }
    }
    #[doc = "Bits 14:15 - Trim for DM Rise Time in FS"]
    #[inline]
    pub fn trdmfs(&mut self) -> _TRDMFSW {
        _TRDMFSW { w: self }
    }
    #[doc = "Bits 16:17 - Trim for DP Fall Time in FS"]
    #[inline]
    pub fn tfdpfs(&mut self) -> _TFDPFSW {
        _TFDPFSW { w: self }
    }
    #[doc = "Bits 18:19 - Trim for DP Rise Time in FS"]
    #[inline]
    pub fn trdpfs(&mut self) -> _TRDPFSW {
        _TRDPFSW { w: self }
    }
}
