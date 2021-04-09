#define REMOTE_SCREEN true
#define SCREEN_CHAR_WIDTH 12

#include "screen.hpp"

#include <Arduino.h>
#include <PCD8544.h>
#include "PCD8544_remote.hpp"

namespace screen {

static unsigned char GLYPH_NO[] = { 0x02, 0x05, 0x72, 0x88, 0x88 };
static unsigned char GLYPH_TICK[] = { 0x30, 0x60, 0x30, 0x18, 0x0c };

static unsigned char GLYPH_TOP[] = { 0x80, 0x80, 0x80, 0x80, 0x80 };
static unsigned char GLYPH_TOP_SUN[] = { 0x94, 0xbe, 0x9c, 0xbe, 0x94 };


#ifdef REMOTE_SCREEN
static RemPCD8544 lcd;
#else
static PCD8544 lcd;
#endif


void lcd_write_str(const char* str) {
  for (char i = 0; str[i] != 0; i++) {
    lcd.write(str[i]);
  }
}


void SetupTask::set_status(SetupStatus status) {
  lcd.setCursor(0, this->index);
  lcd.write((char) status);
}


Setup::Setup() { }


Setup::~Setup() {
  lcd.clear();
  lcd.setCursor(0, 0);
}


SetupTask Setup::add_task(const char* name) {
  lcd.setCursor(0, this->loc);
  lcd.write('.');

  SetupTask r;
  r.index = this->loc++;
  return r;
}


Setup init_setup() {
  lcd.setCursor(0, 0);
  lcd.write('~');

  return Setup();
}


UI::UI() {
  this->next_update = millis();

  for (char i = 0; i < SCREEN_CHAR_WIDTH; i++) {
    lcd.write('_');
  }
}


void UI::loop(void) {
  long now = millis();
  if (now < this->next_update) {
    return;
  }
  this->next_update = now;

  char ix;
  lcd.setCursor(SCREEN_CHAR_WIDTH - 5, 0);
  for (char i = SCREEN_CHAR_WIDTH - 5; i < SCREEN_CHAR_WIDTH; i++) {
    char c = '_';
    if (ix++ >= this->lightness) {
      c = 2;
    }
    lcd.write(c);
  }
}


void init(void) {
  lcd.begin(84, 48);
  lcd.createChar(4, GLYPH_NO);
  lcd.createChar(1, GLYPH_TICK);
  lcd.createChar('_', GLYPH_TOP);
  lcd.createChar(2, GLYPH_TOP_SUN);
  // pinMode(PIN_SCR_BACKLIGHT, OUTPUT);
  // digitalWrite(ledPin, HIGH);
}


void loop(void) {
  long int r1 = analogRead(A5);
  float r = ((float)r1 * (100. / 1023.) - 20.) * 25. / 54.;

  int r2 = analogRead(A4);
  lcd.setCursor(0, 0);
  lcd_write_str("Temp:");
  lcd.print(r, DEC);
  lcd.write(4);
  lcd.write(' ');
  lcd.write(' ');
  lcd.write(' ');

  lcd.setCursor(0, 1);
  lcd_write_str("Hum:");
  lcd.print(r2, DEC);
  lcd.write(' ');
  lcd.write(' ');
  lcd.write(' ');

  int r3 = analogRead(A3);
  lcd.setCursor(0, 2);
  lcd_write_str("Lum:");
  lcd.print(r3, DEC);
  lcd.write(' ');
  lcd.write(' ');
  lcd.write(' ');

  int r4 = analogRead(A2);
  lcd.setCursor(0, 3);
  lcd_write_str("Hum (t):");
  lcd.print(r4, DEC);
  lcd.write(' ');
  lcd.write(' ');
  lcd.write(' ');
  Serial.println(r4);
}

}
