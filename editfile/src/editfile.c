#include <stdio.h>

char fileInput[999];

int editfile(char file) {
    FILE *fp;
    printf("File input: ");
    scanf("%s", fileInput);
    fp = fopen(&file, "a");
    fputs(fileInput, fp);
    return 0;
}
