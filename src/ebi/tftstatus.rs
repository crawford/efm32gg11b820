#[doc = "Reader of register TFTSTATUS"]
pub type R = crate::R<u32, super::TFTSTATUS>;
#[doc = "Reader of field `HCNT`"]
pub type HCNT_R = crate::R<u16, u16>;
#[doc = "Reader of field `VCNT`"]
pub type VCNT_R = crate::R<u16, u16>;
impl R {
    #[doc = "Bits 0:10 - Horizontal Count"]
    #[inline(always)]
    pub fn hcnt(&self) -> HCNT_R {
        HCNT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:30 - Vertical Count"]
    #[inline(always)]
    pub fn vcnt(&self) -> VCNT_R {
        VCNT_R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
