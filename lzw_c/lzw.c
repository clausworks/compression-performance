#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include "lzw.h"
#include "io.h"
#include "bitFunctions.h"

void printUsageAndExit(void) {
   fprintf(stderr, "Usage: %s [-rN] file\n", PROGRAM_NAME);
   fprintf(stderr, "Where: N is a number from 9 to 15 specifying "
      "the recycle code as a power of 2\n");
   exit(EXIT_FAILURE);
}

static void initTrie(Trie *trie, unsigned short size) {
   trie->size = size;
   trie->nextIndex = 1;
   trie->nextCode = ALPHABET_SIZE + 1; /* code ALPHABET_SIZE reserved for EOD */
}

static void growTrie(Trie *trie) {
   if (trie->nextIndex == trie->size) {
      int s = trie->size; /* old size */
      trie->size *= 2;
      VERIFY_PTR(trie->nodes = realloc(trie->nodes, trie->size * sizeof(Node)));
      VERIFY_PTR(memset(trie->nodes + s, 0, s * sizeof(Node)));
   }
}

/*
 * static void printTrie(Trie *trie) {
 *    int i, j;
 *    printf("TRIE\n");
 *    printf("size: %u\nnextIndex: %u\nnextCode: %u\n",
 *       trie->size, trie->nextIndex, trie->nextCode);
 *    for (i = 0; i < trie->size; ++i) {
 *       if (i == 0 || trie->nodes[i].code != 0) {
 *       printf("-------------\n");
 *          printf("nodes[%u]: %u\n", i, trie->nodes[i].code);
 *          for (j = 0; j < ALPHABET_SIZE; ++j) {
 *             if (trie->nodes[i].next[j] != 0) {
 *                printf("   next[%u]: %u\n", j, trie->nodes[i].next[j]);
 *             }
 *          }
 *       }
 *    }
 *    printf("-------------\n");
 * }
 */

static void updateAlphabeticalByte(Trie *dic, Byte b) {
   if (dic->nodes[0].next[b] == 0) {
      dic->nodes[0].next[b] = dic->nextIndex;
      dic->nodes[dic->nextIndex].code = b;
      dic->nextIndex++;
      growTrie(dic);
   }
}

static void updateDictionary(Trie *dic, Byte b, int i) {
   /* set "next" index */
   dic->nodes[i].next[b] = dic->nextIndex;
   /* set code of "next" node */
   dic->nodes[dic->nextIndex].code = dic->nextCode;

   printf("Added code to dictionary: %u\n", dic->nextCode);

   dic->nextCode++;
   dic->nextIndex++;

   growTrie(dic);
}

static void updatePacketSize(int code, CompressionState *cs) {
   if (code < (1 << 9))
      cs->packetSize = 9;
   else if (code < (1 << 10))
      cs->packetSize = 10;
   else if (code < (1 << 11))
      cs->packetSize = 11;
   else if (code < (1 << 12))
      cs->packetSize = 12;
   else if (code < (1 << 13))
      cs->packetSize = 13;
   else if (code < (1 << 14))
      cs->packetSize = 14;
   else if (code < (1 << 15))
      cs->packetSize = 15;
   else
      cs->packetSize = 16;
}

/* Returns 1 if recycling occurred, 0 otherwise.
 */
static int recycle(int rc, Trie *dic, Byte b, CompressionState *cs) {
   if (cs->packetSize > cs->rc) {
      initTrie(dic, dic->size);
      VERIFY_PTR(memset(dic->nodes, 0, dic->size * sizeof(Node)));
      updateAlphabeticalByte(dic, b);
      updatePacketSize(dic->nextCode, cs);
      return 1;
      /*printf("RECYCLED\n");*/
   }
   return 0;
}

static void writeByte(FILE *outFile, Byte b) {
   if (fputc(b, outFile) == EOF) {
      fprintf(stderr, "%s: ", PROGRAM_NAME);
      perror(NULL);
      exit(EXIT_FAILURE); 
   }
}

static void writeCode(FILE *outFile, unsigned short code, 
   CompressionState *cs) {
   printf("Writing code %u (%u bits)\n", code, cs->packetSize);
   addToBuffer(code, cs);
   /* write as many bytes as possible */
   while (cs->bitCount > BYTE_BITS)
      writeByte(outFile, getMSBByte(cs));
}

static void finishWrite(FILE *outFile, int i, Trie *dic, 
   CompressionState *cs) {
   writeCode(outFile, dic->nodes[i].code, cs);
   updatePacketSize(dic->nextCode, cs);
   recycle(cs->rc, dic, '\0', cs); /* FIXME: remove null char workaround */
   writeCode(outFile, EOD, cs);
   if (cs->bitCount == 0)
      return;
   writeByte(outFile, getLeftoverByte(cs)); 
}

static void writeKnown(FILE *outFile, Byte b, int i, Trie *dic,
   CompressionState *cs) {
   writeCode(outFile, dic->nodes[i].code, cs);
   updatePacketSize(dic->nextCode, cs);
   if (!recycle(cs->rc, dic, b, cs))
      updateDictionary(dic, b, i);
}

static void initCompressionState(unsigned bits, unsigned bitCount, unsigned rc,
   unsigned packetSize, CompressionState *cs) {
   cs->bits = bits;
   cs->bitCount = bitCount;
   cs->rc = rc;
   cs->packetSize = packetSize;
}

static void compress(FILE *inFile, FILE *outFile, int rc) {
   int b;
   Trie dic;
   unsigned short i = 0;
   CompressionState cs;

   initCompressionState(0, 0, rc, MIN_RC, &cs);
   initTrie(&dic, DEFAULT_TRIE_SIZE);
   VERIFY_PTR(dic.nodes = calloc(dic.size, sizeof(Node)));
   
   /* handle empty file */
   if ((b = fgetc(inFile)) == EOF) {
      free (dic.nodes); 
      return;
   }
   
   /* write recycle code */
   writeByte(outFile, (Byte)rc);
   
   do {
      /* update alphabetical index */
      updateAlphabeticalByte(&dic, b);
      /* reached unknown sequence */
      if (dic.nodes[i].next[b] == 0) {
         /* write known portion */
         writeKnown(outFile, b, i, &dic, &cs);
         i = dic.nodes[0].next[b];
      }
      /* current sequence known: advance */
      else
         i = dic.nodes[i].next[b];
   } while ((b = fgetc(inFile)) != EOF); 

   if (ferror(inFile)) {
      perror(PROGRAM_NAME);
      exit(EXIT_FAILURE);
   }

   /* write last character and EOD */
   finishWrite(outFile, i, &dic, &cs);

   free(dic.nodes);
}

int main(int argc, char *argv[]) {
   int recycleCode;
   FILE *inFile = NULL;
   FILE *outFile = NULL;
   
   if (parseArgs(argc, argv, &inFile, &outFile, &recycleCode) == -1)
      printUsageAndExit();

   compress(inFile, outFile, recycleCode);
   fclose(inFile);
   fclose(outFile);

   return 0;
}
