#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - SDMA System Address Register"]
    pub sdmasysaddr: SDMASYSADDR,
    #[doc = "0x04 - Block Size and Block Count Register"]
    pub blksize: BLKSIZE,
    #[doc = "0x08 - SD Command Argument Register"]
    pub cmdarg1: CMDARG1,
    #[doc = "0x0c - Transfer Mode and Command Register"]
    pub tfrmode: TFRMODE,
    #[doc = "0x10 - Response0 and Response1 Register"]
    pub resp0: RESP0,
    #[doc = "0x14 - Response2 and Response3 Register"]
    pub resp2: RESP2,
    #[doc = "0x18 - Response4 and Response5 Register"]
    pub resp4: RESP4,
    #[doc = "0x1c - Response6 and Response7 Register"]
    pub resp6: RESP6,
    #[doc = "0x20 - Buffer Data Register"]
    pub bufdatport: BUFDATPORT,
    #[doc = "0x24 - Present State Register"]
    pub prsstat: PRSSTAT,
    #[doc = "0x28 - Host Control1, Power, Block Gap and Wakeup-up Control Register"]
    pub hostctrl1: HOSTCTRL1,
    #[doc = "0x2c - Clock Control, Timeout Control and Software Register"]
    pub clockctrl: CLOCKCTRL,
    #[doc = "0x30 - Normal and Error Interrupt Status Register"]
    pub ifcr: IFCR,
    #[doc = "0x34 - Normal and Error Interrupt Status Enable Register"]
    pub ifenc: IFENC,
    #[doc = "0x38 - Normal and Error Interrupt Signal Enable Register"]
    pub ien: IEN,
    #[doc = "0x3c - AUTO CMD12 Error Status and Host Control2 Register"]
    pub ac12errstat: AC12ERRSTAT,
    #[doc = "0x40 - Capabilities Register to Hold Bits 31~0"]
    pub capab0: CAPAB0,
    #[doc = "0x44 - Capabilities Register to Hold Bits 63~32"]
    pub capab2: CAPAB2,
    #[doc = "0x48 - Maximum Current Capabilities Register"]
    pub maxcurcapab: MAXCURCAPAB,
    _reserved19: [u8; 4usize],
    #[doc = "0x50 - Force Event Register for Auto CMD Error Status"]
    pub fevterrstat: FEVTERRSTAT,
    #[doc = "0x54 - ADMA Error Status Register"]
    pub admaes: ADMAES,
    #[doc = "0x58 - ADMA System Address Register"]
    pub adsaddr: ADSADDR,
    _reserved22: [u8; 4usize],
    #[doc = "0x60 - Preset Value for Initialization and Default Speed Mode"]
    pub prstval0: PRSTVAL0,
    #[doc = "0x64 - Preset Value for High Speed and SDR12 Modes"]
    pub prstval2: PRSTVAL2,
    #[doc = "0x68 - Preset Value for SDR25 and SDR50 Modes"]
    pub prstval4: PRSTVAL4,
    #[doc = "0x6c - Preset Value for SDR104 and DDR50 Modes"]
    pub prstval6: PRSTVAL6,
    #[doc = "0x70 - Boot Timeout Control Register"]
    pub boottoctrl: BOOTTOCTRL,
    _reserved27: [u8; 136usize],
    #[doc = "0xfc - Slot Interrupt Status Register"]
    pub slotintstat: SLOTINTSTAT,
    _reserved28: [u8; 1792usize],
    #[doc = "0x800 - Core Control Signals"]
    pub ctrl: CTRL,
    #[doc = "0x804 - Core Configuration 0"]
    pub cfg0: CFG0,
    #[doc = "0x808 - Core Configuration 1"]
    pub cfg1: CFG1,
    #[doc = "0x80c - Core Configuration Preset Value 0"]
    pub cfgpresetval0: CFGPRESETVAL0,
    #[doc = "0x810 - Core Configuration Preset Value 1"]
    pub cfgpresetval1: CFGPRESETVAL1,
    #[doc = "0x814 - Core Configuration Preset Value 2"]
    pub cfgpresetval2: CFGPRESETVAL2,
    #[doc = "0x818 - Core Configuration Preset Value 3"]
    pub cfgpresetval3: CFGPRESETVAL3,
    #[doc = "0x81c - I/O LOCATION Register"]
    pub routeloc0: ROUTELOC0,
    #[doc = "0x820 - I/O LOCATION Register"]
    pub routeloc1: ROUTELOC1,
    #[doc = "0x824 - I/O LOCATION Enable Register"]
    pub routepen: ROUTEPEN,
}
#[doc = "SDMA System Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [sdmasysaddr](sdmasysaddr) module"]
pub type SDMASYSADDR = crate::Reg<u32, _SDMASYSADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SDMASYSADDR;
#[doc = "`read()` method returns [sdmasysaddr::R](sdmasysaddr::R) reader structure"]
impl crate::Readable for SDMASYSADDR {}
#[doc = "`write(|w| ..)` method takes [sdmasysaddr::W](sdmasysaddr::W) writer structure"]
impl crate::Writable for SDMASYSADDR {}
#[doc = "SDMA System Address Register"]
pub mod sdmasysaddr;
#[doc = "Block Size and Block Count Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [blksize](blksize) module"]
pub type BLKSIZE = crate::Reg<u32, _BLKSIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BLKSIZE;
#[doc = "`read()` method returns [blksize::R](blksize::R) reader structure"]
impl crate::Readable for BLKSIZE {}
#[doc = "`write(|w| ..)` method takes [blksize::W](blksize::W) writer structure"]
impl crate::Writable for BLKSIZE {}
#[doc = "Block Size and Block Count Register"]
pub mod blksize;
#[doc = "SD Command Argument Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cmdarg1](cmdarg1) module"]
pub type CMDARG1 = crate::Reg<u32, _CMDARG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMDARG1;
#[doc = "`read()` method returns [cmdarg1::R](cmdarg1::R) reader structure"]
impl crate::Readable for CMDARG1 {}
#[doc = "`write(|w| ..)` method takes [cmdarg1::W](cmdarg1::W) writer structure"]
impl crate::Writable for CMDARG1 {}
#[doc = "SD Command Argument Register"]
pub mod cmdarg1;
#[doc = "Transfer Mode and Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [tfrmode](tfrmode) module"]
pub type TFRMODE = crate::Reg<u32, _TFRMODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TFRMODE;
#[doc = "`read()` method returns [tfrmode::R](tfrmode::R) reader structure"]
impl crate::Readable for TFRMODE {}
#[doc = "`write(|w| ..)` method takes [tfrmode::W](tfrmode::W) writer structure"]
impl crate::Writable for TFRMODE {}
#[doc = "Transfer Mode and Command Register"]
pub mod tfrmode;
#[doc = "Response0 and Response1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [resp0](resp0) module"]
pub type RESP0 = crate::Reg<u32, _RESP0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP0;
#[doc = "`read()` method returns [resp0::R](resp0::R) reader structure"]
impl crate::Readable for RESP0 {}
#[doc = "Response0 and Response1 Register"]
pub mod resp0;
#[doc = "Response2 and Response3 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [resp2](resp2) module"]
pub type RESP2 = crate::Reg<u32, _RESP2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP2;
#[doc = "`read()` method returns [resp2::R](resp2::R) reader structure"]
impl crate::Readable for RESP2 {}
#[doc = "Response2 and Response3 Register"]
pub mod resp2;
#[doc = "Response4 and Response5 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [resp4](resp4) module"]
pub type RESP4 = crate::Reg<u32, _RESP4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP4;
#[doc = "`read()` method returns [resp4::R](resp4::R) reader structure"]
impl crate::Readable for RESP4 {}
#[doc = "Response4 and Response5 Register"]
pub mod resp4;
#[doc = "Response6 and Response7 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [resp6](resp6) module"]
pub type RESP6 = crate::Reg<u32, _RESP6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RESP6;
#[doc = "`read()` method returns [resp6::R](resp6::R) reader structure"]
impl crate::Readable for RESP6 {}
#[doc = "Response6 and Response7 Register"]
pub mod resp6;
#[doc = "Buffer Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [bufdatport](bufdatport) module"]
pub type BUFDATPORT = crate::Reg<u32, _BUFDATPORT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BUFDATPORT;
#[doc = "`read()` method returns [bufdatport::R](bufdatport::R) reader structure"]
impl crate::Readable for BUFDATPORT {}
#[doc = "`write(|w| ..)` method takes [bufdatport::W](bufdatport::W) writer structure"]
impl crate::Writable for BUFDATPORT {}
#[doc = "Buffer Data Register"]
pub mod bufdatport;
#[doc = "Present State Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prsstat](prsstat) module"]
pub type PRSSTAT = crate::Reg<u32, _PRSSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRSSTAT;
#[doc = "`read()` method returns [prsstat::R](prsstat::R) reader structure"]
impl crate::Readable for PRSSTAT {}
#[doc = "Present State Register"]
pub mod prsstat;
#[doc = "Host Control1, Power, Block Gap and Wakeup-up Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [hostctrl1](hostctrl1) module"]
pub type HOSTCTRL1 = crate::Reg<u32, _HOSTCTRL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HOSTCTRL1;
#[doc = "`read()` method returns [hostctrl1::R](hostctrl1::R) reader structure"]
impl crate::Readable for HOSTCTRL1 {}
#[doc = "`write(|w| ..)` method takes [hostctrl1::W](hostctrl1::W) writer structure"]
impl crate::Writable for HOSTCTRL1 {}
#[doc = "Host Control1, Power, Block Gap and Wakeup-up Control Register"]
pub mod hostctrl1;
#[doc = "Clock Control, Timeout Control and Software Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [clockctrl](clockctrl) module"]
pub type CLOCKCTRL = crate::Reg<u32, _CLOCKCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLOCKCTRL;
#[doc = "`read()` method returns [clockctrl::R](clockctrl::R) reader structure"]
impl crate::Readable for CLOCKCTRL {}
#[doc = "`write(|w| ..)` method takes [clockctrl::W](clockctrl::W) writer structure"]
impl crate::Writable for CLOCKCTRL {}
#[doc = "Clock Control, Timeout Control and Software Register"]
pub mod clockctrl;
#[doc = "Normal and Error Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ifcr](ifcr) module"]
pub type IFCR = crate::Reg<u32, _IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFCR;
#[doc = "`read()` method returns [ifcr::R](ifcr::R) reader structure"]
impl crate::Readable for IFCR {}
#[doc = "`write(|w| ..)` method takes [ifcr::W](ifcr::W) writer structure"]
impl crate::Writable for IFCR {}
#[doc = "Normal and Error Interrupt Status Register"]
pub mod ifcr;
#[doc = "Normal and Error Interrupt Status Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ifenc](ifenc) module"]
pub type IFENC = crate::Reg<u32, _IFENC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFENC;
#[doc = "`read()` method returns [ifenc::R](ifenc::R) reader structure"]
impl crate::Readable for IFENC {}
#[doc = "`write(|w| ..)` method takes [ifenc::W](ifenc::W) writer structure"]
impl crate::Writable for IFENC {}
#[doc = "Normal and Error Interrupt Status Enable Register"]
pub mod ifenc;
#[doc = "Normal and Error Interrupt Signal Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ien](ien) module"]
pub type IEN = crate::Reg<u32, _IEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IEN;
#[doc = "`read()` method returns [ien::R](ien::R) reader structure"]
impl crate::Readable for IEN {}
#[doc = "`write(|w| ..)` method takes [ien::W](ien::W) writer structure"]
impl crate::Writable for IEN {}
#[doc = "Normal and Error Interrupt Signal Enable Register"]
pub mod ien;
#[doc = "AUTO CMD12 Error Status and Host Control2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ac12errstat](ac12errstat) module"]
pub type AC12ERRSTAT = crate::Reg<u32, _AC12ERRSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AC12ERRSTAT;
#[doc = "`read()` method returns [ac12errstat::R](ac12errstat::R) reader structure"]
impl crate::Readable for AC12ERRSTAT {}
#[doc = "`write(|w| ..)` method takes [ac12errstat::W](ac12errstat::W) writer structure"]
impl crate::Writable for AC12ERRSTAT {}
#[doc = "AUTO CMD12 Error Status and Host Control2 Register"]
pub mod ac12errstat;
#[doc = "Capabilities Register to Hold Bits 31~0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [capab0](capab0) module"]
pub type CAPAB0 = crate::Reg<u32, _CAPAB0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPAB0;
#[doc = "`read()` method returns [capab0::R](capab0::R) reader structure"]
impl crate::Readable for CAPAB0 {}
#[doc = "Capabilities Register to Hold Bits 31~0"]
pub mod capab0;
#[doc = "Capabilities Register to Hold Bits 63~32\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [capab2](capab2) module"]
pub type CAPAB2 = crate::Reg<u32, _CAPAB2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CAPAB2;
#[doc = "`read()` method returns [capab2::R](capab2::R) reader structure"]
impl crate::Readable for CAPAB2 {}
#[doc = "Capabilities Register to Hold Bits 63~32"]
pub mod capab2;
#[doc = "Maximum Current Capabilities Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [maxcurcapab](maxcurcapab) module"]
pub type MAXCURCAPAB = crate::Reg<u32, _MAXCURCAPAB>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAXCURCAPAB;
#[doc = "`read()` method returns [maxcurcapab::R](maxcurcapab::R) reader structure"]
impl crate::Readable for MAXCURCAPAB {}
#[doc = "Maximum Current Capabilities Register"]
pub mod maxcurcapab;
#[doc = "Force Event Register for Auto CMD Error Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [fevterrstat](fevterrstat) module"]
pub type FEVTERRSTAT = crate::Reg<u32, _FEVTERRSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FEVTERRSTAT;
#[doc = "`read()` method returns [fevterrstat::R](fevterrstat::R) reader structure"]
impl crate::Readable for FEVTERRSTAT {}
#[doc = "`write(|w| ..)` method takes [fevterrstat::W](fevterrstat::W) writer structure"]
impl crate::Writable for FEVTERRSTAT {}
#[doc = "Force Event Register for Auto CMD Error Status"]
pub mod fevterrstat;
#[doc = "ADMA Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [admaes](admaes) module"]
pub type ADMAES = crate::Reg<u32, _ADMAES>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADMAES;
#[doc = "`read()` method returns [admaes::R](admaes::R) reader structure"]
impl crate::Readable for ADMAES {}
#[doc = "ADMA Error Status Register"]
pub mod admaes;
#[doc = "ADMA System Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [adsaddr](adsaddr) module"]
pub type ADSADDR = crate::Reg<u32, _ADSADDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADSADDR;
#[doc = "`read()` method returns [adsaddr::R](adsaddr::R) reader structure"]
impl crate::Readable for ADSADDR {}
#[doc = "`write(|w| ..)` method takes [adsaddr::W](adsaddr::W) writer structure"]
impl crate::Writable for ADSADDR {}
#[doc = "ADMA System Address Register"]
pub mod adsaddr;
#[doc = "Preset Value for Initialization and Default Speed Mode\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prstval0](prstval0) module"]
pub type PRSTVAL0 = crate::Reg<u32, _PRSTVAL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRSTVAL0;
#[doc = "`read()` method returns [prstval0::R](prstval0::R) reader structure"]
impl crate::Readable for PRSTVAL0 {}
#[doc = "Preset Value for Initialization and Default Speed Mode"]
pub mod prstval0;
#[doc = "Preset Value for High Speed and SDR12 Modes\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prstval2](prstval2) module"]
pub type PRSTVAL2 = crate::Reg<u32, _PRSTVAL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRSTVAL2;
#[doc = "`read()` method returns [prstval2::R](prstval2::R) reader structure"]
impl crate::Readable for PRSTVAL2 {}
#[doc = "Preset Value for High Speed and SDR12 Modes"]
pub mod prstval2;
#[doc = "Preset Value for SDR25 and SDR50 Modes\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prstval4](prstval4) module"]
pub type PRSTVAL4 = crate::Reg<u32, _PRSTVAL4>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRSTVAL4;
#[doc = "`read()` method returns [prstval4::R](prstval4::R) reader structure"]
impl crate::Readable for PRSTVAL4 {}
#[doc = "Preset Value for SDR25 and SDR50 Modes"]
pub mod prstval4;
#[doc = "Preset Value for SDR104 and DDR50 Modes\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [prstval6](prstval6) module"]
pub type PRSTVAL6 = crate::Reg<u32, _PRSTVAL6>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRSTVAL6;
#[doc = "`read()` method returns [prstval6::R](prstval6::R) reader structure"]
impl crate::Readable for PRSTVAL6 {}
#[doc = "Preset Value for SDR104 and DDR50 Modes"]
pub mod prstval6;
#[doc = "Boot Timeout Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [boottoctrl](boottoctrl) module"]
pub type BOOTTOCTRL = crate::Reg<u32, _BOOTTOCTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BOOTTOCTRL;
#[doc = "`read()` method returns [boottoctrl::R](boottoctrl::R) reader structure"]
impl crate::Readable for BOOTTOCTRL {}
#[doc = "`write(|w| ..)` method takes [boottoctrl::W](boottoctrl::W) writer structure"]
impl crate::Writable for BOOTTOCTRL {}
#[doc = "Boot Timeout Control Register"]
pub mod boottoctrl;
#[doc = "Slot Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [slotintstat](slotintstat) module"]
pub type SLOTINTSTAT = crate::Reg<u32, _SLOTINTSTAT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SLOTINTSTAT;
#[doc = "`read()` method returns [slotintstat::R](slotintstat::R) reader structure"]
impl crate::Readable for SLOTINTSTAT {}
#[doc = "Slot Interrupt Status Register"]
pub mod slotintstat;
#[doc = "Core Control Signals\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ctrl](ctrl) module"]
pub type CTRL = crate::Reg<u32, _CTRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTRL;
#[doc = "`read()` method returns [ctrl::R](ctrl::R) reader structure"]
impl crate::Readable for CTRL {}
#[doc = "`write(|w| ..)` method takes [ctrl::W](ctrl::W) writer structure"]
impl crate::Writable for CTRL {}
#[doc = "Core Control Signals"]
pub mod ctrl;
#[doc = "Core Configuration 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cfg0](cfg0) module"]
pub type CFG0 = crate::Reg<u32, _CFG0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG0;
#[doc = "`read()` method returns [cfg0::R](cfg0::R) reader structure"]
impl crate::Readable for CFG0 {}
#[doc = "`write(|w| ..)` method takes [cfg0::W](cfg0::W) writer structure"]
impl crate::Writable for CFG0 {}
#[doc = "Core Configuration 0"]
pub mod cfg0;
#[doc = "Core Configuration 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cfg1](cfg1) module"]
pub type CFG1 = crate::Reg<u32, _CFG1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFG1;
#[doc = "`read()` method returns [cfg1::R](cfg1::R) reader structure"]
impl crate::Readable for CFG1 {}
#[doc = "`write(|w| ..)` method takes [cfg1::W](cfg1::W) writer structure"]
impl crate::Writable for CFG1 {}
#[doc = "Core Configuration 1"]
pub mod cfg1;
#[doc = "Core Configuration Preset Value 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cfgpresetval0](cfgpresetval0) module"]
pub type CFGPRESETVAL0 = crate::Reg<u32, _CFGPRESETVAL0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGPRESETVAL0;
#[doc = "`read()` method returns [cfgpresetval0::R](cfgpresetval0::R) reader structure"]
impl crate::Readable for CFGPRESETVAL0 {}
#[doc = "`write(|w| ..)` method takes [cfgpresetval0::W](cfgpresetval0::W) writer structure"]
impl crate::Writable for CFGPRESETVAL0 {}
#[doc = "Core Configuration Preset Value 0"]
pub mod cfgpresetval0;
#[doc = "Core Configuration Preset Value 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cfgpresetval1](cfgpresetval1) module"]
pub type CFGPRESETVAL1 = crate::Reg<u32, _CFGPRESETVAL1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGPRESETVAL1;
#[doc = "`read()` method returns [cfgpresetval1::R](cfgpresetval1::R) reader structure"]
impl crate::Readable for CFGPRESETVAL1 {}
#[doc = "`write(|w| ..)` method takes [cfgpresetval1::W](cfgpresetval1::W) writer structure"]
impl crate::Writable for CFGPRESETVAL1 {}
#[doc = "Core Configuration Preset Value 1"]
pub mod cfgpresetval1;
#[doc = "Core Configuration Preset Value 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cfgpresetval2](cfgpresetval2) module"]
pub type CFGPRESETVAL2 = crate::Reg<u32, _CFGPRESETVAL2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGPRESETVAL2;
#[doc = "`read()` method returns [cfgpresetval2::R](cfgpresetval2::R) reader structure"]
impl crate::Readable for CFGPRESETVAL2 {}
#[doc = "`write(|w| ..)` method takes [cfgpresetval2::W](cfgpresetval2::W) writer structure"]
impl crate::Writable for CFGPRESETVAL2 {}
#[doc = "Core Configuration Preset Value 2"]
pub mod cfgpresetval2;
#[doc = "Core Configuration Preset Value 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [cfgpresetval3](cfgpresetval3) module"]
pub type CFGPRESETVAL3 = crate::Reg<u32, _CFGPRESETVAL3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CFGPRESETVAL3;
#[doc = "`read()` method returns [cfgpresetval3::R](cfgpresetval3::R) reader structure"]
impl crate::Readable for CFGPRESETVAL3 {}
#[doc = "`write(|w| ..)` method takes [cfgpresetval3::W](cfgpresetval3::W) writer structure"]
impl crate::Writable for CFGPRESETVAL3 {}
#[doc = "Core Configuration Preset Value 3"]
pub mod cfgpresetval3;
#[doc = "I/O LOCATION Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [routeloc0](routeloc0) module"]
pub type ROUTELOC0 = crate::Reg<u32, _ROUTELOC0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROUTELOC0;
#[doc = "`read()` method returns [routeloc0::R](routeloc0::R) reader structure"]
impl crate::Readable for ROUTELOC0 {}
#[doc = "`write(|w| ..)` method takes [routeloc0::W](routeloc0::W) writer structure"]
impl crate::Writable for ROUTELOC0 {}
#[doc = "I/O LOCATION Register"]
pub mod routeloc0;
#[doc = "I/O LOCATION Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [routeloc1](routeloc1) module"]
pub type ROUTELOC1 = crate::Reg<u32, _ROUTELOC1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROUTELOC1;
#[doc = "`read()` method returns [routeloc1::R](routeloc1::R) reader structure"]
impl crate::Readable for ROUTELOC1 {}
#[doc = "`write(|w| ..)` method takes [routeloc1::W](routeloc1::W) writer structure"]
impl crate::Writable for ROUTELOC1 {}
#[doc = "I/O LOCATION Register"]
pub mod routeloc1;
#[doc = "I/O LOCATION Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [routepen](routepen) module"]
pub type ROUTEPEN = crate::Reg<u32, _ROUTEPEN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ROUTEPEN;
#[doc = "`read()` method returns [routepen::R](routepen::R) reader structure"]
impl crate::Readable for ROUTEPEN {}
#[doc = "`write(|w| ..)` method takes [routepen::W](routepen::W) writer structure"]
impl crate::Writable for ROUTEPEN {}
#[doc = "I/O LOCATION Enable Register"]
pub mod routepen;
