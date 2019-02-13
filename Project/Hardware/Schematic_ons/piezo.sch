EESchema Schematic File Version 4
EELAYER 26 0
EELAYER END
$Descr User 5512 5512
encoding utf-8
Sheet 6 7
Title ""
Date "2019-02-12"
Rev "1"
Comp "PASC"
Comment1 ""
Comment2 ""
Comment3 ""
Comment4 ""
$EndDescr
$Comp
L PASC:piezo U7
U 1 1 5C5E3EC0
P 3800 1800
F 0 "U7" H 4077 1846 50  0000 L CNN
F 1 "piezo" H 4077 1755 50  0000 L CNN
F 2 "PASC:Piezo" H 3800 1800 50  0001 C CNN
F 3 "" H 3800 1800 50  0001 C CNN
	1    3800 1800
	1    0    0    -1  
$EndComp
$Comp
L Device:R R14
U 1 1 5C5EB38A
P 2900 1600
F 0 "R14" H 2970 1646 50  0000 L CNN
F 1 "1k" H 2970 1555 50  0000 L CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" V 2830 1600 50  0001 C CNN
F 3 "~" H 2900 1600 50  0001 C CNN
	1    2900 1600
	1    0    0    -1  
$EndComp
$Comp
L Diode:1N4148 D1
U 1 1 5C5EB4E1
P 2900 2050
F 0 "D1" V 2854 2129 50  0000 L CNN
F 1 "1N4148" V 2945 2129 50  0000 L CNN
F 2 "Diode_THT:D_DO-35_SOD27_P7.62mm_Horizontal" H 2900 1875 50  0001 C CNN
F 3 "http://www.nxp.com/documents/data_sheet/1N4148_1N4448.pdf" H 2900 2050 50  0001 C CNN
	1    2900 2050
	0    1    1    0   
$EndComp
Wire Wire Line
	2900 1100 2900 1350
Wire Wire Line
	2900 1750 2900 1900
Wire Wire Line
	2900 2200 2900 2300
Wire Wire Line
	3350 1700 3350 1350
Wire Wire Line
	3350 1350 2900 1350
Connection ~ 2900 1350
Wire Wire Line
	2900 1350 2900 1450
Wire Wire Line
	3350 1900 3350 2300
Wire Wire Line
	3350 2300 2900 2300
Connection ~ 2900 2300
Wire Wire Line
	2900 2300 2900 2500
$Comp
L Device:R R13
U 1 1 5C5EB823
P 2100 2700
F 0 "R13" V 1893 2700 50  0000 C CNN
F 1 "1k" V 1984 2700 50  0000 C CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" V 2030 2700 50  0001 C CNN
F 3 "~" H 2100 2700 50  0001 C CNN
	1    2100 2700
	0    1    1    0   
$EndComp
Wire Wire Line
	2250 2700 2600 2700
$Comp
L power:GND #PWR0137
U 1 1 5C5EB928
P 2900 3150
F 0 "#PWR0137" H 2900 2900 50  0001 C CNN
F 1 "GND" H 2905 2977 50  0000 C CNN
F 2 "" H 2900 3150 50  0001 C CNN
F 3 "" H 2900 3150 50  0001 C CNN
	1    2900 3150
	1    0    0    -1  
$EndComp
Wire Wire Line
	2900 2900 2900 3150
Wire Wire Line
	1950 2700 1500 2700
Text HLabel 1500 2700 0    50   Input ~ 0
PIEZOPWM
$Comp
L power:+3.3V #PWR0138
U 1 1 5C6DC10D
P 2900 1100
F 0 "#PWR0138" H 2900 950 50  0001 C CNN
F 1 "+3.3V" H 2915 1273 50  0000 C CNN
F 2 "" H 2900 1100 50  0001 C CNN
F 3 "" H 2900 1100 50  0001 C CNN
	1    2900 1100
	1    0    0    -1  
$EndComp
$Comp
L Transistor_BJT:2N3904 Q1
U 1 1 5C64EC5C
P 2800 2700
F 0 "Q1" H 2991 2746 50  0000 L CNN
F 1 "2N3904" H 2991 2655 50  0000 L CNN
F 2 "Package_TO_SOT_THT:TO-92_Inline_Wide" H 3000 2625 50  0001 L CIN
F 3 "https://www.fairchildsemi.com/datasheets/2N/2N3904.pdf" H 2800 2700 50  0001 L CNN
	1    2800 2700
	1    0    0    -1  
$EndComp
$EndSCHEMATC
