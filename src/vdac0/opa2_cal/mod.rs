#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::OPA2_CAL {
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
pub struct CM1R {
    bits: u8,
}
impl CM1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CM2R {
    bits: u8,
}
impl CM2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct CM3R {
    bits: u8,
}
impl CM3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GMR {
    bits: u8,
}
impl GMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct GM3R {
    bits: u8,
}
impl GM3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OFFSETPR {
    bits: u8,
}
impl OFFSETPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct OFFSETNR {
    bits: u8,
}
impl OFFSETNR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CM1W<'a> {
    w: &'a mut W,
}
impl<'a> _CM1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CM2W<'a> {
    w: &'a mut W,
}
impl<'a> _CM2W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CM3W<'a> {
    w: &'a mut W,
}
impl<'a> _CM3W<'a> {
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
pub struct _GMW<'a> {
    w: &'a mut W,
}
impl<'a> _GMW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GM3W<'a> {
    w: &'a mut W,
}
impl<'a> _GM3W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OFFSETPW<'a> {
    w: &'a mut W,
}
impl<'a> _OFFSETPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OFFSETNW<'a> {
    w: &'a mut W,
}
impl<'a> _OFFSETNW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 26;
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
    #[doc = "Bits 0:3 - Compensation Cap Cm1 Trim Value"]
    #[inline]
    pub fn cm1(&self) -> CM1R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CM1R { bits }
    }
    #[doc = "Bits 5:8 - Compensation Cap Cm2 Trim Value"]
    #[inline]
    pub fn cm2(&self) -> CM2R {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CM2R { bits }
    }
    #[doc = "Bits 10:11 - Compensation Cap Cm3 Trim Value"]
    #[inline]
    pub fn cm3(&self) -> CM3R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CM3R { bits }
    }
    #[doc = "Bits 13:15 - Gm Trim Value"]
    #[inline]
    pub fn gm(&self) -> GMR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GMR { bits }
    }
    #[doc = "Bits 17:18 - Gm3 Trim Value"]
    #[inline]
    pub fn gm3(&self) -> GM3R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 17;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        GM3R { bits }
    }
    #[doc = "Bits 20:24 - OPAx Non-Inverting Input Offset Configuration Value"]
    #[inline]
    pub fn offsetp(&self) -> OFFSETPR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OFFSETPR { bits }
    }
    #[doc = "Bits 26:30 - OPAx Inverting Input Offset Configuration Value"]
    #[inline]
    pub fn offsetn(&self) -> OFFSETNR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 26;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        OFFSETNR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 32999 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Compensation Cap Cm1 Trim Value"]
    #[inline]
    pub fn cm1(&mut self) -> _CM1W {
        _CM1W { w: self }
    }
    #[doc = "Bits 5:8 - Compensation Cap Cm2 Trim Value"]
    #[inline]
    pub fn cm2(&mut self) -> _CM2W {
        _CM2W { w: self }
    }
    #[doc = "Bits 10:11 - Compensation Cap Cm3 Trim Value"]
    #[inline]
    pub fn cm3(&mut self) -> _CM3W {
        _CM3W { w: self }
    }
    #[doc = "Bits 13:15 - Gm Trim Value"]
    #[inline]
    pub fn gm(&mut self) -> _GMW {
        _GMW { w: self }
    }
    #[doc = "Bits 17:18 - Gm3 Trim Value"]
    #[inline]
    pub fn gm3(&mut self) -> _GM3W {
        _GM3W { w: self }
    }
    #[doc = "Bits 20:24 - OPAx Non-Inverting Input Offset Configuration Value"]
    #[inline]
    pub fn offsetp(&mut self) -> _OFFSETPW {
        _OFFSETPW { w: self }
    }
    #[doc = "Bits 26:30 - OPAx Inverting Input Offset Configuration Value"]
    #[inline]
    pub fn offsetn(&mut self) -> _OFFSETNW {
        _OFFSETNW { w: self }
    }
}
