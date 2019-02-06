EESchema Schematic File Version 4
LIBS:lab-cache
EELAYER 26 0
EELAYER END
$Descr User 5906 5906
encoding utf-8
Sheet 5 7
Title ""
Date ""
Rev ""
Comp ""
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L power:+5V #PWR?
U 1 1 5C58DDEB
P 1750 950
F 0 "#PWR?" H 1750 800 50  0001 C CNN
F 1 "+5V" H 1765 1123 50  0000 C CNN
F 2 "" H 1750 950 50  0001 C CNN
F 3 "" H 1750 950 50  0001 C CNN
	1    1750 950 
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR?
U 1 1 5C58DE2B
P 1750 1550
F 0 "#PWR?" H 1750 1300 50  0001 C CNN
F 1 "GND" H 1755 1377 50  0000 C CNN
F 2 "" H 1750 1550 50  0001 C CNN
F 3 "" H 1750 1550 50  0001 C CNN
	1    1750 1550
	1    0    0    -1  
$EndComp
$Comp
L Device:C C?
U 1 1 5C58DF2A
P 1750 1250
F 0 "C?" H 1865 1296 50  0000 L CNN
F 1 "2.2u" H 1865 1205 50  0000 L CNN
F 2 "" H 1788 1100 50  0001 C CNN
F 3 "~" H 1750 1250 50  0001 C CNN
	1    1750 1250
	1    0    0    -1  
$EndComp
Wire Wire Line
	1750 1100 1750 1050
Wire Wire Line
	1750 1400 1750 1450
Wire Wire Line
	1750 1050 1950 1050
Connection ~ 1750 1050
Wire Wire Line
	1750 1050 1750 950 
Wire Wire Line
	1750 1450 1950 1450
Connection ~ 1750 1450
Wire Wire Line
	1750 1450 1750 1550
Text Label 1950 1050 0    50   ~ 0
LCD_VDD
Text Label 1950 1450 0    50   ~ 0
LCD_VSS
$Comp
L Transistor_FET:BSS138 Q?
U 1 1 5C58DFD0
P 1800 2750
F 0 "Q?" H 2005 2796 50  0000 L CNN
F 1 "BSS138" H 2005 2705 50  0000 L CNN
F 2 "Package_TO_SOT_SMD:SOT-23" H 2000 2675 50  0001 L CIN
F 3 "https://www.fairchildsemi.com/datasheets/BS/BSS138.pdf" H 1800 2750 50  0001 L CNN
	1    1800 2750
	1    0    0    -1  
$EndComp
Text Label 1200 2750 2    50   ~ 0
LCD_BACKLIGHT
Wire Wire Line
	1900 2550 1900 2450
Wire Wire Line
	1900 2950 1900 3050
$Comp
L Device:R R?
U 1 1 5C58E912
P 1400 2750
F 0 "R?" V 1300 2750 50  0000 C CNN
F 1 "100" V 1500 2750 50  0000 C CNN
F 2 "" V 1330 2750 50  0001 C CNN
F 3 "~" H 1400 2750 50  0001 C CNN
	1    1400 2750
	0    1    1    0   
$EndComp
Wire Wire Line
	1600 2750 1550 2750
Wire Wire Line
	1250 2750 1200 2750
$Comp
L PASC:DEM16220SYM U?
U 1 1 5C69A8D5
P 4500 1900
F 0 "U?" H 4200 700 50  0000 L CNN
F 1 "DEM16220SYM" H 4200 800 50  0000 L CNN
F 2 "" H 4500 1900 50  0001 C CNN
F 3 "" H 4500 1900 50  0001 C CNN
	1    4500 1900
	1    0    0    -1  
$EndComp
Text Label 4100 1250 2    50   ~ 0
LCD_VSS
Text Label 4100 1350 2    50   ~ 0
LCS_VDD
Text Label 4100 1550 2    50   ~ 0
LCD_RS
Text Label 4100 1650 2    50   ~ 0
LCD_RW
Text Label 4100 1750 2    50   ~ 0
LCD_E
Text Label 4100 1850 2    50   ~ 0
LCD_DB0
Text Label 4100 1950 2    50   ~ 0
LCD_DB1
Text Label 4100 2050 2    50   ~ 0
LCD_DB2
Text Label 4100 2150 2    50   ~ 0
LCD_DB3
Text Label 4100 2250 2    50   ~ 0
LCD_DB4
Text Label 4100 2350 2    50   ~ 0
LCD_DB5
Text Label 4100 2450 2    50   ~ 0
LCD_DB6
Text Label 4100 2550 2    50   ~ 0
LCD_DB7
Text Label 4100 2800 2    50   ~ 0
LCD_VLED+
Text Label 4100 2700 2    50   ~ 0
LCD_VLED-
$Comp
L power:+5V #PWR?
U 1 1 5C5BE7E6
P 1900 2450
F 0 "#PWR?" H 1900 2300 50  0001 C CNN
F 1 "+5V" H 1915 2623 50  0000 C CNN
F 2 "" H 1900 2450 50  0001 C CNN
F 3 "" H 1900 2450 50  0001 C CNN
	1    1900 2450
	1    0    0    -1  
$EndComp
Wire Wire Line
	1900 3050 2000 3050
Text Label 2000 3050 0    50   ~ 0
LCD_VLED+
Text Label 2000 3250 0    50   ~ 0
LCD_VLED-
Wire Wire Line
	1900 3250 2000 3250
$Comp
L power:GND #PWR?
U 1 1 5C5BF407
P 1900 3550
F 0 "#PWR?" H 1900 3300 50  0001 C CNN
F 1 "GND" H 1905 3377 50  0000 C CNN
F 2 "" H 1900 3550 50  0001 C CNN
F 3 "" H 1900 3550 50  0001 C CNN
	1    1900 3550
	1    0    0    -1  
$EndComp
Text Label 4100 1450 2    50   ~ 0
V0
Wire Wire Line
	1900 3250 1900 3550
Text Notes 3350 1000 0    50   ~ 0
Connector to flatcable for display.
Wire Notes Line
	3300 900  5000 900 
Wire Notes Line
	5000 900  5000 3200
Wire Notes Line
	5000 3200 3300 3200
Wire Notes Line
	3300 3200 3300 900 
$EndSCHEMATC
