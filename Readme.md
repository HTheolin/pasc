# Start by a High Level Specification
Hälsoarband som mäter puls och steg med pulsmätare och accelerometer.
Knapp som startar och återställer stegräknare. Stömspar som stänger display, ev efter tid av ingen puls läsning.
##Purpose (what problem it solves)
Mäta puls och känna av om pulsen blir låg/hög, varna vid dessa.
##Limitations (what problem(s) it cannot solve)
Visar endast realtid puls, yttre temperatur och steg, inget minne. Larmar endast närmaste omgivnignen. 
##Behaviour (how the problem is solved)
Genom att mäta puls med en sensor en gång per sek, om den blir låg eller hög starta larm med högtalare. Visar data på display. Återställer data och startar genom knapptryck. Med detta system skapas en tryggare värld.

##Safety (what it must not do, e.g., electrocute the user)
Eftersom systemet är designat att vara I kontakt med person, måste tåla viss mängd fukt och inte ge elektiska stötar till användaren.
##Liveness (what it must eventually do, e.g., process data) 
Måste visa puls minst var 10e sekund under batteritid. 
##Robustness (how it should react to non-expected input, e.g., mal-formed packages)
Om ingen puls stäng system. 
