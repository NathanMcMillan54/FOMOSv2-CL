#include <string.h>

int add_str(char str1, char str2) {
    strcat(&str1, &str2);
    return str1;
}
