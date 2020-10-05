#include <stdint.h>

uint8_t led_animation_id = 0;

extern uint16_t keymaps[][2][3];

void matrix_init_user(void) {
    led_animation_id = 1;
}

void matrix_scan_user(void);

int main() {
    matrix_init_user();
    matrix_scan_user();
    return keymaps[0][0][0];
}
