#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DEVSIZECONFIG {
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
pub struct NUMADDRBYTESR {
    bits: u8,
}
impl NUMADDRBYTESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BYTESPERDEVICEPAGER {
    bits: u16,
}
impl BYTESPERDEVICEPAGER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct BYTESPERSUBSECTORR {
    bits: u8,
}
impl BYTESPERSUBSECTORR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MEMSIZEONCS0R {
    bits: u8,
}
impl MEMSIZEONCS0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MEMSIZEONCS1R {
    bits: u8,
}
impl MEMSIZEONCS1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _NUMADDRBYTESW<'a> {
    w: &'a mut W,
}
impl<'a> _NUMADDRBYTESW<'a> {
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
pub struct _BYTESPERDEVICEPAGEW<'a> {
    w: &'a mut W,
}
impl<'a> _BYTESPERDEVICEPAGEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 4095;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BYTESPERSUBSECTORW<'a> {
    w: &'a mut W,
}
impl<'a> _BYTESPERSUBSECTORW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MEMSIZEONCS0W<'a> {
    w: &'a mut W,
}
impl<'a> _MEMSIZEONCS0W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _MEMSIZEONCS1W<'a> {
    w: &'a mut W,
}
impl<'a> _MEMSIZEONCS1W<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 23;
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
    #[doc = "Bits 0:3 - Number of Address Bytes"]
    #[inline]
    pub fn numaddrbytes(&self) -> NUMADDRBYTESR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NUMADDRBYTESR { bits }
    }
    #[doc = "Bits 4:15 - Number of Bytes Per Device Page"]
    #[inline]
    pub fn bytesperdevicepage(&self) -> BYTESPERDEVICEPAGER {
        let bits = {
            const MASK: u16 = 4095;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        BYTESPERDEVICEPAGER { bits }
    }
    #[doc = "Bits 16:20 - Number of Bytes Per Block"]
    #[inline]
    pub fn bytespersubsector(&self) -> BYTESPERSUBSECTORR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        BYTESPERSUBSECTORR { bits }
    }
    #[doc = "Bits 21:22 - Size of Flash Device Connected to CS[0] Pin"]
    #[inline]
    pub fn memsizeoncs0(&self) -> MEMSIZEONCS0R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MEMSIZEONCS0R { bits }
    }
    #[doc = "Bits 23:24 - Size of Flash Device Connected to CS[1] Pin"]
    #[inline]
    pub fn memsizeoncs1(&self) -> MEMSIZEONCS1R {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MEMSIZEONCS1R { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 1052674 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Number of Address Bytes"]
    #[inline]
    pub fn numaddrbytes(&mut self) -> _NUMADDRBYTESW {
        _NUMADDRBYTESW { w: self }
    }
    #[doc = "Bits 4:15 - Number of Bytes Per Device Page"]
    #[inline]
    pub fn bytesperdevicepage(&mut self) -> _BYTESPERDEVICEPAGEW {
        _BYTESPERDEVICEPAGEW { w: self }
    }
    #[doc = "Bits 16:20 - Number of Bytes Per Block"]
    #[inline]
    pub fn bytespersubsector(&mut self) -> _BYTESPERSUBSECTORW {
        _BYTESPERSUBSECTORW { w: self }
    }
    #[doc = "Bits 21:22 - Size of Flash Device Connected to CS[0] Pin"]
    #[inline]
    pub fn memsizeoncs0(&mut self) -> _MEMSIZEONCS0W {
        _MEMSIZEONCS0W { w: self }
    }
    #[doc = "Bits 23:24 - Size of Flash Device Connected to CS[1] Pin"]
    #[inline]
    pub fn memsizeoncs1(&mut self) -> _MEMSIZEONCS1W {
        _MEMSIZEONCS1W { w: self }
    }
}
