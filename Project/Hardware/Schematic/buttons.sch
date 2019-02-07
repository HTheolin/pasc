EESchema Schematic File Version 4
LIBS:lab-cache
EELAYER 26 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 7 7
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
L PASC:MomentSwitch U8
U 1 1 5C5ECE12
P 3800 2550
F 0 "U8" H 3825 2975 50  0000 C CNN
F 1 "MomentSwitch" H 3825 2884 50  0000 C CNN
F 2 "PASC:Momentswitch" H 3800 2550 50  0001 C CNN
F 3 "" H 3800 2550 50  0001 C CNN
	1    3800 2550
	1    0    0    -1  
$EndComp
$Comp
L PASC:MomentSwitch U7
U 1 1 5C5ECEC3
P 3800 1700
F 0 "U7" H 3825 2125 50  0000 C CNN
F 1 "MomentSwitch" H 3825 2034 50  0000 C CNN
F 2 "PASC:Momentswitch" H 3800 1700 50  0001 C CNN
F 3 "" H 3800 1700 50  0001 C CNN
	1    3800 1700
	1    0    0    -1  
$EndComp
Wire Wire Line
	3450 1500 3350 1500
Wire Wire Line
	3350 1500 3350 1700
Wire Wire Line
	3350 1900 3450 1900
Wire Wire Line
	3450 2350 3350 2350
Wire Wire Line
	3350 2350 3350 2750
Wire Wire Line
	3350 2750 3450 2750
Wire Wire Line
	4150 1500 4250 1500
Wire Wire Line
	4250 1500 4250 1700
Wire Wire Line
	4250 1900 4150 1900
Wire Wire Line
	4150 2350 4250 2350
Wire Wire Line
	4250 2350 4250 2750
Wire Wire Line
	4250 2750 4150 2750
Wire Wire Line
	4250 1700 4350 1700
Connection ~ 4250 1700
Wire Wire Line
	4250 1700 4250 1900
Text Label 4350 1700 0    50   ~ 0
NRST
Wire Wire Line
	3350 1700 3250 1700
Connection ~ 3350 1700
Wire Wire Line
	3350 1700 3350 1900
$Comp
L power:GND #PWR?
U 1 1 5C5ED863
P 3250 1700
F 0 "#PWR?" H 3250 1450 50  0001 C CNN
F 1 "GND" H 3255 1527 50  0000 C CNN
F 2 "" H 3250 1700 50  0001 C CNN
F 3 "" H 3250 1700 50  0001 C CNN
	1    3250 1700
	1    0    0    -1  
$EndComp
$EndSCHEMATC
