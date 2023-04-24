#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "lzw.h"

/* Parses an argument of type "-rX", where X is an integer between MIN_RC and
 * MAX_RC. Returns X on success or 0 on failure;
 */
static int parseNumArg(char *arg) {
   int n = 0;
   if (sscanf(arg, "-r%d", &n) == 1 && n >= MIN_RC && n <= MAX_RC)
      return n;
   else
      return 0;
}

/* Opens a file, if possible. On failure, prints an error message and exits 
 * with EXIT_FAILURE.
 */
static void openInputFile(char *fname, FILE **fp) {
   if ((*fp = fopen(fname, "r")) == NULL) {
      fprintf(stderr, "%s: %s: ", PROGRAM_NAME, fname);
      perror(NULL);
      exit(EXIT_FAILURE);
   }
}

void openOutputFile(char *in, FILE **fp) {
   char *out;
   /* ".lzw" extension, plus '\0' */
   VERIFY_PTR(out = malloc(5 + strlen(in)));
   out = strcpy(out, in);
   out = strcat(out, ".lzw");
   if ((*fp = fopen(out, "w")) == NULL) {
      fprintf(stderr, "%s: %s: ", PROGRAM_NAME, out);
      perror(NULL);
      exit(EXIT_FAILURE);
   }
   free(out);
}

void openFiles(char *in, FILE **inFile, FILE **outFile) {
   openInputFile(in, inFile);
   openOutputFile(in, outFile);
}

static int handle1Arg(char *argv[], FILE **inFile, FILE **outFile, int *rc) {
   if (argv[1][0] == '-')
      return -1;
   openFiles(argv[1], inFile, outFile);
   *rc = DEFAULT_RC;
   return 0;
}

static int handle2Args(char *argv[], FILE **inFile, FILE **outFile, int *rc) {
   if (argv[1][0] == '-' && (*rc = parseNumArg(argv[1])) > 0)
      openFiles(argv[2], inFile, outFile);
   else if (argv[2][0] == '-' && (*rc = parseNumArg(argv[2])) > 0)
      openFiles(argv[1], inFile, outFile);
   else
      return -1;
   return 0;
}

int parseArgs(int argc, char *argv[], FILE **inFile, FILE **outFile, int *rc) {
   if (argc == 2)
      return handle1Arg(argv, inFile, outFile, rc);
   else if (argc == 3)
      return handle2Args(argv, inFile, outFile, rc);
   else
      return -1;

   return 0;
}
