#[doc = "Reader of register IF"]
pub type R = crate::R<u32, super::IF>;
#[doc = "Reader of field `ERASE`"]
pub type ERASE_R = crate::R<bool, bool>;
#[doc = "Reader of field `WRITE`"]
pub type WRITE_R = crate::R<bool, bool>;
#[doc = "Reader of field `CHOF`"]
pub type CHOF_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMOF`"]
pub type CMOF_R = crate::R<bool, bool>;
#[doc = "Reader of field `PWRUPF`"]
pub type PWRUPF_R = crate::R<bool, bool>;
#[doc = "Reader of field `ICACHERR`"]
pub type ICACHERR_R = crate::R<bool, bool>;
#[doc = "Reader of field `WDATAOV`"]
pub type WDATAOV_R = crate::R<bool, bool>;
#[doc = "Reader of field `LVEWRITE`"]
pub type LVEWRITE_R = crate::R<bool, bool>;
#[doc = "Reader of field `RAMERR1B`"]
pub type RAMERR1B_R = crate::R<bool, bool>;
#[doc = "Reader of field `RAMERR2B`"]
pub type RAMERR2B_R = crate::R<bool, bool>;
#[doc = "Reader of field `RAM1ERR1B`"]
pub type RAM1ERR1B_R = crate::R<bool, bool>;
#[doc = "Reader of field `RAM1ERR2B`"]
pub type RAM1ERR2B_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Erase Done Interrupt Read Flag"]
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Write Done Interrupt Read Flag"]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Cache Hits Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn chof(&self) -> CHOF_R {
        CHOF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Cache Misses Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn cmof(&self) -> CMOF_R {
        CMOF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Flash Power Up Sequence Complete Flag"]
    #[inline(always)]
    pub fn pwrupf(&self) -> PWRUPF_R {
        PWRUPF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - ICache RAM Parity Error Flag"]
    #[inline(always)]
    pub fn icacherr(&self) -> ICACHERR_R {
        ICACHERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Flash Controller Write Buffer Overflow"]
    #[inline(always)]
    pub fn wdataov(&self) -> WDATAOV_R {
        WDATAOV_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Flash LVE Write Error Flag"]
    #[inline(always)]
    pub fn lvewrite(&self) -> LVEWRITE_R {
        LVEWRITE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RAM 1-bit ECC Error Interrupt Flag"]
    #[inline(always)]
    pub fn ramerr1b(&self) -> RAMERR1B_R {
        RAMERR1B_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RAM 2-bit ECC Error Interrupt Flag"]
    #[inline(always)]
    pub fn ramerr2b(&self) -> RAMERR2B_R {
        RAMERR2B_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - RAM1 1-bit ECC Error Interrupt Flag"]
    #[inline(always)]
    pub fn ram1err1b(&self) -> RAM1ERR1B_R {
        RAM1ERR1B_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - RAM1 2-bit ECC Error Interrupt Flag"]
    #[inline(always)]
    pub fn ram1err2b(&self) -> RAM1ERR2B_R {
        RAM1ERR2B_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
