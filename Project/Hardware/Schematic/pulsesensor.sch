EESchema Schematic File Version 4
LIBS:lab-cache
EELAYER 26 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 3 7
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
L PASC:Pulse_sensor U?
U 1 1 5C5DA663
P 5700 3400
F 0 "U?" H 5725 3765 50  0000 C CNN
F 1 "Pulse_sensor" H 5725 3674 50  0000 C CNN
F 2 "Pin_Headers:Pin_Header_Straight_1x03_Pitch2.54mm" H 5700 3400 50  0001 C CNN
F 3 "" H 5700 3400 50  0001 C CNN
	1    5700 3400
	1    0    0    -1  
$EndComp
$Comp
L power:+3V3 #PWR?
U 1 1 5C5DA797
P 6700 3000
F 0 "#PWR?" H 6700 2850 50  0001 C CNN
F 1 "+3V3" H 6715 3173 50  0000 C CNN
F 2 "" H 6700 3000 50  0001 C CNN
F 3 "" H 6700 3000 50  0001 C CNN
	1    6700 3000
	1    0    0    -1  
$EndComp
$Comp
L Device:C C?
U 1 1 5C5DA7DA
P 6550 3600
F 0 "C?" H 6665 3646 50  0000 L CNN
F 1 "100n" H 6665 3555 50  0000 L CNN
F 2 "" H 6588 3450 50  0001 C CNN
F 3 "~" H 6550 3600 50  0001 C CNN
	1    6550 3600
	1    0    0    -1  
$EndComp
Wire Wire Line
	6200 3300 6550 3300
Wire Wire Line
	6700 3300 6700 3000
$Comp
L power:GND #PWR?
U 1 1 5C5DA906
P 5700 4050
F 0 "#PWR?" H 5700 3800 50  0001 C CNN
F 1 "GND" H 5705 3877 50  0000 C CNN
F 2 "" H 5700 4050 50  0001 C CNN
F 3 "" H 5700 4050 50  0001 C CNN
	1    5700 4050
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR?
U 1 1 5C5DA96D
P 6550 4100
F 0 "#PWR?" H 6550 3850 50  0001 C CNN
F 1 "GND" H 6555 3927 50  0000 C CNN
F 2 "" H 6550 4100 50  0001 C CNN
F 3 "" H 6550 4100 50  0001 C CNN
	1    6550 4100
	1    0    0    -1  
$EndComp
Wire Wire Line
	6550 3750 6550 4100
Wire Wire Line
	6550 3450 6550 3300
Connection ~ 6550 3300
Wire Wire Line
	6550 3300 6700 3300
Wire Wire Line
	5700 3800 5700 4050
Text HLabel 5250 3300 0    50   Input ~ 0
PULSESIG
$EndSCHEMATC
