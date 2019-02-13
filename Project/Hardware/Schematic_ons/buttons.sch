EESchema Schematic File Version 4
EELAYER 26 0
EELAYER END
$Descr User 5512 9055
encoding utf-8
Sheet 7 7
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
L PASC:MomentSwitch U8
U 1 1 5C5ECEC3
P 2700 1400
F 0 "U8" H 2725 1825 50  0000 C CNN
F 1 "MomentSwitch" H 2725 1734 50  0000 C CNN
F 2 "PASC:Momentswitch" H 2700 1400 50  0001 C CNN
F 3 "" H 2700 1400 50  0001 C CNN
	1    2700 1400
	1    0    0    -1  
$EndComp
Wire Wire Line
	2350 1200 2250 1200
Wire Wire Line
	2250 1200 2250 1400
Wire Wire Line
	2250 1600 2350 1600
Wire Wire Line
	3050 1200 3150 1200
Wire Wire Line
	3150 1200 3150 1400
Wire Wire Line
	3150 1600 3050 1600
Wire Wire Line
	3150 1400 3250 1400
Connection ~ 3150 1400
Wire Wire Line
	3150 1400 3150 1600
Wire Wire Line
	2250 1400 2150 1400
Connection ~ 2250 1400
Wire Wire Line
	2250 1400 2250 1600
$Comp
L power:GND #PWR0139
U 1 1 5C5ED863
P 2150 1400
F 0 "#PWR0139" H 2150 1150 50  0001 C CNN
F 1 "GND" H 2155 1227 50  0000 C CNN
F 2 "" H 2150 1400 50  0001 C CNN
F 3 "" H 2150 1400 50  0001 C CNN
	1    2150 1400
	1    0    0    -1  
$EndComp
Text HLabel 3250 1400 2    50   Input ~ 0
NRST
$Comp
L PASC:MomentSwitch U9
U 1 1 5C68DC82
P 2700 3200
F 0 "U9" H 2700 3625 50  0000 C CNN
F 1 "MomentSwitch" H 2700 3534 50  0000 C CNN
F 2 "PASC:Momentswitch" H 2700 3200 50  0001 C CNN
F 3 "" H 2700 3200 50  0001 C CNN
	1    2700 3200
	1    0    0    -1  
$EndComp
Wire Wire Line
	3050 3000 3150 3000
Wire Wire Line
	3150 3400 3050 3400
Wire Wire Line
	2350 3000 2250 3000
Wire Wire Line
	2250 3400 2350 3400
Text HLabel 3650 3200 2    50   Input ~ 0
TIM_START
Wire Wire Line
	2250 3000 2250 3200
$Comp
L power:+3.3V #PWR0140
U 1 1 5C68E9D0
P 2050 3100
F 0 "#PWR0140" H 2050 2950 50  0001 C CNN
F 1 "+3.3V" H 2065 3273 50  0000 C CNN
F 2 "" H 2050 3100 50  0001 C CNN
F 3 "" H 2050 3100 50  0001 C CNN
	1    2050 3100
	1    0    0    -1  
$EndComp
Wire Wire Line
	2050 3100 2050 3200
Wire Wire Line
	2050 3200 2250 3200
Connection ~ 2250 3200
Wire Wire Line
	2250 3200 2250 3400
Wire Wire Line
	3150 3000 3150 3200
$Comp
L Device:R R15
U 1 1 5C68EFD4
P 3450 3550
F 0 "R15" H 3520 3596 50  0000 L CNN
F 1 "1k" H 3520 3505 50  0000 L CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" V 3380 3550 50  0001 C CNN
F 3 "~" H 3450 3550 50  0001 C CNN
	1    3450 3550
	1    0    0    -1  
$EndComp
Wire Wire Line
	3150 3200 3450 3200
Connection ~ 3150 3200
Wire Wire Line
	3150 3200 3150 3400
$Comp
L power:GND #PWR0141
U 1 1 5C68F32C
P 3450 3800
F 0 "#PWR0141" H 3450 3550 50  0001 C CNN
F 1 "GND" H 3455 3627 50  0000 C CNN
F 2 "" H 3450 3800 50  0001 C CNN
F 3 "" H 3450 3800 50  0001 C CNN
	1    3450 3800
	1    0    0    -1  
$EndComp
Wire Wire Line
	3450 3700 3450 3800
Wire Wire Line
	3450 3400 3450 3200
Connection ~ 3450 3200
Wire Wire Line
	3450 3200 3650 3200
$Comp
L PASC:MomentSwitch U10
U 1 1 5C68FABB
P 2700 4550
F 0 "U10" H 2700 4975 50  0000 C CNN
F 1 "MomentSwitch" H 2700 4884 50  0000 C CNN
F 2 "PASC:Momentswitch" H 2700 4550 50  0001 C CNN
F 3 "" H 2700 4550 50  0001 C CNN
	1    2700 4550
	1    0    0    -1  
$EndComp
Wire Wire Line
	3050 4350 3150 4350
Wire Wire Line
	3150 4750 3050 4750
Wire Wire Line
	2350 4350 2250 4350
Wire Wire Line
	2250 4750 2350 4750
Text HLabel 3650 4550 2    50   Input ~ 0
TIM_INC
Wire Wire Line
	2250 4350 2250 4550
$Comp
L power:+3.3V #PWR0142
U 1 1 5C68FAC8
P 2050 4450
F 0 "#PWR0142" H 2050 4300 50  0001 C CNN
F 1 "+3.3V" H 2065 4623 50  0000 C CNN
F 2 "" H 2050 4450 50  0001 C CNN
F 3 "" H 2050 4450 50  0001 C CNN
	1    2050 4450
	1    0    0    -1  
$EndComp
Wire Wire Line
	2050 4450 2050 4550
Wire Wire Line
	2050 4550 2250 4550
Connection ~ 2250 4550
Wire Wire Line
	2250 4550 2250 4750
Wire Wire Line
	3150 4350 3150 4550
$Comp
L Device:R R16
U 1 1 5C68FAD3
P 3450 4900
F 0 "R16" H 3520 4946 50  0000 L CNN
F 1 "1k" H 3520 4855 50  0000 L CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" V 3380 4900 50  0001 C CNN
F 3 "~" H 3450 4900 50  0001 C CNN
	1    3450 4900
	1    0    0    -1  
$EndComp
Wire Wire Line
	3150 4550 3450 4550
Connection ~ 3150 4550
Wire Wire Line
	3150 4550 3150 4750
$Comp
L power:GND #PWR0143
U 1 1 5C68FADD
P 3450 5150
F 0 "#PWR0143" H 3450 4900 50  0001 C CNN
F 1 "GND" H 3455 4977 50  0000 C CNN
F 2 "" H 3450 5150 50  0001 C CNN
F 3 "" H 3450 5150 50  0001 C CNN
	1    3450 5150
	1    0    0    -1  
$EndComp
Wire Wire Line
	3450 5050 3450 5150
Wire Wire Line
	3450 4750 3450 4550
Connection ~ 3450 4550
Wire Wire Line
	3450 4550 3650 4550
$Comp
L PASC:MomentSwitch U11
U 1 1 5C6902F1
P 2700 5750
F 0 "U11" H 2700 6175 50  0000 C CNN
F 1 "MomentSwitch" H 2700 6084 50  0000 C CNN
F 2 "PASC:Momentswitch" H 2700 5750 50  0001 C CNN
F 3 "" H 2700 5750 50  0001 C CNN
	1    2700 5750
	1    0    0    -1  
$EndComp
Wire Wire Line
	3050 5550 3150 5550
Wire Wire Line
	3150 5950 3050 5950
Wire Wire Line
	2350 5550 2250 5550
Wire Wire Line
	2250 5950 2350 5950
Text HLabel 3650 5750 2    50   Input ~ 0
TIM_RES
Wire Wire Line
	2250 5550 2250 5750
$Comp
L power:+3.3V #PWR0144
U 1 1 5C6902FE
P 2050 5650
F 0 "#PWR0144" H 2050 5500 50  0001 C CNN
F 1 "+3.3V" H 2065 5823 50  0000 C CNN
F 2 "" H 2050 5650 50  0001 C CNN
F 3 "" H 2050 5650 50  0001 C CNN
	1    2050 5650
	1    0    0    -1  
$EndComp
Wire Wire Line
	2050 5650 2050 5750
Wire Wire Line
	2050 5750 2250 5750
Connection ~ 2250 5750
Wire Wire Line
	2250 5750 2250 5950
Wire Wire Line
	3150 5550 3150 5750
$Comp
L Device:R R17
U 1 1 5C690309
P 3450 6100
F 0 "R17" H 3520 6146 50  0000 L CNN
F 1 "1k" H 3520 6055 50  0000 L CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" V 3380 6100 50  0001 C CNN
F 3 "~" H 3450 6100 50  0001 C CNN
	1    3450 6100
	1    0    0    -1  
$EndComp
Wire Wire Line
	3150 5750 3450 5750
Connection ~ 3150 5750
Wire Wire Line
	3150 5750 3150 5950
$Comp
L power:GND #PWR0145
U 1 1 5C690313
P 3450 6350
F 0 "#PWR0145" H 3450 6100 50  0001 C CNN
F 1 "GND" H 3455 6177 50  0000 C CNN
F 2 "" H 3450 6350 50  0001 C CNN
F 3 "" H 3450 6350 50  0001 C CNN
	1    3450 6350
	1    0    0    -1  
$EndComp
Wire Wire Line
	3450 6250 3450 6350
Wire Wire Line
	3450 5950 3450 5750
Connection ~ 3450 5750
Wire Wire Line
	3450 5750 3650 5750
$EndSCHEMATC
