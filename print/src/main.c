#include <stdio.h>

extern int print(char *arg);

int main( int argc, char *argv[] )  {
    char *printText = argv[1];
    print(printText);
    return 0;
}
