#ifndef LZW_H
#define LZW_H

#define PROGRAM_NAME "lzwCompress"

#define MAX_RC 15
#define MIN_RC 9
#define DEFAULT_RC 12

#define ALPHABET_SIZE 256
#define EOD 256
#define DEFAULT_TRIE_SIZE 256

#define BYTE_BITS 8

#define VERIFY_PTR(_PTR) {\
   void *_ptr = _PTR;\
   if (_ptr == NULL) {\
      fprintf(stderr, "%s: malloc failed on line %d in %s\n", PROGRAM_NAME,\
         __LINE__, __FILE__);\
      exit(EXIT_FAILURE);\
   }\
}

typedef unsigned char Byte;

typedef struct {
   unsigned short code;
   unsigned short next[ALPHABET_SIZE];
} Node;

typedef struct {
   unsigned short size;
   unsigned short nextIndex;
   unsigned short nextCode;
   Node *nodes;
} Trie;

typedef struct {
   unsigned bits;
   unsigned bitCount;
   unsigned rc;
   unsigned packetSize;
} CompressionState;

/*
typedef struct {
   char *inFP;
   char *outFPe;
   FILE *inFile;
   FILE *outFile;
}
*/

void printUsageAndExit(void);

#endif
