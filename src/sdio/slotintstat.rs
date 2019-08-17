#[doc = "Reader of register SLOTINTSTAT"]
pub type R = crate::R<u32, super::SLOTINTSTAT>;
#[doc = "Reader of field `INTSLOT0`"]
pub type INTSLOT0_R = crate::R<bool, bool>;
#[doc = "Reader of field `SPECVERNUM`"]
pub type SPECVERNUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `VENDVERNUM`"]
pub type VENDVERNUM_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bit 0 - Interrupt Signal for Slot#0"]
    #[inline(always)]
    pub fn intslot0(&self) -> INTSLOT0_R {
        INTSLOT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Host Controller Compliant Spec Version Number"]
    #[inline(always)]
    pub fn specvernum(&self) -> SPECVERNUM_R {
        SPECVERNUM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Vendor Version Number"]
    #[inline(always)]
    pub fn vendvernum(&self) -> VENDVERNUM_R {
        VENDVERNUM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
