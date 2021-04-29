#include <iostream>
using namespace std;

void risolvi() {
    // Leggi input
    // cin >> variabile ...

    // Scrivi output
    cout << 42 << endl;
}

int main() {
    int T;
    cin >> T;

    for (int i = 1; i <= T; i++) {
        cout << "Case #" << i << ": ";

        risolvi();
    }
}