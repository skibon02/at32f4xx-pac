#[doc = "Register `DTCTRL` reader"]
pub type R = crate::R<DTCTRL_SPEC>;
#[doc = "Register `DTCTRL` writer"]
pub type W = crate::W<DTCTRL_SPEC>;
#[doc = "DTEN\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFREN_A {
    #[doc = "0: Data transfer disabled"]
    Disabled = 0,
    #[doc = "1: Data transfer enabled. DCSM enterst the Wait_S or Wait_R state"]
    Enabled = 1,
}
impl From<TFREN_A> for bool {
    #[inline(always)]
    fn from(variant: TFREN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFREN` reader - DTEN"]
pub type TFREN_R = crate::BitReader<TFREN_A>;
impl TFREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TFREN_A {
        match self.bits {
            false => TFREN_A::Disabled,
            true => TFREN_A::Enabled,
        }
    }
    #[doc = "Data transfer disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TFREN_A::Disabled
    }
    #[doc = "Data transfer enabled. DCSM enterst the Wait_S or Wait_R state"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TFREN_A::Enabled
    }
}
#[doc = "Field `TFREN` writer - DTEN"]
pub type TFREN_W<'a, REG> = crate::BitWriter<'a, REG, TFREN_A>;
impl<'a, REG> TFREN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data transfer disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TFREN_A::Disabled)
    }
    #[doc = "Data transfer enabled. DCSM enterst the Wait_S or Wait_R state"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TFREN_A::Enabled)
    }
}
#[doc = "DTDIR\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFRDIR_A {
    #[doc = "0: Data transfer direction: to card"]
    ToCard = 0,
    #[doc = "1: Data transfer direction: from card"]
    FromCard = 1,
}
impl From<TFRDIR_A> for bool {
    #[inline(always)]
    fn from(variant: TFRDIR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFRDIR` reader - DTDIR"]
pub type TFRDIR_R = crate::BitReader<TFRDIR_A>;
impl TFRDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TFRDIR_A {
        match self.bits {
            false => TFRDIR_A::ToCard,
            true => TFRDIR_A::FromCard,
        }
    }
    #[doc = "Data transfer direction: to card"]
    #[inline(always)]
    pub fn is_to_card(&self) -> bool {
        *self == TFRDIR_A::ToCard
    }
    #[doc = "Data transfer direction: from card"]
    #[inline(always)]
    pub fn is_from_card(&self) -> bool {
        *self == TFRDIR_A::FromCard
    }
}
#[doc = "Field `TFRDIR` writer - DTDIR"]
pub type TFRDIR_W<'a, REG> = crate::BitWriter<'a, REG, TFRDIR_A>;
impl<'a, REG> TFRDIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data transfer direction: to card"]
    #[inline(always)]
    pub fn to_card(self) -> &'a mut crate::W<REG> {
        self.variant(TFRDIR_A::ToCard)
    }
    #[doc = "Data transfer direction: from card"]
    #[inline(always)]
    pub fn from_card(self) -> &'a mut crate::W<REG> {
        self.variant(TFRDIR_A::FromCard)
    }
}
#[doc = "DTMODE\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFRMODE_A {
    #[doc = "0: Block data transfer mode"]
    Block = 0,
    #[doc = "1: Stream data transfer mode"]
    Stream = 1,
}
impl From<TFRMODE_A> for bool {
    #[inline(always)]
    fn from(variant: TFRMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFRMODE` reader - DTMODE"]
pub type TFRMODE_R = crate::BitReader<TFRMODE_A>;
impl TFRMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TFRMODE_A {
        match self.bits {
            false => TFRMODE_A::Block,
            true => TFRMODE_A::Stream,
        }
    }
    #[doc = "Block data transfer mode"]
    #[inline(always)]
    pub fn is_block(&self) -> bool {
        *self == TFRMODE_A::Block
    }
    #[doc = "Stream data transfer mode"]
    #[inline(always)]
    pub fn is_stream(&self) -> bool {
        *self == TFRMODE_A::Stream
    }
}
#[doc = "Field `TFRMODE` writer - DTMODE"]
pub type TFRMODE_W<'a, REG> = crate::BitWriter<'a, REG, TFRMODE_A>;
impl<'a, REG> TFRMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Block data transfer mode"]
    #[inline(always)]
    pub fn block(self) -> &'a mut crate::W<REG> {
        self.variant(TFRMODE_A::Block)
    }
    #[doc = "Stream data transfer mode"]
    #[inline(always)]
    pub fn stream(self) -> &'a mut crate::W<REG> {
        self.variant(TFRMODE_A::Stream)
    }
}
#[doc = "DMAEN\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN_A {
    #[doc = "0: DMA disabled"]
    Disabled = 0,
    #[doc = "1: DMA enabled"]
    Enabled = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAEN` reader - DMAEN"]
pub type DMAEN_R = crate::BitReader<DMAEN_A>;
impl DMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::Disabled,
            true => DMAEN_A::Enabled,
        }
    }
    #[doc = "DMA disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN_A::Disabled
    }
    #[doc = "DMA enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN_A::Enabled
    }
}
#[doc = "Field `DMAEN` writer - DMAEN"]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG, DMAEN_A>;
impl<'a, REG> DMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN_A::Disabled)
    }
    #[doc = "DMA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMAEN_A::Enabled)
    }
}
#[doc = "DBLOCKSIZE\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BLKSIZE_A {
    #[doc = "0: 1-byte data transfer"]
    B1 = 0,
    #[doc = "1: 2-byte data transfer"]
    B2 = 1,
    #[doc = "2: 4-byte data transfer"]
    B4 = 2,
    #[doc = "3: 8-byte data transfer"]
    B8 = 3,
    #[doc = "4: 16-byte data transfer"]
    B16 = 4,
    #[doc = "5: 32-byte data transfer"]
    B32 = 5,
    #[doc = "6: 64-byte data transfer"]
    B64 = 6,
    #[doc = "7: 128-byte data transfer"]
    B128 = 7,
    #[doc = "8: 256-byte data transfer"]
    B256 = 8,
    #[doc = "9: 512-byte data transfer"]
    B512 = 9,
    #[doc = "10: 1024-byte data transfer"]
    B1024 = 10,
    #[doc = "11: 2048-byte data transfer"]
    B2048 = 11,
    #[doc = "12: 4096-byte data transfer"]
    B4096 = 12,
    #[doc = "13: 8192-byte data transfer"]
    B8192 = 13,
    #[doc = "14: 16384-byte data transfer"]
    B16384 = 14,
}
impl From<BLKSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: BLKSIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BLKSIZE_A {
    type Ux = u8;
}
impl crate::IsEnum for BLKSIZE_A {}
#[doc = "Field `BLKSIZE` reader - DBLOCKSIZE"]
pub type BLKSIZE_R = crate::FieldReader<BLKSIZE_A>;
impl BLKSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BLKSIZE_A> {
        match self.bits {
            0 => Some(BLKSIZE_A::B1),
            1 => Some(BLKSIZE_A::B2),
            2 => Some(BLKSIZE_A::B4),
            3 => Some(BLKSIZE_A::B8),
            4 => Some(BLKSIZE_A::B16),
            5 => Some(BLKSIZE_A::B32),
            6 => Some(BLKSIZE_A::B64),
            7 => Some(BLKSIZE_A::B128),
            8 => Some(BLKSIZE_A::B256),
            9 => Some(BLKSIZE_A::B512),
            10 => Some(BLKSIZE_A::B1024),
            11 => Some(BLKSIZE_A::B2048),
            12 => Some(BLKSIZE_A::B4096),
            13 => Some(BLKSIZE_A::B8192),
            14 => Some(BLKSIZE_A::B16384),
            _ => None,
        }
    }
    #[doc = "1-byte data transfer"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == BLKSIZE_A::B1
    }
    #[doc = "2-byte data transfer"]
    #[inline(always)]
    pub fn is_b2(&self) -> bool {
        *self == BLKSIZE_A::B2
    }
    #[doc = "4-byte data transfer"]
    #[inline(always)]
    pub fn is_b4(&self) -> bool {
        *self == BLKSIZE_A::B4
    }
    #[doc = "8-byte data transfer"]
    #[inline(always)]
    pub fn is_b8(&self) -> bool {
        *self == BLKSIZE_A::B8
    }
    #[doc = "16-byte data transfer"]
    #[inline(always)]
    pub fn is_b16(&self) -> bool {
        *self == BLKSIZE_A::B16
    }
    #[doc = "32-byte data transfer"]
    #[inline(always)]
    pub fn is_b32(&self) -> bool {
        *self == BLKSIZE_A::B32
    }
    #[doc = "64-byte data transfer"]
    #[inline(always)]
    pub fn is_b64(&self) -> bool {
        *self == BLKSIZE_A::B64
    }
    #[doc = "128-byte data transfer"]
    #[inline(always)]
    pub fn is_b128(&self) -> bool {
        *self == BLKSIZE_A::B128
    }
    #[doc = "256-byte data transfer"]
    #[inline(always)]
    pub fn is_b256(&self) -> bool {
        *self == BLKSIZE_A::B256
    }
    #[doc = "512-byte data transfer"]
    #[inline(always)]
    pub fn is_b512(&self) -> bool {
        *self == BLKSIZE_A::B512
    }
    #[doc = "1024-byte data transfer"]
    #[inline(always)]
    pub fn is_b1024(&self) -> bool {
        *self == BLKSIZE_A::B1024
    }
    #[doc = "2048-byte data transfer"]
    #[inline(always)]
    pub fn is_b2048(&self) -> bool {
        *self == BLKSIZE_A::B2048
    }
    #[doc = "4096-byte data transfer"]
    #[inline(always)]
    pub fn is_b4096(&self) -> bool {
        *self == BLKSIZE_A::B4096
    }
    #[doc = "8192-byte data transfer"]
    #[inline(always)]
    pub fn is_b8192(&self) -> bool {
        *self == BLKSIZE_A::B8192
    }
    #[doc = "16384-byte data transfer"]
    #[inline(always)]
    pub fn is_b16384(&self) -> bool {
        *self == BLKSIZE_A::B16384
    }
}
#[doc = "Field `BLKSIZE` writer - DBLOCKSIZE"]
pub type BLKSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 4, BLKSIZE_A>;
impl<'a, REG> BLKSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1-byte data transfer"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(BLKSIZE_A::B1)
    }
    #[doc = "2-byte data transfer"]
    #[inline(always)]
    pub fn b2(self) -> &'a mut crate::W<REG> {
        self.variant(BLKSIZE_A::B2)
    }
    #[doc = "4-byte data transfer"]
    #[inline(always)]
    pub fn b4(self) -> &'a mut crate::W<REG> {
        self.variant(BLKSIZE_A::B4)
    }
    #[doc = "8-byte data transfer"]
    #[inline(always)]
    pub fn b8(self) -> &'a mut crate::W<REG> {
        self.variant(BLKSIZE_A::B8)
    }
    #[doc = "16-byte data transfer"]
    #[inline(always)]
    pub fn b16(self) -> &'a mut crate::W<REG> {
        self.variant(BLKSIZE_A::B16)
    }
    #[doc = "32-byte data transfer"]
    #[inline(always)]
    pub fn b32(self) -> &'a mut crate::W<REG> {
        self.variant(BLKSIZE_A::B32)
    }
    #[doc = "64-byte data transfer"]
    #[inline(always)]
    pub fn b64(self) -> &'a mut crate::W<REG> {
        self.variant(BLKSIZE_A::B64)
    }
    #[doc = "128-byte data transfer"]
    #[inline(always)]
    pub fn b128(self) -> &'a mut crate::W<REG> {
        self.variant(BLKSIZE_A::B128)
    }
    #[doc = "256-byte data transfer"]
    #[inline(always)]
    pub fn b256(self) -> &'a mut crate::W<REG> {
        self.variant(BLKSIZE_A::B256)
    }
    #[doc = "512-byte data transfer"]
    #[inline(always)]
    pub fn b512(self) -> &'a mut crate::W<REG> {
        self.variant(BLKSIZE_A::B512)
    }
    #[doc = "1024-byte data transfer"]
    #[inline(always)]
    pub fn b1024(self) -> &'a mut crate::W<REG> {
        self.variant(BLKSIZE_A::B1024)
    }
    #[doc = "2048-byte data transfer"]
    #[inline(always)]
    pub fn b2048(self) -> &'a mut crate::W<REG> {
        self.variant(BLKSIZE_A::B2048)
    }
    #[doc = "4096-byte data transfer"]
    #[inline(always)]
    pub fn b4096(self) -> &'a mut crate::W<REG> {
        self.variant(BLKSIZE_A::B4096)
    }
    #[doc = "8192-byte data transfer"]
    #[inline(always)]
    pub fn b8192(self) -> &'a mut crate::W<REG> {
        self.variant(BLKSIZE_A::B8192)
    }
    #[doc = "16384-byte data transfer"]
    #[inline(always)]
    pub fn b16384(self) -> &'a mut crate::W<REG> {
        self.variant(BLKSIZE_A::B16384)
    }
}
#[doc = "PWSTART\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDWTSTART_A {
    #[doc = "0: Read wait disabled"]
    Disabled = 0,
    #[doc = "1: Read wait enabled"]
    Enabled = 1,
}
impl From<RDWTSTART_A> for bool {
    #[inline(always)]
    fn from(variant: RDWTSTART_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDWTSTART` reader - PWSTART"]
pub type RDWTSTART_R = crate::BitReader<RDWTSTART_A>;
impl RDWTSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RDWTSTART_A {
        match self.bits {
            false => RDWTSTART_A::Disabled,
            true => RDWTSTART_A::Enabled,
        }
    }
    #[doc = "Read wait disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RDWTSTART_A::Disabled
    }
    #[doc = "Read wait enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RDWTSTART_A::Enabled
    }
}
#[doc = "Field `RDWTSTART` writer - PWSTART"]
pub type RDWTSTART_W<'a, REG> = crate::BitWriter<'a, REG, RDWTSTART_A>;
impl<'a, REG> RDWTSTART_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read wait disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RDWTSTART_A::Disabled)
    }
    #[doc = "Read wait enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RDWTSTART_A::Enabled)
    }
}
#[doc = "PWSTOP\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDWTSTOP_A {
    #[doc = "0: Read wait is in progress if the RDWTSTART is set."]
    InProgress = 0,
    #[doc = "1: Read wait is stopped if the RDWTSTART is set."]
    Stopped = 1,
}
impl From<RDWTSTOP_A> for bool {
    #[inline(always)]
    fn from(variant: RDWTSTOP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDWTSTOP` reader - PWSTOP"]
pub type RDWTSTOP_R = crate::BitReader<RDWTSTOP_A>;
impl RDWTSTOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RDWTSTOP_A {
        match self.bits {
            false => RDWTSTOP_A::InProgress,
            true => RDWTSTOP_A::Stopped,
        }
    }
    #[doc = "Read wait is in progress if the RDWTSTART is set."]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == RDWTSTOP_A::InProgress
    }
    #[doc = "Read wait is stopped if the RDWTSTART is set."]
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == RDWTSTOP_A::Stopped
    }
}
#[doc = "Field `RDWTSTOP` writer - PWSTOP"]
pub type RDWTSTOP_W<'a, REG> = crate::BitWriter<'a, REG, RDWTSTOP_A>;
impl<'a, REG> RDWTSTOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read wait is in progress if the RDWTSTART is set."]
    #[inline(always)]
    pub fn in_progress(self) -> &'a mut crate::W<REG> {
        self.variant(RDWTSTOP_A::InProgress)
    }
    #[doc = "Read wait is stopped if the RDWTSTART is set."]
    #[inline(always)]
    pub fn stopped(self) -> &'a mut crate::W<REG> {
        self.variant(RDWTSTOP_A::Stopped)
    }
}
#[doc = "RWMOD\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDWTMODE_A {
    #[doc = "0: SDIO_D2 controls data read wait"]
    SdioD2 = 0,
    #[doc = "1: SDIO_CK controls data read wait"]
    SdioCk = 1,
}
impl From<RDWTMODE_A> for bool {
    #[inline(always)]
    fn from(variant: RDWTMODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDWTMODE` reader - RWMOD"]
pub type RDWTMODE_R = crate::BitReader<RDWTMODE_A>;
impl RDWTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RDWTMODE_A {
        match self.bits {
            false => RDWTMODE_A::SdioD2,
            true => RDWTMODE_A::SdioCk,
        }
    }
    #[doc = "SDIO_D2 controls data read wait"]
    #[inline(always)]
    pub fn is_sdio_d2(&self) -> bool {
        *self == RDWTMODE_A::SdioD2
    }
    #[doc = "SDIO_CK controls data read wait"]
    #[inline(always)]
    pub fn is_sdio_ck(&self) -> bool {
        *self == RDWTMODE_A::SdioCk
    }
}
#[doc = "Field `RDWTMODE` writer - RWMOD"]
pub type RDWTMODE_W<'a, REG> = crate::BitWriter<'a, REG, RDWTMODE_A>;
impl<'a, REG> RDWTMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SDIO_D2 controls data read wait"]
    #[inline(always)]
    pub fn sdio_d2(self) -> &'a mut crate::W<REG> {
        self.variant(RDWTMODE_A::SdioD2)
    }
    #[doc = "SDIO_CK controls data read wait"]
    #[inline(always)]
    pub fn sdio_ck(self) -> &'a mut crate::W<REG> {
        self.variant(RDWTMODE_A::SdioCk)
    }
}
#[doc = "SD I/O function enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOEN_A {
    #[doc = "0: SD I/O disabled"]
    Disabled = 0,
    #[doc = "1: SD I/O enabled"]
    Enabled = 1,
}
impl From<IOEN_A> for bool {
    #[inline(always)]
    fn from(variant: IOEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOEN` reader - SD I/O function enable"]
pub type IOEN_R = crate::BitReader<IOEN_A>;
impl IOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IOEN_A {
        match self.bits {
            false => IOEN_A::Disabled,
            true => IOEN_A::Enabled,
        }
    }
    #[doc = "SD I/O disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IOEN_A::Disabled
    }
    #[doc = "SD I/O enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IOEN_A::Enabled
    }
}
#[doc = "Field `IOEN` writer - SD I/O function enable"]
pub type IOEN_W<'a, REG> = crate::BitWriter<'a, REG, IOEN_A>;
impl<'a, REG> IOEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SD I/O disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IOEN_A::Disabled)
    }
    #[doc = "SD I/O enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IOEN_A::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - DTEN"]
    #[inline(always)]
    pub fn tfren(&self) -> TFREN_R {
        TFREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DTDIR"]
    #[inline(always)]
    pub fn tfrdir(&self) -> TFRDIR_R {
        TFRDIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DTMODE"]
    #[inline(always)]
    pub fn tfrmode(&self) -> TFRMODE_R {
        TFRMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - DBLOCKSIZE"]
    #[inline(always)]
    pub fn blksize(&self) -> BLKSIZE_R {
        BLKSIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - PWSTART"]
    #[inline(always)]
    pub fn rdwtstart(&self) -> RDWTSTART_R {
        RDWTSTART_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PWSTOP"]
    #[inline(always)]
    pub fn rdwtstop(&self) -> RDWTSTOP_R {
        RDWTSTOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RWMOD"]
    #[inline(always)]
    pub fn rdwtmode(&self) -> RDWTMODE_R {
        RDWTMODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SD I/O function enable"]
    #[inline(always)]
    pub fn ioen(&self) -> IOEN_R {
        IOEN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTCTRL")
            .field("tfren", &self.tfren())
            .field("tfrdir", &self.tfrdir())
            .field("tfrmode", &self.tfrmode())
            .field("dmaen", &self.dmaen())
            .field("blksize", &self.blksize())
            .field("rdwtstart", &self.rdwtstart())
            .field("rdwtstop", &self.rdwtstop())
            .field("rdwtmode", &self.rdwtmode())
            .field("ioen", &self.ioen())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - DTEN"]
    #[inline(always)]
    pub fn tfren(&mut self) -> TFREN_W<'_, DTCTRL_SPEC> {
        TFREN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DTDIR"]
    #[inline(always)]
    pub fn tfrdir(&mut self) -> TFRDIR_W<'_, DTCTRL_SPEC> {
        TFRDIR_W::new(self, 1)
    }
    #[doc = "Bit 2 - DTMODE"]
    #[inline(always)]
    pub fn tfrmode(&mut self) -> TFRMODE_W<'_, DTCTRL_SPEC> {
        TFRMODE_W::new(self, 2)
    }
    #[doc = "Bit 3 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W<'_, DTCTRL_SPEC> {
        DMAEN_W::new(self, 3)
    }
    #[doc = "Bits 4:7 - DBLOCKSIZE"]
    #[inline(always)]
    pub fn blksize(&mut self) -> BLKSIZE_W<'_, DTCTRL_SPEC> {
        BLKSIZE_W::new(self, 4)
    }
    #[doc = "Bit 8 - PWSTART"]
    #[inline(always)]
    pub fn rdwtstart(&mut self) -> RDWTSTART_W<'_, DTCTRL_SPEC> {
        RDWTSTART_W::new(self, 8)
    }
    #[doc = "Bit 9 - PWSTOP"]
    #[inline(always)]
    pub fn rdwtstop(&mut self) -> RDWTSTOP_W<'_, DTCTRL_SPEC> {
        RDWTSTOP_W::new(self, 9)
    }
    #[doc = "Bit 10 - RWMOD"]
    #[inline(always)]
    pub fn rdwtmode(&mut self) -> RDWTMODE_W<'_, DTCTRL_SPEC> {
        RDWTMODE_W::new(self, 10)
    }
    #[doc = "Bit 11 - SD I/O function enable"]
    #[inline(always)]
    pub fn ioen(&mut self) -> IOEN_W<'_, DTCTRL_SPEC> {
        IOEN_W::new(self, 11)
    }
}
#[doc = "SDIO data control register (SDIO_DCTRL)\n\nYou can [`read`](crate::Reg::read) this register and get [`dtctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTCTRL_SPEC;
impl crate::RegisterSpec for DTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtctrl::R`](R) reader structure"]
impl crate::Readable for DTCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dtctrl::W`](W) writer structure"]
impl crate::Writable for DTCTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTCTRL to value 0"]
impl crate::Resettable for DTCTRL_SPEC {}
