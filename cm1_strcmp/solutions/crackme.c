#include <stdio.h>
#include <string.h>
#include "../../common/ui.h"

int main(void) {
    print_banner(1, 1, "strcmp hardcodeado",
        "Un guardian compara tu clave directamente con strcmp.");

    char input[256];
    read_input("Contrasena : ", input, sizeof(input));

    if (strcmp(input, "s3cr3t0") == 0) {
        print_ok("Correcto! Bien hecho.");
        return 0;
    }
    print_err("Incorrecto. Intenta de nuevo.");
    return 1;
}
