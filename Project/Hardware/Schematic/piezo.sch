EESchema Schematic File Version 4
LIBS:lab-cache
EELAYER 26 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 6 7
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
L PASC:piezo U7
U 1 1 5C5E3EC0
P 6600 3700
F 0 "U7" H 6877 3746 50  0000 L CNN
F 1 "piezo" H 6877 3655 50  0000 L CNN
F 2 "PASC:Piezo" H 6600 3700 50  0001 C CNN
F 3 "" H 6600 3700 50  0001 C CNN
	1    6600 3700
	1    0    0    -1  
$EndComp
$Comp
L Device:R R14
U 1 1 5C5EB38A
P 5700 3500
F 0 "R14" H 5770 3546 50  0000 L CNN
F 1 "1k" H 5770 3455 50  0000 L CNN
F 2 "" V 5630 3500 50  0001 C CNN
F 3 "~" H 5700 3500 50  0001 C CNN
	1    5700 3500
	1    0    0    -1  
$EndComp
$Comp
L Diode:1N4148 D1
U 1 1 5C5EB4E1
P 5700 3950
F 0 "D1" V 5654 4029 50  0000 L CNN
F 1 "1N4148" V 5745 4029 50  0000 L CNN
F 2 "Diode_THT:D_DO-35_SOD27_P7.62mm_Horizontal" H 5700 3775 50  0001 C CNN
F 3 "http://www.nxp.com/documents/data_sheet/1N4148_1N4448.pdf" H 5700 3950 50  0001 C CNN
	1    5700 3950
	0    1    1    0   
$EndComp
Wire Wire Line
	5700 3000 5700 3250
Wire Wire Line
	5700 3650 5700 3800
Wire Wire Line
	5700 4100 5700 4200
Wire Wire Line
	6150 3600 6150 3250
Wire Wire Line
	6150 3250 5700 3250
Connection ~ 5700 3250
Wire Wire Line
	5700 3250 5700 3350
Wire Wire Line
	6150 3800 6150 4200
Wire Wire Line
	6150 4200 5700 4200
Connection ~ 5700 4200
Wire Wire Line
	5700 4200 5700 4400
$Comp
L Device:R R13
U 1 1 5C5EB823
P 4900 4600
F 0 "R13" V 4693 4600 50  0000 C CNN
F 1 "1k" V 4784 4600 50  0000 C CNN
F 2 "" V 4830 4600 50  0001 C CNN
F 3 "~" H 4900 4600 50  0001 C CNN
	1    4900 4600
	0    1    1    0   
$EndComp
Wire Wire Line
	5050 4600 5400 4600
$Comp
L power:GND #PWR0137
U 1 1 5C5EB928
P 5700 5050
F 0 "#PWR0137" H 5700 4800 50  0001 C CNN
F 1 "GND" H 5705 4877 50  0000 C CNN
F 2 "" H 5700 5050 50  0001 C CNN
F 3 "" H 5700 5050 50  0001 C CNN
	1    5700 5050
	1    0    0    -1  
$EndComp
Wire Wire Line
	5700 4800 5700 5050
Wire Wire Line
	4750 4600 4300 4600
Text HLabel 4300 4600 0    50   Input ~ 0
PIEZOPWM
$Comp
L Transistor_BJT:2N3906 Q1
U 1 1 5C68E108
P 5600 4600
F 0 "Q1" H 5791 4646 50  0000 L CNN
F 1 "2N3906" H 5791 4555 50  0000 L CNN
F 2 "TO_SOT_Packages_THT:TO-92_Inline_Wide" H 5800 4525 50  0001 L CIN
F 3 "https://www.fairchildsemi.com/datasheets/2N/2N3906.pdf" H 5600 4600 50  0001 L CNN
	1    5600 4600
	1    0    0    -1  
$EndComp
$Comp
L power:+3.3V #PWR0138
U 1 1 5C6DC10D
P 5700 3000
F 0 "#PWR0138" H 5700 2850 50  0001 C CNN
F 1 "+3.3V" H 5715 3173 50  0000 C CNN
F 2 "" H 5700 3000 50  0001 C CNN
F 3 "" H 5700 3000 50  0001 C CNN
	1    5700 3000
	1    0    0    -1  
$EndComp
$EndSCHEMATC
