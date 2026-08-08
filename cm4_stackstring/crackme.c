#include <stdio.h>
#include <string.h>
#include "../../common/ui.h"

int main(void) {
    print_banner(1, 4, "Stack string",
        "La clave se construye caracter a caracter en la pila.");

    char input[256];
    read_input("Contrasena : ", input, sizeof(input));

    char pw[10];
    pw[0]='f'; pw[1]='l'; pw[2]='a'; pw[3]='g'; pw[4]='_';
    pw[5]='2'; pw[6]='0'; pw[7]='2'; pw[8]='4'; pw[9]='\0';

    if (strcmp(input, pw) == 0) {
        print_ok("Excelente! Flag encontrado.");
        return 0;
    }
    print_err("Nope.");
    return 1;
}
