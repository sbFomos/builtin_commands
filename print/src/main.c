extern int print(char *arg);
extern int easter_egg();

int main(int argc, char *argv[]) {
    char *printText = argv[1];
    if (*printText == "__rick_astley__") {
        easter_egg();
    } else {
        print(printText);
    }
    return 0;
}
