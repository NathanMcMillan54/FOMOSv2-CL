/* kernel/src/clear.c
 *
 * Build/compile:
 * cargo build
 *
 * Description:
 * Clears all the text that is currently on the screen
 */

#include <unistd.h>

int clearScreen() {
    const char *clear = "\e[1;1H\e[2J";
    write(STDOUT_FILENO, clear, 12);
    return 0;
}
