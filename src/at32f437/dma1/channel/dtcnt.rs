#[doc = "Register `DTCNT` reader"]
pub type R = crate::R<DTCNT_SPEC>;
#[doc = "Register `DTCNT` writer"]
pub type W = crate::W<DTCNT_SPEC>;
#[doc = "Field `CNT` reader - Number of data to transfer"]
pub type CNT_R = crate::FieldReader<u16>;
#[doc = "Field `CNT` writer - Number of data to transfer"]
pub type CNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of data to transfer"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTCNT").field("cnt", &self.cnt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data to transfer"]
    #[inline(always)]
    pub fn cnt(&mut self) -> CNT_W<'_, DTCNT_SPEC> {
        CNT_W::new(self, 0)
    }
}
#[doc = "DMA channel number of data to transfer register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtcnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtcnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTCNT_SPEC;
impl crate::RegisterSpec for DTCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtcnt::R`](R) reader structure"]
impl crate::Readable for DTCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dtcnt::W`](W) writer structure"]
impl crate::Writable for DTCNT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTCNT to value 0"]
impl crate::Resettable for DTCNT_SPEC {}
