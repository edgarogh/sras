#define PIN_PUMP 13

#include <Arduino.h>
#include "pump.hpp"


namespace pump {

long dispense_end = 0;


void setup(void) {
  pinMode(PIN_PUMP, OUTPUT);
}


void dispense(int duration) {
  if (dispense_end != 0) {
    return;
  }

  dispense_end = millis() + duration;

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
