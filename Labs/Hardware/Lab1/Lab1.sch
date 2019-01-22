EESchema Schematic File Version 4
EELAYER 26 0
EELAYER END
$Descr A4 11693 8268
encoding utf-8
Sheet 1 1
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
L Device:C C?
U 1 1 5C472960
P 5700 1200
F 0 "C?" H 5815 1246 50  0000 L CNN
F 1 "100n" H 5815 1155 50  0000 L CNN
F 2 "" H 5738 1050 50  0001 C CNN
F 3 "~" H 5700 1200 50  0001 C CNN
	1    5700 1200
	1    0    0    -1  
$EndComp
$Comp
L Device:C C?
U 1 1 5C472B7C
P 6250 1200
F 0 "C?" H 6365 1246 50  0000 L CNN
F 1 "100n" H 6365 1155 50  0000 L CNN
F 2 "" H 6288 1050 50  0001 C CNN
F 3 "~" H 6250 1200 50  0001 C CNN
	1    6250 1200
	1    0    0    -1  
$EndComp
$Comp
L Device:C C?
U 1 1 5C472BCA
P 6650 1200
F 0 "C?" H 6765 1246 50  0000 L CNN
F 1 "100n" H 6765 1155 50  0000 L CNN
F 2 "" H 6688 1050 50  0001 C CNN
F 3 "~" H 6650 1200 50  0001 C CNN
	1    6650 1200
	1    0    0    -1  
$EndComp
$Comp
L Device:C C?
U 1 1 5C472D06
P 7050 1200
F 0 "C?" H 7165 1246 50  0000 L CNN
F 1 "100n" H 7165 1155 50  0000 L CNN
F 2 "" H 7088 1050 50  0001 C CNN
F 3 "~" H 7050 1200 50  0001 C CNN
	1    7050 1200
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR?
U 1 1 5C472DD8
P 6450 1600
F 0 "#PWR?" H 6450 1350 50  0001 C CNN
F 1 "GND" H 6455 1427 50  0000 C CNN
F 2 "" H 6450 1600 50  0001 C CNN
F 3 "" H 6450 1600 50  0001 C CNN
	1    6450 1600
	1    0    0    -1  
$EndComp
Wire Wire Line
	6450 1600 5950 1600
Wire Wire Line
	5950 1600 5950 1400
Wire Wire Line
	6250 1350 6250 1400
Wire Wire Line
	6250 1400 5950 1400
Wire Wire Line
	6650 1400 6650 1350
Wire Wire Line
	7050 1400 7050 1350
$Comp
L power:+3.3V #PWR?
U 1 1 5C473043
P 5400 1750
F 0 "#PWR?" H 5400 1600 50  0001 C CNN
F 1 "+3.3V" H 5415 1923 50  0000 C CNN
F 2 "" H 5400 1750 50  0001 C CNN
F 3 "" H 5400 1750 50  0001 C CNN
	1    5400 1750
	1    0    0    -1  
$EndComp
Wire Wire Line
	5400 1750 5400 1850
Wire Wire Line
	5400 1850 5500 1850
Wire Wire Line
	5600 1850 5600 2100
Wire Wire Line
	5500 2100 5500 1850
Connection ~ 5500 1850
Wire Wire Line
	5500 1850 5600 1850
Wire Wire Line
	5400 2100 5400 1850
Connection ~ 5400 1850
Wire Wire Line
	5300 2100 5300 1850
Wire Wire Line
	5300 1850 5400 1850
$Comp
L Device:C C?
U 1 1 5C473812
P 7500 1200
F 0 "C?" H 7615 1246 50  0000 L CNN
F 1 "4.7u" H 7615 1155 50  0000 L CNN
F 2 "" H 7538 1050 50  0001 C CNN
F 3 "~" H 7500 1200 50  0001 C CNN
	1    7500 1200
	1    0    0    -1  
$EndComp
Wire Wire Line
	7500 1400 7500 1350
$Comp
L power:+3.3V #PWR?
U 1 1 5C473B59
P 6600 700
F 0 "#PWR?" H 6600 550 50  0001 C CNN
F 1 "+3.3V" H 6615 873 50  0000 C CNN
F 2 "" H 6600 700 50  0001 C CNN
F 3 "" H 6600 700 50  0001 C CNN
	1    6600 700 
	1    0    0    -1  
$EndComp
Wire Wire Line
	6600 700  6600 850 
Wire Wire Line
	5700 850  5700 1050
Wire Wire Line
	6250 1050 6250 850 
Connection ~ 6250 850 
Wire Wire Line
	6650 1050 6650 850 
Wire Wire Line
	7050 850  7050 1050
Wire Wire Line
	7500 850  7500 1050
Wire Wire Line
	5700 850  6250 850 
Wire Wire Line
	5700 1350 5700 1400
Wire Wire Line
	5700 1400 5950 1400
Connection ~ 5950 1400
Wire Wire Line
	6250 850  6600 850 
Wire Wire Line
	6250 1400 6650 1400
Connection ~ 6250 1400
Wire Wire Line
	6650 850  6600 850 
Connection ~ 6600 850 
Connection ~ 6650 850 
Wire Wire Line
	6650 850  7050 850 
Wire Wire Line
	7050 850  7500 850 
Connection ~ 7050 850 
Wire Wire Line
	7050 1400 7500 1400
Connection ~ 7050 1400
Wire Wire Line
	6650 1400 7050 1400
Connection ~ 6650 1400
Wire Wire Line
	5200 2100 5200 1850
Wire Wire Line
	5200 1850 5300 1850
Connection ~ 5300 1850
$Comp
L MCU_ST_STM32F4:STM32F411RETx U?
U 1 1 5C47C3AE
P 5400 3800
F 0 "U?" H 5400 1914 50  0000 C CNN
F 1 "STM32F411RETx" H 5400 1823 50  0000 C CNN
F 2 "Package_QFP:LQFP-64_10x10mm_P0.5mm" H 4800 2100 50  0001 R CNN
F 3 "http://www.st.com/st-web-ui/static/active/en/resource/technical/document/datasheet/DM00115249.pdf" H 5400 3800 50  0001 C CNN
	1    5400 3800
	1    0    0    -1  
$EndComp
$Comp
L Device:C C?
U 1 1 5C47C813
P 6600 2250
F 0 "C?" H 6715 2296 50  0000 L CNN
F 1 "1u" H 6715 2205 50  0000 L CNN
F 2 "" H 6638 2100 50  0001 C CNN
F 3 "~" H 6600 2250 50  0001 C CNN
	1    6600 2250
	1    0    0    -1  
$EndComp
$Comp
L Device:C C?
U 1 1 5C47C892
P 7150 2250
F 0 "C?" H 7265 2296 50  0000 L CNN
F 1 "100n" H 7265 2205 50  0000 L CNN
F 2 "" H 7188 2100 50  0001 C CNN
F 3 "~" H 7150 2250 50  0001 C CNN
	1    7150 2250
	1    0    0    -1  
$EndComp
Wire Wire Line
	5700 2100 5700 2000
Wire Wire Line
	5700 2000 6600 2000
Wire Wire Line
	6600 2000 6600 2100
Wire Wire Line
	6600 2000 7150 2000
Wire Wire Line
	7150 2000 7150 2100
Connection ~ 6600 2000
$Comp
L power:GND #PWR?
U 1 1 5C47D6B0
P 6850 2600
F 0 "#PWR?" H 6850 2350 50  0001 C CNN
F 1 "GND" H 6855 2427 50  0000 C CNN
F 2 "" H 6850 2600 50  0001 C CNN
F 3 "" H 6850 2600 50  0001 C CNN
	1    6850 2600
	1    0    0    -1  
$EndComp
Wire Wire Line
	6600 2400 6600 2500
Wire Wire Line
	6600 2500 6850 2500
Wire Wire Line
	6850 2500 6850 2600
Wire Wire Line
	7150 2400 7150 2500
Wire Wire Line
	7150 2500 6850 2500
Connection ~ 6850 2500
$Comp
L Device:C C?
U 1 1 5C47EE83
P 4000 2900
F 0 "C?" H 4115 2946 50  0000 L CNN
F 1 "4.7u" H 4115 2855 50  0000 L CNN
F 2 "" H 4038 2750 50  0001 C CNN
F 3 "~" H 4000 2900 50  0001 C CNN
	1    4000 2900
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR?
U 1 1 5C47EF2C
P 4000 3200
F 0 "#PWR?" H 4000 2950 50  0001 C CNN
F 1 "GND" H 4005 3027 50  0000 C CNN
F 2 "" H 4000 3200 50  0001 C CNN
F 3 "" H 4000 3200 50  0001 C CNN
	1    4000 3200
	1    0    0    -1  
$EndComp
Wire Wire Line
	4000 3200 4000 3050
Wire Wire Line
	4000 2750 4000 2700
Wire Wire Line
	4000 2700 4700 2700
$Comp
L power:GND #PWR?
U 1 1 5C484167
P 5400 5950
F 0 "#PWR?" H 5400 5700 50  0001 C CNN
F 1 "GND" H 5405 5777 50  0000 C CNN
F 2 "" H 5400 5950 50  0001 C CNN
F 3 "" H 5400 5950 50  0001 C CNN
	1    5400 5950
	1    0    0    -1  
$EndComp
Wire Wire Line
	5200 5600 5200 5850
Wire Wire Line
	5200 5850 5300 5850
Wire Wire Line
	5400 5850 5400 5950
Wire Wire Line
	5300 5600 5300 5850
Connection ~ 5300 5850
Wire Wire Line
	5300 5850 5400 5850
Wire Wire Line
	5400 5600 5400 5850
Connection ~ 5400 5850
Wire Wire Line
	5500 5600 5500 5850
Wire Wire Line
	5500 5850 5400 5850
Wire Wire Line
	5600 5600 5600 5850
Wire Wire Line
	5600 5850 5500 5850
Connection ~ 5500 5850
$Comp
L Device:R R?
U 1 1 5C488244
P 3850 2050
F 0 "R?" H 3920 2096 50  0000 L CNN
F 1 "10k" H 3920 2005 50  0000 L CNN
F 2 "" V 3780 2050 50  0001 C CNN
F 3 "~" H 3850 2050 50  0001 C CNN
	1    3850 2050
	1    0    0    -1  
$EndComp
Wire Wire Line
	4700 2300 3850 2300
Wire Wire Line
	3850 2300 3850 2200
$Comp
L power:+3.3V #PWR?
U 1 1 5C4894D8
P 3850 1700
F 0 "#PWR?" H 3850 1550 50  0001 C CNN
F 1 "+3.3V" H 3865 1873 50  0000 C CNN
F 2 "" H 3850 1700 50  0001 C CNN
F 3 "" H 3850 1700 50  0001 C CNN
	1    3850 1700
	1    0    0    -1  
$EndComp
Wire Wire Line
	3850 1700 3850 1900
$Comp
L Device:R R?
U 1 1 5C48A51D
P 3500 2750
F 0 "R?" H 3570 2796 50  0000 L CNN
F 1 "10k" H 3570 2705 50  0000 L CNN
F 2 "" V 3430 2750 50  0001 C CNN
F 3 "~" H 3500 2750 50  0001 C CNN
	1    3500 2750
	1    0    0    -1  
$EndComp
Wire Wire Line
	4700 2500 3500 2500
Wire Wire Line
	3500 2500 3500 2600
$Comp
L power:GND #PWR?
U 1 1 5C48B6DC
P 3500 3200
F 0 "#PWR?" H 3500 2950 50  0001 C CNN
F 1 "GND" H 3505 3027 50  0000 C CNN
F 2 "" H 3500 3200 50  0001 C CNN
F 3 "" H 3500 3200 50  0001 C CNN
	1    3500 3200
	1    0    0    -1  
$EndComp
Wire Wire Line
	3500 2900 3500 3200
$Comp
L Device:Crystal Y?
U 1 1 5C48CC6E
P 3100 3950
F 0 "Y?" V 3054 4081 50  0000 L CNN
F 1 "8.0MHz" V 3145 4081 50  0000 L CNN
F 2 "" H 3100 3950 50  0001 C CNN
F 3 "~" H 3100 3950 50  0001 C CNN
	1    3100 3950
	0    1    1    0   
$EndComp
Wire Wire Line
	4100 3500 4700 3500
Wire Wire Line
	3100 4100 3300 4100
Wire Wire Line
	4100 3500 4100 4100
Wire Wire Line
	3100 3400 4700 3400
Wire Wire Line
	3100 3400 3100 3800
$Comp
L power:GND #PWR?
U 1 1 5C494CB3
P 3300 4550
F 0 "#PWR?" H 3300 4300 50  0001 C CNN
F 1 "GND" H 3305 4377 50  0000 C CNN
F 2 "" H 3300 4550 50  0001 C CNN
F 3 "" H 3300 4550 50  0001 C CNN
	1    3300 4550
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR?
U 1 1 5C494CEE
P 2600 4550
F 0 "#PWR?" H 2600 4300 50  0001 C CNN
F 1 "GND" H 2605 4377 50  0000 C CNN
F 2 "" H 2600 4550 50  0001 C CNN
F 3 "" H 2600 4550 50  0001 C CNN
	1    2600 4550
	1    0    0    -1  
$EndComp
$Comp
L Device:C C?
U 1 1 5C4962DD
P 2600 4100
F 0 "C?" H 2715 4146 50  0000 L CNN
F 1 "18p" H 2715 4055 50  0000 L CNN
F 2 "" H 2638 3950 50  0001 C CNN
F 3 "~" H 2600 4100 50  0001 C CNN
	1    2600 4100
	1    0    0    -1  
$EndComp
$Comp
L Device:C C?
U 1 1 5C496355
P 3300 4300
F 0 "C?" H 3415 4346 50  0000 L CNN
F 1 "18p" H 3415 4255 50  0000 L CNN
F 2 "" H 3338 4150 50  0001 C CNN
F 3 "~" H 3300 4300 50  0001 C CNN
	1    3300 4300
	1    0    0    -1  
$EndComp
Wire Wire Line
	3300 4150 3300 4100
Connection ~ 3300 4100
Wire Wire Line
	3300 4100 4100 4100
Wire Wire Line
	3300 4450 3300 4550
Wire Wire Line
	2600 4250 2600 4550
Wire Wire Line
	2600 3950 2600 3400
Wire Wire Line
	2600 3400 3100 3400
Connection ~ 3100 3400
$Comp
L Connector:Conn_01x06_Male J?
U 1 1 5C49DCCF
P 8600 3950
F 0 "J?" H 8706 4328 50  0000 C CNN
F 1 "Conn_01x06_Male" H 8706 4237 50  0000 C CNN
F 2 "" H 8600 3950 50  0001 C CNN
F 3 "~" H 8600 3950 50  0001 C CNN
	1    8600 3950
	1    0    0    -1  
$EndComp
Wire Wire Line
	8800 3850 9450 3850
Text Label 9450 3850 0    50   ~ 0
SWCLK
Wire Wire Line
	8800 4050 9450 4050
Text Label 9450 4050 0    50   ~ 0
SWDIO
Wire Wire Line
	8800 4150 9450 4150
Text Label 9450 4150 0    50   ~ 0
NRST
Wire Wire Line
	8800 4250 9450 4250
Text Label 9450 4250 0    50   ~ 0
SWO
Text Label 4450 2300 0    50   ~ 0
NRST
$Comp
L power:GND #PWR?
U 1 1 5C4AC7AE
P 10200 3950
F 0 "#PWR?" H 10200 3700 50  0001 C CNN
F 1 "GND" V 10205 3822 50  0000 R CNN
F 2 "" H 10200 3950 50  0001 C CNN
F 3 "" H 10200 3950 50  0001 C CNN
	1    10200 3950
	0    -1   -1   0   
$EndComp
Wire Wire Line
	8800 3950 10150 3950
$Comp
L power:+3.3V #PWR?
U 1 1 5C4B1FE4
P 9700 3300
F 0 "#PWR?" H 9700 3150 50  0001 C CNN
F 1 "+3.3V" H 9715 3473 50  0000 C CNN
F 2 "" H 9700 3300 50  0001 C CNN
F 3 "" H 9700 3300 50  0001 C CNN
	1    9700 3300
	1    0    0    -1  
$EndComp
Wire Wire Line
	9700 3750 9700 3350
Wire Wire Line
	8800 3750 9700 3750
$Comp
L Device:C C?
U 1 1 5C4B4024
P 10150 3500
F 0 "C?" H 10265 3546 50  0000 L CNN
F 1 "100n" H 10265 3455 50  0000 L CNN
F 2 "" H 10188 3350 50  0001 C CNN
F 3 "~" H 10150 3500 50  0001 C CNN
	1    10150 3500
	1    0    0    -1  
$EndComp
Wire Wire Line
	9700 3350 10150 3350
Connection ~ 9700 3350
Wire Wire Line
	9700 3350 9700 3300
Wire Wire Line
	10150 3650 10150 3950
Connection ~ 10150 3950
Wire Wire Line
	10150 3950 10200 3950
Wire Wire Line
	6100 3700 6400 3700
Text Label 6400 3700 0    50   ~ 0
SWCLK
Wire Wire Line
	6100 3600 6400 3600
Text Label 6400 3600 0    50   ~ 0
SWDIO
Wire Wire Line
	6100 4300 6400 4300
Text Label 6400 4300 0    50   ~ 0
SWO
$EndSCHEMATC
