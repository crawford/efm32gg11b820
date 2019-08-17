#[doc = "Reader of register IF"]
pub type R = crate::R<u32, super::IF>;
#[doc = "Reader of field `UF`"]
pub type UF_R = crate::R<bool, bool>;
#[doc = "Reader of field `OF`"]
pub type OF_R = crate::R<bool, bool>;
#[doc = "Reader of field `DIRCNG`"]
pub type DIRCNG_R = crate::R<bool, bool>;
#[doc = "Reader of field `AUXOF`"]
pub type AUXOF_R = crate::R<bool, bool>;
#[doc = "Reader of field `TCC`"]
pub type TCC_R = crate::R<bool, bool>;
#[doc = "Reader of field `OQSTERR`"]
pub type OQSTERR_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Underflow Interrupt Read Flag"]
    #[inline(always)]
    pub fn uf(&self) -> UF_R {
        UF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Overflow Interrupt Read Flag"]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Direction Change Detect Interrupt Flag"]
    #[inline(always)]
    pub fn dircng(&self) -> DIRCNG_R {
        DIRCNG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Auxiliary Overflow Interrupt Read Flag"]
    #[inline(always)]
    pub fn auxof(&self) -> AUXOF_R {
        AUXOF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Triggered Compare Interrupt Read Flag"]
    #[inline(always)]
    pub fn tcc(&self) -> TCC_R {
        TCC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Oversampling Quadrature State Error Interrupt"]
    #[inline(always)]
    pub fn oqsterr(&self) -> OQSTERR_R {
        OQSTERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
