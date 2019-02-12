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
L power:GND #PWR?
U 1 1 5C58DE2B
P 850 1600
AR Path="/5C564F2E/5C58DE2B" Ref="#PWR?"  Part="1" 
AR Path="/5C5FFA20/5C58DE2B" Ref="#PWR?"  Part="1" 
F 0 "#PWR?" H 850 1350 50  0001 C CNN
F 1 "GND" H 855 1427 50  0000 C CNN
F 2 "" H 850 1600 50  0001 C CNN
F 3 "" H 850 1600 50  0001 C CNN
	1    850  1600
	1    0    0    -1  
$EndComp
$Comp
L Device:C C?
U 1 1 5C58DF2A
P 850 1300
AR Path="/5C564F2E/5C58DF2A" Ref="C?"  Part="1" 
AR Path="/5C5FFA20/5C58DF2A" Ref="C?"  Part="1" 
F 0 "C?" H 965 1346 50  0000 L CNN
F 1 "1u" H 965 1255 50  0000 L CNN
F 2 "" H 888 1150 50  0001 C CNN
F 3 "~" H 850 1300 50  0001 C CNN
	1    850  1300
	1    0    0    -1  
$EndComp
Wire Wire Line
	850  1150 850  1100
Wire Wire Line
	850  1450 850  1500
Wire Wire Line
	850  1100 1050 1100
Connection ~ 850  1100
Wire Wire Line
	850  1100 850  1000
Wire Wire Line
	850  1500 1050 1500
Connection ~ 850  1500
Wire Wire Line
	850  1500 850  1600
Text Label 1050 1100 0    50   ~ 0
LCD_VDD
Text Label 1050 1500 0    50   ~ 0
LCD_VSS
$Comp
L power:+3.3V #PWR?
U 1 1 5C66DBDE
P 850 1000
AR Path="/5C564F2E/5C66DBDE" Ref="#PWR?"  Part="1" 
AR Path="/5C5FFA20/5C66DBDE" Ref="#PWR?"  Part="1" 
F 0 "#PWR?" H 850 850 50  0001 C CNN
F 1 "+3.3V" H 865 1173 50  0000 C CNN
F 2 "" H 850 1000 50  0001 C CNN
F 3 "" H 850 1000 50  0001 C CNN
	1    850  1000
	1    0    0    -1  
$EndComp
$Comp
L PASC:Nokia5110 U?
U 1 1 5C63A382
P 3950 2200
F 0 "U?" H 3950 2675 50  0000 C CNN
F 1 "Nokia5110" H 3950 2584 50  0000 C CNN
F 2 "PASC:Nokia5110" H 4000 2200 50  0001 C CNN
F 3 "" H 4000 2200 50  0001 C CNN
	1    3950 2200
	1    0    0    -1  
$EndComp
Text Label 3450 1950 2    50   ~ 0
LCD_VDD
$Comp
L Device:R R?
U 1 1 5C63A8F8
P 1700 1500
F 0 "R?" H 1770 1546 50  0000 L CNN
F 1 "330" H 1770 1455 50  0000 L CNN
F 2 "" V 1630 1500 50  0001 C CNN
F 3 "~" H 1700 1500 50  0001 C CNN
	1    1700 1500
	1    0    0    -1  
$EndComp
Wire Wire Line
	1700 1650 1700 1850
Text Label 1700 1850 0    50   ~ 0
LED
Text Label 3450 2050 2    50   ~ 0
LED
Wire Wire Line
	1700 1350 1700 1150
Text HLabel 1700 1150 1    50   Input ~ 0
LCD_LED
$Comp
L power:GND #PWR?
U 1 1 5C63AC65
P 4050 2800
F 0 "#PWR?" H 4050 2550 50  0001 C CNN
F 1 "GND" H 4055 2627 50  0000 C CNN
F 2 "" H 4050 2800 50  0001 C CNN
F 3 "" H 4050 2800 50  0001 C CNN
	1    4050 2800
	1    0    0    -1  
$EndComp
Wire Wire Line
	4050 2650 4050 2800
Text HLabel 3450 2300 0    50   Input ~ 0
LCD_DN
Text HLabel 3450 2400 0    50   Input ~ 0
LCD_SCK
Text HLabel 4450 2050 2    50   Input ~ 0
LCD_SCE
Text HLabel 4450 2150 2    50   Input ~ 0
LCD_RST
Text HLabel 4450 2250 2    50   Input ~ 0
LCD_DC
$EndSCHEMATC
