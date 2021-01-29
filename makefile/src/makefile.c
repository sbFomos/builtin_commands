#include <stdio.h>

void makefile(char file_name) {
    FILE *fp;
    fp = fopen(&file_name, "w");
    if (fp == NULL) {
        printf("Couldn't create file: %s\n", file_name);
    } else {
        printf("%s created successfully\n", file_name);
    }
    fclose(fp);
}
