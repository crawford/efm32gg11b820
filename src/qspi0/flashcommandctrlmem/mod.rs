#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FLASHCOMMANDCTRLMEM {
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
pub struct MEMBANKREQINPROGRESSR {
    bits: bool,
}
impl MEMBANKREQINPROGRESSR {
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
pub struct MEMBANKREADDATAR {
    bits: u8,
}
impl MEMBANKREADDATAR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NBOFSTIGREADBYTESR {
    bits: u8,
}
impl NBOFSTIGREADBYTESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct MEMBANKADDRR {
    bits: u16,
}
impl MEMBANKADDRR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _TRIGGERMEMBANKREQW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGGERMEMBANKREQW<'a> {
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
pub struct _NBOFSTIGREADBYTESW<'a> {
    w: &'a mut W,
}
impl<'a> _NBOFSTIGREADBYTESW<'a> {
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
pub struct _MEMBANKADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _MEMBANKADDRW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        const MASK: u16 = 511;
        const OFFSET: u8 = 20;
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
    #[doc = "Bit 1 - Memory Bank Data Request in Progress"]
    #[inline]
    pub fn membankreqinprogress(&self) -> MEMBANKREQINPROGRESSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MEMBANKREQINPROGRESSR { bits }
    }
    #[doc = "Bits 8:15 - Last Requested Data From the STIG Memory Bank"]
    #[inline]
    pub fn membankreaddata(&self) -> MEMBANKREADDATAR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        MEMBANKREADDATAR { bits }
    }
    #[doc = "Bits 16:18 - Number of Read Bytes for the Extended STIG"]
    #[inline]
    pub fn nbofstigreadbytes(&self) -> NBOFSTIGREADBYTESR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NBOFSTIGREADBYTESR { bits }
    }
    #[doc = "Bits 20:28 - Memory Bank Address"]
    #[inline]
    pub fn membankaddr(&self) -> MEMBANKADDRR {
        let bits = {
            const MASK: u16 = 511;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u16
        };
        MEMBANKADDRR { bits }
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
    #[doc = "Bit 0 - Trigger the Memory Bank Data Request"]
    #[inline]
    pub fn triggermembankreq(&mut self) -> _TRIGGERMEMBANKREQW {
        _TRIGGERMEMBANKREQW { w: self }
    }
    #[doc = "Bits 16:18 - Number of Read Bytes for the Extended STIG"]
    #[inline]
    pub fn nbofstigreadbytes(&mut self) -> _NBOFSTIGREADBYTESW {
        _NBOFSTIGREADBYTESW { w: self }
    }
    #[doc = "Bits 20:28 - Memory Bank Address"]
    #[inline]
    pub fn membankaddr(&mut self) -> _MEMBANKADDRW {
        _MEMBANKADDRW { w: self }
    }
}
