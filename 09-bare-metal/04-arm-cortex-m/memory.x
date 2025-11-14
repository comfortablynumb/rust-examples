/* Memory layout for generic ARM Cortex-M4F/M7F microcontroller */
/* Adjust these values based on your specific MCU */

MEMORY
{
  /* Flash memory - where program code and constants are stored */
  /* Example: 256KB flash starting at 0x0800_0000 (typical for STM32F4) */
  FLASH : ORIGIN = 0x08000000, LENGTH = 256K

  /* RAM - where variables and stack are stored */
  /* Example: 64KB RAM starting at 0x2000_0000 */
  RAM : ORIGIN = 0x20000000, LENGTH = 64K
}

/* This is used by cortex-m-rt to place the stack */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);

/*
 * Additional notes:
 *
 * Different MCUs have different memory layouts. Examples:
 *
 * STM32F103 (Cortex-M3):
 *   FLASH: ORIGIN = 0x08000000, LENGTH = 64K
 *   RAM:   ORIGIN = 0x20000000, LENGTH = 20K
 *
 * STM32F407 (Cortex-M4F):
 *   FLASH: ORIGIN = 0x08000000, LENGTH = 1024K
 *   RAM:   ORIGIN = 0x20000000, LENGTH = 128K
 *
 * nRF52840 (Cortex-M4F):
 *   FLASH: ORIGIN = 0x00000000, LENGTH = 1024K
 *   RAM:   ORIGIN = 0x20000000, LENGTH = 256K
 *
 * Always check your MCU's datasheet for the correct values!
 */
