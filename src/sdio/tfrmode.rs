#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TFRMODE {
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
pub struct DMAENR {
    bits: bool,
}
impl DMAENR {
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
pub struct BLKCNTENR {
    bits: bool,
}
impl BLKCNTENR {
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
#[doc = "Possible values of the field `AUTOCMDEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTOCMDENR {
    #[doc = "Auto CMD Disabled"]
    ACMDDISABLED,
    #[doc = "Auto CMD12 Enable"]
    ACMD12EN,
    #[doc = "Auto CMD23 Enable"]
    ACMD23EN,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl AUTOCMDENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            AUTOCMDENR::ACMDDISABLED => 0,
            AUTOCMDENR::ACMD12EN => 1,
            AUTOCMDENR::ACMD23EN => 2,
            AUTOCMDENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> AUTOCMDENR {
        match value {
            0 => AUTOCMDENR::ACMDDISABLED,
            1 => AUTOCMDENR::ACMD12EN,
            2 => AUTOCMDENR::ACMD23EN,
            i => AUTOCMDENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ACMDDISABLED`"]
    #[inline]
    pub fn is_acmddisabled(&self) -> bool {
        *self == AUTOCMDENR::ACMDDISABLED
    }
    #[doc = "Checks if the value of the field is `ACMD12EN`"]
    #[inline]
    pub fn is_acmd12en(&self) -> bool {
        *self == AUTOCMDENR::ACMD12EN
    }
    #[doc = "Checks if the value of the field is `ACMD23EN`"]
    #[inline]
    pub fn is_acmd23en(&self) -> bool {
        *self == AUTOCMDENR::ACMD23EN
    }
}
#[doc = r" Value of the field"]
pub struct DATDIRSELR {
    bits: bool,
}
impl DATDIRSELR {
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
pub struct MULTSINGBLKSELR {
    bits: bool,
}
impl MULTSINGBLKSELR {
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
#[doc = "Possible values of the field `RESPTYPESEL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RESPTYPESELR {
    #[doc = "No RESP"]
    NORESP,
    #[doc = "RESP Length 136"]
    RESP136,
    #[doc = "RESP Length 48"]
    RESP48,
    #[doc = "RESP Length 48 check busy after RESP"]
    BUSYAFTRESP,
}
impl RESPTYPESELR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RESPTYPESELR::NORESP => 0,
            RESPTYPESELR::RESP136 => 1,
            RESPTYPESELR::RESP48 => 2,
            RESPTYPESELR::BUSYAFTRESP => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RESPTYPESELR {
        match value {
            0 => RESPTYPESELR::NORESP,
            1 => RESPTYPESELR::RESP136,
            2 => RESPTYPESELR::RESP48,
            3 => RESPTYPESELR::BUSYAFTRESP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORESP`"]
    #[inline]
    pub fn is_noresp(&self) -> bool {
        *self == RESPTYPESELR::NORESP
    }
    #[doc = "Checks if the value of the field is `RESP136`"]
    #[inline]
    pub fn is_resp136(&self) -> bool {
        *self == RESPTYPESELR::RESP136
    }
    #[doc = "Checks if the value of the field is `RESP48`"]
    #[inline]
    pub fn is_resp48(&self) -> bool {
        *self == RESPTYPESELR::RESP48
    }
    #[doc = "Checks if the value of the field is `BUSYAFTRESP`"]
    #[inline]
    pub fn is_busyaftresp(&self) -> bool {
        *self == RESPTYPESELR::BUSYAFTRESP
    }
}
#[doc = r" Value of the field"]
pub struct CMDCRCCHKENR {
    bits: bool,
}
impl CMDCRCCHKENR {
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
pub struct CMDINDXCHKENR {
    bits: bool,
}
impl CMDINDXCHKENR {
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
pub struct DATPRESSELR {
    bits: bool,
}
impl DATPRESSELR {
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
#[doc = "Possible values of the field `CMDTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMDTYPER {
    #[doc = "Normal Command"]
    NORMAL,
    #[doc = "Suspend command"]
    SUSPEND,
    #[doc = "Resume command"]
    RESUME,
    #[doc = "Abort command"]
    ABORT,
}
impl CMDTYPER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CMDTYPER::NORMAL => 0,
            CMDTYPER::SUSPEND => 1,
            CMDTYPER::RESUME => 2,
            CMDTYPER::ABORT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CMDTYPER {
        match value {
            0 => CMDTYPER::NORMAL,
            1 => CMDTYPER::SUSPEND,
            2 => CMDTYPER::RESUME,
            3 => CMDTYPER::ABORT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline]
    pub fn is_normal(&self) -> bool {
        *self == CMDTYPER::NORMAL
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline]
    pub fn is_suspend(&self) -> bool {
        *self == CMDTYPER::SUSPEND
    }
    #[doc = "Checks if the value of the field is `RESUME`"]
    #[inline]
    pub fn is_resume(&self) -> bool {
        *self == CMDTYPER::RESUME
    }
    #[doc = "Checks if the value of the field is `ABORT`"]
    #[inline]
    pub fn is_abort(&self) -> bool {
        *self == CMDTYPER::ABORT
    }
}
#[doc = r" Value of the field"]
pub struct CMDINDEXR {
    bits: u8,
}
impl CMDINDEXR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _DMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _DMAENW<'a> {
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
pub struct _BLKCNTENW<'a> {
    w: &'a mut W,
}
impl<'a> _BLKCNTENW<'a> {
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
#[doc = "Values that can be written to the field `AUTOCMDEN`"]
pub enum AUTOCMDENW {
    #[doc = "Auto CMD Disabled"]
    ACMDDISABLED,
    #[doc = "Auto CMD12 Enable"]
    ACMD12EN,
    #[doc = "Auto CMD23 Enable"]
    ACMD23EN,
}
impl AUTOCMDENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            AUTOCMDENW::ACMDDISABLED => 0,
            AUTOCMDENW::ACMD12EN => 1,
            AUTOCMDENW::ACMD23EN => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _AUTOCMDENW<'a> {
    w: &'a mut W,
}
impl<'a> _AUTOCMDENW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: AUTOCMDENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Auto CMD Disabled"]
    #[inline]
    pub fn acmddisabled(self) -> &'a mut W {
        self.variant(AUTOCMDENW::ACMDDISABLED)
    }
    #[doc = "Auto CMD12 Enable"]
    #[inline]
    pub fn acmd12en(self) -> &'a mut W {
        self.variant(AUTOCMDENW::ACMD12EN)
    }
    #[doc = "Auto CMD23 Enable"]
    #[inline]
    pub fn acmd23en(self) -> &'a mut W {
        self.variant(AUTOCMDENW::ACMD23EN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _DATDIRSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DATDIRSELW<'a> {
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
pub struct _MULTSINGBLKSELW<'a> {
    w: &'a mut W,
}
impl<'a> _MULTSINGBLKSELW<'a> {
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
#[doc = "Values that can be written to the field `RESPTYPESEL`"]
pub enum RESPTYPESELW {
    #[doc = "No RESP"]
    NORESP,
    #[doc = "RESP Length 136"]
    RESP136,
    #[doc = "RESP Length 48"]
    RESP48,
    #[doc = "RESP Length 48 check busy after RESP"]
    BUSYAFTRESP,
}
impl RESPTYPESELW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RESPTYPESELW::NORESP => 0,
            RESPTYPESELW::RESP136 => 1,
            RESPTYPESELW::RESP48 => 2,
            RESPTYPESELW::BUSYAFTRESP => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RESPTYPESELW<'a> {
    w: &'a mut W,
}
impl<'a> _RESPTYPESELW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RESPTYPESELW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No RESP"]
    #[inline]
    pub fn noresp(self) -> &'a mut W {
        self.variant(RESPTYPESELW::NORESP)
    }
    #[doc = "RESP Length 136"]
    #[inline]
    pub fn resp136(self) -> &'a mut W {
        self.variant(RESPTYPESELW::RESP136)
    }
    #[doc = "RESP Length 48"]
    #[inline]
    pub fn resp48(self) -> &'a mut W {
        self.variant(RESPTYPESELW::RESP48)
    }
    #[doc = "RESP Length 48 check busy after RESP"]
    #[inline]
    pub fn busyaftresp(self) -> &'a mut W {
        self.variant(RESPTYPESELW::BUSYAFTRESP)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CMDCRCCHKENW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDCRCCHKENW<'a> {
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
pub struct _CMDINDXCHKENW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDINDXCHKENW<'a> {
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
pub struct _DATPRESSELW<'a> {
    w: &'a mut W,
}
impl<'a> _DATPRESSELW<'a> {
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
#[doc = "Values that can be written to the field `CMDTYPE`"]
pub enum CMDTYPEW {
    #[doc = "Normal Command"]
    NORMAL,
    #[doc = "Suspend command"]
    SUSPEND,
    #[doc = "Resume command"]
    RESUME,
    #[doc = "Abort command"]
    ABORT,
}
impl CMDTYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CMDTYPEW::NORMAL => 0,
            CMDTYPEW::SUSPEND => 1,
            CMDTYPEW::RESUME => 2,
            CMDTYPEW::ABORT => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CMDTYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDTYPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CMDTYPEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Normal Command"]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(CMDTYPEW::NORMAL)
    }
    #[doc = "Suspend command"]
    #[inline]
    pub fn suspend(self) -> &'a mut W {
        self.variant(CMDTYPEW::SUSPEND)
    }
    #[doc = "Resume command"]
    #[inline]
    pub fn resume(self) -> &'a mut W {
        self.variant(CMDTYPEW::RESUME)
    }
    #[doc = "Abort command"]
    #[inline]
    pub fn abort(self) -> &'a mut W {
        self.variant(CMDTYPEW::ABORT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CMDINDEXW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDINDEXW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
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
    #[doc = "Bit 0 - DMA Enable"]
    #[inline]
    pub fn dmaen(&self) -> DMAENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DMAENR { bits }
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline]
    pub fn blkcnten(&self) -> BLKCNTENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        BLKCNTENR { bits }
    }
    #[doc = "Bits 2:3 - Auto Command Enable"]
    #[inline]
    pub fn autocmden(&self) -> AUTOCMDENR {
        AUTOCMDENR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Data Transfer Direction Select"]
    #[inline]
    pub fn datdirsel(&self) -> DATDIRSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DATDIRSELR { bits }
    }
    #[doc = "Bit 5 - Multiple or Single Block Data Transfer Selection"]
    #[inline]
    pub fn multsingblksel(&self) -> MULTSINGBLKSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 5;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        MULTSINGBLKSELR { bits }
    }
    #[doc = "Bits 16:17 - Response Type Select"]
    #[inline]
    pub fn resptypesel(&self) -> RESPTYPESELR {
        RESPTYPESELR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 19 - Command CRC Check Enable"]
    #[inline]
    pub fn cmdcrcchken(&self) -> CMDCRCCHKENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 19;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CMDCRCCHKENR { bits }
    }
    #[doc = "Bit 20 - Command Index Check Enable"]
    #[inline]
    pub fn cmdindxchken(&self) -> CMDINDXCHKENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CMDINDXCHKENR { bits }
    }
    #[doc = "Bit 21 - Data Present Select"]
    #[inline]
    pub fn datpressel(&self) -> DATPRESSELR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        DATPRESSELR { bits }
    }
    #[doc = "Bits 22:23 - Command Type"]
    #[inline]
    pub fn cmdtype(&self) -> CMDTYPER {
        CMDTYPER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:29 - Command Index"]
    #[inline]
    pub fn cmdindex(&self) -> CMDINDEXR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        CMDINDEXR { bits }
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
    #[doc = "Bit 0 - DMA Enable"]
    #[inline]
    pub fn dmaen(&mut self) -> _DMAENW {
        _DMAENW { w: self }
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline]
    pub fn blkcnten(&mut self) -> _BLKCNTENW {
        _BLKCNTENW { w: self }
    }
    #[doc = "Bits 2:3 - Auto Command Enable"]
    #[inline]
    pub fn autocmden(&mut self) -> _AUTOCMDENW {
        _AUTOCMDENW { w: self }
    }
    #[doc = "Bit 4 - Data Transfer Direction Select"]
    #[inline]
    pub fn datdirsel(&mut self) -> _DATDIRSELW {
        _DATDIRSELW { w: self }
    }
    #[doc = "Bit 5 - Multiple or Single Block Data Transfer Selection"]
    #[inline]
    pub fn multsingblksel(&mut self) -> _MULTSINGBLKSELW {
        _MULTSINGBLKSELW { w: self }
    }
    #[doc = "Bits 16:17 - Response Type Select"]
    #[inline]
    pub fn resptypesel(&mut self) -> _RESPTYPESELW {
        _RESPTYPESELW { w: self }
    }
    #[doc = "Bit 19 - Command CRC Check Enable"]
    #[inline]
    pub fn cmdcrcchken(&mut self) -> _CMDCRCCHKENW {
        _CMDCRCCHKENW { w: self }
    }
    #[doc = "Bit 20 - Command Index Check Enable"]
    #[inline]
    pub fn cmdindxchken(&mut self) -> _CMDINDXCHKENW {
        _CMDINDXCHKENW { w: self }
    }
    #[doc = "Bit 21 - Data Present Select"]
    #[inline]
    pub fn datpressel(&mut self) -> _DATPRESSELW {
        _DATPRESSELW { w: self }
    }
    #[doc = "Bits 22:23 - Command Type"]
    #[inline]
    pub fn cmdtype(&mut self) -> _CMDTYPEW {
        _CMDTYPEW { w: self }
    }
    #[doc = "Bits 24:29 - Command Index"]
    #[inline]
    pub fn cmdindex(&mut self) -> _CMDINDEXW {
        _CMDINDEXW { w: self }
    }
}
