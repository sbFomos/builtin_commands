#include <stdio.h>

char fileInput[999];

int editfile(char file) {
    FILE *fp;
    scanf("%s", fileInput);
    fp = fopen(&file, "a");
    fputs(fileInput, fp);
    return 0;
}
