#include <stdio.h>

int main(int argc, char **argv) {
    static int x = 5;
    x = 5;
    printf("%d\n", x);
}