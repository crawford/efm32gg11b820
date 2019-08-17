#[doc = "Reader of register IF"]
pub type R = crate::R<u32, super::IF>;
#[doc = "Reader of field `VSYNC`"]
pub type VSYNC_R = crate::R<bool, bool>;
#[doc = "Reader of field `HSYNC`"]
pub type HSYNC_R = crate::R<bool, bool>;
#[doc = "Reader of field `VBPORCH`"]
pub type VBPORCH_R = crate::R<bool, bool>;
#[doc = "Reader of field `VFPORCH`"]
pub type VFPORCH_R = crate::R<bool, bool>;
#[doc = "Reader of field `DDEMPTY`"]
pub type DDEMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `DDJIT`"]
pub type DDJIT_R = crate::R<bool, bool>;
#[doc = "Reader of field `TFTPIXEL0EMPTY`"]
pub type TFTPIXEL0EMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `TFTPIXEL1EMPTY`"]
pub type TFTPIXEL1EMPTY_R = crate::R<bool, bool>;
#[doc = "Reader of field `TFTPIXELFULL`"]
pub type TFTPIXELFULL_R = crate::R<bool, bool>;
#[doc = "Reader of field `TFTPIXELOF`"]
pub type TFTPIXELOF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - Vertical Sync Interrupt Flag"]
    #[inline(always)]
    pub fn vsync(&self) -> VSYNC_R {
        VSYNC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Horizontal Sync Interrupt Flag"]
    #[inline(always)]
    pub fn hsync(&self) -> HSYNC_R {
        HSYNC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Vertical Back Porch Interrupt Flag"]
    #[inline(always)]
    pub fn vbporch(&self) -> VBPORCH_R {
        VBPORCH_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Vertical Front Porch Interrupt Flag"]
    #[inline(always)]
    pub fn vfporch(&self) -> VFPORCH_R {
        VFPORCH_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Direct Drive Data Empty Interrupt Flag"]
    #[inline(always)]
    pub fn ddempty(&self) -> DDEMPTY_R {
        DDEMPTY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Direct Drive Jitter Interrupt Flag"]
    #[inline(always)]
    pub fn ddjit(&self) -> DDJIT_R {
        DDJIT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - EBI_TFTPIXEL0 is Empty Interrupt Flag"]
    #[inline(always)]
    pub fn tftpixel0empty(&self) -> TFTPIXEL0EMPTY_R {
        TFTPIXEL0EMPTY_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - EBI_TFTPIXEL1 is Empty Interrupt Flag"]
    #[inline(always)]
    pub fn tftpixel1empty(&self) -> TFTPIXEL1EMPTY_R {
        TFTPIXEL1EMPTY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - EBI_TFTPIXEL is Full Interrupt Flag"]
    #[inline(always)]
    pub fn tftpixelfull(&self) -> TFTPIXELFULL_R {
        TFTPIXELFULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - EBI_TFTPIXEL Register Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn tftpixelof(&self) -> TFTPIXELOF_R {
        TFTPIXELOF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
