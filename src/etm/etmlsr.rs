#[doc = "Reader of register ETMLSR"]
pub type R = crate::R<u32, super::ETMLSR>;
#[doc = "Reader of field `LOCKIMP`"]
pub type LOCKIMP_R = crate::R<bool, bool>;
#[doc = "Reader of field `LOCKED`"]
pub type LOCKED_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - ETM Locking Implemented"]
    #[inline(always)]
    pub fn lockimp(&self) -> LOCKIMP_R {
        LOCKIMP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ETM locked"]
    #[inline(always)]
    pub fn locked(&self) -> LOCKED_R {
        LOCKED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
