int main() {
    int i = 1;
    serial_init();
    set_output(LED_0_PIN);

    printf("FOMOSv2-CL \n");
    while (i == 1) {
        port_pin_toggle_output_level(LED_0_PIN);
        for (int i = 0; i < 100000; ++i) {}
    }
}
