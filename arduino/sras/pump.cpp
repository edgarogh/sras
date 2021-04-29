#define PIN_PUMP 13

#include <Arduino.h>
#include "pump.hpp"


namespace pump {

unsigned long dispense_end = 0;
unsigned long dispense_lock = 0;


void setup(void) {
  pinMode(PIN_PUMP, OUTPUT);
}


void dispense(uint32_t duration, uint32_t lock) {
  long now = millis();
  
  if (dispense_end != 0) {
    return;
  }

  if (now < dispense_lock) {
    return;
  }

  dispense_end = now + duration;
  dispense_lock = dispense_end + lock;

  digitalWrite(PIN_PUMP, HIGH);
}


void loop(void) {
  long now = millis();
  if (now >= dispense_end) {
    dispense_end = 0;
    digitalWrite(PIN_PUMP, LOW);
  }
}

}
