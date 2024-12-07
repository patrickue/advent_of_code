#include <stdio.h>
#include <stdlib.h>  // For strtol
#include <string.h>  // For strtok


typedef struct  {
    int result;
    int *ops;
    int size;
} Equation;

void free_equation(Equation* eq) {
    if (eq != NULL) {
        free(eq->ops);  // Free the array memory
        free(eq);       // Free the struct memory
    }
}

Equation* parse_to_equation(char line[]) {
    char *token;
    int value;
    Equation* eq = malloc(sizeof(Equation));
    eq->result = 0;
    eq->ops = NULL;
    eq->size = 0;

    // Get the result first
    token = strtok(line, ": ");
    value = strtol(token, NULL, 10);
    // printf("Parsed result: %d from token %s\n", value, token);

    eq->result = value;

    // Now get all the operands
    token = strtok(NULL, " ");
    while (token != NULL) {
        // Resize the array to hold one more integer
        eq->ops = realloc(eq->ops, (eq->size + 2) * sizeof(int));

        // Check if realloc failed
        if (eq->ops == NULL) {
            fprintf(stderr, "Memory allocation failed\n");
            return NULL;
        }

        // Convert the token to an integer and store it in the array
        value = strtol(token, NULL, 10);
        // printf("Parsed integer: %d from token %s\n", value, token);

        eq->ops[eq->size] = value;
        eq->size++;

        token = strtok(NULL, " ");

    }
    return eq;
};

int main() {

    FILE *filePointer;
    filePointer = fopen("input.txt", "r");

    if (filePointer == NULL) {
        printf("File not opened.\n");
        return 2;
    }

    // Declare the variable for the data to be read from
    // file
    char dataToBeRead[151];

    while (fgets(dataToBeRead, 151, filePointer)
           != NULL) {

        // Print the dataToBeRead
        printf("%s", dataToBeRead);
        Equation* eq = parse_to_equation(dataToBeRead);
        if (eq != NULL) {
            printf("Equation: res: %d first: %d size: %d\n", eq->result, eq->ops[1], eq->size);
        }

        free_equation(eq);
    }
    fclose(filePointer);
    return 1;
};
