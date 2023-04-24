#ifndef BITFUNCTIONS_H
#define BITFUNCTIONS_H

#include "lzw.h"

void addToBuffer(unsigned short, CompressionState *);
Byte getMSBByte(CompressionState *);
Byte getLeftoverByte(CompressionState *);

#endif
