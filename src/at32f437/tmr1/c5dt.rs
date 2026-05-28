#[doc = "Register `C5DT` reader"]
pub type R = crate::R<C5DT_SPEC>;
#[doc = "Register `C5DT` writer"]
pub type W = crate::W<C5DT_SPEC>;
#[doc = "Field `C5DT` reader - Channel 5 data register"]
pub type C5DT_R = crate::FieldReader<u16>;
#[doc = "Field `C5DT` writer - Channel 5 data register"]
pub type C5DT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16, crate::Safe>;
impl R {
    #[doc = "Bits 0:15 - Channel 5 data register"]
    #[inline(always)]
    pub fn c5dt(&self) -> C5DT_R {
        C5DT_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C5DT").field("c5dt", &self.c5dt()).finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Channel 5 data register"]
    #[inline(always)]
    pub fn c5dt(&mut self) -> C5DT_W<'_, C5DT_SPEC> {
        C5DT_W::new(self, 0)
    }
}
#[doc = "Channel 5 data register\n\nYou can [`read`](crate::Reg::read) this register and get [`c5dt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5dt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C5DT_SPEC;
impl crate::RegisterSpec for C5DT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c5dt::R`](R) reader structure"]
impl crate::Readable for C5DT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`c5dt::W`](W) writer structure"]
impl crate::Writable for C5DT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C5DT to value 0"]
impl crate::Resettable for C5DT_SPEC {}
