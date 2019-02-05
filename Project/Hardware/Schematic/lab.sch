EESchema Schematic File Version 4
LIBS:lab-cache
EELAYER 26 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 1 7
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
L MCU_ST_STM32F4:STM32F411RETx U2
U 1 1 5C49BAEC
P 6800 4200
F 0 "U2" H 6800 4200 50  0000 C CNN
F 1 "STM32F411RETx" H 6800 4100 50  0000 C CNN
F 2 "Package_QFP:LQFP-64_10x10mm_P0.5mm" H 6200 2500 50  0001 R CNN
F 3 "http://www.st.com/st-web-ui/static/active/en/resource/technical/document/datasheet/DM00115249.pdf" H 6800 4200 50  0001 C CNN
	1    6800 4200
	1    0    0    -1  
$EndComp
$Comp
L Device:C_Small C12
U 1 1 5C49BC7E
P 6400 1300
F 0 "C12" H 6500 1300 50  0000 L CNN
F 1 "100n" H 6400 1400 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 6400 1300 50  0001 C CNN
F 3 "~" H 6400 1300 50  0001 C CNN
	1    6400 1300
	1    0    0    -1  
$EndComp
$Comp
L Device:C_Small C13
U 1 1 5C49BCF8
P 6700 1300
F 0 "C13" H 6800 1300 50  0000 L CNN
F 1 "100n" H 6700 1400 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 6700 1300 50  0001 C CNN
F 3 "~" H 6700 1300 50  0001 C CNN
	1    6700 1300
	1    0    0    -1  
$EndComp
$Comp
L Device:C_Small C14
U 1 1 5C49BD16
P 7000 1300
F 0 "C14" H 7100 1300 50  0000 L CNN
F 1 "100n" H 7000 1400 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 7000 1300 50  0001 C CNN
F 3 "~" H 7000 1300 50  0001 C CNN
	1    7000 1300
	1    0    0    -1  
$EndComp
$Comp
L Device:C_Small C15
U 1 1 5C49BD30
P 7300 1300
F 0 "C15" H 7400 1300 50  0000 L CNN
F 1 "100n" H 7300 1400 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 7300 1300 50  0001 C CNN
F 3 "~" H 7300 1300 50  0001 C CNN
	1    7300 1300
	1    0    0    -1  
$EndComp
$Comp
L Device:C_Small C11
U 1 1 5C49BD4E
P 6100 1300
F 0 "C11" H 6200 1300 50  0000 L CNN
F 1 "100n" H 6100 1400 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 6100 1300 50  0001 C CNN
F 3 "~" H 6100 1300 50  0001 C CNN
	1    6100 1300
	1    0    0    -1  
$EndComp
$Comp
L power:+3V3 #PWR0101
U 1 1 5C49BD99
P 6100 1000
F 0 "#PWR0101" H 6100 850 50  0001 C CNN
F 1 "+3V3" H 6115 1173 50  0000 C CNN
F 2 "" H 6100 1000 50  0001 C CNN
F 3 "" H 6100 1000 50  0001 C CNN
	1    6100 1000
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR0102
U 1 1 5C49BDDC
P 6100 1600
F 0 "#PWR0102" H 6100 1350 50  0001 C CNN
F 1 "GND" H 6105 1427 50  0000 C CNN
F 2 "" H 6100 1600 50  0001 C CNN
F 3 "" H 6100 1600 50  0001 C CNN
	1    6100 1600
	1    0    0    -1  
$EndComp
Wire Wire Line
	6100 1000 6100 1100
Wire Wire Line
	6100 1100 6400 1100
Wire Wire Line
	6400 1100 6400 1200
Connection ~ 6100 1100
Wire Wire Line
	6100 1100 6100 1200
Wire Wire Line
	6400 1100 6700 1100
Wire Wire Line
	6700 1100 6700 1200
Connection ~ 6400 1100
Wire Wire Line
	6700 1100 7000 1100
Wire Wire Line
	7000 1100 7000 1200
Connection ~ 6700 1100
Wire Wire Line
	7000 1100 7300 1100
Wire Wire Line
	7300 1100 7300 1200
Connection ~ 7000 1100
Wire Wire Line
	7300 1400 7300 1500
Wire Wire Line
	7300 1500 7000 1500
Wire Wire Line
	6100 1500 6100 1600
Wire Wire Line
	6100 1400 6100 1500
Connection ~ 6100 1500
Wire Wire Line
	6400 1400 6400 1500
Connection ~ 6400 1500
Wire Wire Line
	6400 1500 6100 1500
Wire Wire Line
	6700 1400 6700 1500
Connection ~ 6700 1500
Wire Wire Line
	6700 1500 6400 1500
Wire Wire Line
	7000 1400 7000 1500
Connection ~ 7000 1500
Wire Wire Line
	7000 1500 6700 1500
$Comp
L power:+3V3 #PWR0103
U 1 1 5C49C6AF
P 6800 2300
F 0 "#PWR0103" H 6800 2150 50  0001 C CNN
F 1 "+3V3" H 6815 2473 50  0000 C CNN
F 2 "" H 6800 2300 50  0001 C CNN
F 3 "" H 6800 2300 50  0001 C CNN
	1    6800 2300
	1    0    0    -1  
$EndComp
Wire Wire Line
	6800 2400 6800 2300
Wire Wire Line
	6700 2500 6700 2400
Wire Wire Line
	6700 2400 6800 2400
Wire Wire Line
	6800 2500 6800 2400
Connection ~ 6800 2400
Wire Wire Line
	6900 2500 6900 2400
Wire Wire Line
	6900 2400 6800 2400
Wire Wire Line
	7000 2500 7000 2400
Wire Wire Line
	7000 2400 6900 2400
Connection ~ 6900 2400
Wire Wire Line
	7100 2500 7100 2400
Wire Wire Line
	7100 2400 7000 2400
Connection ~ 7000 2400
$Comp
L power:GND #PWR0104
U 1 1 5C49DB13
P 6800 6200
F 0 "#PWR0104" H 6800 5950 50  0001 C CNN
F 1 "GND" H 6805 6027 50  0000 C CNN
F 2 "" H 6800 6200 50  0001 C CNN
F 3 "" H 6800 6200 50  0001 C CNN
	1    6800 6200
	1    0    0    -1  
$EndComp
Wire Wire Line
	6600 6000 6600 6100
Wire Wire Line
	6600 6100 6700 6100
Wire Wire Line
	6700 6000 6700 6100
Connection ~ 6700 6100
Wire Wire Line
	6700 6100 6800 6100
Wire Wire Line
	6800 6000 6800 6100
Connection ~ 6800 6100
Wire Wire Line
	6900 6000 6900 6100
Wire Wire Line
	6900 6100 6800 6100
Wire Wire Line
	7000 6000 7000 6100
Wire Wire Line
	7000 6100 6900 6100
Connection ~ 6900 6100
Wire Wire Line
	6800 6100 6800 6200
$Comp
L Device:C_Small C9
U 1 1 5C49FE02
P 5800 1300
F 0 "C9" H 5900 1300 50  0000 L CNN
F 1 "4.7u" H 5800 1400 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 5800 1300 50  0001 C CNN
F 3 "~" H 5800 1300 50  0001 C CNN
	1    5800 1300
	1    0    0    -1  
$EndComp
Wire Wire Line
	6100 1100 5800 1100
Wire Wire Line
	5800 1100 5800 1200
Wire Wire Line
	5800 1400 5800 1500
Wire Wire Line
	5800 1500 6100 1500
$Comp
L Device:C_Small C10
U 1 1 5C4A8076
P 5900 3100
F 0 "C10" V 6000 2900 50  0000 L CNN
F 1 "4.7u" V 6000 3100 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 5900 3100 50  0001 C CNN
F 3 "~" H 5900 3100 50  0001 C CNN
	1    5900 3100
	0    1    1    0   
$EndComp
$Comp
L power:GND #PWR0105
U 1 1 5C4A893E
P 5600 3200
F 0 "#PWR0105" H 5600 2950 50  0001 C CNN
F 1 "GND" H 5605 3027 50  0000 C CNN
F 2 "" H 5600 3200 50  0001 C CNN
F 3 "" H 5600 3200 50  0001 C CNN
	1    5600 3200
	1    0    0    -1  
$EndComp
Connection ~ 7300 1100
Connection ~ 7300 1500
Wire Wire Line
	6700 2400 6600 2400
Wire Wire Line
	6600 2400 6600 2500
Connection ~ 6700 2400
$Comp
L Device:R_Small R5
U 1 1 5C4AD49E
P 6000 2500
F 0 "R5" H 6059 2546 50  0000 L CNN
F 1 "10k" H 6059 2455 50  0000 L CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 6000 2500 50  0001 C CNN
F 3 "~" H 6000 2500 50  0001 C CNN
	1    6000 2500
	1    0    0    -1  
$EndComp
$Comp
L power:+3V3 #PWR0106
U 1 1 5C4AD51D
P 6000 2300
F 0 "#PWR0106" H 6000 2150 50  0001 C CNN
F 1 "+3V3" H 6015 2473 50  0000 C CNN
F 2 "" H 6000 2300 50  0001 C CNN
F 3 "" H 6000 2300 50  0001 C CNN
	1    6000 2300
	1    0    0    -1  
$EndComp
Wire Wire Line
	6000 2600 6000 2700
Wire Wire Line
	6000 2700 6100 2700
Wire Wire Line
	6000 2300 6000 2400
$Comp
L Device:R_Small R4
U 1 1 5C4AF9C4
P 5900 2900
F 0 "R4" V 6000 2700 50  0000 L CNN
F 1 "10k" V 6000 2900 50  0000 L CNN
F 2 "Resistor_SMD:R_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 5900 2900 50  0001 C CNN
F 3 "~" H 5900 2900 50  0001 C CNN
	1    5900 2900
	0    1    1    0   
$EndComp
$Comp
L Device:Crystal Y1
U 1 1 5C4B6BEC
P 5600 4000
F 0 "Y1" V 5700 3900 50  0000 R CNN
F 1 "8MHz" V 5500 3900 50  0000 R CNN
F 2 "Crystal:Crystal_SMD_HC49-SD_HandSoldering" H 5600 4000 50  0001 C CNN
F 3 "~" H 5600 4000 50  0001 C CNN
	1    5600 4000
	0    -1   -1   0   
$EndComp
Wire Wire Line
	5600 3850 5600 3800
Wire Wire Line
	5600 4150 5600 4200
Wire Wire Line
	5600 3800 6100 3800
Wire Wire Line
	6100 3900 6000 3900
Wire Wire Line
	6000 3900 6000 4200
Wire Wire Line
	6000 4200 5600 4200
$Comp
L Device:C_Small C7
U 1 1 5C4C9EDF
P 5400 3800
F 0 "C7" V 5300 3700 50  0000 C CNN
F 1 "18p" V 5300 3900 50  0000 C CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 5400 3800 50  0001 C CNN
F 3 "~" H 5400 3800 50  0001 C CNN
	1    5400 3800
	0    1    1    0   
$EndComp
$Comp
L Device:C_Small C8
U 1 1 5C4C9F37
P 5400 4200
F 0 "C8" V 5500 4100 50  0000 C CNN
F 1 "18p" V 5500 4300 50  0000 C CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 5400 4200 50  0001 C CNN
F 3 "~" H 5400 4200 50  0001 C CNN
	1    5400 4200
	0    1    1    0   
$EndComp
Wire Wire Line
	5600 3800 5500 3800
Connection ~ 5600 3800
$Comp
L power:GND #PWR0107
U 1 1 5C4CB08E
P 5200 4300
F 0 "#PWR0107" H 5200 4050 50  0001 C CNN
F 1 "GND" H 5100 4200 50  0000 C CNN
F 2 "" H 5200 4300 50  0001 C CNN
F 3 "" H 5200 4300 50  0001 C CNN
	1    5200 4300
	1    0    0    -1  
$EndComp
Wire Wire Line
	5300 3800 5200 3800
Wire Wire Line
	5200 3800 5200 4200
Wire Wire Line
	5300 4200 5200 4200
Connection ~ 5200 4200
Wire Wire Line
	5200 4200 5200 4300
Wire Wire Line
	5600 4200 5500 4200
Connection ~ 5600 4200
$Comp
L Connector:Conn_01x06_Male J2
U 1 1 5C4D1603
P 2800 4400
F 0 "J2" H 2400 4500 50  0000 C CNN
F 1 "6P Male" H 2500 4400 50  0000 C CNN
F 2 "Connector_PinHeader_2.54mm:PinHeader_1x06_P2.54mm_Vertical" H 2800 4400 50  0001 C CNN
F 3 "~" H 2800 4400 50  0001 C CNN
	1    2800 4400
	1    0    0    -1  
$EndComp
Wire Wire Line
	3000 4300 3200 4300
Wire Wire Line
	3000 4500 3200 4500
Wire Wire Line
	3000 4600 3200 4600
Wire Wire Line
	3000 4700 3200 4700
Text Label 3200 4500 0    50   ~ 0
SWDIO
Text Label 3200 4600 0    50   ~ 0
NRST
Text Label 3200 4700 0    50   ~ 0
SWO
$Comp
L power:GND #PWR0108
U 1 1 5C4DD1C0
P 4100 4800
F 0 "#PWR0108" H 4100 4550 50  0001 C CNN
F 1 "GND" H 4100 4600 50  0000 C CNN
F 2 "" H 4100 4800 50  0001 C CNN
F 3 "" H 4100 4800 50  0001 C CNN
	1    4100 4800
	1    0    0    -1  
$EndComp
Wire Wire Line
	4100 4500 4100 4600
$Comp
L power:+3V3 #PWR0109
U 1 1 5C4E23FA
P 4100 4100
F 0 "#PWR0109" H 4100 3950 50  0001 C CNN
F 1 "+3V3" H 4115 4273 50  0000 C CNN
F 2 "" H 4100 4100 50  0001 C CNN
F 3 "" H 4100 4100 50  0001 C CNN
	1    4100 4100
	1    0    0    -1  
$EndComp
Wire Wire Line
	4100 4100 4100 4200
$Comp
L Device:C_Small C2
U 1 1 5C4E5BA4
P 4100 4400
F 0 "C2" H 4192 4446 50  0000 L CNN
F 1 "100n" H 4192 4355 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 4100 4400 50  0001 C CNN
F 3 "~" H 4100 4400 50  0001 C CNN
	1    4100 4400
	1    0    0    -1  
$EndComp
Connection ~ 4100 4200
Wire Wire Line
	4100 4200 4100 4300
Connection ~ 4100 4600
Wire Wire Line
	4100 4600 4100 4800
Wire Wire Line
	6000 2700 5600 2700
Connection ~ 6000 2700
Text Label 5600 2700 0    50   ~ 0
NRST
Wire Wire Line
	7500 4100 7900 4100
Text Label 7900 4100 0    50   ~ 0
SWCLK
Wire Wire Line
	7500 4000 7900 4000
Text Label 7900 4000 0    50   ~ 0
SWDIO
Wire Wire Line
	7500 4700 7900 4700
Text Label 7900 4700 0    50   ~ 0
SWO
$Comp
L Device:C_Small C1
U 1 1 5C500035
P 2400 1300
F 0 "C1" H 2492 1346 50  0000 L CNN
F 1 "10u" H 2492 1255 50  0000 L CNN
F 2 "Capacitor_THT:CP_Radial_D5.0mm_P2.00mm" H 2400 1300 50  0001 C CNN
F 3 "~" H 2400 1300 50  0001 C CNN
	1    2400 1300
	1    0    0    -1  
$EndComp
Wire Wire Line
	2400 1100 2400 1200
Connection ~ 2400 1100
Wire Wire Line
	2400 1100 2700 1100
Wire Wire Line
	2400 1600 2700 1600
$Comp
L Device:C_Small C5
U 1 1 5C514E5C
P 3900 1300
F 0 "C5" H 3992 1346 50  0000 L CNN
F 1 "10u" H 3992 1255 50  0000 L CNN
F 2 "Capacitor_THT:CP_Radial_D5.0mm_P2.00mm" H 3900 1300 50  0001 C CNN
F 3 "~" H 3900 1300 50  0001 C CNN
	1    3900 1300
	1    0    0    -1  
$EndComp
Wire Wire Line
	3900 1100 3900 1200
Wire Wire Line
	3900 1400 3900 1600
$Comp
L Device:C_Small C6
U 1 1 5C51CB4F
P 4200 1300
F 0 "C6" H 4292 1346 50  0000 L CNN
F 1 "100n" H 4292 1255 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 4200 1300 50  0001 C CNN
F 3 "~" H 4200 1300 50  0001 C CNN
	1    4200 1300
	1    0    0    -1  
$EndComp
$Comp
L Device:C_Small C3
U 1 1 5C51F65E
P 2700 1300
F 0 "C3" H 2792 1346 50  0000 L CNN
F 1 "100n" H 2792 1255 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 2700 1300 50  0001 C CNN
F 3 "~" H 2700 1300 50  0001 C CNN
	1    2700 1300
	1    0    0    -1  
$EndComp
Wire Wire Line
	2700 1200 2700 1100
Wire Wire Line
	2700 1400 2700 1600
Wire Wire Line
	3900 1100 4200 1100
Wire Wire Line
	4200 1100 4200 1200
Wire Wire Line
	4200 1400 4200 1600
Wire Wire Line
	4200 1600 3900 1600
Wire Wire Line
	2400 900  2400 1100
Text Label 3200 4300 0    50   ~ 0
SWCLK
Wire Wire Line
	3000 4200 4100 4200
Wire Wire Line
	3600 4400 3600 4600
Wire Wire Line
	3000 4400 3600 4400
Wire Wire Line
	3600 4600 4100 4600
$Comp
L power:GND #PWR0114
U 1 1 5C56D6CB
P 3300 1700
F 0 "#PWR0114" H 3300 1450 50  0001 C CNN
F 1 "GND" H 3305 1527 50  0000 C CNN
F 2 "" H 3300 1700 50  0001 C CNN
F 3 "" H 3300 1700 50  0001 C CNN
	1    3300 1700
	1    0    0    -1  
$EndComp
Wire Wire Line
	4200 1100 4200 900 
Connection ~ 4200 1100
NoConn ~ 7500 2800
NoConn ~ 7500 2900
NoConn ~ 7500 3000
NoConn ~ 7500 3100
NoConn ~ 7500 3300
NoConn ~ 7500 3400
NoConn ~ 7500 3800
NoConn ~ 7500 3900
NoConn ~ 7500 4200
NoConn ~ 7500 4400
NoConn ~ 7500 4600
NoConn ~ 7500 4500
NoConn ~ 7500 4800
NoConn ~ 7500 4900
NoConn ~ 7500 5000
NoConn ~ 7500 5500
NoConn ~ 7500 5600
NoConn ~ 7500 5700
NoConn ~ 7500 5800
NoConn ~ 6100 5800
NoConn ~ 6100 5700
NoConn ~ 6100 5600
NoConn ~ 6100 5500
NoConn ~ 6100 5400
NoConn ~ 6100 5300
NoConn ~ 6100 5200
NoConn ~ 6100 5100
NoConn ~ 6100 5000
NoConn ~ 6100 4900
NoConn ~ 6100 4800
NoConn ~ 6100 4700
NoConn ~ 6100 4600
NoConn ~ 6100 4500
NoConn ~ 6100 4400
NoConn ~ 6100 4300
NoConn ~ 6100 4100
$Comp
L Device:C_Small C17
U 1 1 5C6BB510
P 7600 1300
F 0 "C17" H 7700 1300 50  0000 L CNN
F 1 "1u" H 7600 1400 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 7600 1300 50  0001 C CNN
F 3 "~" H 7600 1300 50  0001 C CNN
	1    7600 1300
	1    0    0    -1  
$EndComp
Wire Wire Line
	7600 1100 7600 1200
Wire Wire Line
	7600 1400 7600 1500
Wire Wire Line
	7300 1100 7600 1100
Wire Wire Line
	7300 1500 7600 1500
Wire Wire Line
	5600 2900 5600 3100
Wire Wire Line
	5800 2900 5600 2900
Wire Wire Line
	6000 2900 6100 2900
Wire Wire Line
	5800 3100 5600 3100
Connection ~ 5600 3100
Wire Wire Line
	5600 3100 5600 3200
Wire Wire Line
	6000 3100 6100 3100
Text Label 5600 3800 0    50   ~ 0
XTAL+
Text Label 5600 4200 0    50   ~ 0
XTAL-
$Sheet
S 9650 1250 1300 750 
U 5C556F23
F0 "accelerometer" 50
F1 "accelerometer.sch" 50
$EndSheet
$Sheet
S 9700 2600 1200 950 
U 5C564E6B
F0 "Pulse sensor" 50
F1 "pulsesensor.sch" 50
$EndSheet
$Sheet
S 9700 3900 1100 900 
U 5C564F10
F0 "Temp Sensor" 50
F1 "tempsensor.sch" 50
$EndSheet
$Sheet
S 9700 5350 950  850 
U 5C564F2E
F0 "Display" 50
F1 "display.sch" 50
$EndSheet
$Sheet
S 1100 6650 1200 650 
U 5C564F82
F0 "Piezo" 50
F1 "piezo.sch" 50
$EndSheet
$Sheet
S 2900 6700 1050 650 
U 5C564FA0
F0 "Buttons" 50
F1 "buttons.sch" 50
$EndSheet
$Comp
L Device:C_Small C?
U 1 1 5C5CED5E
P 2400 2900
F 0 "C?" H 2492 2946 50  0000 L CNN
F 1 "10u" H 2492 2855 50  0000 L CNN
F 2 "Capacitor_THT:CP_Radial_D5.0mm_P2.00mm" H 2400 2900 50  0001 C CNN
F 3 "~" H 2400 2900 50  0001 C CNN
	1    2400 2900
	1    0    0    -1  
$EndComp
Wire Wire Line
	2400 2700 2400 2800
Connection ~ 2400 2700
Wire Wire Line
	2400 2700 2700 2700
$Comp
L Device:C_Small C?
U 1 1 5C5CED70
P 3900 2900
F 0 "C?" H 3992 2946 50  0000 L CNN
F 1 "10u" H 3992 2855 50  0000 L CNN
F 2 "Capacitor_THT:CP_Radial_D5.0mm_P2.00mm" H 3900 2900 50  0001 C CNN
F 3 "~" H 3900 2900 50  0001 C CNN
	1    3900 2900
	1    0    0    -1  
$EndComp
Wire Wire Line
	3900 2700 3900 2800
Wire Wire Line
	3900 3000 3900 3200
$Comp
L Device:C_Small C?
U 1 1 5C5CED79
P 4200 2900
F 0 "C?" H 4292 2946 50  0000 L CNN
F 1 "100n" H 4292 2855 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 4200 2900 50  0001 C CNN
F 3 "~" H 4200 2900 50  0001 C CNN
	1    4200 2900
	1    0    0    -1  
$EndComp
$Comp
L Device:C_Small C?
U 1 1 5C5CED80
P 2700 2900
F 0 "C?" H 2792 2946 50  0000 L CNN
F 1 "100n" H 2792 2855 50  0000 L CNN
F 2 "Capacitor_SMD:C_0805_2012Metric_Pad1.15x1.40mm_HandSolder" H 2700 2900 50  0001 C CNN
F 3 "~" H 2700 2900 50  0001 C CNN
	1    2700 2900
	1    0    0    -1  
$EndComp
Wire Wire Line
	2700 2800 2700 2700
Wire Wire Line
	2700 3000 2700 3200
Wire Wire Line
	3900 2700 4200 2700
Wire Wire Line
	4200 2700 4200 2800
Wire Wire Line
	4200 3200 3900 3200
Wire Wire Line
	2400 2500 2400 2700
$Comp
L power:+3V3 #PWR?
U 1 1 5C5CED9E
P 4200 2500
F 0 "#PWR?" H 4200 2350 50  0001 C CNN
F 1 "+3V3" H 4215 2673 50  0000 C CNN
F 2 "" H 4200 2500 50  0001 C CNN
F 3 "" H 4200 2500 50  0001 C CNN
	1    4200 2500
	1    0    0    -1  
$EndComp
Wire Wire Line
	4200 2700 4200 2500
Connection ~ 4200 2700
$Comp
L Regulator_Linear:NCP1117-5.0_SOT223 U?
U 1 1 5C5D36EE
P 3300 1100
F 0 "U?" H 3300 1342 50  0000 C CNN
F 1 "NCP1117-5.0_SOT223" H 3300 1251 50  0000 C CNN
F 2 "Package_TO_SOT_SMD:SOT-223-3_TabPin2" H 3300 1300 50  0001 C CNN
F 3 "http://www.onsemi.com/pub_link/Collateral/NCP1117-D.PDF" H 3400 850 50  0001 C CNN
	1    3300 1100
	1    0    0    -1  
$EndComp
$Comp
L Regulator_Linear:NCP1117-3.3_SOT223 U?
U 1 1 5C5D379C
P 3300 2700
F 0 "U?" H 3300 2942 50  0000 C CNN
F 1 "NCP1117-3.3_SOT223" H 3300 2851 50  0000 C CNN
F 2 "Package_TO_SOT_SMD:SOT-223-3_TabPin2" H 3300 2900 50  0001 C CNN
F 3 "http://www.onsemi.com/pub_link/Collateral/NCP1117-D.PDF" H 3400 2450 50  0001 C CNN
	1    3300 2700
	1    0    0    -1  
$EndComp
$Comp
L power:+BATT #PWR?
U 1 1 5C5D3900
P 2400 900
F 0 "#PWR?" H 2400 750 50  0001 C CNN
F 1 "+BATT" H 2415 1073 50  0000 C CNN
F 2 "" H 2400 900 50  0001 C CNN
F 3 "" H 2400 900 50  0001 C CNN
	1    2400 900 
	1    0    0    -1  
$EndComp
Wire Wire Line
	2700 1100 3000 1100
Connection ~ 2700 1100
Wire Wire Line
	3600 1100 3900 1100
Connection ~ 3900 1100
$Comp
L power:+5V #PWR?
U 1 1 5C5E5791
P 4200 900
F 0 "#PWR?" H 4200 750 50  0001 C CNN
F 1 "+5V" H 4215 1073 50  0000 C CNN
F 2 "" H 4200 900 50  0001 C CNN
F 3 "" H 4200 900 50  0001 C CNN
	1    4200 900 
	1    0    0    -1  
$EndComp
Wire Wire Line
	3900 1600 3300 1600
Wire Wire Line
	3300 1600 3300 1700
Connection ~ 3900 1600
Wire Wire Line
	3300 1600 2700 1600
Connection ~ 3300 1600
Connection ~ 2700 1600
Wire Wire Line
	2400 1600 2400 1400
Wire Wire Line
	3300 1400 3300 1600
Wire Wire Line
	2700 2700 3000 2700
Connection ~ 2700 2700
Wire Wire Line
	3600 2700 3900 2700
Connection ~ 3900 2700
Wire Wire Line
	3300 3000 3300 3200
Wire Wire Line
	3300 3200 2700 3200
Wire Wire Line
	3300 3200 3900 3200
Connection ~ 3300 3200
Connection ~ 3900 3200
$Comp
L power:GND #PWR?
U 1 1 5C61536D
P 3300 3300
F 0 "#PWR?" H 3300 3050 50  0001 C CNN
F 1 "GND" H 3305 3127 50  0000 C CNN
F 2 "" H 3300 3300 50  0001 C CNN
F 3 "" H 3300 3300 50  0001 C CNN
	1    3300 3300
	1    0    0    -1  
$EndComp
Wire Wire Line
	3300 3300 3300 3200
Wire Wire Line
	2400 3000 2400 3200
Wire Wire Line
	2400 3200 2700 3200
Connection ~ 2700 3200
Wire Wire Line
	4200 3200 4200 3000
$Comp
L power:+BATT #PWR?
U 1 1 5C62486C
P 2400 2500
F 0 "#PWR?" H 2400 2350 50  0001 C CNN
F 1 "+BATT" H 2415 2673 50  0000 C CNN
F 2 "" H 2400 2500 50  0001 C CNN
F 3 "" H 2400 2500 50  0001 C CNN
	1    2400 2500
	1    0    0    -1  
$EndComp
$Comp
L Device:Battery BT?
U 1 1 5C624D2E
P 1200 1300
F 0 "BT?" H 1308 1346 50  0000 L CNN
F 1 "Battery" H 1308 1255 50  0000 L CNN
F 2 "" V 1200 1360 50  0001 C CNN
F 3 "~" V 1200 1360 50  0001 C CNN
	1    1200 1300
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR?
U 1 1 5C6250C7
P 1200 1700
F 0 "#PWR?" H 1200 1450 50  0001 C CNN
F 1 "GND" H 1205 1527 50  0000 C CNN
F 2 "" H 1200 1700 50  0001 C CNN
F 3 "" H 1200 1700 50  0001 C CNN
	1    1200 1700
	1    0    0    -1  
$EndComp
$Comp
L power:+BATT #PWR?
U 1 1 5C6251FB
P 1200 900
F 0 "#PWR?" H 1200 750 50  0001 C CNN
F 1 "+BATT" H 1215 1073 50  0000 C CNN
F 2 "" H 1200 900 50  0001 C CNN
F 3 "" H 1200 900 50  0001 C CNN
	1    1200 900 
	1    0    0    -1  
$EndComp
Wire Wire Line
	1200 900  1200 1100
Wire Wire Line
	1200 1500 1200 1700
Wire Wire Line
	7500 3500 7900 3500
Text Label 7900 3500 0    50   ~ 0
USART1_CK
Wire Wire Line
	7500 3600 7900 3600
Text Label 7900 3600 0    50   ~ 0
USART1_TX
Wire Wire Line
	7500 3700 7900 3700
Text Label 7900 3700 0    50   ~ 0
USART1_RX
Wire Wire Line
	7500 5100 7900 5100
Text Label 7900 5100 0    50   ~ 0
I2C1_SDA
Wire Wire Line
	7500 5200 7900 5200
Text Label 7900 5200 0    50   ~ 0
I2C1_SCL
Wire Wire Line
	7500 5300 7900 5300
Text Label 7900 5300 0    50   ~ 0
CS_XL
Wire Wire Line
	7500 5400 7900 5400
Text Label 7900 5400 0    50   ~ 0
CS_INT
Wire Wire Line
	7500 2700 7900 2700
Text Label 7900 2700 0    50   ~ 0
TEMPOUT
Wire Wire Line
	7500 3200 7900 3200
Text Label 7900 3200 0    50   ~ 0
TIM2_CH1
$EndSCHEMATC
