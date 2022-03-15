#include <stdio.h>

extern int yyparse();

int main() {
  if (yyparse() == 0) {
    fprintf(stderr, "failed\n");
    return 1;
  }
  return 0;
}