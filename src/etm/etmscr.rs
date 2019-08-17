#[doc = "Reader of register ETMSCR"]
pub type R = crate::R<u32, super::ETMSCR>;
#[doc = "Reader of field `MAXPORTSIZE`"]
pub type MAXPORTSIZE_R = crate::R<u8, u8>;
#[doc = "Reader of field `FIFOFULL`"]
pub type FIFOFULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `MAXPORTSIZE3`"]
pub type MAXPORTSIZE3_R = crate::R<bool, bool>;
#[doc = "Reader of field `PORTSIZE`"]
pub type PORTSIZE_R = crate::R<bool, bool>;
#[doc = "Reader of field `PORTMODE`"]
pub type PORTMODE_R = crate::R<bool, bool>;
#[doc = "Reader of field `PROCNUM`"]
pub type PROCNUM_R = crate::R<u8, u8>;
#[doc = "Reader of field `NOFETCHCOMP`"]
pub type NOFETCHCOMP_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bits 0:2 - Maximum Port Size"]
    #[inline(always)]
    pub fn maxportsize(&self) -> MAXPORTSIZE_R {
        MAXPORTSIZE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 8 - FIFO FULL Supported"]
    #[inline(always)]
    pub fn fifofull(&self) -> FIFOFULL_R {
        FIFOFULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Max Port Size\\[3\\]"]
    #[inline(always)]
    pub fn maxportsize3(&self) -> MAXPORTSIZE3_R {
        MAXPORTSIZE3_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Port Size Supported"]
    #[inline(always)]
    pub fn portsize(&self) -> PORTSIZE_R {
        PORTSIZE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Port Mode Supported"]
    #[inline(always)]
    pub fn portmode(&self) -> PORTMODE_R {
        PORTMODE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 12:14 - Number of Supported Processros"]
    #[inline(always)]
    pub fn procnum(&self) -> PROCNUM_R {
        PROCNUM_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 17 - No Fetch Comparison"]
    #[inline(always)]
    pub fn nofetchcomp(&self) -> NOFETCHCOMP_R {
        NOFETCHCOMP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
