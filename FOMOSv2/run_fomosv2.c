int power = 1;
extern int init();
extern int shutdown();
extern int power_err();

int main() {
    for (int i = 0; i < 2; ++i) {
        if (power == 1) {
            init();
            power = 0;
        } else if (power == 0) {
            shutdown();
        } else {
            power_err();
        }
    }
    return 0;
}
