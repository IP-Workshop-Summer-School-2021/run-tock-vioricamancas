

```
panicked at 'Process faulty_app had a fault', kernel/src/process_standard.rs:323:17
        Kernel version 463e9fc

---| No debug queue found. You can set it with the DebugQueue component.

---| Cortex-M Fault Status |---
Data Access Violation:              true
Undefined Instruction Usage Fault:  true
Forced Hard Fault:                  true
Faulting Memory Address:            0x00000000
Fault Status Register (CFSR):       0x00010082
Hard Fault Status Register (HFSR):  0x40000000

---| App Status |---
𝐀𝐩𝐩: example_app   -   [Unstarted]
 Events Queued: 1   Syscall Count: 0   Dropped Upcall Count: 0
 Restart Count: 0
 Last Syscall: None


 ╔═══════════╤══════════════════════════════════════════╗
 ║  Address  │ Region Name    Used | Allocated (bytes)  ║
 ╚0x20006000═╪══════════════════════════════════════════╝
             │ ▼ Grant        1128 |   1128          
  0x20005B98 ┼───────────────────────────────────────────
             │ Unused
  0x20004020 ┼───────────────────────────────────────────
             │ ▲ Heap            ? |      ?               S
  ?????????? ┼─────────────────────────────────────────── R
             │ Data              ? |      ?               A
  ?????????? ┼─────────────────────────────────────────── M
             │ ▼ Stack           ? |      ?
  0x00000000 ┼───────────────────────────────────────────
             │ Unused
  0x20004000 ┴───────────────────────────────────────────
             .....
  0x00042000 ┬─────────────────────────────────────────── F
             │ App Flash      8136                        L
  0x00040038 ┼─────────────────────────────────────────── A
             │ Protected        56                        S
  0x00040000 ┴─────────────────────────────────────────── H

  R0 : 0x00000080    R6 : 0x00000000
  R1 : 0x20004A6C    R7 : 0x00000000
  R2 : 0x00002000    R8 : 0x00000000
  R3 : 0x20004020    R10: 0x00000000
  R4 : 0x00000000    R11: 0x00000000
  R5 : 0x00000000    R12: 0x32768C4A
  R9 : 0x00000000 (Static Base Register)
  SP : 0x20004000 (Process Stack Pointer)
  LR : 0x00000001
  PC : 0x00040092
 YPC : 0x00000000

 APSR: N 0 Z 0 C 1 V 0 Q 0
       GE 0 0 0 0
 EPSR: ICI.IT 0x00
       ThumbBit true 

 Total number of grant regions defined: 15
  Grant  0 : --          Grant  5 : --          Grant 10 : --        
  Grant  1 : --          Grant  6 : --          Grant 11 : --        
  Grant  2 : --          Grant  7 : --          Grant 12 : --        
  Grant  3 : --          Grant  8 : --          Grant 13 : --        
  Grant  4 : --          Grant  9 : --          Grant 14 : --        

 Cortex-M MPU
  Region 0: [0x20004000:0x20006000], length: 8192 bytes; ReadWrite (0x3)
    Sub-region 0: [0x20004000:0x20004400], Enabled
    Sub-region 1: [0x20004400:0x20004800], Disabled
    Sub-region 2: [0x20004800:0x20004C00], Disabled
    Sub-region 3: [0x20004C00:0x20005000], Disabled
    Sub-region 4: [0x20005000:0x20005400], Disabled
    Sub-region 5: [0x20005400:0x20005800], Disabled
    Sub-region 6: [0x20005800:0x20005C00], Disabled
    Sub-region 7: [0x20005C00:0x20006000], Disabled
  Region 1: [0x00040000:0x00042000], length: 8192 bytes; UnprivilegedReadOnly (0x2)
    Sub-region 0: [0x00040000:0x00040400], Enabled
    Sub-region 1: [0x00040400:0x00040800], Enabled
    Sub-region 2: [0x00040800:0x00040C00], Enabled
    Sub-region 3: [0x00040C00:0x00041000], Enabled
    Sub-region 4: [0x00041000:0x00041400], Enabled
    Sub-region 5: [0x00041400:0x00041800], Enabled
    Sub-region 6: [0x00041800:0x00041C00], Enabled
    Sub-region 7: [0x00041C00:0x00042000], Enabled
  Region 2: Unused
  Region 3: Unused
  Region 4: Unused
  Region 5: Unused
  Region 6: Unused
  Region 7: Unused

To debug, run `make debug RAM_START=0x20004000 FLASH_INIT=0x40061`
in the app's folder and open the .lst file.

𝐀𝐩𝐩: faulty_app   -   [Faulted]
 Events Queued: 0   Syscall Count: 3   Dropped Upcall Count: 0
 Restart Count: 0
 Last Syscall: Some(Memop { operand: 11, arg0: 536898020 })


 ╔═══════════╤══════════════════════════════════════════╗
 ║  Address  │ Region Name    Used | Allocated (bytes)  ║
 ╚0x20008000═╪══════════════════════════════════════════╝
             │ ▼ Grant        1128 |   1128          
  0x20007B98 ┼───────────────────────────────────────────
             │ Unused
  0x200069E4 ┼───────────────────────────────────────────
             │ ▲ Heap            0 |   4532               S
  0x200069E4 ┼─────────────────────────────────────────── R
             │ Data            484 |    484               A
  0x20006800 ┼─────────────────────────────────────────── M
             │ ▼ Stack        2048 |   2048          
  0x20006000 ┼───────────────────────────────────────────
             │ Unused
  0x20006000 ┴───────────────────────────────────────────
             .....
  0x00042400 ┬─────────────────────────────────────────── F
             │ App Flash       968                        L
  0x00042038 ┼─────────────────────────────────────────── A
             │ Protected        56                        S
  0x00042000 ┴─────────────────────────────────────────── H

  R0 : 0x0000087C    R6 : 0x00042038
  R1 : 0x00042298    R7 : 0x20006000
  R2 : 0x0000000A    R8 : 0x00000000
  R3 : 0x00000000    R10: 0x00000000
  R4 : 0x00042038    R11: 0x00000000
  R5 : 0x00000000    R12: 0x159EBD51
  R9 : 0x20006800 (Static Base Register)
  SP : 0x200067D8 (Process Stack Pointer)
  LR : 0x000420EF
  PC : 0x000420A8
 YPC : 0x00042092

 APSR: N 0 Z 1 C 1 V 0 Q 0
       GE 0 0 0 0
 EPSR: ICI.IT 0x00
       ThumbBit true 

 Total number of grant regions defined: 15
  Grant  0 : --          Grant  5 : --          Grant 10 : --        
  Grant  1 : --          Grant  6 : --          Grant 11 : --        
  Grant  2 : --          Grant  7 : --          Grant 12 : --        
  Grant  3 : --          Grant  8 : --          Grant 13 : --        
  Grant  4 : --          Grant  9 : --          Grant 14 : --        

 Cortex-M MPU
  Region 0: [0x20006000:0x20008000], length: 8192 bytes; ReadWrite (0x3)
    Sub-region 0: [0x20006000:0x20006400], Enabled
    Sub-region 1: [0x20006400:0x20006800], Enabled
    Sub-region 2: [0x20006800:0x20006C00], Enabled
    Sub-region 3: [0x20006C00:0x20007000], Disabled
    Sub-region 4: [0x20007000:0x20007400], Disabled
    Sub-region 5: [0x20007400:0x20007800], Disabled
    Sub-region 6: [0x20007800:0x20007C00], Disabled
    Sub-region 7: [0x20007C00:0x20008000], Disabled
  Region 1: [0x00042000:0x00042400], length: 1024 bytes; UnprivilegedReadOnly (0x2)
    Sub-region 0: [0x00042000:0x00042080], Enabled
    Sub-region 1: [0x00042080:0x00042100], Enabled
    Sub-region 2: [0x00042100:0x00042180], Enabled
    Sub-region 3: [0x00042180:0x00042200], Enabled
    Sub-region 4: [0x00042200:0x00042280], Enabled
    Sub-region 5: [0x00042280:0x00042300], Enabled
    Sub-region 6: [0x00042300:0x00042380], Enabled
    Sub-region 7: [0x00042380:0x00042400], Enabled
  Region 2: Unused
  Region 3: Unused
  Region 4: Unused
  Region 5: Unused
  Region 6: Unused
  Region 7: Unused

To debug, run `make debug RAM_START=0x20006000 FLASH_INIT=0x42061`
in the app's folder and open the .lst file.

```