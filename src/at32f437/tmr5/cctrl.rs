#[doc = "Register `CCTRL` reader"]
pub type R = crate::R<CCTRL_SPEC>;
#[doc = "Register `CCTRL` writer"]
pub type W = crate::W<CCTRL_SPEC>;
#[doc = "Field `C1EN` reader - Channel 1 enable"]
pub type C1EN_R = crate::BitReader;
#[doc = "Field `C1EN` writer - Channel 1 enable"]
pub type C1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C1P` reader - Channel 1 Polarity"]
pub type C1P_R = crate::BitReader;
#[doc = "Field `C1P` writer - Channel 1 Polarity"]
pub type C1P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2EN` reader - Channel 2 enable"]
pub type C2EN_R = crate::BitReader;
#[doc = "Field `C2EN` writer - Channel 2 enable"]
pub type C2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C2P` reader - Channel 2 Polarity"]
pub type C2P_R = crate::BitReader;
#[doc = "Field `C2P` writer - Channel 2 Polarity"]
pub type C2P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C3EN` reader - Channel 3 enable"]
pub type C3EN_R = crate::BitReader;
#[doc = "Field `C3EN` writer - Channel 3 enable"]
pub type C3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C3P` reader - Channel 3 Polarity"]
pub type C3P_R = crate::BitReader;
#[doc = "Field `C3P` writer - Channel 3 Polarity"]
pub type C3P_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C4EN` reader - Channel 4 enable"]
pub type C4EN_R = crate::BitReader;
#[doc = "Field `C4EN` writer - Channel 4 enable"]
pub type C4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C4P` reader - Channel 4 Polarity"]
pub type C4P_R = crate::BitReader;
#[doc = "Field `C4P` writer - Channel 4 Polarity"]
pub type C4P_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel 1 enable"]
    #[inline(always)]
    pub fn c1en(&self) -> C1EN_R {
        C1EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Polarity"]
    #[inline(always)]
    pub fn c1p(&self) -> C1P_R {
        C1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 2 enable"]
    #[inline(always)]
    pub fn c2en(&self) -> C2EN_R {
        C2EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 2 Polarity"]
    #[inline(always)]
    pub fn c2p(&self) -> C2P_R {
        C2P_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Channel 3 enable"]
    #[inline(always)]
    pub fn c3en(&self) -> C3EN_R {
        C3EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Channel 3 Polarity"]
    #[inline(always)]
    pub fn c3p(&self) -> C3P_R {
        C3P_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Channel 4 enable"]
    #[inline(always)]
    pub fn c4en(&self) -> C4EN_R {
        C4EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Channel 4 Polarity"]
    #[inline(always)]
    pub fn c4p(&self) -> C4P_R {
        C4P_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCTRL")
            .field("c4p", &self.c4p())
            .field("c4en", &self.c4en())
            .field("c3p", &self.c3p())
            .field("c3en", &self.c3en())
            .field("c2p", &self.c2p())
            .field("c2en", &self.c2en())
            .field("c1p", &self.c1p())
            .field("c1en", &self.c1en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Channel 1 enable"]
    #[inline(always)]
    pub fn c1en(&mut self) -> C1EN_W<'_, CCTRL_SPEC> {
        C1EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Channel 1 Polarity"]
    #[inline(always)]
    pub fn c1p(&mut self) -> C1P_W<'_, CCTRL_SPEC> {
        C1P_W::new(self, 1)
    }
    #[doc = "Bit 4 - Channel 2 enable"]
    #[inline(always)]
    pub fn c2en(&mut self) -> C2EN_W<'_, CCTRL_SPEC> {
        C2EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Channel 2 Polarity"]
    #[inline(always)]
    pub fn c2p(&mut self) -> C2P_W<'_, CCTRL_SPEC> {
        C2P_W::new(self, 5)
    }
    #[doc = "Bit 8 - Channel 3 enable"]
    #[inline(always)]
    pub fn c3en(&mut self) -> C3EN_W<'_, CCTRL_SPEC> {
        C3EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Channel 3 Polarity"]
    #[inline(always)]
    pub fn c3p(&mut self) -> C3P_W<'_, CCTRL_SPEC> {
        C3P_W::new(self, 9)
    }
    #[doc = "Bit 12 - Channel 4 enable"]
    #[inline(always)]
    pub fn c4en(&mut self) -> C4EN_W<'_, CCTRL_SPEC> {
        C4EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - Channel 4 Polarity"]
    #[inline(always)]
    pub fn c4p(&mut self) -> C4P_W<'_, CCTRL_SPEC> {
        C4P_W::new(self, 13)
    }
}
#[doc = "Channel control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCTRL_SPEC;
impl crate::RegisterSpec for CCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cctrl::R`](R) reader structure"]
impl crate::Readable for CCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cctrl::W`](W) writer structure"]
impl crate::Writable for CCTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCTRL to value 0"]
impl crate::Resettable for CCTRL_SPEC {}
