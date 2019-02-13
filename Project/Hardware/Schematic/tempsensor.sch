EESchema Schematic File Version 4
LIBS:lab-cache
EELAYER 26 0
EELAYER END
$Descr User 9055 6693
encoding utf-8
Sheet 4 7
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
L Sensor_Temperature:PT1000 TH1
U 1 1 5C561FEF
P 2700 2000
F 0 "TH1" H 2798 2046 50  0000 L CNN
F 1 "PT1000" H 2798 1955 50  0000 L CNN
F 2 "Package_TO_SOT_THT:TO-92-2" H 2700 2050 50  0001 C CNN
F 3 "https://www.heraeus.com/media/media/group/doc_group/products_1/hst/sot_to/de_15/to_92_d.pdf" H 2700 2050 50  0001 C CNN
	1    2700 2000
	1    0    0    -1  
$EndComp
$Comp
L Device:R R7
U 1 1 5C5621D5
P 2700 1600
F 0 "R7" H 2770 1646 50  0000 L CNN
F 1 "1k" H 2770 1555 50  0000 L CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" V 2630 1600 50  0001 C CNN
F 3 "~" H 2700 1600 50  0001 C CNN
	1    2700 1600
	1    0    0    -1  
$EndComp
$Comp
L Device:R R3
U 1 1 5C5622D1
P 1400 2000
F 0 "R3" H 1470 2046 50  0000 L CNN
F 1 "8.2k" H 1470 1955 50  0000 L CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" V 1330 2000 50  0001 C CNN
F 3 "~" H 1400 2000 50  0001 C CNN
	1    1400 2000
	1    0    0    -1  
$EndComp
$Comp
L Device:R R8
U 1 1 5C562312
P 2700 2600
F 0 "R8" H 2770 2646 50  0000 L CNN
F 1 "1.8k" H 2770 2555 50  0000 L CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" V 2630 2600 50  0001 C CNN
F 3 "~" H 2700 2600 50  0001 C CNN
	1    2700 2600
	1    0    0    -1  
$EndComp
$Comp
L Device:R R4
U 1 1 5C56234C
P 1400 2600
F 0 "R4" H 1470 2646 50  0000 L CNN
F 1 "8.2k" H 1470 2555 50  0000 L CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" V 1330 2600 50  0001 C CNN
F 3 "~" H 1400 2600 50  0001 C CNN
	1    1400 2600
	1    0    0    -1  
$EndComp
$Comp
L PASC:MAX4460ESA+ U?
U 1 1 5C562619
P 4500 1500
AR Path="/5C562619" Ref="U?"  Part="1" 
AR Path="/5C564F10/5C562619" Ref="U5"  Part="1" 
F 0 "U5" H 5600 1887 60  0000 C CNN
F 1 "MAX4460ESA+" H 5600 1781 60  0000 C CNN
F 2 "Package_TO_SOT_SMD:SOT-23-8_Handsoldering" H 5600 1740 60  0001 C CNN
F 3 "" H 4500 1500 60  0000 C CNN
	1    4500 1500
	1    0    0    -1  
$EndComp
$Comp
L power:+3V3 #PWR0128
U 1 1 5C5626CF
P 1100 2100
F 0 "#PWR0128" H 1100 1950 50  0001 C CNN
F 1 "+3V3" H 1115 2273 50  0000 C CNN
F 2 "" H 1100 2100 50  0001 C CNN
F 3 "" H 1100 2100 50  0001 C CNN
	1    1100 2100
	1    0    0    -1  
$EndComp
Wire Wire Line
	1100 2100 1100 2300
Wire Wire Line
	1100 2300 1400 2300
Wire Wire Line
	1400 2300 1400 2150
Wire Wire Line
	1400 2300 1400 2450
Connection ~ 1400 2300
Wire Wire Line
	1400 2750 1400 2900
Wire Wire Line
	1400 2900 1800 2900
Wire Wire Line
	2700 2900 2700 2750
Wire Wire Line
	2700 2450 2700 2300
Wire Wire Line
	1400 1850 1400 1300
Wire Wire Line
	1400 1300 1800 1300
Wire Wire Line
	2700 1750 2700 1850
$Comp
L power:GND #PWR0129
U 1 1 5C562A08
P 3100 2450
F 0 "#PWR0129" H 3100 2200 50  0001 C CNN
F 1 "GND" H 3105 2277 50  0000 C CNN
F 2 "" H 3100 2450 50  0001 C CNN
F 3 "" H 3100 2450 50  0001 C CNN
	1    3100 2450
	1    0    0    -1  
$EndComp
Wire Wire Line
	3100 2450 3100 2300
Wire Wire Line
	3100 2300 2700 2300
Connection ~ 2700 2300
Wire Wire Line
	2700 2300 2700 2150
Wire Wire Line
	2700 1300 2700 1450
Wire Wire Line
	2700 1300 3200 1300
Connection ~ 2700 1300
Wire Wire Line
	3200 2900 2700 2900
Connection ~ 2700 2900
Text Label 3200 2900 0    50   ~ 0
WBRIDGE-
Text Label 3200 1300 0    50   ~ 0
WBRIDGE+
Text Label 4500 1700 2    50   ~ 0
WBRIDGE+
Text Label 6700 1700 0    50   ~ 0
WBRIDGE-
Text Label 6700 1500 0    50   ~ 0
INAMPFB
$Comp
L Device:R R9
U 1 1 5C5639FE
P 4100 3100
F 0 "R9" H 4170 3146 50  0000 L CNN
F 1 "100k" H 4170 3055 50  0000 L CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" V 4030 3100 50  0001 C CNN
F 3 "~" H 4100 3100 50  0001 C CNN
	1    4100 3100
	1    0    0    -1  
$EndComp
$Comp
L Device:R R10
U 1 1 5C563A36
P 4100 3500
F 0 "R10" H 4170 3546 50  0000 L CNN
F 1 "2.7k" H 4170 3455 50  0000 L CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" V 4030 3500 50  0001 C CNN
F 3 "~" H 4100 3500 50  0001 C CNN
	1    4100 3500
	1    0    0    -1  
$EndComp
Wire Wire Line
	4100 2950 4100 2900
Wire Wire Line
	4100 3250 4100 3300
Wire Wire Line
	4100 3300 4300 3300
Connection ~ 4100 3300
Wire Wire Line
	4100 3300 4100 3350
Text Label 4300 3300 0    50   ~ 0
INAMPFB
Text Label 4100 2900 0    50   ~ 0
TEMPOUT
$Comp
L power:GND #PWR0130
U 1 1 5C5640C7
P 4100 4150
F 0 "#PWR0130" H 4100 3900 50  0001 C CNN
F 1 "GND" H 4105 3977 50  0000 C CNN
F 2 "" H 4100 4150 50  0001 C CNN
F 3 "" H 4100 4150 50  0001 C CNN
	1    4100 4150
	1    0    0    -1  
$EndComp
Text Notes 4700 3000 0    50   ~ 0
Set gain: 1 + R2/R1; R1 + R2 approx. = 100k\nR2 between OUT and FB,\nR1 between FB and GND.
Wire Notes Line
	3900 2700 6600 2700
Wire Notes Line
	6600 2700 6600 4500
Wire Notes Line
	6600 4500 3900 4500
Wire Notes Line
	3900 4500 3900 2700
$Comp
L power:+3V3 #PWR0131
U 1 1 5C56452F
P 7200 1400
F 0 "#PWR0131" H 7200 1250 50  0001 C CNN
F 1 "+3V3" H 7100 1600 50  0000 C CNN
F 2 "" H 7200 1400 50  0001 C CNN
F 3 "" H 7200 1400 50  0001 C CNN
	1    7200 1400
	1    0    0    -1  
$EndComp
Wire Wire Line
	7200 1400 7200 1600
Wire Wire Line
	7200 1600 6700 1600
$Comp
L power:GND #PWR0132
U 1 1 5C564858
P 4000 1800
F 0 "#PWR0132" H 4000 1550 50  0001 C CNN
F 1 "GND" H 4005 1627 50  0000 C CNN
F 2 "" H 4000 1800 50  0001 C CNN
F 3 "" H 4000 1800 50  0001 C CNN
	1    4000 1800
	1    0    0    -1  
$EndComp
Wire Wire Line
	4500 1600 4000 1600
Wire Wire Line
	4000 1600 4000 1800
Wire Notes Line
	7600 2600 7600 800 
Text Notes 4200 1000 0    50   ~ 0
Instrumentation Amplifier
Text Notes 1000 1000 0    50   ~ 0
Wheatstone Bridge for Pt1000 thermistor.\n
$Comp
L Device:C C18
U 1 1 5C565EBC
P 7200 1900
F 0 "C18" H 7315 1946 50  0000 L CNN
F 1 "100n" H 7315 1855 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 7238 1750 50  0001 C CNN
F 3 "~" H 7200 1900 50  0001 C CNN
	1    7200 1900
	1    0    0    -1  
$EndComp
Wire Wire Line
	7200 1600 7200 1750
Connection ~ 7200 1600
$Comp
L power:GND #PWR0133
U 1 1 5C5665E0
P 7200 2200
F 0 "#PWR0133" H 7200 1950 50  0001 C CNN
F 1 "GND" H 7205 2027 50  0000 C CNN
F 2 "" H 7200 2200 50  0001 C CNN
F 3 "" H 7200 2200 50  0001 C CNN
	1    7200 2200
	1    0    0    -1  
$EndComp
Wire Wire Line
	7200 2050 7200 2200
Wire Notes Line
	3900 2600 7600 2600
Wire Notes Line
	3900 800  7600 800 
Wire Notes Line
	3900 800  3900 2600
Wire Notes Line
	3700 800  800  800 
Wire Notes Line
	800  800  800  3100
Wire Notes Line
	800  3100 3700 3100
Wire Notes Line
	3700 3100 3700 800 
$Comp
L Device:R R5
U 1 1 5C58B777
P 1800 2000
F 0 "R5" H 1870 2046 50  0000 L CNN
F 1 "270k" H 1870 1955 50  0000 L CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" V 1730 2000 50  0001 C CNN
F 3 "~" H 1800 2000 50  0001 C CNN
	1    1800 2000
	1    0    0    -1  
$EndComp
$Comp
L Device:R R6
U 1 1 5C58B7A7
P 1800 2600
F 0 "R6" H 1870 2646 50  0000 L CNN
F 1 "270k" H 1870 2555 50  0000 L CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" V 1730 2600 50  0001 C CNN
F 3 "~" H 1800 2600 50  0001 C CNN
	1    1800 2600
	1    0    0    -1  
$EndComp
$Comp
L Device:R R11
U 1 1 5C58C2D0
P 4100 3900
F 0 "R11" H 4170 3946 50  0000 L CNN
F 1 "820" H 4170 3855 50  0000 L CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" V 4030 3900 50  0001 C CNN
F 3 "~" H 4100 3900 50  0001 C CNN
	1    4100 3900
	1    0    0    -1  
$EndComp
Wire Wire Line
	4100 3750 4100 3650
Wire Wire Line
	4100 4050 4100 4150
Wire Wire Line
	1800 2900 1800 2750
Connection ~ 1800 2900
Wire Wire Line
	1800 2900 2700 2900
Wire Wire Line
	1800 2450 1800 2300
Wire Wire Line
	1400 2300 1800 2300
Connection ~ 1800 2300
Wire Wire Line
	1800 2300 1800 2150
Wire Wire Line
	1800 1850 1800 1300
Connection ~ 1800 1300
Wire Wire Line
	1800 1300 2700 1300
Text HLabel 4500 1500 0    50   Input ~ 0
TEMPOUT
$EndSCHEMATC
