#[doc = "Register `CFG1` reader"]
pub type R = crate::R<CFG1_SPEC>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<CFG1_SPEC>;
#[doc = "Memory address mapping selection bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MEM_MAP_SEL_A {
    #[doc = "0: Main Flash memory mapped at 0x00000000"]
    MainFlash = 0,
    #[doc = "1: Boot memory mapped at 0x00000000"]
    Boot = 1,
    #[doc = "2: XMC BANK1 mapped at 0x00000000"]
    XmcBank1 = 2,
    #[doc = "3: Embedded SRAM mapped at 0x00000000"]
    EmbeddedSram = 3,
    #[doc = "4: XMC SDRAM BANK1 mapped at 0x00000000"]
    XmcSdramBank1 = 4,
}
impl From<MEM_MAP_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MEM_MAP_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MEM_MAP_SEL_A {
    type Ux = u8;
}
impl crate::IsEnum for MEM_MAP_SEL_A {}
#[doc = "Field `MEM_MAP_SEL` reader - Memory address mapping selection bits"]
pub type MEM_MAP_SEL_R = crate::FieldReader<MEM_MAP_SEL_A>;
impl MEM_MAP_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MEM_MAP_SEL_A> {
        match self.bits {
            0 => Some(MEM_MAP_SEL_A::MainFlash),
            1 => Some(MEM_MAP_SEL_A::Boot),
            2 => Some(MEM_MAP_SEL_A::XmcBank1),
            3 => Some(MEM_MAP_SEL_A::EmbeddedSram),
            4 => Some(MEM_MAP_SEL_A::XmcSdramBank1),
            _ => None,
        }
    }
    #[doc = "Main Flash memory mapped at 0x00000000"]
    #[inline(always)]
    pub fn is_main_flash(&self) -> bool {
        *self == MEM_MAP_SEL_A::MainFlash
    }
    #[doc = "Boot memory mapped at 0x00000000"]
    #[inline(always)]
    pub fn is_boot(&self) -> bool {
        *self == MEM_MAP_SEL_A::Boot
    }
    #[doc = "XMC BANK1 mapped at 0x00000000"]
    #[inline(always)]
    pub fn is_xmc_bank1(&self) -> bool {
        *self == MEM_MAP_SEL_A::XmcBank1
    }
    #[doc = "Embedded SRAM mapped at 0x00000000"]
    #[inline(always)]
    pub fn is_embedded_sram(&self) -> bool {
        *self == MEM_MAP_SEL_A::EmbeddedSram
    }
    #[doc = "XMC SDRAM BANK1 mapped at 0x00000000"]
    #[inline(always)]
    pub fn is_xmc_sdram_bank1(&self) -> bool {
        *self == MEM_MAP_SEL_A::XmcSdramBank1
    }
}
#[doc = "Field `MEM_MAP_SEL` writer - Memory address mapping selection bits"]
pub type MEM_MAP_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MEM_MAP_SEL_A>;
impl<'a, REG> MEM_MAP_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Main Flash memory mapped at 0x00000000"]
    #[inline(always)]
    pub fn main_flash(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MAP_SEL_A::MainFlash)
    }
    #[doc = "Boot memory mapped at 0x00000000"]
    #[inline(always)]
    pub fn boot(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MAP_SEL_A::Boot)
    }
    #[doc = "XMC BANK1 mapped at 0x00000000"]
    #[inline(always)]
    pub fn xmc_bank1(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MAP_SEL_A::XmcBank1)
    }
    #[doc = "Embedded SRAM mapped at 0x00000000"]
    #[inline(always)]
    pub fn embedded_sram(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MAP_SEL_A::EmbeddedSram)
    }
    #[doc = "XMC SDRAM BANK1 mapped at 0x00000000"]
    #[inline(always)]
    pub fn xmc_sdram_bank1(self) -> &'a mut crate::W<REG> {
        self.variant(MEM_MAP_SEL_A::XmcSdramBank1)
    }
}
#[doc = "IR output polarity selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IR_POL_A {
    #[doc = "0: Infrared output (IR_OUT) is not inverted"]
    Normal = 0,
    #[doc = "1: Infrared output (IR_OUT) is inverted"]
    Inverted = 1,
}
impl From<IR_POL_A> for bool {
    #[inline(always)]
    fn from(variant: IR_POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IR_POL` reader - IR output polarity selection"]
pub type IR_POL_R = crate::BitReader<IR_POL_A>;
impl IR_POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IR_POL_A {
        match self.bits {
            false => IR_POL_A::Normal,
            true => IR_POL_A::Inverted,
        }
    }
    #[doc = "Infrared output (IR_OUT) is not inverted"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == IR_POL_A::Normal
    }
    #[doc = "Infrared output (IR_OUT) is inverted"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == IR_POL_A::Inverted
    }
}
#[doc = "Field `IR_POL` writer - IR output polarity selection"]
pub type IR_POL_W<'a, REG> = crate::BitWriter<'a, REG, IR_POL_A>;
impl<'a, REG> IR_POL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Infrared output (IR_OUT) is not inverted"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(IR_POL_A::Normal)
    }
    #[doc = "Infrared output (IR_OUT) is inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut crate::W<REG> {
        self.variant(IR_POL_A::Inverted)
    }
}
#[doc = "IR signal source selection\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IR_SRC_SEL_A {
    #[doc = "0: Timer 10 is used to select the infrared modulation envelope signal source"]
    Tmr10 = 0,
}
impl From<IR_SRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: IR_SRC_SEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IR_SRC_SEL_A {
    type Ux = u8;
}
impl crate::IsEnum for IR_SRC_SEL_A {}
#[doc = "Field `IR_SRC_SEL` reader - IR signal source selection"]
pub type IR_SRC_SEL_R = crate::FieldReader<IR_SRC_SEL_A>;
impl IR_SRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IR_SRC_SEL_A> {
        match self.bits {
            0 => Some(IR_SRC_SEL_A::Tmr10),
            _ => None,
        }
    }
    #[doc = "Timer 10 is used to select the infrared modulation envelope signal source"]
    #[inline(always)]
    pub fn is_tmr10(&self) -> bool {
        *self == IR_SRC_SEL_A::Tmr10
    }
}
#[doc = "Field `IR_SRC_SEL` writer - IR signal source selection"]
pub type IR_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, IR_SRC_SEL_A>;
impl<'a, REG> IR_SRC_SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer 10 is used to select the infrared modulation envelope signal source"]
    #[inline(always)]
    pub fn tmr10(self) -> &'a mut crate::W<REG> {
        self.variant(IR_SRC_SEL_A::Tmr10)
    }
}
#[doc = "XMC address mapping swap bit 0\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWAP_XMC_10_A {
    #[doc = "0: No XMC address mapping swap"]
    NoSwap = 0,
    #[doc = "1: SDRAM addresses are mapped at 0x60000000 and 0x70000000. NOR/PSRAM/SRAM/NAND2 memory addresses are mapped at 0xC0000000 and 0xD0000000"]
    SwapSdram = 1,
}
impl From<SWAP_XMC_10_A> for bool {
    #[inline(always)]
    fn from(variant: SWAP_XMC_10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWAP_XMC_10` reader - XMC address mapping swap bit 0"]
pub type SWAP_XMC_10_R = crate::BitReader<SWAP_XMC_10_A>;
impl SWAP_XMC_10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWAP_XMC_10_A {
        match self.bits {
            false => SWAP_XMC_10_A::NoSwap,
            true => SWAP_XMC_10_A::SwapSdram,
        }
    }
    #[doc = "No XMC address mapping swap"]
    #[inline(always)]
    pub fn is_no_swap(&self) -> bool {
        *self == SWAP_XMC_10_A::NoSwap
    }
    #[doc = "SDRAM addresses are mapped at 0x60000000 and 0x70000000. NOR/PSRAM/SRAM/NAND2 memory addresses are mapped at 0xC0000000 and 0xD0000000"]
    #[inline(always)]
    pub fn is_swap_sdram(&self) -> bool {
        *self == SWAP_XMC_10_A::SwapSdram
    }
}
#[doc = "Field `SWAP_XMC_10` writer - XMC address mapping swap bit 0"]
pub type SWAP_XMC_10_W<'a, REG> = crate::BitWriter<'a, REG, SWAP_XMC_10_A>;
impl<'a, REG> SWAP_XMC_10_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No XMC address mapping swap"]
    #[inline(always)]
    pub fn no_swap(self) -> &'a mut crate::W<REG> {
        self.variant(SWAP_XMC_10_A::NoSwap)
    }
    #[doc = "SDRAM addresses are mapped at 0x60000000 and 0x70000000. NOR/PSRAM/SRAM/NAND2 memory addresses are mapped at 0xC0000000 and 0xD0000000"]
    #[inline(always)]
    pub fn swap_sdram(self) -> &'a mut crate::W<REG> {
        self.variant(SWAP_XMC_10_A::SwapSdram)
    }
}
#[doc = "XMC address mapping swap bit 1\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWAP_XMC_11_A {
    #[doc = "0: No XMC address mapping swap"]
    NoSwap = 0,
    #[doc = "1: QSPI2 memory addresses are mapped at 0x80000000. NAND3 memory is mapped at 0xB0000000"]
    SwapQspiNand3 = 1,
}
impl From<SWAP_XMC_11_A> for bool {
    #[inline(always)]
    fn from(variant: SWAP_XMC_11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWAP_XMC_11` reader - XMC address mapping swap bit 1"]
pub type SWAP_XMC_11_R = crate::BitReader<SWAP_XMC_11_A>;
impl SWAP_XMC_11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWAP_XMC_11_A {
        match self.bits {
            false => SWAP_XMC_11_A::NoSwap,
            true => SWAP_XMC_11_A::SwapQspiNand3,
        }
    }
    #[doc = "No XMC address mapping swap"]
    #[inline(always)]
    pub fn is_no_swap(&self) -> bool {
        *self == SWAP_XMC_11_A::NoSwap
    }
    #[doc = "QSPI2 memory addresses are mapped at 0x80000000. NAND3 memory is mapped at 0xB0000000"]
    #[inline(always)]
    pub fn is_swap_qspi_nand3(&self) -> bool {
        *self == SWAP_XMC_11_A::SwapQspiNand3
    }
}
#[doc = "Field `SWAP_XMC_11` writer - XMC address mapping swap bit 1"]
pub type SWAP_XMC_11_W<'a, REG> = crate::BitWriter<'a, REG, SWAP_XMC_11_A>;
impl<'a, REG> SWAP_XMC_11_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No XMC address mapping swap"]
    #[inline(always)]
    pub fn no_swap(self) -> &'a mut crate::W<REG> {
        self.variant(SWAP_XMC_11_A::NoSwap)
    }
    #[doc = "QSPI2 memory addresses are mapped at 0x80000000. NAND3 memory is mapped at 0xB0000000"]
    #[inline(always)]
    pub fn swap_qspi_nand3(self) -> &'a mut crate::W<REG> {
        self.variant(SWAP_XMC_11_A::SwapQspiNand3)
    }
}
impl R {
    #[doc = "Bits 0:2 - Memory address mapping selection bits"]
    #[inline(always)]
    pub fn mem_map_sel(&self) -> MEM_MAP_SEL_R {
        MEM_MAP_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 5 - IR output polarity selection"]
    #[inline(always)]
    pub fn ir_pol(&self) -> IR_POL_R {
        IR_POL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - IR signal source selection"]
    #[inline(always)]
    pub fn ir_src_sel(&self) -> IR_SRC_SEL_R {
        IR_SRC_SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 10 - XMC address mapping swap bit 0"]
    #[inline(always)]
    pub fn swap_xmc_10(&self) -> SWAP_XMC_10_R {
        SWAP_XMC_10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - XMC address mapping swap bit 1"]
    #[inline(always)]
    pub fn swap_xmc_11(&self) -> SWAP_XMC_11_R {
        SWAP_XMC_11_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG1")
            .field("mem_map_sel", &self.mem_map_sel())
            .field("ir_pol", &self.ir_pol())
            .field("ir_src_sel", &self.ir_src_sel())
            .field("swap_xmc_10", &self.swap_xmc_10())
            .field("swap_xmc_11", &self.swap_xmc_11())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Memory address mapping selection bits"]
    #[inline(always)]
    pub fn mem_map_sel(&mut self) -> MEM_MAP_SEL_W<'_, CFG1_SPEC> {
        MEM_MAP_SEL_W::new(self, 0)
    }
    #[doc = "Bit 5 - IR output polarity selection"]
    #[inline(always)]
    pub fn ir_pol(&mut self) -> IR_POL_W<'_, CFG1_SPEC> {
        IR_POL_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - IR signal source selection"]
    #[inline(always)]
    pub fn ir_src_sel(&mut self) -> IR_SRC_SEL_W<'_, CFG1_SPEC> {
        IR_SRC_SEL_W::new(self, 6)
    }
    #[doc = "Bit 10 - XMC address mapping swap bit 0"]
    #[inline(always)]
    pub fn swap_xmc_10(&mut self) -> SWAP_XMC_10_W<'_, CFG1_SPEC> {
        SWAP_XMC_10_W::new(self, 10)
    }
    #[doc = "Bit 11 - XMC address mapping swap bit 1"]
    #[inline(always)]
    pub fn swap_xmc_11(&mut self) -> SWAP_XMC_11_W<'_, CFG1_SPEC> {
        SWAP_XMC_11_W::new(self, 11)
    }
}
#[doc = "configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG1_SPEC;
impl crate::RegisterSpec for CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for CFG1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for CFG1_SPEC {}
