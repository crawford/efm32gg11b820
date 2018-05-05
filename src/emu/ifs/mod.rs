#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IFS {
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
}
#[doc = r" Proxy"]
pub struct _VMONAVDDFALLW<'a> {
    w: &'a mut W,
}
impl<'a> _VMONAVDDFALLW<'a> {
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
pub struct _VMONAVDDRISEW<'a> {
    w: &'a mut W,
}
impl<'a> _VMONAVDDRISEW<'a> {
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
pub struct _VMONALTAVDDFALLW<'a> {
    w: &'a mut W,
}
impl<'a> _VMONALTAVDDFALLW<'a> {
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
pub struct _VMONALTAVDDRISEW<'a> {
    w: &'a mut W,
}
impl<'a> _VMONALTAVDDRISEW<'a> {
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
pub struct _VMONDVDDFALLW<'a> {
    w: &'a mut W,
}
impl<'a> _VMONDVDDFALLW<'a> {
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
pub struct _VMONDVDDRISEW<'a> {
    w: &'a mut W,
}
impl<'a> _VMONDVDDRISEW<'a> {
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
pub struct _VMONIO0FALLW<'a> {
    w: &'a mut W,
}
impl<'a> _VMONIO0FALLW<'a> {
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
pub struct _VMONIO0RISEW<'a> {
    w: &'a mut W,
}
impl<'a> _VMONIO0RISEW<'a> {
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
pub struct _VMONIO1FALLW<'a> {
    w: &'a mut W,
}
impl<'a> _VMONIO1FALLW<'a> {
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
pub struct _VMONIO1RISEW<'a> {
    w: &'a mut W,
}
impl<'a> _VMONIO1RISEW<'a> {
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
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _R5VREADYW<'a> {
    w: &'a mut W,
}
impl<'a> _R5VREADYW<'a> {
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
pub struct _VMONBUVDDFALLW<'a> {
    w: &'a mut W,
}
impl<'a> _VMONBUVDDFALLW<'a> {
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _VMONBUVDDRISEW<'a> {
    w: &'a mut W,
}
impl<'a> _VMONBUVDDRISEW<'a> {
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
pub struct _PFETOVERCURRENTLIMITW<'a> {
    w: &'a mut W,
}
impl<'a> _PFETOVERCURRENTLIMITW<'a> {
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
pub struct _NFETOVERCURRENTLIMITW<'a> {
    w: &'a mut W,
}
impl<'a> _NFETOVERCURRENTLIMITW<'a> {
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
        const OFFSET: u8 = 17;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DCDCLPRUNNINGW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDCLPRUNNINGW<'a> {
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
pub struct _DCDCLNRUNNINGW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDCLNRUNNINGW<'a> {
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
pub struct _DCDCINBYPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _DCDCINBYPASSW<'a> {
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
pub struct _BURDYW<'a> {
    w: &'a mut W,
}
impl<'a> _BURDYW<'a> {
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _R5VVSINTW<'a> {
    w: &'a mut W,
}
impl<'a> _R5VVSINTW<'a> {
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
pub struct _EM23WAKEUPW<'a> {
    w: &'a mut W,
}
impl<'a> _EM23WAKEUPW<'a> {
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
#[doc = r" Proxy"]
pub struct _VSCALEDONEW<'a> {
    w: &'a mut W,
}
impl<'a> _VSCALEDONEW<'a> {
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
        const OFFSET: u8 = 25;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TEMPW<'a> {
    w: &'a mut W,
}
impl<'a> _TEMPW<'a> {
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
        const OFFSET: u8 = 29;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TEMPLOWW<'a> {
    w: &'a mut W,
}
impl<'a> _TEMPLOWW<'a> {
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
        const OFFSET: u8 = 30;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _TEMPHIGHW<'a> {
    w: &'a mut W,
}
impl<'a> _TEMPHIGHW<'a> {
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
        const OFFSET: u8 = 31;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[doc = "Bit 0 - Set VMONAVDDFALL Interrupt Flag"]
    #[inline]
    pub fn vmonavddfall(&mut self) -> _VMONAVDDFALLW {
        _VMONAVDDFALLW { w: self }
    }
    #[doc = "Bit 1 - Set VMONAVDDRISE Interrupt Flag"]
    #[inline]
    pub fn vmonavddrise(&mut self) -> _VMONAVDDRISEW {
        _VMONAVDDRISEW { w: self }
    }
    #[doc = "Bit 2 - Set VMONALTAVDDFALL Interrupt Flag"]
    #[inline]
    pub fn vmonaltavddfall(&mut self) -> _VMONALTAVDDFALLW {
        _VMONALTAVDDFALLW { w: self }
    }
    #[doc = "Bit 3 - Set VMONALTAVDDRISE Interrupt Flag"]
    #[inline]
    pub fn vmonaltavddrise(&mut self) -> _VMONALTAVDDRISEW {
        _VMONALTAVDDRISEW { w: self }
    }
    #[doc = "Bit 4 - Set VMONDVDDFALL Interrupt Flag"]
    #[inline]
    pub fn vmondvddfall(&mut self) -> _VMONDVDDFALLW {
        _VMONDVDDFALLW { w: self }
    }
    #[doc = "Bit 5 - Set VMONDVDDRISE Interrupt Flag"]
    #[inline]
    pub fn vmondvddrise(&mut self) -> _VMONDVDDRISEW {
        _VMONDVDDRISEW { w: self }
    }
    #[doc = "Bit 6 - Set VMONIO0FALL Interrupt Flag"]
    #[inline]
    pub fn vmonio0fall(&mut self) -> _VMONIO0FALLW {
        _VMONIO0FALLW { w: self }
    }
    #[doc = "Bit 7 - Set VMONIO0RISE Interrupt Flag"]
    #[inline]
    pub fn vmonio0rise(&mut self) -> _VMONIO0RISEW {
        _VMONIO0RISEW { w: self }
    }
    #[doc = "Bit 8 - Set VMONIO1FALL Interrupt Flag"]
    #[inline]
    pub fn vmonio1fall(&mut self) -> _VMONIO1FALLW {
        _VMONIO1FALLW { w: self }
    }
    #[doc = "Bit 9 - Set VMONIO1RISE Interrupt Flag"]
    #[inline]
    pub fn vmonio1rise(&mut self) -> _VMONIO1RISEW {
        _VMONIO1RISEW { w: self }
    }
    #[doc = "Bit 10 - Set R5VREADY Interrupt Flag"]
    #[inline]
    pub fn r5vready(&mut self) -> _R5VREADYW {
        _R5VREADYW { w: self }
    }
    #[doc = "Bit 12 - Set VMONBUVDDFALL Interrupt Flag"]
    #[inline]
    pub fn vmonbuvddfall(&mut self) -> _VMONBUVDDFALLW {
        _VMONBUVDDFALLW { w: self }
    }
    #[doc = "Bit 13 - Set VMONBUVDDRISE Interrupt Flag"]
    #[inline]
    pub fn vmonbuvddrise(&mut self) -> _VMONBUVDDRISEW {
        _VMONBUVDDRISEW { w: self }
    }
    #[doc = "Bit 16 - Set PFETOVERCURRENTLIMIT Interrupt Flag"]
    #[inline]
    pub fn pfetovercurrentlimit(&mut self) -> _PFETOVERCURRENTLIMITW {
        _PFETOVERCURRENTLIMITW { w: self }
    }
    #[doc = "Bit 17 - Set NFETOVERCURRENTLIMIT Interrupt Flag"]
    #[inline]
    pub fn nfetovercurrentlimit(&mut self) -> _NFETOVERCURRENTLIMITW {
        _NFETOVERCURRENTLIMITW { w: self }
    }
    #[doc = "Bit 18 - Set DCDCLPRUNNING Interrupt Flag"]
    #[inline]
    pub fn dcdclprunning(&mut self) -> _DCDCLPRUNNINGW {
        _DCDCLPRUNNINGW { w: self }
    }
    #[doc = "Bit 19 - Set DCDCLNRUNNING Interrupt Flag"]
    #[inline]
    pub fn dcdclnrunning(&mut self) -> _DCDCLNRUNNINGW {
        _DCDCLNRUNNINGW { w: self }
    }
    #[doc = "Bit 20 - Set DCDCINBYPASS Interrupt Flag"]
    #[inline]
    pub fn dcdcinbypass(&mut self) -> _DCDCINBYPASSW {
        _DCDCINBYPASSW { w: self }
    }
    #[doc = "Bit 22 - Set BURDY Interrupt Flag"]
    #[inline]
    pub fn burdy(&mut self) -> _BURDYW {
        _BURDYW { w: self }
    }
    #[doc = "Bit 23 - Set R5VVSINT Interrupt Flag"]
    #[inline]
    pub fn r5vvsint(&mut self) -> _R5VVSINTW {
        _R5VVSINTW { w: self }
    }
    #[doc = "Bit 24 - Set EM23WAKEUP Interrupt Flag"]
    #[inline]
    pub fn em23wakeup(&mut self) -> _EM23WAKEUPW {
        _EM23WAKEUPW { w: self }
    }
    #[doc = "Bit 25 - Set VSCALEDONE Interrupt Flag"]
    #[inline]
    pub fn vscaledone(&mut self) -> _VSCALEDONEW {
        _VSCALEDONEW { w: self }
    }
    #[doc = "Bit 29 - Set TEMP Interrupt Flag"]
    #[inline]
    pub fn temp(&mut self) -> _TEMPW {
        _TEMPW { w: self }
    }
    #[doc = "Bit 30 - Set TEMPLOW Interrupt Flag"]
    #[inline]
    pub fn templow(&mut self) -> _TEMPLOWW {
        _TEMPLOWW { w: self }
    }
    #[doc = "Bit 31 - Set TEMPHIGH Interrupt Flag"]
    #[inline]
    pub fn temphigh(&mut self) -> _TEMPHIGHW {
        _TEMPHIGHW { w: self }
    }
}
