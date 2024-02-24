# All bootloaders must specify an entry point named kstart
ENTRY(kstart)

SECTIONS {
    .rodata :
    {
     	__PG_SIZE = 0x1000;
    }
}