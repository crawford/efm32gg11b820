#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FLASHCMDCTRL {
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
pub struct CMDEXECSTATUSR {
    bits: bool,
}
impl CMDEXECSTATUSR {
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
pub struct STIGMEMBANKENR {
    bits: bool,
}
impl STIGMEMBANKENR {
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
pub struct NUMDUMMYCYCLESR {
    bits: u8,
}
impl NUMDUMMYCYCLESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NUMWRDATABYTESR {
    bits: u8,
}
impl NUMWRDATABYTESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ENBWRITEDATAR {
    bits: bool,
}
impl ENBWRITEDATAR {
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
pub struct ENBMODEBITR {
    bits: bool,
}
impl ENBMODEBITR {
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
pub struct ENBCOMDADDRR {
    bits: bool,
}
impl ENBCOMDADDRR {
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
pub struct NUMRDDATABYTESR {
    bits: u8,
}
impl NUMRDDATABYTESR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct ENBREADDATAR {
    bits: bool,
}
impl ENBREADDATAR {
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
pub struct CMDOPCODER {
    bits: u8,
}
impl CMDOPCODER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _CMDEXECW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDEXECW<'a> {
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
pub struct _STIGMEMBANKENW<'a> {
    w: &'a mut W,
}
impl<'a> _STIGMEMBANKENW<'a> {
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
pub struct _NUMDUMMYCYCLESW<'a> {
    w: &'a mut W,
}
impl<'a> _NUMDUMMYCYCLESW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 31;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NUMWRDATABYTESW<'a> {
    w: &'a mut W,
}
impl<'a> _NUMWRDATABYTESW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENBWRITEDATAW<'a> {
    w: &'a mut W,
}
impl<'a> _ENBWRITEDATAW<'a> {
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
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _ENBMODEBITW<'a> {
    w: &'a mut W,
}
impl<'a> _ENBMODEBITW<'a> {
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
#[doc = r" Proxy"]
pub struct _ENBCOMDADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _ENBCOMDADDRW<'a> {
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
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NUMRDDATABYTESW<'a> {
    w: &'a mut W,
}
impl<'a> _NUMRDDATABYTESW<'a> {
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
pub struct _ENBREADDATAW<'a> {
    w: &'a mut W,
}
impl<'a> _ENBREADDATAW<'a> {
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
pub struct _CMDOPCODEW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDOPCODEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
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
    #[doc = "Bit 1 - Command Execution in Progress"]
    #[inline]
    pub fn cmdexecstatus(&self) -> CMDEXECSTATUSR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CMDEXECSTATUSR { bits }
    }
    #[doc = "Bit 2 - STIG Memory Bank Enable Bit"]
    #[inline]
    pub fn stigmembanken(&self) -> STIGMEMBANKENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        STIGMEMBANKENR { bits }
    }
    #[doc = "Bits 7:11 - Number of Dummy Cycles"]
    #[inline]
    pub fn numdummycycles(&self) -> NUMDUMMYCYCLESR {
        let bits = {
            const MASK: u8 = 31;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NUMDUMMYCYCLESR { bits }
    }
    #[doc = "Bits 12:14 - Number of Write Data Bytes"]
    #[inline]
    pub fn numwrdatabytes(&self) -> NUMWRDATABYTESR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NUMWRDATABYTESR { bits }
    }
    #[doc = "Bit 15 - Write Data Enable"]
    #[inline]
    pub fn enbwritedata(&self) -> ENBWRITEDATAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENBWRITEDATAR { bits }
    }
    #[doc = "Bits 16:17 - Number of Address Bytes"]
    #[inline]
    pub fn numaddrbytes(&self) -> NUMADDRBYTESR {
        let bits = {
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NUMADDRBYTESR { bits }
    }
    #[doc = "Bit 18 - Mode Bit Enable"]
    #[inline]
    pub fn enbmodebit(&self) -> ENBMODEBITR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENBMODEBITR { bits }
    }
    #[doc = "Bit 19 - Command Address Enable"]
    #[inline]
    pub fn enbcomdaddr(&self) -> ENBCOMDADDRR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENBCOMDADDRR { bits }
    }
    #[doc = "Bits 20:22 - Number of Read Data Bytes"]
    #[inline]
    pub fn numrddatabytes(&self) -> NUMRDDATABYTESR {
        let bits = {
            const MASK: u8 = 7;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NUMRDDATABYTESR { bits }
    }
    #[doc = "Bit 23 - Read Data Enable"]
    #[inline]
    pub fn enbreaddata(&self) -> ENBREADDATAR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        ENBREADDATAR { bits }
    }
    #[doc = "Bits 24:31 - Command Opcode"]
    #[inline]
    pub fn cmdopcode(&self) -> CMDOPCODER {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMDOPCODER { bits }
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
    #[doc = "Bit 0 - Execute the Command"]
    #[inline]
    pub fn cmdexec(&mut self) -> _CMDEXECW {
        _CMDEXECW { w: self }
    }
    #[doc = "Bit 2 - STIG Memory Bank Enable Bit"]
    #[inline]
    pub fn stigmembanken(&mut self) -> _STIGMEMBANKENW {
        _STIGMEMBANKENW { w: self }
    }
    #[doc = "Bits 7:11 - Number of Dummy Cycles"]
    #[inline]
    pub fn numdummycycles(&mut self) -> _NUMDUMMYCYCLESW {
        _NUMDUMMYCYCLESW { w: self }
    }
    #[doc = "Bits 12:14 - Number of Write Data Bytes"]
    #[inline]
    pub fn numwrdatabytes(&mut self) -> _NUMWRDATABYTESW {
        _NUMWRDATABYTESW { w: self }
    }
    #[doc = "Bit 15 - Write Data Enable"]
    #[inline]
    pub fn enbwritedata(&mut self) -> _ENBWRITEDATAW {
        _ENBWRITEDATAW { w: self }
    }
    #[doc = "Bits 16:17 - Number of Address Bytes"]
    #[inline]
    pub fn numaddrbytes(&mut self) -> _NUMADDRBYTESW {
        _NUMADDRBYTESW { w: self }
    }
    #[doc = "Bit 18 - Mode Bit Enable"]
    #[inline]
    pub fn enbmodebit(&mut self) -> _ENBMODEBITW {
        _ENBMODEBITW { w: self }
    }
    #[doc = "Bit 19 - Command Address Enable"]
    #[inline]
    pub fn enbcomdaddr(&mut self) -> _ENBCOMDADDRW {
        _ENBCOMDADDRW { w: self }
    }
    #[doc = "Bits 20:22 - Number of Read Data Bytes"]
    #[inline]
    pub fn numrddatabytes(&mut self) -> _NUMRDDATABYTESW {
        _NUMRDDATABYTESW { w: self }
    }
    #[doc = "Bit 23 - Read Data Enable"]
    #[inline]
    pub fn enbreaddata(&mut self) -> _ENBREADDATAW {
        _ENBREADDATAW { w: self }
    }
    #[doc = "Bits 24:31 - Command Opcode"]
    #[inline]
    pub fn cmdopcode(&mut self) -> _CMDOPCODEW {
        _CMDOPCODEW { w: self }
    }
}
