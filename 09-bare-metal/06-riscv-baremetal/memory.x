/* Memory layout for QEMU RISC-V virt machine */

MEMORY
{
  /* RAM starts at 0x8000_0000 for QEMU virt machine */
  RAM : ORIGIN = 0x80000000, LENGTH = 128M
}

/*
 * Region Aliases
 * These are used by the riscv-rt crate
 */
REGION_ALIAS("REGION_TEXT", RAM);
REGION_ALIAS("REGION_RODATA", RAM);
REGION_ALIAS("REGION_DATA", RAM);
REGION_ALIAS("REGION_BSS", RAM);
REGION_ALIAS("REGION_HEAP", RAM);
REGION_ALIAS("REGION_STACK", RAM);

/*
 * Notes for different RISC-V platforms:
 *
 * QEMU virt machine:
 *   RAM: ORIGIN = 0x80000000, LENGTH = 128M
 *
 * SiFive HiFive1 (FE310):
 *   FLASH: ORIGIN = 0x20000000, LENGTH = 16M
 *   RAM:   ORIGIN = 0x80000000, LENGTH = 16K
 *
 * Kendryte K210:
 *   RAM: ORIGIN = 0x80000000, LENGTH = 6M
 *
 * ESP32-C3:
 *   IRAM: ORIGIN = 0x4037C000, LENGTH = 400K
 *   DRAM: ORIGIN = 0x3FC80000, LENGTH = 400K
 *
 * Always check your SoC's datasheet for correct memory regions!
 */
