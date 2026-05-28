#[doc = "Register `MACMIIADDR` reader"]
pub type R = crate::R<MACMIIADDR_SPEC>;
#[doc = "Register `MACMIIADDR` writer"]
pub type W = crate::W<MACMIIADDR_SPEC>;
#[doc = "MII busy\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mbr {
    #[doc = "0: The MII is not busy, new operation can be started"]
    NoBusy = 0,
    #[doc = "1: The MII is busy, operation in progress"]
    Busy = 1,
}
impl From<Mbr> for bool {
    #[inline(always)]
    fn from(variant: Mbr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB` reader - MII busy"]
pub type MB_R = crate::BitReader<Mbr>;
impl MB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mbr {
        match self.bits {
            false => Mbr::NoBusy,
            true => Mbr::Busy,
        }
    }
    #[doc = "The MII is not busy, new operation can be started"]
    #[inline(always)]
    pub fn is_no_busy(&self) -> bool {
        *self == Mbr::NoBusy
    }
    #[doc = "The MII is busy, operation in progress"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Mbr::Busy
    }
}
#[doc = "MII busy\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MbwWO {
    #[doc = "0: The MII is not busy, new operation can be started"]
    NoBusy = 0,
    #[doc = "1: Start an MII operation by writing 1 to this bit. This bit is automatically cleared by hardware when the operation is completed"]
    StartOperation = 1,
}
impl From<MbwWO> for bool {
    #[inline(always)]
    fn from(variant: MbwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MB` writer - MII busy"]
pub type MB_W<'a, REG> = crate::BitWriter<'a, REG, MbwWO>;
impl<'a, REG> MB_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The MII is not busy, new operation can be started"]
    #[inline(always)]
    pub fn no_busy(self) -> &'a mut crate::W<REG> {
        self.variant(MbwWO::NoBusy)
    }
    #[doc = "Start an MII operation by writing 1 to this bit. This bit is automatically cleared by hardware when the operation is completed"]
    #[inline(always)]
    pub fn start_operation(self) -> &'a mut crate::W<REG> {
        self.variant(MbwWO::StartOperation)
    }
}
#[doc = "MII write\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MW_A {
    #[doc = "0: Write operation"]
    Write = 0,
    #[doc = "1: Read operation"]
    Read = 1,
}
impl From<MW_A> for bool {
    #[inline(always)]
    fn from(variant: MW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MW` reader - MII write"]
pub type MW_R = crate::BitReader<MW_A>;
impl MW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MW_A {
        match self.bits {
            false => MW_A::Write,
            true => MW_A::Read,
        }
    }
    #[doc = "Write operation"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == MW_A::Write
    }
    #[doc = "Read operation"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == MW_A::Read
    }
}
#[doc = "Field `MW` writer - MII write"]
pub type MW_W<'a, REG> = crate::BitWriter<'a, REG, MW_A>;
impl<'a, REG> MW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write operation"]
    #[inline(always)]
    pub fn write(self) -> &'a mut crate::W<REG> {
        self.variant(MW_A::Write)
    }
    #[doc = "Read operation"]
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(MW_A::Read)
    }
}
#[doc = "Clock range\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CR_A {
    #[doc = "0: CSR clock frequency is 60-100 MHz and MDC clock frequency is CSR/42 MHz"]
    CsrFreq60_100 = 0,
    #[doc = "1: CSR clock frequency is 100-150 MHz and MDC clock frequency is CSR/62 MHz"]
    CsrFreq100_150 = 1,
    #[doc = "2: CSR clock frequency is 20-35 MHz and MDC clock frequency is CSR/16 MHz"]
    CsrFreq20_35 = 2,
    #[doc = "3: CSR clock frequency is 35-60 MHz and MDC clock frequency is CSR/26 MHz"]
    CsrFreq35_60 = 3,
    #[doc = "4: CSR clock frequency is 150-250 MHz and MDC clock frequency is CSR/102 MHz"]
    CsrFreq150_250 = 4,
    #[doc = "5: CSR clock frequency is 250-288 MHz and MDC clock frequency is CSR/124 MHz"]
    CsrFreq250_288 = 5,
}
impl From<CR_A> for u8 {
    #[inline(always)]
    fn from(variant: CR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CR_A {
    type Ux = u8;
}
impl crate::IsEnum for CR_A {}
#[doc = "Field `CR` reader - Clock range"]
pub type CR_R = crate::FieldReader<CR_A>;
impl CR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CR_A> {
        match self.bits {
            0 => Some(CR_A::CsrFreq60_100),
            1 => Some(CR_A::CsrFreq100_150),
            2 => Some(CR_A::CsrFreq20_35),
            3 => Some(CR_A::CsrFreq35_60),
            4 => Some(CR_A::CsrFreq150_250),
            5 => Some(CR_A::CsrFreq250_288),
            _ => None,
        }
    }
    #[doc = "CSR clock frequency is 60-100 MHz and MDC clock frequency is CSR/42 MHz"]
    #[inline(always)]
    pub fn is_csr_freq60_100(&self) -> bool {
        *self == CR_A::CsrFreq60_100
    }
    #[doc = "CSR clock frequency is 100-150 MHz and MDC clock frequency is CSR/62 MHz"]
    #[inline(always)]
    pub fn is_csr_freq100_150(&self) -> bool {
        *self == CR_A::CsrFreq100_150
    }
    #[doc = "CSR clock frequency is 20-35 MHz and MDC clock frequency is CSR/16 MHz"]
    #[inline(always)]
    pub fn is_csr_freq20_35(&self) -> bool {
        *self == CR_A::CsrFreq20_35
    }
    #[doc = "CSR clock frequency is 35-60 MHz and MDC clock frequency is CSR/26 MHz"]
    #[inline(always)]
    pub fn is_csr_freq35_60(&self) -> bool {
        *self == CR_A::CsrFreq35_60
    }
    #[doc = "CSR clock frequency is 150-250 MHz and MDC clock frequency is CSR/102 MHz"]
    #[inline(always)]
    pub fn is_csr_freq150_250(&self) -> bool {
        *self == CR_A::CsrFreq150_250
    }
    #[doc = "CSR clock frequency is 250-288 MHz and MDC clock frequency is CSR/124 MHz"]
    #[inline(always)]
    pub fn is_csr_freq250_288(&self) -> bool {
        *self == CR_A::CsrFreq250_288
    }
}
#[doc = "Field `CR` writer - Clock range"]
pub type CR_W<'a, REG> = crate::FieldWriter<'a, REG, 3, CR_A>;
impl<'a, REG> CR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CSR clock frequency is 60-100 MHz and MDC clock frequency is CSR/42 MHz"]
    #[inline(always)]
    pub fn csr_freq60_100(self) -> &'a mut crate::W<REG> {
        self.variant(CR_A::CsrFreq60_100)
    }
    #[doc = "CSR clock frequency is 100-150 MHz and MDC clock frequency is CSR/62 MHz"]
    #[inline(always)]
    pub fn csr_freq100_150(self) -> &'a mut crate::W<REG> {
        self.variant(CR_A::CsrFreq100_150)
    }
    #[doc = "CSR clock frequency is 20-35 MHz and MDC clock frequency is CSR/16 MHz"]
    #[inline(always)]
    pub fn csr_freq20_35(self) -> &'a mut crate::W<REG> {
        self.variant(CR_A::CsrFreq20_35)
    }
    #[doc = "CSR clock frequency is 35-60 MHz and MDC clock frequency is CSR/26 MHz"]
    #[inline(always)]
    pub fn csr_freq35_60(self) -> &'a mut crate::W<REG> {
        self.variant(CR_A::CsrFreq35_60)
    }
    #[doc = "CSR clock frequency is 150-250 MHz and MDC clock frequency is CSR/102 MHz"]
    #[inline(always)]
    pub fn csr_freq150_250(self) -> &'a mut crate::W<REG> {
        self.variant(CR_A::CsrFreq150_250)
    }
    #[doc = "CSR clock frequency is 250-288 MHz and MDC clock frequency is CSR/124 MHz"]
    #[inline(always)]
    pub fn csr_freq250_288(self) -> &'a mut crate::W<REG> {
        self.variant(CR_A::CsrFreq250_288)
    }
}
#[doc = "Field `MII` reader - MII register"]
pub type MII_R = crate::FieldReader;
#[doc = "Field `MII` writer - MII register"]
pub type MII_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
#[doc = "Field `PA` reader - PHY address"]
pub type PA_R = crate::FieldReader;
#[doc = "Field `PA` writer - PHY address"]
pub type PA_W<'a, REG> = crate::FieldWriter<'a, REG, 5, u8, crate::Safe>;
impl R {
    #[doc = "Bit 0 - MII busy"]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MII write"]
    #[inline(always)]
    pub fn mw(&self) -> MW_R {
        MW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Clock range"]
    #[inline(always)]
    pub fn cr(&self) -> CR_R {
        CR_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 6:10 - MII register"]
    #[inline(always)]
    pub fn mii(&self) -> MII_R {
        MII_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:15 - PHY address"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MACMIIADDR")
            .field("mb", &self.mb())
            .field("mw", &self.mw())
            .field("cr", &self.cr())
            .field("mii", &self.mii())
            .field("pa", &self.pa())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - MII busy"]
    #[inline(always)]
    pub fn mb(&mut self) -> MB_W<'_, MACMIIADDR_SPEC> {
        MB_W::new(self, 0)
    }
    #[doc = "Bit 1 - MII write"]
    #[inline(always)]
    pub fn mw(&mut self) -> MW_W<'_, MACMIIADDR_SPEC> {
        MW_W::new(self, 1)
    }
    #[doc = "Bits 2:4 - Clock range"]
    #[inline(always)]
    pub fn cr(&mut self) -> CR_W<'_, MACMIIADDR_SPEC> {
        CR_W::new(self, 2)
    }
    #[doc = "Bits 6:10 - MII register"]
    #[inline(always)]
    pub fn mii(&mut self) -> MII_W<'_, MACMIIADDR_SPEC> {
        MII_W::new(self, 6)
    }
    #[doc = "Bits 11:15 - PHY address"]
    #[inline(always)]
    pub fn pa(&mut self) -> PA_W<'_, MACMIIADDR_SPEC> {
        PA_W::new(self, 11)
    }
}
#[doc = "Ethernet MAC MII address register\n\nYou can [`read`](crate::Reg::read) this register and get [`macmiiaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macmiiaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACMIIADDR_SPEC;
impl crate::RegisterSpec for MACMIIADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macmiiaddr::R`](R) reader structure"]
impl crate::Readable for MACMIIADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`macmiiaddr::W`](W) writer structure"]
impl crate::Writable for MACMIIADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACMIIADDR to value 0"]
impl crate::Resettable for MACMIIADDR_SPEC {}
