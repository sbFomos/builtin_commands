#include <stdio.h>
#include <string.h>

int checkPassword() {
    FILE *fp;
    char askuInput[24];
    char passwordFile[24];
    scanf("%s", askuInput);
    fp = fopen("/configs/user/password", "r");
    while (fgets(passwordFile, 24, fp) != NULL)
    if (!strcmp(askuInput, passwordFile)) {
        return 1;
    } else {
        return 0;
    }
}
