EESchema Schematic File Version 4
LIBS:Lab1-cache
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
L Device:C C8
U 1 1 5C472960
P 5700 1200
F 0 "C8" H 5815 1246 50  0000 L CNN
F 1 "100n" H 5815 1155 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 5738 1050 50  0001 C CNN
F 3 "~" H 5700 1200 50  0001 C CNN
	1    5700 1200
	1    0    0    -1  
$EndComp
$Comp
L Device:C C9
U 1 1 5C472B7C
P 6250 1200
F 0 "C9" H 6365 1246 50  0000 L CNN
F 1 "100n" H 6365 1155 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 6288 1050 50  0001 C CNN
F 3 "~" H 6250 1200 50  0001 C CNN
	1    6250 1200
	1    0    0    -1  
$EndComp
$Comp
L Device:C C11
U 1 1 5C472BCA
P 6650 1200
F 0 "C11" H 6765 1246 50  0000 L CNN
F 1 "100n" H 6765 1155 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 6688 1050 50  0001 C CNN
F 3 "~" H 6650 1200 50  0001 C CNN
	1    6650 1200
	1    0    0    -1  
$EndComp
$Comp
L Device:C C12
U 1 1 5C472D06
P 7050 1200
F 0 "C12" H 7165 1246 50  0000 L CNN
F 1 "100n" H 7165 1155 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 7088 1050 50  0001 C CNN
F 3 "~" H 7050 1200 50  0001 C CNN
	1    7050 1200
	1    0    0    -1  
$EndComp
Wire Wire Line
	6250 1350 6250 1400
Wire Wire Line
	6650 1400 6650 1350
Wire Wire Line
	7050 1400 7050 1350
$Comp
L power:+3.3V #PWR013
U 1 1 5C473043
P 5400 1750
F 0 "#PWR013" H 5400 1600 50  0001 C CNN
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
L Device:C C14
U 1 1 5C473812
P 7500 1200
F 0 "C14" H 7615 1246 50  0000 L CNN
F 1 "4.7u" H 7615 1155 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 7538 1050 50  0001 C CNN
F 3 "~" H 7500 1200 50  0001 C CNN
	1    7500 1200
	1    0    0    -1  
$EndComp
Wire Wire Line
	7500 1400 7500 1350
$Comp
L power:+3.3V #PWR017
U 1 1 5C473B59
P 6600 700
F 0 "#PWR017" H 6600 550 50  0001 C CNN
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
	6250 850  6600 850 
Wire Wire Line
	6250 1400 6400 1400
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
L MCU_ST_STM32F4:STM32F411RETx U2
U 1 1 5C47C3AE
P 5400 3800
F 0 "U2" H 5400 1914 50  0000 C CNN
F 1 "STM32F411RETx" H 5400 1823 50  0000 C CNN
F 2 "Housings_QFP:LQFP-64_10x10mm_Pitch0.5mm" H 4800 2100 50  0001 R CNN
F 3 "http://www.st.com/st-web-ui/static/active/en/resource/technical/document/datasheet/DM00115249.pdf" H 5400 3800 50  0001 C CNN
	1    5400 3800
	1    0    0    -1  
$EndComp
$Comp
L Device:C C10
U 1 1 5C47C813
P 6600 2250
F 0 "C10" H 6715 2296 50  0000 L CNN
F 1 "1u" H 6715 2205 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 6638 2100 50  0001 C CNN
F 3 "~" H 6600 2250 50  0001 C CNN
	1    6600 2250
	1    0    0    -1  
$EndComp
$Comp
L Device:C C13
U 1 1 5C47C892
P 7150 2250
F 0 "C13" H 7265 2296 50  0000 L CNN
F 1 "100n" H 7265 2205 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 7188 2100 50  0001 C CNN
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
L power:GND #PWR018
U 1 1 5C47D6B0
P 6850 2600
F 0 "#PWR018" H 6850 2350 50  0001 C CNN
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
L Device:C C6
U 1 1 5C47EE83
P 4000 2900
F 0 "C6" H 4115 2946 50  0000 L CNN
F 1 "4.7u" H 4115 2855 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 4038 2750 50  0001 C CNN
F 3 "~" H 4000 2900 50  0001 C CNN
	1    4000 2900
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR010
U 1 1 5C47EF2C
P 4000 3200
F 0 "#PWR010" H 4000 2950 50  0001 C CNN
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
L power:GND #PWR014
U 1 1 5C484167
P 5400 5950
F 0 "#PWR014" H 5400 5700 50  0001 C CNN
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
L Device:R R2
U 1 1 5C488244
P 3850 2050
F 0 "R2" H 3920 2096 50  0000 L CNN
F 1 "10k" H 3920 2005 50  0000 L CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" V 3780 2050 50  0001 C CNN
F 3 "~" H 3850 2050 50  0001 C CNN
	1    3850 2050
	1    0    0    -1  
$EndComp
Wire Wire Line
	4700 2300 3850 2300
Wire Wire Line
	3850 2300 3850 2200
$Comp
L power:+3.3V #PWR09
U 1 1 5C4894D8
P 3850 1700
F 0 "#PWR09" H 3850 1550 50  0001 C CNN
F 1 "+3.3V" H 3865 1873 50  0000 C CNN
F 2 "" H 3850 1700 50  0001 C CNN
F 3 "" H 3850 1700 50  0001 C CNN
	1    3850 1700
	1    0    0    -1  
$EndComp
Wire Wire Line
	3850 1700 3850 1900
$Comp
L Device:R R1
U 1 1 5C48A51D
P 3500 2750
F 0 "R1" H 3570 2796 50  0000 L CNN
F 1 "10k" H 3570 2705 50  0000 L CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" V 3430 2750 50  0001 C CNN
F 3 "~" H 3500 2750 50  0001 C CNN
	1    3500 2750
	1    0    0    -1  
$EndComp
Wire Wire Line
	4700 2500 3500 2500
Wire Wire Line
	3500 2500 3500 2600
$Comp
L power:GND #PWR08
U 1 1 5C48B6DC
P 3500 3200
F 0 "#PWR08" H 3500 2950 50  0001 C CNN
F 1 "GND" H 3505 3027 50  0000 C CNN
F 2 "" H 3500 3200 50  0001 C CNN
F 3 "" H 3500 3200 50  0001 C CNN
	1    3500 3200
	1    0    0    -1  
$EndComp
Wire Wire Line
	3500 2900 3500 3200
$Comp
L Device:Crystal Y1
U 1 1 5C48CC6E
P 3100 3950
F 0 "Y1" V 3054 4081 50  0000 L CNN
F 1 "8.0MHz" V 3145 4081 50  0000 L CNN
F 2 "Crystals:Crystal_HC49-U_Vertical" H 3100 3950 50  0001 C CNN
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
L power:GND #PWR07
U 1 1 5C494CB3
P 3300 4550
F 0 "#PWR07" H 3300 4300 50  0001 C CNN
F 1 "GND" H 3305 4377 50  0000 C CNN
F 2 "" H 3300 4550 50  0001 C CNN
F 3 "" H 3300 4550 50  0001 C CNN
	1    3300 4550
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR04
U 1 1 5C494CEE
P 2600 4550
F 0 "#PWR04" H 2600 4300 50  0001 C CNN
F 1 "GND" H 2605 4377 50  0000 C CNN
F 2 "" H 2600 4550 50  0001 C CNN
F 3 "" H 2600 4550 50  0001 C CNN
	1    2600 4550
	1    0    0    -1  
$EndComp
$Comp
L Device:C C2
U 1 1 5C4962DD
P 2600 4100
F 0 "C2" H 2715 4146 50  0000 L CNN
F 1 "18p" H 2715 4055 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 2638 3950 50  0001 C CNN
F 3 "~" H 2600 4100 50  0001 C CNN
	1    2600 4100
	1    0    0    -1  
$EndComp
$Comp
L Device:C C5
U 1 1 5C496355
P 3300 4300
F 0 "C5" H 3415 4346 50  0000 L CNN
F 1 "18p" H 3415 4255 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 3338 4150 50  0001 C CNN
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
L Connector:Conn_01x06_Male J2
U 1 1 5C49DCCF
P 8600 3950
F 0 "J2" H 8706 4328 50  0000 C CNN
F 1 "Conn_01x06_Male" H 8706 4237 50  0000 C CNN
F 2 "Pin_Headers:Pin_Header_Straight_1x06_Pitch2.54mm_SMD_Pin1Left" H 8600 3950 50  0001 C CNN
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
L power:GND #PWR020
U 1 1 5C4AC7AE
P 10200 3950
F 0 "#PWR020" H 10200 3700 50  0001 C CNN
F 1 "GND" V 10205 3822 50  0000 R CNN
F 2 "" H 10200 3950 50  0001 C CNN
F 3 "" H 10200 3950 50  0001 C CNN
	1    10200 3950
	0    -1   -1   0   
$EndComp
Wire Wire Line
	8800 3950 10150 3950
$Comp
L power:+3.3V #PWR019
U 1 1 5C4B1FE4
P 9700 3300
F 0 "#PWR019" H 9700 3150 50  0001 C CNN
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
L Device:C C15
U 1 1 5C4B4024
P 10150 3500
F 0 "C15" H 10265 3546 50  0000 L CNN
F 1 "100n" H 10265 3455 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 10188 3350 50  0001 C CNN
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
$Comp
L E7020E:LM1117ADJ U1
U 1 1 5C5041B7
P 3800 7600
F 0 "U1" H 4100 9115 50  0000 C CNN
F 1 "LM1117ADJ" H 4100 9024 50  0000 C CNN
F 2 "TO_SOT_Packages_SMD:SOT-223" H 4050 9000 50  0001 C CNN
F 3 "" H 4050 9000 50  0001 C CNN
	1    3800 7600
	1    0    0    -1  
$EndComp
$Comp
L Connector:USB_OTG J1
U 1 1 5C5045B7
P 1000 6650
F 0 "J1" H 1055 7117 50  0000 C CNN
F 1 "USB_OTG" H 1055 7026 50  0000 C CNN
F 2 "Connector_USB:USB_Mini-B_AdamTech_MUSB-B5-S-VT-TSMT-1_SMD_Vertical" H 1150 6600 50  0001 C CNN
F 3 " ~" H 1150 6600 50  0001 C CNN
	1    1000 6650
	1    0    0    -1  
$EndComp
Wire Wire Line
	1300 6450 1600 6450
$Comp
L power:+5V #PWR02
U 1 1 5C50D09C
P 1600 6150
F 0 "#PWR02" H 1600 6000 50  0001 C CNN
F 1 "+5V" H 1615 6323 50  0000 C CNN
F 2 "" H 1600 6150 50  0001 C CNN
F 3 "" H 1600 6150 50  0001 C CNN
	1    1600 6150
	1    0    0    -1  
$EndComp
Wire Wire Line
	1600 6150 1600 6450
Connection ~ 1600 6450
$Comp
L power:+3.3V #PWR012
U 1 1 5C51661E
P 5200 6450
F 0 "#PWR012" H 5200 6300 50  0001 C CNN
F 1 "+3.3V" H 5215 6623 50  0000 C CNN
F 2 "" H 5200 6450 50  0001 C CNN
F 3 "" H 5200 6450 50  0001 C CNN
	1    5200 6450
	1    0    0    -1  
$EndComp
Wire Wire Line
	4700 6450 4850 6450
$Comp
L power:GND #PWR01
U 1 1 5C51DD06
P 1000 7200
F 0 "#PWR01" H 1000 6950 50  0001 C CNN
F 1 "GND" H 1005 7027 50  0000 C CNN
F 2 "" H 1000 7200 50  0001 C CNN
F 3 "" H 1000 7200 50  0001 C CNN
	1    1000 7200
	1    0    0    -1  
$EndComp
Wire Wire Line
	1000 7050 1000 7150
$Comp
L Device:R R3
U 1 1 5C520562
P 5000 6950
F 0 "R3" H 5070 6996 50  0000 L CNN
F 1 "3,3k" H 5070 6905 50  0000 L CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" V 4930 6950 50  0001 C CNN
F 3 "~" H 5000 6950 50  0001 C CNN
	1    5000 6950
	1    0    0    -1  
$EndComp
$Comp
L Device:R R4
U 1 1 5C5205FF
P 5000 7400
F 0 "R4" H 5070 7446 50  0000 L CNN
F 1 "5,6k" H 5070 7355 50  0000 L CNN
F 2 "Resistors_SMD:R_0805_HandSoldering" V 4930 7400 50  0001 C CNN
F 3 "~" H 5000 7400 50  0001 C CNN
	1    5000 7400
	1    0    0    -1  
$EndComp
Wire Wire Line
	4000 7050 4000 7150
Wire Wire Line
	4000 7150 5000 7150
Wire Wire Line
	5000 7150 5000 7100
Wire Wire Line
	5000 7250 5000 7150
Connection ~ 5000 7150
$Comp
L power:GND #PWR011
U 1 1 5C528838
P 5000 7650
F 0 "#PWR011" H 5000 7400 50  0001 C CNN
F 1 "GND" H 5005 7477 50  0000 C CNN
F 2 "" H 5000 7650 50  0001 C CNN
F 3 "" H 5000 7650 50  0001 C CNN
	1    5000 7650
	1    0    0    -1  
$EndComp
Wire Wire Line
	5000 7550 5000 7650
$Comp
L Device:C C4
U 1 1 5C52B732
P 3150 6750
F 0 "C4" H 3265 6796 50  0000 L CNN
F 1 "100n" H 3265 6705 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 3188 6600 50  0001 C CNN
F 3 "~" H 3150 6750 50  0001 C CNN
	1    3150 6750
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR06
U 1 1 5C52C0CC
P 3150 7150
F 0 "#PWR06" H 3150 6900 50  0001 C CNN
F 1 "GND" H 3155 6977 50  0000 C CNN
F 2 "" H 3150 7150 50  0001 C CNN
F 3 "" H 3150 7150 50  0001 C CNN
	1    3150 7150
	1    0    0    -1  
$EndComp
Wire Wire Line
	3150 6450 3150 6600
Wire Wire Line
	3150 6900 3150 7150
Wire Wire Line
	4700 6600 4850 6600
Wire Wire Line
	4850 6600 4850 6450
Connection ~ 4850 6450
Wire Wire Line
	4850 6450 5000 6450
Wire Wire Line
	5000 6450 5000 6800
Connection ~ 5000 6450
Wire Wire Line
	5000 6450 5200 6450
$Comp
L Device:C C7
U 1 1 5C53B486
P 5450 6800
F 0 "C7" H 5565 6846 50  0000 L CNN
F 1 "100n" H 5565 6755 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 5488 6650 50  0001 C CNN
F 3 "~" H 5450 6800 50  0001 C CNN
	1    5450 6800
	1    0    0    -1  
$EndComp
Wire Wire Line
	5200 6450 5450 6450
Wire Wire Line
	5450 6450 5450 6650
Connection ~ 5200 6450
$Comp
L power:GND #PWR015
U 1 1 5C53EAB9
P 5450 7200
F 0 "#PWR015" H 5450 6950 50  0001 C CNN
F 1 "GND" H 5455 7027 50  0000 C CNN
F 2 "" H 5450 7200 50  0001 C CNN
F 3 "" H 5450 7200 50  0001 C CNN
	1    5450 7200
	1    0    0    -1  
$EndComp
Connection ~ 3150 6450
Wire Wire Line
	3150 6450 3500 6450
Wire Wire Line
	1600 6450 2000 6450
$Comp
L Device:C C1
U 1 1 5C545A22
P 2000 6750
F 0 "C1" H 2115 6796 50  0000 L CNN
F 1 "100n" H 2115 6705 50  0000 L CNN
F 2 "Capacitors_SMD:C_0805_HandSoldering" H 2038 6600 50  0001 C CNN
F 3 "~" H 2000 6750 50  0001 C CNN
	1    2000 6750
	1    0    0    -1  
$EndComp
$Comp
L power:GND #PWR03
U 1 1 5C545A29
P 2000 7150
F 0 "#PWR03" H 2000 6900 50  0001 C CNN
F 1 "GND" H 2005 6977 50  0000 C CNN
F 2 "" H 2000 7150 50  0001 C CNN
F 3 "" H 2000 7150 50  0001 C CNN
	1    2000 7150
	1    0    0    -1  
$EndComp
Wire Wire Line
	2000 6900 2000 7150
Wire Wire Line
	2000 6600 2000 6450
Connection ~ 2000 6450
$Comp
L power:GND #PWR05
U 1 1 5C554E15
P 2650 7150
F 0 "#PWR05" H 2650 6900 50  0001 C CNN
F 1 "GND" H 2655 6977 50  0000 C CNN
F 2 "" H 2650 7150 50  0001 C CNN
F 3 "" H 2650 7150 50  0001 C CNN
	1    2650 7150
	1    0    0    -1  
$EndComp
$Comp
L Device:CP C3
U 1 1 5C558E09
P 2650 6750
F 0 "C3" H 2768 6796 50  0000 L CNN
F 1 "10u" H 2768 6705 50  0000 L CNN
F 2 "Capacitors_SMD:CP_Elec_5x5.3" H 2688 6600 50  0001 C CNN
F 3 "~" H 2650 6750 50  0001 C CNN
	1    2650 6750
	1    0    0    -1  
$EndComp
Wire Wire Line
	2000 6450 2650 6450
Wire Wire Line
	2650 6600 2650 6450
Connection ~ 2650 6450
Wire Wire Line
	2650 6450 3150 6450
Wire Wire Line
	2650 7150 2650 6900
$Comp
L Device:CP C16
U 1 1 5C57F118
P 5900 6800
F 0 "C16" H 6018 6846 50  0000 L CNN
F 1 "10u" H 6018 6755 50  0000 L CNN
F 2 "Capacitors_SMD:CP_Elec_5x5.3" H 5938 6650 50  0001 C CNN
F 3 "~" H 5900 6800 50  0001 C CNN
	1    5900 6800
	1    0    0    -1  
$EndComp
Wire Wire Line
	5450 6450 5900 6450
Wire Wire Line
	5900 6450 5900 6650
Connection ~ 5450 6450
$Comp
L power:GND #PWR021
U 1 1 5C583663
P 5900 7200
F 0 "#PWR021" H 5900 6950 50  0001 C CNN
F 1 "GND" H 5905 7027 50  0000 C CNN
F 2 "" H 5900 7200 50  0001 C CNN
F 3 "" H 5900 7200 50  0001 C CNN
	1    5900 7200
	1    0    0    -1  
$EndComp
Wire Wire Line
	5900 7200 5900 6950
Wire Wire Line
	1000 7150 900  7150
Wire Wire Line
	900  7150 900  7050
Connection ~ 1000 7150
Wire Wire Line
	1000 7150 1000 7200
Wire Wire Line
	1300 6650 1450 6650
Text Label 1450 6650 0    50   ~ 0
D+
Wire Wire Line
	1300 6750 1450 6750
Text Label 1450 6750 0    50   ~ 0
D-
Wire Wire Line
	1300 6850 1450 6850
Text Label 1450 6850 0    50   ~ 0
ID
Text Label 6100 2300 0    50   ~ 0
PA0
Text Label 6100 2400 0    50   ~ 0
PA1
Wire Wire Line
	5450 7200 5450 6950
$Comp
L power:+3.3V #PWR0101
U 1 1 5C54647F
P 7150 1800
F 0 "#PWR0101" H 7150 1650 50  0001 C CNN
F 1 "+3.3V" H 7165 1973 50  0000 C CNN
F 2 "" H 7150 1800 50  0001 C CNN
F 3 "" H 7150 1800 50  0001 C CNN
	1    7150 1800
	1    0    0    -1  
$EndComp
Wire Wire Line
	7150 1800 7150 2000
Connection ~ 7150 2000
NoConn ~ 1450 6650
NoConn ~ 1450 6750
NoConn ~ 1450 6850
NoConn ~ 4700 3700
NoConn ~ 4700 3950
NoConn ~ 4700 4000
NoConn ~ 4700 3900
NoConn ~ 4700 4100
NoConn ~ 4700 4200
NoConn ~ 4700 4300
NoConn ~ 4700 4400
NoConn ~ 4700 4500
NoConn ~ 4700 4600
NoConn ~ 4700 4700
NoConn ~ 4700 4800
NoConn ~ 4700 4900
NoConn ~ 4700 5000
NoConn ~ 4700 5100
NoConn ~ 4700 5200
NoConn ~ 4700 5300
NoConn ~ 4700 5400
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
NoConn ~ 6100 4200
NoConn ~ 6100 4100
NoConn ~ 6100 4000
NoConn ~ 6100 3800
NoConn ~ 6100 3500
NoConn ~ 6100 3400
NoConn ~ 6100 3300
NoConn ~ 6100 3200
NoConn ~ 6100 3100
NoConn ~ 6100 3000
NoConn ~ 6100 2900
NoConn ~ 6100 2800
NoConn ~ 6100 2700
NoConn ~ 6100 2600
NoConn ~ 6100 2500
NoConn ~ 6100 2400
NoConn ~ 6100 2300
Wire Wire Line
	5700 1400 6250 1400
$Comp
L power:GND #PWR0102
U 1 1 5C6354F6
P 6400 1550
F 0 "#PWR0102" H 6400 1300 50  0001 C CNN
F 1 "GND" H 6405 1377 50  0000 C CNN
F 2 "" H 6400 1550 50  0001 C CNN
F 3 "" H 6400 1550 50  0001 C CNN
	1    6400 1550
	1    0    0    -1  
$EndComp
Wire Wire Line
	6400 1550 6400 1400
Connection ~ 6400 1400
Wire Wire Line
	6400 1400 6650 1400
$EndSCHEMATC
