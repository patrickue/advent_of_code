#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>  // For strtol
#include <string.h>  // For strtok


typedef struct  {
    unsigned long long int result;
    unsigned long long int *ops;
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
    unsigned long long int value;
    Equation* eq = malloc(sizeof(Equation));
    eq->result = 0;
    eq->ops = NULL;
    eq->size = 0;

    // Get the result first
    token = strtok(line, ": ");
    value = strtoull(token, NULL, 10);
    // printf("Parsed result: %d from token %s\n", value, token);

    eq->result = value;

    // Now get all the operands
    token = strtok(NULL, " ");
    while (token != NULL) {
        // Resize the array to hold one more integer
        eq->ops = realloc(eq->ops, (eq->size + 2) * sizeof(unsigned long long int));

        // Check if realloc failed
        if (eq->ops == NULL) {
            fprintf(stderr, "Memory allocation failed\n");
            return NULL;
        }

        value = strtoull(token, NULL, 10);
        // printf("Parsed integer: %d from token %s\n", value, token);

        eq->ops[eq->size] = value;
        eq->size++;

        token = strtok(NULL, " ");

    }
    return eq;
};

bool try_operators(Equation* eq) {

    // 2 ^ of eq->size in normal languages
    unsigned int amount_of_trials = (2 << eq->size);
    printf("Amount of trials: %d\n", amount_of_trials);

    for (unsigned int trial = 0; trial < amount_of_trials; trial++)
    {
        // printf("\nTrial %d", trial);
        unsigned long long int trial_res = eq->ops[0];
        for (unsigned int i = 1; i < eq->size; i++)
        {
            if (((1 << i-1) & trial) != 0)
            {
                trial_res *= eq->ops[i];
                // printf("Mul; ");
            }
            else
            {
                trial_res += eq->ops[i];
                // printf("Add; ");
            }
        }

        if (trial_res == eq->result){
            return true;
        }
    }
    return false;
}


int main() {

    FILE *filePointer;
    filePointer = fopen("input.txt", "r");

    if (filePointer == NULL) {
        printf("File not opened.\n");
        return 2;
    }

    // Declare the variable for the data to be read from
    // file
    char dataToBeRead[200];

    unsigned long long int sum_of_test_values = 0;
    int total_equations = 0;
    while (fgets(dataToBeRead, 200, filePointer) != NULL)
    {
        total_equations++;
        // Print the dataToBeRead
        printf("%s", dataToBeRead);
        Equation* eq = parse_to_equation(dataToBeRead);
        if (eq != NULL) {
            printf("Equation: res: %llu first: %llu size: %d\n", eq->result, eq->ops[1], eq->size);
        }
        if (try_operators(eq) != 0) {
            sum_of_test_values += eq->result;
        }
        free_equation(eq);
        // break;
    }
    printf("This many equations in total: %d\n", total_equations);
    printf("This is the sum of test values for equations which work: %llu\n", sum_of_test_values);
    fclose(filePointer);
    return 1;
};
