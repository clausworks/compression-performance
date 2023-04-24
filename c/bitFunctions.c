#include "lzw.h"
#include "bitFunctions.h"

void addToBuffer(unsigned short code, CompressionState *cs) {
   cs->bits = cs->bits << cs->packetSize;
   cs->bits += (unsigned)code;
   cs->bitCount += cs->packetSize;
}

Byte getMSBByte(CompressionState *cs) {
   unsigned mask = 0xFFu << (cs->bitCount - BYTE_BITS);
   unsigned ret = cs->bits & mask;
   ret = ret >> (cs->bitCount - BYTE_BITS);
   cs->bits = cs->bits & (~mask);
   cs->bitCount -= BYTE_BITS; 
   return ret;
}

Byte getLeftoverByte(CompressionState *cs) {
   unsigned mask = (1 << cs->bitCount) - 1;
   unsigned ret = mask & cs->bits;
   ret = ret << (BYTE_BITS - cs->bitCount);
   cs->bits = cs->bitCount = 0;
   return ret;
}

