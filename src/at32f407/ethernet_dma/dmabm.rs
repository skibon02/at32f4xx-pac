#[doc = "Register `DMABM` reader"]
pub type R = crate::R<DMABM_SPEC>;
#[doc = "Register `DMABM` writer"]
pub type W = crate::W<DMABM_SPEC>;
#[doc = "Software reset\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Swrr {
    #[doc = "0: Software reset is not active"]
    NoReset = 0,
    #[doc = "1: Software reset is active. This flag will be cleared when valid clocks are detected on appropriate pins"]
    Resetting = 1,
}
impl From<Swrr> for bool {
    #[inline(always)]
    fn from(variant: Swrr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWR` reader - Software reset"]
pub type SWR_R = crate::BitReader<Swrr>;
impl SWR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Swrr {
        match self.bits {
            false => Swrr::NoReset,
            true => Swrr::Resetting,
        }
    }
    #[doc = "Software reset is not active"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == Swrr::NoReset
    }
    #[doc = "Software reset is active. This flag will be cleared when valid clocks are detected on appropriate pins"]
    #[inline(always)]
    pub fn is_resetting(&self) -> bool {
        *self == Swrr::Resetting
    }
}
#[doc = "Software reset\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SwrwWO {
    #[doc = "0: No reset"]
    NoReset = 0,
    #[doc = "1: Begin reset. Set this bit to reset the DMA"]
    StartReset = 1,
}
impl From<SwrwWO> for bool {
    #[inline(always)]
    fn from(variant: SwrwWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWR` writer - Software reset"]
pub type SWR_W<'a, REG> = crate::BitWriter<'a, REG, SwrwWO>;
impl<'a, REG> SWR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No reset"]
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut crate::W<REG> {
        self.variant(SwrwWO::NoReset)
    }
    #[doc = "Begin reset. Set this bit to reset the DMA"]
    #[inline(always)]
    pub fn start_reset(self) -> &'a mut crate::W<REG> {
        self.variant(SwrwWO::StartReset)
    }
}
#[doc = "DMA Arbitration\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DA_A {
    #[doc = "0: Custom priority ration between RX and TX. Controlled by PR"]
    CustomPriority = 0,
    #[doc = "1: Tx has priority over Rx. Or otherwise, depending on bit 27 (TXPR)"]
    FixedPriority = 1,
}
impl From<DA_A> for bool {
    #[inline(always)]
    fn from(variant: DA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DA` reader - DMA Arbitration"]
pub type DA_R = crate::BitReader<DA_A>;
impl DA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DA_A {
        match self.bits {
            false => DA_A::CustomPriority,
            true => DA_A::FixedPriority,
        }
    }
    #[doc = "Custom priority ration between RX and TX. Controlled by PR"]
    #[inline(always)]
    pub fn is_custom_priority(&self) -> bool {
        *self == DA_A::CustomPriority
    }
    #[doc = "Tx has priority over Rx. Or otherwise, depending on bit 27 (TXPR)"]
    #[inline(always)]
    pub fn is_fixed_priority(&self) -> bool {
        *self == DA_A::FixedPriority
    }
}
#[doc = "Field `DA` writer - DMA Arbitration"]
pub type DA_W<'a, REG> = crate::BitWriter<'a, REG, DA_A>;
impl<'a, REG> DA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Custom priority ration between RX and TX. Controlled by PR"]
    #[inline(always)]
    pub fn custom_priority(self) -> &'a mut crate::W<REG> {
        self.variant(DA_A::CustomPriority)
    }
    #[doc = "Tx has priority over Rx. Or otherwise, depending on bit 27 (TXPR)"]
    #[inline(always)]
    pub fn fixed_priority(self) -> &'a mut crate::W<REG> {
        self.variant(DA_A::FixedPriority)
    }
}
#[doc = "Field `DSL` reader - Descriptor skip length"]
pub type DSL_R = crate::FieldReader;
#[doc = "Field `DSL` writer - Descriptor skip length"]
pub type DSL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PBL` reader - Programmable burst length"]
pub type PBL_R = crate::FieldReader;
#[doc = "Field `PBL` writer - Programmable burst length"]
pub type PBL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Priority ratio\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PR_A {
    #[doc = "0: 1:1"]
    Ratio1_1 = 0,
    #[doc = "1: 1:2"]
    Ratio1_2 = 1,
    #[doc = "2: 1:3"]
    Ratio1_3 = 2,
    #[doc = "3: 1:4"]
    Ratio1_4 = 3,
}
impl From<PR_A> for u8 {
    #[inline(always)]
    fn from(variant: PR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PR_A {
    type Ux = u8;
}
impl crate::IsEnum for PR_A {}
#[doc = "Field `PR` reader - Priority ratio"]
pub type PR_R = crate::FieldReader<PR_A>;
impl PR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PR_A {
        match self.bits {
            0 => PR_A::Ratio1_1,
            1 => PR_A::Ratio1_2,
            2 => PR_A::Ratio1_3,
            3 => PR_A::Ratio1_4,
            _ => unreachable!(),
        }
    }
    #[doc = "1:1"]
    #[inline(always)]
    pub fn is_ratio1_1(&self) -> bool {
        *self == PR_A::Ratio1_1
    }
    #[doc = "1:2"]
    #[inline(always)]
    pub fn is_ratio1_2(&self) -> bool {
        *self == PR_A::Ratio1_2
    }
    #[doc = "1:3"]
    #[inline(always)]
    pub fn is_ratio1_3(&self) -> bool {
        *self == PR_A::Ratio1_3
    }
    #[doc = "1:4"]
    #[inline(always)]
    pub fn is_ratio1_4(&self) -> bool {
        *self == PR_A::Ratio1_4
    }
}
#[doc = "Field `PR` writer - Priority ratio"]
pub type PR_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PR_A, crate::Safe>;
impl<'a, REG> PR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1:1"]
    #[inline(always)]
    pub fn ratio1_1(self) -> &'a mut crate::W<REG> {
        self.variant(PR_A::Ratio1_1)
    }
    #[doc = "1:2"]
    #[inline(always)]
    pub fn ratio1_2(self) -> &'a mut crate::W<REG> {
        self.variant(PR_A::Ratio1_2)
    }
    #[doc = "1:3"]
    #[inline(always)]
    pub fn ratio1_3(self) -> &'a mut crate::W<REG> {
        self.variant(PR_A::Ratio1_3)
    }
    #[doc = "1:4"]
    #[inline(always)]
    pub fn ratio1_4(self) -> &'a mut crate::W<REG> {
        self.variant(PR_A::Ratio1_4)
    }
}
#[doc = "Field `FB` reader - Fixed burst"]
pub type FB_R = crate::BitReader;
#[doc = "Field `FB` writer - Fixed burst"]
pub type FB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDP` reader - Rx DMA PBL"]
pub type RDP_R = crate::FieldReader;
#[doc = "Field `RDP` writer - Rx DMA PBL"]
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `USP` reader - Use separate PBL"]
pub type USP_R = crate::BitReader;
#[doc = "Field `USP` writer - Use separate PBL"]
pub type USP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBLx8` reader - PNLx8 mode"]
pub type PBLX8_R = crate::BitReader;
#[doc = "Field `PBLx8` writer - PNLx8 mode"]
pub type PBLX8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AAB` reader - Address-aligned beats"]
pub type AAB_R = crate::BitReader;
#[doc = "Field `AAB` writer - Address-aligned beats"]
pub type AAB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software reset"]
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Arbitration"]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Descriptor skip length"]
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 8:13 - Programmable burst length"]
    #[inline(always)]
    pub fn pbl(&self) -> PBL_R {
        PBL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - Priority ratio"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Fixed burst"]
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:22 - Rx DMA PBL"]
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Use separate PBL"]
    #[inline(always)]
    pub fn usp(&self) -> USP_R {
        USP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PNLx8 mode"]
    #[inline(always)]
    pub fn pblx8(&self) -> PBLX8_R {
        PBLX8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Address-aligned beats"]
    #[inline(always)]
    pub fn aab(&self) -> AAB_R {
        AAB_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMABM")
            .field("swr", &self.swr())
            .field("da", &self.da())
            .field("dsl", &self.dsl())
            .field("pbl", &self.pbl())
            .field("pr", &self.pr())
            .field("fb", &self.fb())
            .field("rdp", &self.rdp())
            .field("usp", &self.usp())
            .field("pblx8", &self.pblx8())
            .field("aab", &self.aab())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Software reset"]
    #[inline(always)]
    pub fn swr(&mut self) -> SWR_W<'_, DMABM_SPEC> {
        SWR_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Arbitration"]
    #[inline(always)]
    pub fn da(&mut self) -> DA_W<'_, DMABM_SPEC> {
        DA_W::new(self, 1)
    }
    #[doc = "Bits 2:6 - Descriptor skip length"]
    #[inline(always)]
    pub fn dsl(&mut self) -> DSL_W<'_, DMABM_SPEC> {
        DSL_W::new(self, 2)
    }
    #[doc = "Bits 8:13 - Programmable burst length"]
    #[inline(always)]
    pub fn pbl(&mut self) -> PBL_W<'_, DMABM_SPEC> {
        PBL_W::new(self, 8)
    }
    #[doc = "Bits 14:15 - Priority ratio"]
    #[inline(always)]
    pub fn pr(&mut self) -> PR_W<'_, DMABM_SPEC> {
        PR_W::new(self, 14)
    }
    #[doc = "Bit 16 - Fixed burst"]
    #[inline(always)]
    pub fn fb(&mut self) -> FB_W<'_, DMABM_SPEC> {
        FB_W::new(self, 16)
    }
    #[doc = "Bits 17:22 - Rx DMA PBL"]
    #[inline(always)]
    pub fn rdp(&mut self) -> RDP_W<'_, DMABM_SPEC> {
        RDP_W::new(self, 17)
    }
    #[doc = "Bit 23 - Use separate PBL"]
    #[inline(always)]
    pub fn usp(&mut self) -> USP_W<'_, DMABM_SPEC> {
        USP_W::new(self, 23)
    }
    #[doc = "Bit 24 - PNLx8 mode"]
    #[inline(always)]
    pub fn pblx8(&mut self) -> PBLX8_W<'_, DMABM_SPEC> {
        PBLX8_W::new(self, 24)
    }
    #[doc = "Bit 25 - Address-aligned beats"]
    #[inline(always)]
    pub fn aab(&mut self) -> AAB_W<'_, DMABM_SPEC> {
        AAB_W::new(self, 25)
    }
}
#[doc = "Ethernet DMA bus mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmabm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmabm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMABM_SPEC;
impl crate::RegisterSpec for DMABM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmabm::R`](R) reader structure"]
impl crate::Readable for DMABM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmabm::W`](W) writer structure"]
impl crate::Writable for DMABM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMABM to value 0x0002_0101"]
impl crate::Resettable for DMABM_SPEC {
    const RESET_VALUE: u32 = 0x0002_0101;
}
