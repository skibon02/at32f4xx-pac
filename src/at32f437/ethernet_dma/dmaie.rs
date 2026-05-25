#[doc = "Register `DMAIE` reader"]
pub type R = crate::R<DMAIE_SPEC>;
#[doc = "Register `DMAIE` writer"]
pub type W = crate::W<DMAIE_SPEC>;
#[doc = "Transmit interrupt enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIE_A {
    #[doc = "0: Interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Interrupt enabled"]
    Enabled = 1,
}
impl From<TIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIE` reader - Transmit interrupt enable"]
pub type TIE_R = crate::BitReader<TIE_A>;
impl TIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIE_A {
        match self.bits {
            false => TIE_A::Disabled,
            true => TIE_A::Enabled,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIE_A::Disabled
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIE_A::Enabled
    }
}
#[doc = "Field `TIE` writer - Transmit interrupt enable"]
pub type TIE_W<'a, REG> = crate::BitWriter<'a, REG, TIE_A>;
impl<'a, REG> TIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIE_A::Disabled)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIE_A::Enabled)
    }
}
#[doc = "Field `TSE` reader - Transmit stopped enable"]
pub use TIE_R as TSE_R;
#[doc = "Field `TUE` reader - Transmit buffer unavailable enable"]
pub use TIE_R as TUE_R;
#[doc = "Field `TJE` reader - Transmit jabber timeout enable"]
pub use TIE_R as TJE_R;
#[doc = "Field `OVE` reader - Overflow interrupt enable"]
pub use TIE_R as OVE_R;
#[doc = "Field `UNE` reader - Underflow interrupt enable"]
pub use TIE_R as UNE_R;
#[doc = "Field `RIE` reader - Receive interrupt enable"]
pub use TIE_R as RIE_R;
#[doc = "Field `RBUE` reader - Receive buffer unavailable enable"]
pub use TIE_R as RBUE_R;
#[doc = "Field `RSE` reader - Receive stopped enable"]
pub use TIE_R as RSE_R;
#[doc = "Field `RWTE` reader - receive watchdog timeout enable"]
pub use TIE_R as RWTE_R;
#[doc = "Field `EIE` reader - Early transmit interrupt enable"]
pub use TIE_R as EIE_R;
#[doc = "Field `FBEE` reader - Fatal bus error enable"]
pub use TIE_R as FBEE_R;
#[doc = "Field `ERE` reader - Early receive interrupt enable"]
pub use TIE_R as ERE_R;
#[doc = "Field `AIE` reader - Abnormal interrupt enable"]
pub use TIE_R as AIE_R;
#[doc = "Field `NIE` reader - Normal interrupt enable"]
pub use TIE_R as NIE_R;
#[doc = "Field `TSE` writer - Transmit stopped enable"]
pub use TIE_W as TSE_W;
#[doc = "Field `TUE` writer - Transmit buffer unavailable enable"]
pub use TIE_W as TUE_W;
#[doc = "Field `TJE` writer - Transmit jabber timeout enable"]
pub use TIE_W as TJE_W;
#[doc = "Field `OVE` writer - Overflow interrupt enable"]
pub use TIE_W as OVE_W;
#[doc = "Field `UNE` writer - Underflow interrupt enable"]
pub use TIE_W as UNE_W;
#[doc = "Field `RIE` writer - Receive interrupt enable"]
pub use TIE_W as RIE_W;
#[doc = "Field `RBUE` writer - Receive buffer unavailable enable"]
pub use TIE_W as RBUE_W;
#[doc = "Field `RSE` writer - Receive stopped enable"]
pub use TIE_W as RSE_W;
#[doc = "Field `RWTE` writer - receive watchdog timeout enable"]
pub use TIE_W as RWTE_W;
#[doc = "Field `EIE` writer - Early transmit interrupt enable"]
pub use TIE_W as EIE_W;
#[doc = "Field `FBEE` writer - Fatal bus error enable"]
pub use TIE_W as FBEE_W;
#[doc = "Field `ERE` writer - Early receive interrupt enable"]
pub use TIE_W as ERE_W;
#[doc = "Field `AIE` writer - Abnormal interrupt enable"]
pub use TIE_W as AIE_W;
#[doc = "Field `NIE` writer - Normal interrupt enable"]
pub use TIE_W as NIE_W;
impl R {
    #[doc = "Bit 0 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit stopped enable"]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable enable"]
    #[inline(always)]
    pub fn tue(&self) -> TUE_R {
        TUE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit jabber timeout enable"]
    #[inline(always)]
    pub fn tje(&self) -> TJE_R {
        TJE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn ove(&self) -> OVE_R {
        OVE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Underflow interrupt enable"]
    #[inline(always)]
    pub fn une(&self) -> UNE_R {
        UNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive interrupt enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive buffer unavailable enable"]
    #[inline(always)]
    pub fn rbue(&self) -> RBUE_R {
        RBUE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive stopped enable"]
    #[inline(always)]
    pub fn rse(&self) -> RSE_R {
        RSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - receive watchdog timeout enable"]
    #[inline(always)]
    pub fn rwte(&self) -> RWTE_R {
        RWTE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early transmit interrupt enable"]
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Fatal bus error enable"]
    #[inline(always)]
    pub fn fbee(&self) -> FBEE_R {
        FBEE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Early receive interrupt enable"]
    #[inline(always)]
    pub fn ere(&self) -> ERE_R {
        ERE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Abnormal interrupt enable"]
    #[inline(always)]
    pub fn aie(&self) -> AIE_R {
        AIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Normal interrupt enable"]
    #[inline(always)]
    pub fn nie(&self) -> NIE_R {
        NIE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAIE")
            .field("tie", &self.tie())
            .field("tse", &self.tse())
            .field("tue", &self.tue())
            .field("tje", &self.tje())
            .field("ove", &self.ove())
            .field("une", &self.une())
            .field("rie", &self.rie())
            .field("rbue", &self.rbue())
            .field("rse", &self.rse())
            .field("rwte", &self.rwte())
            .field("eie", &self.eie())
            .field("fbee", &self.fbee())
            .field("ere", &self.ere())
            .field("aie", &self.aie())
            .field("nie", &self.nie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W<'_, DMAIE_SPEC> {
        TIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit stopped enable"]
    #[inline(always)]
    pub fn tse(&mut self) -> TSE_W<'_, DMAIE_SPEC> {
        TSE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit buffer unavailable enable"]
    #[inline(always)]
    pub fn tue(&mut self) -> TUE_W<'_, DMAIE_SPEC> {
        TUE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit jabber timeout enable"]
    #[inline(always)]
    pub fn tje(&mut self) -> TJE_W<'_, DMAIE_SPEC> {
        TJE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Overflow interrupt enable"]
    #[inline(always)]
    pub fn ove(&mut self) -> OVE_W<'_, DMAIE_SPEC> {
        OVE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Underflow interrupt enable"]
    #[inline(always)]
    pub fn une(&mut self) -> UNE_W<'_, DMAIE_SPEC> {
        UNE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Receive interrupt enable"]
    #[inline(always)]
    pub fn rie(&mut self) -> RIE_W<'_, DMAIE_SPEC> {
        RIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Receive buffer unavailable enable"]
    #[inline(always)]
    pub fn rbue(&mut self) -> RBUE_W<'_, DMAIE_SPEC> {
        RBUE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Receive stopped enable"]
    #[inline(always)]
    pub fn rse(&mut self) -> RSE_W<'_, DMAIE_SPEC> {
        RSE_W::new(self, 8)
    }
    #[doc = "Bit 9 - receive watchdog timeout enable"]
    #[inline(always)]
    pub fn rwte(&mut self) -> RWTE_W<'_, DMAIE_SPEC> {
        RWTE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Early transmit interrupt enable"]
    #[inline(always)]
    pub fn eie(&mut self) -> EIE_W<'_, DMAIE_SPEC> {
        EIE_W::new(self, 10)
    }
    #[doc = "Bit 13 - Fatal bus error enable"]
    #[inline(always)]
    pub fn fbee(&mut self) -> FBEE_W<'_, DMAIE_SPEC> {
        FBEE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Early receive interrupt enable"]
    #[inline(always)]
    pub fn ere(&mut self) -> ERE_W<'_, DMAIE_SPEC> {
        ERE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Abnormal interrupt enable"]
    #[inline(always)]
    pub fn aie(&mut self) -> AIE_W<'_, DMAIE_SPEC> {
        AIE_W::new(self, 15)
    }
    #[doc = "Bit 16 - Normal interrupt enable"]
    #[inline(always)]
    pub fn nie(&mut self) -> NIE_W<'_, DMAIE_SPEC> {
        NIE_W::new(self, 16)
    }
}
#[doc = "Ethernet DMA interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmaie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMAIE_SPEC;
impl crate::RegisterSpec for DMAIE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaie::R`](R) reader structure"]
impl crate::Readable for DMAIE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmaie::W`](W) writer structure"]
impl crate::Writable for DMAIE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMAIE to value 0"]
impl crate::Resettable for DMAIE_SPEC {}
