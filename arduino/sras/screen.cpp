#define REMOTE_SCREEN true
#define SCREEN_CHAR_WIDTH 12

#include "screen.hpp"

#include <Arduino.h>
#include <PCD8544.h>
#include "PCD8544_remote.hpp"
#include "sensor.hpp"

namespace screen {

static unsigned const char* LUM_LEVELS[] = { "Tres somb.", "Sombre    ", "Intermed.  ", "Lumineux  ", "Tres lumi." };

static unsigned char GLYPH_PLUS[] = { 0x08, 0x08, 0x7f, 0x08, 0x08 };
static unsigned char GLYPH_NO[] = { 0x02, 0x05, 0x72, 0x88, 0x88 };
// static unsigned char GLYPH_TICK[] = { 0x30, 0x60, 0x30, 0x18, 0x0c };

static unsigned char GLYPH_SUN[] = { 0x14, 0x3e, 0x1c, 0x3e, 0x14 };
static unsigned char GLYPH_THERMO[] = { 0x00, 0xc0, 0xbf, 0xd5, 0x00 };
static unsigned char GLYPH_HYGRO[] = { 0x70, 0xfc, 0x9e, 0x9f, 0x79 };
static unsigned char GLYPH_HYGRO_T[] = { 0xc0, 0xcc, 0xde, 0xd7, 0xcd };


#ifdef REMOTE_SCREEN
static RemPCD8544 lcd;
#else
static PCD8544 lcd;
#endif


UI::UI() {
  this->next_update = millis();

  /*for (char i = 0; i < SCREEN_CHAR_WIDTH; i++) {
    lcd.write('_');
  }*/
}


void UI::loop(void) {
  long now = millis();
  if (now < this->next_update) {
    return;
  }
  this->next_update = now;

  char ix;
  lcd.setCursor(SCREEN_CHAR_WIDTH - 5, 0);
  /*for (char i = SCREEN_CHAR_WIDTH - 5; i < SCREEN_CHAR_WIDTH; i++) {
    char c = '_';
    if (ix++ >= this->lightness) {
      c = 2;
    }
    lcd.write(c);
  }*/
}


void init_glyphs(void) {
  lcd.createChar('+', GLYPH_PLUS);
  lcd.createChar(4, GLYPH_NO);
  lcd.createChar('$', GLYPH_SUN);
  lcd.createChar('~', GLYPH_THERMO);
  lcd.createChar('^', GLYPH_HYGRO);
  lcd.createChar('_', GLYPH_HYGRO_T);
}


void init(void) {
  lcd.begin(84, 48);
  init_glyphs();
}


long next_update = 0;
unsigned char b;


void loop(void) {
  long now = millis();
  if (now < next_update) return;
  next_update = now + 1000;
  
  long int r1 = analogRead(A5);
  float temp = ((float)r1 * (100. / 1023.) - 20.) * 25. / 54.;

  init_glyphs();

  lcd.setCursor(0, 0);
  switch (b++) {
    case 0: lcd.write('-'); break;
    case 1: lcd.write('\\'); break;
    case 2: lcd.write('|'); break;
    case 3: lcd.write('/'); break;
  }
  b %= 4;
  lcd.print("| valeur");

  lcd.setCursor(0, 1);
  lcd.print("-+----------");

  lcd.setCursor(0, 2);
  lcd.print("~|");
  lcd.print(temp, DEC);
  lcd.write(4);
  lcd.print("     ");

  int hygro = sensor::humidity(temp, 0) * 5; // analogRead(A4);
  lcd.setCursor(0, 3);
  lcd.print("^|");
  lcd.print(hygro, DEC);
  lcd.print("%     ");

  int lum = analogRead(A3);
  lcd.setCursor(0, 4);
  lcd.print("$|");
  lcd.print(LUM_LEVELS[lum * 5 / 1024]);
  Serial.print("+1");
  Serial.println(lum, DEC);

  int hygro_t = analogRead(A2);
  lcd.setCursor(0, 5);
  lcd.print("_|");
  lcd.print(hygro_t, DEC);
  lcd.print("     ");
  Serial.print("+2");
  Serial.println(hygro_t, DEC);
}

}
