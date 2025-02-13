EESchema Schematic File Version 4
LIBS:PCB_Simon_mount_rev2-cache
EELAYER 26 0
EELAYER END
$Descr User 5512 5512
encoding utf-8
Sheet 2 7
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
L PCB_Simon_mount_rev2-rescue:+3.3V-power #PWR0120
U 1 1 5C5AB0A3
P 1300 1850
F 0 "#PWR0120" H 1300 1700 50  0001 C CNN
F 1 "+3.3V" H 1315 2023 50  0000 C CNN
F 2 "" H 1300 1850 50  0001 C CNN
F 3 "" H 1300 1850 50  0001 C CNN
	1    1300 1850
	1    0    0    -1  
$EndComp
Wire Wire Line
	1300 1850 1300 2050
Wire Wire Line
	1800 2050 1300 2050
$Comp
L PCB_Simon_mount_rev2-rescue:GND-power #PWR0121
U 1 1 5C59F3FC
P 2300 2850
F 0 "#PWR0121" H 2300 2600 50  0001 C CNN
F 1 "GND" H 2305 2677 50  0000 C CNN
F 2 "" H 2300 2850 50  0001 C CNN
F 3 "" H 2300 2850 50  0001 C CNN
	1    2300 2850
	1    0    0    -1  
$EndComp
Wire Wire Line
	2300 2750 2300 2850
$Comp
L PCB_Simon_mount_rev2-rescue:GND-power #PWR0122
U 1 1 5C59F76F
P 4100 2500
F 0 "#PWR0122" H 4100 2250 50  0001 C CNN
F 1 "GND" H 4105 2327 50  0000 C CNN
F 2 "" H 4100 2500 50  0001 C CNN
F 3 "" H 4100 2500 50  0001 C CNN
	1    4100 2500
	1    0    0    -1  
$EndComp
NoConn ~ 1800 2200
$Comp
L PCB_Simon_mount_rev2-rescue:C-Device C16
U 1 1 5C5A61DB
P 1300 2350
F 0 "C16" H 1415 2396 50  0000 L CNN
F 1 "100n" H 1415 2305 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 1338 2200 50  0001 C CNN
F 3 "~" H 1300 2350 50  0001 C CNN
	1    1300 2350
	1    0    0    -1  
$EndComp
$Comp
L PCB_Simon_mount_rev2-rescue:GND-power #PWR0123
U 1 1 5C5A62A1
P 1300 2650
F 0 "#PWR0123" H 1300 2400 50  0001 C CNN
F 1 "GND" H 1305 2477 50  0000 C CNN
F 2 "" H 1300 2650 50  0001 C CNN
F 3 "" H 1300 2650 50  0001 C CNN
	1    1300 2650
	1    0    0    -1  
$EndComp
Wire Wire Line
	1300 2050 1300 2200
Connection ~ 1300 2050
Wire Wire Line
	1300 2500 1300 2650
Text Notes 4150 2450 0    50   ~ 0
I2C Address: 0x18
Text Label 3100 2250 0    50   ~ 0
SDO
Wire Wire Line
	4100 2300 4000 2300
Wire Wire Line
	4100 2300 4100 2500
Text Label 4000 2300 2    50   ~ 0
SDO
Text Label 3100 2350 0    50   ~ 0
CS_XL
$Comp
L PCB_Simon_mount_rev2-rescue:+3V3-power #PWR0124
U 1 1 5C5D1727
P 4100 1900
F 0 "#PWR0124" H 4100 1750 50  0001 C CNN
F 1 "+3V3" H 4115 2073 50  0000 C CNN
F 2 "" H 4100 1900 50  0001 C CNN
F 3 "" H 4100 1900 50  0001 C CNN
	1    4100 1900
	1    0    0    -1  
$EndComp
Wire Wire Line
	4100 1900 4100 2100
Wire Wire Line
	4100 2100 4000 2100
Text Label 4000 2100 2    50   ~ 0
CS_XL
Text Notes 4150 2100 0    50   ~ 0
Use I2C.
Text HLabel 3100 2000 2    50   Input ~ 0
I2C1_SCL
Text HLabel 3100 2150 2    50   Input ~ 0
I2C1_SDA
Text HLabel 3100 2450 2    50   Input ~ 0
CS_INT
$Comp
L PASC:lis3dh_board U3
U 1 1 5C5E039C
P 2450 2250
F 0 "U3" H 2450 2725 50  0000 C CNN
F 1 "lis3dh_board" H 2450 2634 50  0000 C CNN
F 2 "PASC:LIS3DH" H 2450 2250 50  0001 C CNN
F 3 "" H 2450 2250 50  0001 C CNN
	1    2450 2250
	1    0    0    -1  
$EndComp
$EndSCHEMATC
