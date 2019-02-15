EESchema Schematic File Version 4
LIBS:PCB_Simon_mount_rev2-cache
EELAYER 26 0
EELAYER END
$Descr User 5512 5512
encoding utf-8
Sheet 3 7
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
L PASC:Pulse_sensor U4
U 1 1 5C5DA663
P 2400 1900
F 0 "U4" H 2425 2265 50  0000 C CNN
F 1 "Pulse_sensor" H 2425 2174 50  0000 C CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_1x03_P2.54mm_Vertical" H 2400 1900 50  0001 C CNN
F 3 "" H 2400 1900 50  0001 C CNN
	1    2400 1900
	1    0    0    -1  
$EndComp
$Comp
L PCB_Simon_mount_rev2-rescue:+3V3-power #PWR0125
U 1 1 5C5DA797
P 3400 1500
F 0 "#PWR0125" H 3400 1350 50  0001 C CNN
F 1 "+3V3" H 3415 1673 50  0000 C CNN
F 2 "" H 3400 1500 50  0001 C CNN
F 3 "" H 3400 1500 50  0001 C CNN
	1    3400 1500
	1    0    0    -1  
$EndComp
$Comp
L PCB_Simon_mount_rev2-rescue:C-Device C17
U 1 1 5C5DA7DA
P 3250 2100
F 0 "C17" H 3365 2146 50  0000 L CNN
F 1 "100n" H 3365 2055 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 3288 1950 50  0001 C CNN
F 3 "~" H 3250 2100 50  0001 C CNN
	1    3250 2100
	1    0    0    -1  
$EndComp
Wire Wire Line
	2900 1800 3250 1800
Wire Wire Line
	3400 1800 3400 1500
$Comp
L PCB_Simon_mount_rev2-rescue:GND-power #PWR0126
U 1 1 5C5DA906
P 2400 2550
F 0 "#PWR0126" H 2400 2300 50  0001 C CNN
F 1 "GND" H 2405 2377 50  0000 C CNN
F 2 "" H 2400 2550 50  0001 C CNN
F 3 "" H 2400 2550 50  0001 C CNN
	1    2400 2550
	1    0    0    -1  
$EndComp
$Comp
L PCB_Simon_mount_rev2-rescue:GND-power #PWR0127
U 1 1 5C5DA96D
P 3250 2600
F 0 "#PWR0127" H 3250 2350 50  0001 C CNN
F 1 "GND" H 3255 2427 50  0000 C CNN
F 2 "" H 3250 2600 50  0001 C CNN
F 3 "" H 3250 2600 50  0001 C CNN
	1    3250 2600
	1    0    0    -1  
$EndComp
Wire Wire Line
	3250 2250 3250 2600
Wire Wire Line
	3250 1950 3250 1800
Connection ~ 3250 1800
Wire Wire Line
	3250 1800 3400 1800
Wire Wire Line
	2400 2300 2400 2550
Wire Wire Line
	1950 1800 1500 1800
Wire Wire Line
	1200 1800 1100 1800
$Comp
L Device:R R18
U 1 1 5C68650D
P 1350 1800
F 0 "R18" V 1143 1800 50  0000 C CNN
F 1 "1k" V 1234 1800 50  0000 C CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" V 1280 1800 50  0001 C CNN
F 3 "~" H 1350 1800 50  0001 C CNN
	1    1350 1800
	0    1    1    0   
$EndComp
Text HLabel 1100 1800 0    50   Input ~ 0
PULSESIG
$EndSCHEMATC
