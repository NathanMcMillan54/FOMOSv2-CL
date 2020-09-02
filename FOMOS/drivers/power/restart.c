#include <unistd.h>
#include <sys/reboot.h>

int main () {
    sync();
    setuid(0);
    reboot(RB_AUTOBOOT);
    return(0);
}