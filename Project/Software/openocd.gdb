target extended-remote :3333

# Detect unhandled exceptions, hard faults and panics: 
break DefaultHandler
break HardFault
break rust_begin_unwind

monitor tpiu config internal /tmp/itm.log uart off 64000000
monitor itm port 0 on

load

stepi