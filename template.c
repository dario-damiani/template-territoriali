#include <stdio.h>

void risolvi() {
    // Leggi input
    // scanf("...")

    // Scrivi output
    printf("42\n");
}

int main() {
    int T;
    scanf("%d", &T);

    for (int i = 1; i <= T; i++) {
        printf("Case #%d: ", i);

        risolvi();
    }
}