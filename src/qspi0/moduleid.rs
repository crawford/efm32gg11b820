#[doc = "Reader of register MODULEID"]
pub type R = crate::R<u32, super::MODULEID>;
#[doc = "Reader of field `CONF`"]
pub type CONF_R = crate::R<u8, u8>;
#[doc = "Reader of field `MODULEID`"]
pub type MODULEID_R = crate::R<u16, u16>;
#[doc = "Reader of field `FIXPATCH`"]
pub type FIXPATCH_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - Configuration ID Number"]
    #[inline(always)]
    pub fn conf(&self) -> CONF_R {
        CONF_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 8:23 - Module/Revision ID Number"]
    #[inline(always)]
    pub fn moduleid(&self) -> MODULEID_R {
        MODULEID_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 24:31 - Fix/patch Number"]
    #[inline(always)]
    pub fn fixpatch(&self) -> FIXPATCH_R {
        FIXPATCH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
