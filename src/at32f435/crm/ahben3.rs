#[doc = "Register `AHBEN3` reader"]
pub type R = crate::R<AHBEN3_SPEC>;
#[doc = "Register `AHBEN3` writer"]
pub type W = crate::W<AHBEN3_SPEC>;
#[doc = "XMC clock enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XMC_A {
    #[doc = "0: Disable peripheral clock"]
    Disable = 0,
    #[doc = "1: Enable peripheral clock"]
    Enable = 1,
}
impl From<XMC_A> for bool {
    #[inline(always)]
    fn from(variant: XMC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `XMC` reader - XMC clock enable"]
pub type XMC_R = crate::BitReader<XMC_A>;
impl XMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> XMC_A {
        match self.bits {
            false => XMC_A::Disable,
            true => XMC_A::Enable,
        }
    }
    #[doc = "Disable peripheral clock"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == XMC_A::Disable
    }
    #[doc = "Enable peripheral clock"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == XMC_A::Enable
    }
}
#[doc = "Field `XMC` writer - XMC clock enable"]
pub type XMC_W<'a, REG> = crate::BitWriter<'a, REG, XMC_A>;
impl<'a, REG> XMC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable peripheral clock"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(XMC_A::Disable)
    }
    #[doc = "Enable peripheral clock"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(XMC_A::Enable)
    }
}
#[doc = "Field `QSPI1` reader - QSPI1 clock enable"]
pub use XMC_R as QSPI1_R;
#[doc = "Field `QSPI2` reader - QSPI2 clock enable"]
pub use XMC_R as QSPI2_R;
#[doc = "Field `SDIO2` reader - SDIO 2 clock enable"]
pub use XMC_R as SDIO2_R;
#[doc = "Field `QSPI1` writer - QSPI1 clock enable"]
pub use XMC_W as QSPI1_W;
#[doc = "Field `QSPI2` writer - QSPI2 clock enable"]
pub use XMC_W as QSPI2_W;
#[doc = "Field `SDIO2` writer - SDIO 2 clock enable"]
pub use XMC_W as SDIO2_W;
impl R {
    #[doc = "Bit 0 - XMC clock enable"]
    #[inline(always)]
    pub fn xmc(&self) -> XMC_R {
        XMC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - QSPI1 clock enable"]
    #[inline(always)]
    pub fn qspi1(&self) -> QSPI1_R {
        QSPI1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 14 - QSPI2 clock enable"]
    #[inline(always)]
    pub fn qspi2(&self) -> QSPI2_R {
        QSPI2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SDIO 2 clock enable"]
    #[inline(always)]
    pub fn sdio2(&self) -> SDIO2_R {
        SDIO2_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHBEN3")
            .field("xmc", &self.xmc())
            .field("qspi1", &self.qspi1())
            .field("qspi2", &self.qspi2())
            .field("sdio2", &self.sdio2())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - XMC clock enable"]
    #[inline(always)]
    pub fn xmc(&mut self) -> XMC_W<'_, AHBEN3_SPEC> {
        XMC_W::new(self, 0)
    }
    #[doc = "Bit 1 - QSPI1 clock enable"]
    #[inline(always)]
    pub fn qspi1(&mut self) -> QSPI1_W<'_, AHBEN3_SPEC> {
        QSPI1_W::new(self, 1)
    }
    #[doc = "Bit 14 - QSPI2 clock enable"]
    #[inline(always)]
    pub fn qspi2(&mut self) -> QSPI2_W<'_, AHBEN3_SPEC> {
        QSPI2_W::new(self, 14)
    }
    #[doc = "Bit 15 - SDIO 2 clock enable"]
    #[inline(always)]
    pub fn sdio2(&mut self) -> SDIO2_W<'_, AHBEN3_SPEC> {
        SDIO2_W::new(self, 15)
    }
}
#[doc = "AHB peripheral clock enable register 3 (CRM_AHBEN3)\n\nYou can [`read`](crate::Reg::read) this register and get [`ahben3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahben3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBEN3_SPEC;
impl crate::RegisterSpec for AHBEN3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahben3::R`](R) reader structure"]
impl crate::Readable for AHBEN3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahben3::W`](W) writer structure"]
impl crate::Writable for AHBEN3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHBEN3 to value 0"]
impl crate::Resettable for AHBEN3_SPEC {}
