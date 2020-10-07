void main(void) {
    int i;
    char s[] = {'F', 'O', 'M', 'O', 'S', 'v', '2', '-', 'C', 'L', ' ', 'v', '2', '.', '2', '.', '5'};
    for (i = 0; i < sizeof(s); ++i) {
        __asm__ (
        "int $0x10" : : "a" ((0x0e << 8) | s[i])
        );
    }
    while (1) {
        __asm__ ("hlt");
    };
}