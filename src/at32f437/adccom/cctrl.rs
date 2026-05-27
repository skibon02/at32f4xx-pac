#[doc = "Register `CCTRL` reader"]
pub type R = crate::R<CCTRL_SPEC>;
#[doc = "Register `CCTRL` writer"]
pub type W = crate::W<CCTRL_SPEC>;
#[doc = "Master slave mode select\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSSEL_A {
    #[doc = "0: Non-combined mode"]
    NonCombined = 0,
    #[doc = "1: Combined ordinary simultaneous+preempted simultaneous modes (single slave)"]
    CombinedOrdinaryPreempted = 1,
    #[doc = "2: Combined ordinary simultaneous+alternate preempted trigger modes (single slave)"]
    CombinedOrdinaryAlternate = 2,
    #[doc = "5: Preempted simultaneous mode (single slave)"]
    PreemptedSimultaneous = 5,
    #[doc = "6: Ordinary simultaneous mode (single slave)"]
    OrdinarySimultaneous = 6,
    #[doc = "7: Ordinary shift mode (single slave)"]
    OrdinaryShift = 7,
    #[doc = "9: Alternate preempted trigger mode (single slave)"]
    AlternatePreemptedTrigger = 9,
    #[doc = "17: Combined ordinary simultaneous+preempted simultaneous modes (dual slaves)"]
    CombinedOrdinaryPreemptedDual = 17,
    #[doc = "18: Combined ordinary simultaneous+alternate preempted trigger modes (dual slaves)"]
    CombinedOrdinaryAlternateDual = 18,
    #[doc = "21: Preempted simultaneous mode (dual slaves)"]
    PreemptedSimultaneousDual = 21,
    #[doc = "22: Ordinary simultaneous mode (dual slaves)"]
    OrdinarySimultaneousDual = 22,
    #[doc = "23: Ordinary shift mode (dual slaves)"]
    OrdinaryShiftDual = 23,
}
impl From<MSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MSSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MSSEL_A {
    type Ux = u8;
}
impl crate::IsEnum for MSSEL_A {}
#[doc = "Field `MSSEL` reader - Master slave mode select"]
pub type MSSEL_R = crate::FieldReader<MSSEL_A>;
impl MSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MSSEL_A> {
        match self.bits {
            0 => Some(MSSEL_A::NonCombined),
            1 => Some(MSSEL_A::CombinedOrdinaryPreempted),
            2 => Some(MSSEL_A::CombinedOrdinaryAlternate),
            5 => Some(MSSEL_A::PreemptedSimultaneous),
            6 => Some(MSSEL_A::OrdinarySimultaneous),
            7 => Some(MSSEL_A::OrdinaryShift),
            9 => Some(MSSEL_A::AlternatePreemptedTrigger),
            17 => Some(MSSEL_A::CombinedOrdinaryPreemptedDual),
            18 => Some(MSSEL_A::CombinedOrdinaryAlternateDual),
            21 => Some(MSSEL_A::PreemptedSimultaneousDual),
            22 => Some(MSSEL_A::OrdinarySimultaneousDual),
            23 => Some(MSSEL_A::OrdinaryShiftDual),
            _ => None,
        }
    }
    #[doc = "Non-combined mode"]
    #[inline(always)]
    pub fn is_non_combined(&self) -> bool {
        *self == MSSEL_A::NonCombined
    }
    #[doc = "Combined ordinary simultaneous+preempted simultaneous modes (single slave)"]
    #[inline(always)]
    pub fn is_combined_ordinary_preempted(&self) -> bool {
        *self == MSSEL_A::CombinedOrdinaryPreempted
    }
    #[doc = "Combined ordinary simultaneous+alternate preempted trigger modes (single slave)"]
    #[inline(always)]
    pub fn is_combined_ordinary_alternate(&self) -> bool {
        *self == MSSEL_A::CombinedOrdinaryAlternate
    }
    #[doc = "Preempted simultaneous mode (single slave)"]
    #[inline(always)]
    pub fn is_preempted_simultaneous(&self) -> bool {
        *self == MSSEL_A::PreemptedSimultaneous
    }
    #[doc = "Ordinary simultaneous mode (single slave)"]
    #[inline(always)]
    pub fn is_ordinary_simultaneous(&self) -> bool {
        *self == MSSEL_A::OrdinarySimultaneous
    }
    #[doc = "Ordinary shift mode (single slave)"]
    #[inline(always)]
    pub fn is_ordinary_shift(&self) -> bool {
        *self == MSSEL_A::OrdinaryShift
    }
    #[doc = "Alternate preempted trigger mode (single slave)"]
    #[inline(always)]
    pub fn is_alternate_preempted_trigger(&self) -> bool {
        *self == MSSEL_A::AlternatePreemptedTrigger
    }
    #[doc = "Combined ordinary simultaneous+preempted simultaneous modes (dual slaves)"]
    #[inline(always)]
    pub fn is_combined_ordinary_preempted_dual(&self) -> bool {
        *self == MSSEL_A::CombinedOrdinaryPreemptedDual
    }
    #[doc = "Combined ordinary simultaneous+alternate preempted trigger modes (dual slaves)"]
    #[inline(always)]
    pub fn is_combined_ordinary_alternate_dual(&self) -> bool {
        *self == MSSEL_A::CombinedOrdinaryAlternateDual
    }
    #[doc = "Preempted simultaneous mode (dual slaves)"]
    #[inline(always)]
    pub fn is_preempted_simultaneous_dual(&self) -> bool {
        *self == MSSEL_A::PreemptedSimultaneousDual
    }
    #[doc = "Ordinary simultaneous mode (dual slaves)"]
    #[inline(always)]
    pub fn is_ordinary_simultaneous_dual(&self) -> bool {
        *self == MSSEL_A::OrdinarySimultaneousDual
    }
    #[doc = "Ordinary shift mode (dual slaves)"]
    #[inline(always)]
    pub fn is_ordinary_shift_dual(&self) -> bool {
        *self == MSSEL_A::OrdinaryShiftDual
    }
}
#[doc = "Field `MSSEL` writer - Master slave mode select"]
pub type MSSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, MSSEL_A>;
impl<'a, REG> MSSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Non-combined mode"]
    #[inline(always)]
    pub fn non_combined(self) -> &'a mut crate::W<REG> {
        self.variant(MSSEL_A::NonCombined)
    }
    #[doc = "Combined ordinary simultaneous+preempted simultaneous modes (single slave)"]
    #[inline(always)]
    pub fn combined_ordinary_preempted(self) -> &'a mut crate::W<REG> {
        self.variant(MSSEL_A::CombinedOrdinaryPreempted)
    }
    #[doc = "Combined ordinary simultaneous+alternate preempted trigger modes (single slave)"]
    #[inline(always)]
    pub fn combined_ordinary_alternate(self) -> &'a mut crate::W<REG> {
        self.variant(MSSEL_A::CombinedOrdinaryAlternate)
    }
    #[doc = "Preempted simultaneous mode (single slave)"]
    #[inline(always)]
    pub fn preempted_simultaneous(self) -> &'a mut crate::W<REG> {
        self.variant(MSSEL_A::PreemptedSimultaneous)
    }
    #[doc = "Ordinary simultaneous mode (single slave)"]
    #[inline(always)]
    pub fn ordinary_simultaneous(self) -> &'a mut crate::W<REG> {
        self.variant(MSSEL_A::OrdinarySimultaneous)
    }
    #[doc = "Ordinary shift mode (single slave)"]
    #[inline(always)]
    pub fn ordinary_shift(self) -> &'a mut crate::W<REG> {
        self.variant(MSSEL_A::OrdinaryShift)
    }
    #[doc = "Alternate preempted trigger mode (single slave)"]
    #[inline(always)]
    pub fn alternate_preempted_trigger(self) -> &'a mut crate::W<REG> {
        self.variant(MSSEL_A::AlternatePreemptedTrigger)
    }
    #[doc = "Combined ordinary simultaneous+preempted simultaneous modes (dual slaves)"]
    #[inline(always)]
    pub fn combined_ordinary_preempted_dual(self) -> &'a mut crate::W<REG> {
        self.variant(MSSEL_A::CombinedOrdinaryPreemptedDual)
    }
    #[doc = "Combined ordinary simultaneous+alternate preempted trigger modes (dual slaves)"]
    #[inline(always)]
    pub fn combined_ordinary_alternate_dual(self) -> &'a mut crate::W<REG> {
        self.variant(MSSEL_A::CombinedOrdinaryAlternateDual)
    }
    #[doc = "Preempted simultaneous mode (dual slaves)"]
    #[inline(always)]
    pub fn preempted_simultaneous_dual(self) -> &'a mut crate::W<REG> {
        self.variant(MSSEL_A::PreemptedSimultaneousDual)
    }
    #[doc = "Ordinary simultaneous mode (dual slaves)"]
    #[inline(always)]
    pub fn ordinary_simultaneous_dual(self) -> &'a mut crate::W<REG> {
        self.variant(MSSEL_A::OrdinarySimultaneousDual)
    }
    #[doc = "Ordinary shift mode (dual slaves)"]
    #[inline(always)]
    pub fn ordinary_shift_dual(self) -> &'a mut crate::W<REG> {
        self.variant(MSSEL_A::OrdinaryShiftDual)
    }
}
#[doc = "Field `ASISEL` reader - Adjacent ADC sampling interval select in ordinary shift mode: {value} + 5 * TADCCLK"]
pub type ASISEL_R = crate::FieldReader;
#[doc = "Field `ASISEL` writer - Adjacent ADC sampling interval select in ordinary shift mode: {value} + 5 * TADCCLK"]
pub type ASISEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "Ordinary channel DMA request continuation enable for master slave mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSDRCEN_A {
    #[doc = "0: MSDRC disabled"]
    Disabled = 0,
    #[doc = "1: MSDRC enabled"]
    Enabled = 1,
}
impl From<MSDRCEN_A> for bool {
    #[inline(always)]
    fn from(variant: MSDRCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MSDRCEN` reader - Ordinary channel DMA request continuation enable for master slave mode"]
pub type MSDRCEN_R = crate::BitReader<MSDRCEN_A>;
impl MSDRCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MSDRCEN_A {
        match self.bits {
            false => MSDRCEN_A::Disabled,
            true => MSDRCEN_A::Enabled,
        }
    }
    #[doc = "MSDRC disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSDRCEN_A::Disabled
    }
    #[doc = "MSDRC enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSDRCEN_A::Enabled
    }
}
#[doc = "Field `MSDRCEN` writer - Ordinary channel DMA request continuation enable for master slave mode"]
pub type MSDRCEN_W<'a, REG> = crate::BitWriter<'a, REG, MSDRCEN_A>;
impl<'a, REG> MSDRCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MSDRC disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MSDRCEN_A::Disabled)
    }
    #[doc = "MSDRC enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MSDRCEN_A::Enabled)
    }
}
#[doc = "Field `MSDMASEL_L` reader - MSDMASEL\\[1:0\\] — low bits of ordinary channel DMA transfer mode select for master/slave mode. Combined with MSDMASEL_H to form MSDMASEL\\[2:0\\]. 0: No DMA transfer, 1: DMA mode 1, 2: DMA mode 2, 3: DMA mode 3, 4: DMA mode 4, 5: DMA mode 5. 6~7: Unused, do not configure."]
pub type MSDMASEL_L_R = crate::FieldReader;
#[doc = "Field `MSDMASEL_L` writer - MSDMASEL\\[1:0\\] — low bits of ordinary channel DMA transfer mode select for master/slave mode. Combined with MSDMASEL_H to form MSDMASEL\\[2:0\\]. 0: No DMA transfer, 1: DMA mode 1, 2: DMA mode 2, 3: DMA mode 3, 4: DMA mode 4, 5: DMA mode 5. 6~7: Unused, do not configure."]
pub type MSDMASEL_L_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADCDIV` reader - ADC division: HCLK / (2 + {value}). Cannot be higher than 80Mhz and PCLK2"]
pub type ADCDIV_R = crate::FieldReader;
#[doc = "Field `ADCDIV` writer - ADC division: HCLK / (2 + {value}). Cannot be higher than 80Mhz and PCLK2"]
pub type ADCDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
#[doc = "VBAT enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATEN_A {
    #[doc = "0: VBAT channel disabled"]
    Disabled = 0,
    #[doc = "1: VBAT channel enabled"]
    Enabled = 1,
}
impl From<VBATEN_A> for bool {
    #[inline(always)]
    fn from(variant: VBATEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VBATEN` reader - VBAT enable"]
pub type VBATEN_R = crate::BitReader<VBATEN_A>;
impl VBATEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VBATEN_A {
        match self.bits {
            false => VBATEN_A::Disabled,
            true => VBATEN_A::Enabled,
        }
    }
    #[doc = "VBAT channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VBATEN_A::Disabled
    }
    #[doc = "VBAT channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VBATEN_A::Enabled
    }
}
#[doc = "Field `VBATEN` writer - VBAT enable"]
pub type VBATEN_W<'a, REG> = crate::BitWriter<'a, REG, VBATEN_A>;
impl<'a, REG> VBATEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBAT channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(VBATEN_A::Disabled)
    }
    #[doc = "VBAT channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(VBATEN_A::Enabled)
    }
}
#[doc = "Internal temperature sensor and VINTRV enable\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITSRVEN_A {
    #[doc = "0: Internal temperature sensor and Vintrv channel disabled"]
    Disabled = 0,
    #[doc = "1: Internal temperature sensor and Vintrv channel enabled"]
    Enabled = 1,
}
impl From<ITSRVEN_A> for bool {
    #[inline(always)]
    fn from(variant: ITSRVEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITSRVEN` reader - Internal temperature sensor and VINTRV enable"]
pub type ITSRVEN_R = crate::BitReader<ITSRVEN_A>;
impl ITSRVEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITSRVEN_A {
        match self.bits {
            false => ITSRVEN_A::Disabled,
            true => ITSRVEN_A::Enabled,
        }
    }
    #[doc = "Internal temperature sensor and Vintrv channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ITSRVEN_A::Disabled
    }
    #[doc = "Internal temperature sensor and Vintrv channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ITSRVEN_A::Enabled
    }
}
#[doc = "Field `ITSRVEN` writer - Internal temperature sensor and VINTRV enable"]
pub type ITSRVEN_W<'a, REG> = crate::BitWriter<'a, REG, ITSRVEN_A>;
impl<'a, REG> ITSRVEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal temperature sensor and Vintrv channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ITSRVEN_A::Disabled)
    }
    #[doc = "Internal temperature sensor and Vintrv channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ITSRVEN_A::Enabled)
    }
}
#[doc = "Field `MSDMASEL_H` reader - MSDMASEL\\[2\\] — high bit of ordinary channel DMA transfer mode select for master/slave mode. Combined with MSDMASEL_L to form MSDMASEL\\[2:0\\]. 0: No DMA transfer, 1: DMA mode 1, 2: DMA mode 2, 3: DMA mode 3, 4: DMA mode 4, 5: DMA mode 5. 6~7: Unused, do not configure."]
pub type MSDMASEL_H_R = crate::BitReader;
#[doc = "Field `MSDMASEL_H` writer - MSDMASEL\\[2\\] — high bit of ordinary channel DMA transfer mode select for master/slave mode. Combined with MSDMASEL_L to form MSDMASEL\\[2:0\\]. 0: No DMA transfer, 1: DMA mode 1, 2: DMA mode 2, 3: DMA mode 3, 4: DMA mode 4, 5: DMA mode 5. 6~7: Unused, do not configure."]
pub type MSDMASEL_H_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Master slave mode select"]
    #[inline(always)]
    pub fn mssel(&self) -> MSSEL_R {
        MSSEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Adjacent ADC sampling interval select in ordinary shift mode: {value} + 5 * TADCCLK"]
    #[inline(always)]
    pub fn asisel(&self) -> ASISEL_R {
        ASISEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Ordinary channel DMA request continuation enable for master slave mode"]
    #[inline(always)]
    pub fn msdrcen(&self) -> MSDRCEN_R {
        MSDRCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - MSDMASEL\\[1:0\\] — low bits of ordinary channel DMA transfer mode select for master/slave mode. Combined with MSDMASEL_H to form MSDMASEL\\[2:0\\]. 0: No DMA transfer, 1: DMA mode 1, 2: DMA mode 2, 3: DMA mode 3, 4: DMA mode 4, 5: DMA mode 5. 6~7: Unused, do not configure."]
    #[inline(always)]
    pub fn msdmasel_l(&self) -> MSDMASEL_L_R {
        MSDMASEL_L_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19 - ADC division: HCLK / (2 + {value}). Cannot be higher than 80Mhz and PCLK2"]
    #[inline(always)]
    pub fn adcdiv(&self) -> ADCDIV_R {
        ADCDIV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - VBAT enable"]
    #[inline(always)]
    pub fn vbaten(&self) -> VBATEN_R {
        VBATEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Internal temperature sensor and VINTRV enable"]
    #[inline(always)]
    pub fn itsrven(&self) -> ITSRVEN_R {
        ITSRVEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 28 - MSDMASEL\\[2\\] — high bit of ordinary channel DMA transfer mode select for master/slave mode. Combined with MSDMASEL_L to form MSDMASEL\\[2:0\\]. 0: No DMA transfer, 1: DMA mode 1, 2: DMA mode 2, 3: DMA mode 3, 4: DMA mode 4, 5: DMA mode 5. 6~7: Unused, do not configure."]
    #[inline(always)]
    pub fn msdmasel_h(&self) -> MSDMASEL_H_R {
        MSDMASEL_H_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCTRL")
            .field("msdmasel_h", &self.msdmasel_h())
            .field("itsrven", &self.itsrven())
            .field("vbaten", &self.vbaten())
            .field("adcdiv", &self.adcdiv())
            .field("msdmasel_l", &self.msdmasel_l())
            .field("msdrcen", &self.msdrcen())
            .field("asisel", &self.asisel())
            .field("mssel", &self.mssel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Master slave mode select"]
    #[inline(always)]
    pub fn mssel(&mut self) -> MSSEL_W<'_, CCTRL_SPEC> {
        MSSEL_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Adjacent ADC sampling interval select in ordinary shift mode: {value} + 5 * TADCCLK"]
    #[inline(always)]
    pub fn asisel(&mut self) -> ASISEL_W<'_, CCTRL_SPEC> {
        ASISEL_W::new(self, 8)
    }
    #[doc = "Bit 13 - Ordinary channel DMA request continuation enable for master slave mode"]
    #[inline(always)]
    pub fn msdrcen(&mut self) -> MSDRCEN_W<'_, CCTRL_SPEC> {
        MSDRCEN_W::new(self, 13)
    }
    #[doc = "Bits 14:15 - MSDMASEL\\[1:0\\] — low bits of ordinary channel DMA transfer mode select for master/slave mode. Combined with MSDMASEL_H to form MSDMASEL\\[2:0\\]. 0: No DMA transfer, 1: DMA mode 1, 2: DMA mode 2, 3: DMA mode 3, 4: DMA mode 4, 5: DMA mode 5. 6~7: Unused, do not configure."]
    #[inline(always)]
    pub fn msdmasel_l(&mut self) -> MSDMASEL_L_W<'_, CCTRL_SPEC> {
        MSDMASEL_L_W::new(self, 14)
    }
    #[doc = "Bits 16:19 - ADC division: HCLK / (2 + {value}). Cannot be higher than 80Mhz and PCLK2"]
    #[inline(always)]
    pub fn adcdiv(&mut self) -> ADCDIV_W<'_, CCTRL_SPEC> {
        ADCDIV_W::new(self, 16)
    }
    #[doc = "Bit 22 - VBAT enable"]
    #[inline(always)]
    pub fn vbaten(&mut self) -> VBATEN_W<'_, CCTRL_SPEC> {
        VBATEN_W::new(self, 22)
    }
    #[doc = "Bit 23 - Internal temperature sensor and VINTRV enable"]
    #[inline(always)]
    pub fn itsrven(&mut self) -> ITSRVEN_W<'_, CCTRL_SPEC> {
        ITSRVEN_W::new(self, 23)
    }
    #[doc = "Bit 28 - MSDMASEL\\[2\\] — high bit of ordinary channel DMA transfer mode select for master/slave mode. Combined with MSDMASEL_L to form MSDMASEL\\[2:0\\]. 0: No DMA transfer, 1: DMA mode 1, 2: DMA mode 2, 3: DMA mode 3, 4: DMA mode 4, 5: DMA mode 5. 6~7: Unused, do not configure."]
    #[inline(always)]
    pub fn msdmasel_h(&mut self) -> MSDMASEL_H_W<'_, CCTRL_SPEC> {
        MSDMASEL_H_W::new(self, 28)
    }
}
#[doc = "Common control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
