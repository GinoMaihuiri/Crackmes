#include <stdio.h>
#include <string.h>
#include "../../common/ui.h"

int main(void) {
    print_banner(1, 3, "XOR decode",
        "La clave esta cifrada. Aplica XOR para revelarla.");

    char input[256];
    read_input("Contrasena : ", input, sizeof(input));

    /* "pwned" XOR 0x13 */
    unsigned char enc[] = {0x63, 0x64, 0x7D, 0x76, 0x77};
    char decoded[6];
    int i;
    for (i = 0; i < 5; i++) decoded[i] = enc[i] ^ 0x13;
    decoded[5] = '\0';

    if (strcmp(input, decoded) == 0) {
        print_ok("Password correcto!");
        return 0;
    }
    print_err("Password incorrecto.");
    return 1;
}
