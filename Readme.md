# Användbara länkar

## Rust
svd2rust - https://docs.rs/svd2rust/0.14.0/svd2rust/  \
Rustboken - https://doc.rust-lang.org/1.4.0/book/README.html  \
Embedde rust - https://rust-embedded.github.io/book/intro/index.html  \
CubeMX och Rust - http://www.hashmismatch.net/pragmatic-bare-metal-rust/

## Hårdvara
STM32F401RE datasheet - https://www.st.com/resource/en/datasheet/stm32f401re.pdf

## Dokument
High Level specification - https://docs.google.com/document/d/1pT679ALJbP6M4RAu0LiHIDUVMe5iSUqeqp4-EVYOUyA/edit# \
Komponenter - https://docs.google.com/spreadsheets/d/1eK-V_FMpTHM5hmX8ijiI2q-JYo_6xdile7O38F0DbRs/edit#gid=0

# High Level Specification
Hälsoarmband som mäter puls och steg med pulsmätare och accelerometer.
Knapp som startar och återställer stegräknare. Strömspar som stänger display, sänker sampling frekvens, ... ev. efter lång tid utan knapptryck.
## Purpose (what problem it solves)
Mäta puls och känna av om pulsen blir låg/hög, varna vid dessa.
## Limitations (what problem(s) it cannot solve)
Visar endast realtid puls, yttre temperatur och steg, inget minne. Larmar endast närmaste omgivningen. 
## Behaviour (how the problem is solved)
Genom att mäta puls med en sensor en gång per sek, om den blir låg eller hög starta larm med högtalare. Visar data på display. Återställer data och startar genom knapptryck. Med detta system skapas en tryggare värld.

## Safety (what it must not do, e.g., electrocute the user)
Eftersom systemet är designat att vara i kontakt med person, måste tåla viss mängd fukt och inte ge elektiska stötar till användaren.
## Liveness (what it must eventually do, e.g., process data) 
Måste visa puls minst var 10e sekund under batteritid. 
## Robustness (how it should react to non-expected input, e.g., mal-formed packages)
Om ingen puls stäng system. 

# Gradings
## Grade 3
Pulse measurements from analog in. \
Timer that's set using buttons. \
Buzzer that sounds alarm when high pulse or timer reaching zero. \
USART for serial communication with com.
## Grade 4
Temperature measurements using analog in. \
Output to display using SPI \
Low power mode
## Grade 5
Accelerometer detecting steps, using accelerometer values or tap interrupts over I2C. \
Menu for selecting output to display and other settings.

# Group division
All - STM 32 init, Soldering \
August - ADC, Buttons, Display Layouts \
Simon - USART, Display \
Ridge - Timer, Piezo \
Henrik - Display, Acccelerometer

![Alt text](/Project/Design/ProjectHigh.png?raw=true "System design")
