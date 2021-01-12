#include <stdio.h>

int makeUserName() {
    FILE *fp;
    char inputName[100];
    scanf("%[^\n]", inputName);
    printf("Hello %s!\n", inputName);
    fp = fopen("/configs/user/name", "w+");
    fputs(inputName, fp);
    fclose(fp);
}
