int main() {
    int i = 1;
    serial_init();
    set_output(LED_0_PIN);

    printf("FOMOSV2-CL\n");
    printf("Starting FOMOS 1/2 Finished \n");
    // something to start ../setup.c
    // this makes FOMOS never stop
    while (i == 1) {
        port_pin_toggle_output_level(LED_0_PIN);
        for (int i = 0; i < 100000; ++i) {}
    }
}
