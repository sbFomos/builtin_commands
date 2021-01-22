#include <stdio.h>

int checkPassword() {
    FILE *fp;
    char askuInput[24];
    char passwordFile[24];
    fp = fopen("/configs/user/password", "r");
    while (fgets(passwordFile, 24, fp) != NULL);
    scanf("%s", askuInput);
    if (askuInput == passwordFile) {
        return 1;
    } return 0;
}
