#[doc = "Register `TADJ` writer"]
pub type W = crate::W<TADJ_SPEC>;
#[doc = "Field `DECSBS` writer - Decrease sub-second value. Delay (ADD1S=0): Delay = DECSBS/(DIVB+1). Advance (ADD1S=1) : Advance =1-(DECSBS/(DIVB+1))"]
pub type DECSBS_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16, crate::Safe>;
#[doc = "Add 1 second. This bit can be written only when TADJF=0. It is intended to be used with DECSBS in order to fine-tune the time\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADD1S_A {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Add one second"]
    AddOneSecond = 1,
}
impl From<ADD1S_A> for bool {
    #[inline(always)]
    fn from(variant: ADD1S_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADD1S` writer - Add 1 second. This bit can be written only when TADJF=0. It is intended to be used with DECSBS in order to fine-tune the time"]
pub type ADD1S_W<'a, REG> = crate::BitWriter<'a, REG, ADD1S_A>;
impl<'a, REG> ADD1S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(ADD1S_A::NoEffect)
    }
    #[doc = "Add one second"]
    #[inline(always)]
    pub fn add_one_second(self) -> &'a mut crate::W<REG> {
        self.variant(ADD1S_A::AddOneSecond)
    }
}
impl core::fmt::Debug for crate::generic::Reg<TADJ_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:14 - Decrease sub-second value. Delay (ADD1S=0): Delay = DECSBS/(DIVB+1). Advance (ADD1S=1) : Advance =1-(DECSBS/(DIVB+1))"]
    #[inline(always)]
    pub fn decsbs(&mut self) -> DECSBS_W<'_, TADJ_SPEC> {
        DECSBS_W::new(self, 0)
    }
    #[doc = "Bit 31 - Add 1 second. This bit can be written only when TADJF=0. It is intended to be used with DECSBS in order to fine-tune the time"]
    #[inline(always)]
    pub fn add1s(&mut self) -> ADD1S_W<'_, TADJ_SPEC> {
        ADD1S_W::new(self, 31)
    }
}
#[doc = "time adjust register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tadj::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TADJ_SPEC;
impl crate::RegisterSpec for TADJ_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tadj::W`](W) writer structure"]
impl crate::Writable for TADJ_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TADJ to value 0"]
impl crate::Resettable for TADJ_SPEC {}
