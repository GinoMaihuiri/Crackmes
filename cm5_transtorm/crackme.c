#include <stdio.h>
#include <string.h>
#include "../../common/ui.h"

int main(void) {
    print_banner(1, 5, "Transform",
        "Cada caracter es transformado antes de comparar.");

    char input[256];
    read_input("Contrasena : ", input, sizeof(input));

    /* each char of "crackme" + 5 = expected */
    unsigned char expected[] = {0x68, 0x77, 0x66, 0x68, 0x70, 0x72, 0x6A};
    int len = 7;

    if ((int)strlen(input) != len) { print_err("Longitud incorrecta."); return 1; }

    int i;
    for (i = 0; i < len; i++) {
        if ((unsigned char)(input[i] + 5) != expected[i]) {
            print_err("Password invalido.");
            return 1;
        }
    }
    print_ok("Password valido!");
    return 0;
}
