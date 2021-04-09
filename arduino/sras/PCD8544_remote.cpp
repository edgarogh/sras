#include "PCD8544_remote.hpp"

#include <Arduino.h>

void RemPCD8544::begin(int width, int height) { }

void RemPCD8544::createChar(char c, unsigned char* glyph) {
  Serial.print("+g");
  Serial.print(c, DEC);
  Serial.write(',');
  Serial.print(glyph[0], DEC);
  Serial.write(',');
  Serial.print(glyph[1], DEC);
  Serial.write(',');
  Serial.print(glyph[2], DEC);
  Serial.write(',');
  Serial.print(glyph[3], DEC);
  Serial.write(',');
  Serial.println(glyph[4], DEC);
}

void RemPCD8544::write(char c) {
  Serial.print("+p");
  Serial.write(c);
  Serial.write('\n');
}

void RemPCD8544::print(int num, int radix) {
  Serial.print("+p");
  Serial.println(num, radix);
}

void RemPCD8544::setCursor(char x, char y) {
  Serial.print("+m");
  Serial.print(x, DEC);
  Serial.write(',');
  Serial.println(y, DEC);
}

void RemPCD8544::clear(void) {
  Serial.println("+c");
}
