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
pub struct _CH0CDW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0CDW<'a> {
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
pub struct _CH1CDW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1CDW<'a> {
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
pub struct _CH0OFW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0OFW<'a> {
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
pub struct _CH1OFW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1OFW<'a> {
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
pub struct _CH0UFW<'a> {
    w: &'a mut W,
}
impl<'a> _CH0UFW<'a> {
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
pub struct _CH1UFW<'a> {
    w: &'a mut W,
}
impl<'a> _CH1UFW<'a> {
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
pub struct _EM23ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _EM23ERRW<'a> {
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
pub struct _OPA0APORTCONFLICTW<'a> {
    w: &'a mut W,
}
impl<'a> _OPA0APORTCONFLICTW<'a> {
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
pub struct _OPA1APORTCONFLICTW<'a> {
    w: &'a mut W,
}
impl<'a> _OPA1APORTCONFLICTW<'a> {
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
pub struct _OPA2APORTCONFLICTW<'a> {
    w: &'a mut W,
}
impl<'a> _OPA2APORTCONFLICTW<'a> {
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
pub struct _OPA3APORTCONFLICTW<'a> {
    w: &'a mut W,
}
impl<'a> _OPA3APORTCONFLICTW<'a> {
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
pub struct _OPA0PRSTIMEDERRW<'a> {
    w: &'a mut W,
}
impl<'a> _OPA0PRSTIMEDERRW<'a> {
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
pub struct _OPA1PRSTIMEDERRW<'a> {
    w: &'a mut W,
}
impl<'a> _OPA1PRSTIMEDERRW<'a> {
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OPA2PRSTIMEDERRW<'a> {
    w: &'a mut W,
}
impl<'a> _OPA2PRSTIMEDERRW<'a> {
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
pub struct _OPA3PRSTIMEDERRW<'a> {
    w: &'a mut W,
}
impl<'a> _OPA3PRSTIMEDERRW<'a> {
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
pub struct _OPA0OUTVALIDW<'a> {
    w: &'a mut W,
}
impl<'a> _OPA0OUTVALIDW<'a> {
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
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _OPA1OUTVALIDW<'a> {
    w: &'a mut W,
}
impl<'a> _OPA1OUTVALIDW<'a> {
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
pub struct _OPA2OUTVALIDW<'a> {
    w: &'a mut W,
}
impl<'a> _OPA2OUTVALIDW<'a> {
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
pub struct _OPA3OUTVALIDW<'a> {
    w: &'a mut W,
}
impl<'a> _OPA3OUTVALIDW<'a> {
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
    #[doc = "Bit 0 - Set CH0CD Interrupt Flag"]
    #[inline]
    pub fn ch0cd(&mut self) -> _CH0CDW {
        _CH0CDW { w: self }
    }
    #[doc = "Bit 1 - Set CH1CD Interrupt Flag"]
    #[inline]
    pub fn ch1cd(&mut self) -> _CH1CDW {
        _CH1CDW { w: self }
    }
    #[doc = "Bit 2 - Set CH0OF Interrupt Flag"]
    #[inline]
    pub fn ch0of(&mut self) -> _CH0OFW {
        _CH0OFW { w: self }
    }
    #[doc = "Bit 3 - Set CH1OF Interrupt Flag"]
    #[inline]
    pub fn ch1of(&mut self) -> _CH1OFW {
        _CH1OFW { w: self }
    }
    #[doc = "Bit 4 - Set CH0UF Interrupt Flag"]
    #[inline]
    pub fn ch0uf(&mut self) -> _CH0UFW {
        _CH0UFW { w: self }
    }
    #[doc = "Bit 5 - Set CH1UF Interrupt Flag"]
    #[inline]
    pub fn ch1uf(&mut self) -> _CH1UFW {
        _CH1UFW { w: self }
    }
    #[doc = "Bit 15 - Set EM23ERR Interrupt Flag"]
    #[inline]
    pub fn em23err(&mut self) -> _EM23ERRW {
        _EM23ERRW { w: self }
    }
    #[doc = "Bit 16 - Set OPA0APORTCONFLICT Interrupt Flag"]
    #[inline]
    pub fn opa0aportconflict(&mut self) -> _OPA0APORTCONFLICTW {
        _OPA0APORTCONFLICTW { w: self }
    }
    #[doc = "Bit 17 - Set OPA1APORTCONFLICT Interrupt Flag"]
    #[inline]
    pub fn opa1aportconflict(&mut self) -> _OPA1APORTCONFLICTW {
        _OPA1APORTCONFLICTW { w: self }
    }
    #[doc = "Bit 18 - Set OPA2APORTCONFLICT Interrupt Flag"]
    #[inline]
    pub fn opa2aportconflict(&mut self) -> _OPA2APORTCONFLICTW {
        _OPA2APORTCONFLICTW { w: self }
    }
    #[doc = "Bit 19 - Set OPA3APORTCONFLICT Interrupt Flag"]
    #[inline]
    pub fn opa3aportconflict(&mut self) -> _OPA3APORTCONFLICTW {
        _OPA3APORTCONFLICTW { w: self }
    }
    #[doc = "Bit 20 - Set OPA0PRSTIMEDERR Interrupt Flag"]
    #[inline]
    pub fn opa0prstimederr(&mut self) -> _OPA0PRSTIMEDERRW {
        _OPA0PRSTIMEDERRW { w: self }
    }
    #[doc = "Bit 21 - Set OPA1PRSTIMEDERR Interrupt Flag"]
    #[inline]
    pub fn opa1prstimederr(&mut self) -> _OPA1PRSTIMEDERRW {
        _OPA1PRSTIMEDERRW { w: self }
    }
    #[doc = "Bit 22 - Set OPA2PRSTIMEDERR Interrupt Flag"]
    #[inline]
    pub fn opa2prstimederr(&mut self) -> _OPA2PRSTIMEDERRW {
        _OPA2PRSTIMEDERRW { w: self }
    }
    #[doc = "Bit 23 - Set OPA3PRSTIMEDERR Interrupt Flag"]
    #[inline]
    pub fn opa3prstimederr(&mut self) -> _OPA3PRSTIMEDERRW {
        _OPA3PRSTIMEDERRW { w: self }
    }
    #[doc = "Bit 28 - Set OPA0OUTVALID Interrupt Flag"]
    #[inline]
    pub fn opa0outvalid(&mut self) -> _OPA0OUTVALIDW {
        _OPA0OUTVALIDW { w: self }
    }
    #[doc = "Bit 29 - Set OPA1OUTVALID Interrupt Flag"]
    #[inline]
    pub fn opa1outvalid(&mut self) -> _OPA1OUTVALIDW {
        _OPA1OUTVALIDW { w: self }
    }
    #[doc = "Bit 30 - Set OPA2OUTVALID Interrupt Flag"]
    #[inline]
    pub fn opa2outvalid(&mut self) -> _OPA2OUTVALIDW {
        _OPA2OUTVALIDW { w: self }
    }
    #[doc = "Bit 31 - Set OPA3OUTVALID Interrupt Flag"]
    #[inline]
    pub fn opa3outvalid(&mut self) -> _OPA3OUTVALIDW {
        _OPA3OUTVALIDW { w: self }
    }
}
