#include <unistd.h>
#include <sys/reboot.h>

int main() {
    printf("Shutting down... \n");
    sync();
    reboot(RB_POWER_OFF);
    return 0;
}