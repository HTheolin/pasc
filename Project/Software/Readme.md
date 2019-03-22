# PASC software

## How To: Debug/Program

```
$ cargo build
$ openocd -f openocd.cfg
```

```
$ arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/Software -x openocd.gdb
```

## Difference between boards:
Layout differences between boards:

### Schematic_ons
```
    Button U9  -> MCU pin 26 (PB0)
    Button U10 -> MCU pin 27 (PB1)
    Button U11 -> MCU pin 28 (PB2)
```
LCD:
```
        3: LCD_SCE -> MCU pin 8 (PC0)
        4: LCD_RST -> MCU pin 9 (PC1)
        5: LCD_DC -> MCU pin 10 (PC2)
        6: SPI1_MOSI -> MCU pin 23 (PA7)
        7: SPI1_SCK -> MCU pin 21 (PA5)
        8: LED -> MCU pin 22 (PA6)
```

### Schematic_Ubuntu
```
    Button U9  -> MCU pin 38 (PC7)
    Button U10 -> MCU pin 39 (PC8)
    Button U11 -> MCU pin 40 (PC9)
```
LCD:
```
        3: LCD_SCE -> MCU pin 25 (PC5)
        4: LCD_RST -> MCU pin 24 (PC4)
        5: LCD_DC -> MCU pin 26 (PB0)
        6: SPI1_MOSI -> MCU pin 23 (PA7, set to SPI1_MOSI, AF05)
        7: SPI1_SCK -> MCU pin 21 (PA5, set to SPI1_SK, AF05)
        8: LED -> MCU pin 37 (PC6, set to TIM3_CH1, AF02)
```