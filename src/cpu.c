int a[10];
void brandName(int eaxValues)
{
    if (eaxValues == 1) {
        __asm__("mov $0x80000002 , %eax\n\t");
    }
    __asm__("cpuid\n\t");
    __asm__("mov %%eax, %0\n\t":"=r" ((a[0])));
}

void cpuid()
{
    __asm__("xor %eax , %eax\n\t");
    __asm__("xor %ebx , %ebx\n\t");
    __asm__("xor %ecx , %ecx\n\t");
    __asm__("xor %edx , %edx\n\t");
    brandName(1);
}

int checkArch()
{
    cpuid();
    return a[1];
}
