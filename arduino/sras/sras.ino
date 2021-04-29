#define PIN_LIGHT 0
#define PIN_TEMP 0
#define PIN_HUMI 0

#include "config.hpp"
#include "mqe.hpp"
#include "pump.hpp"
#include "screen.hpp"

#include <Arduino.h>

screen::UI ui;


config::Config cfg = {
  .soil_min = 0,
  .soil_max = 1023,

  .temp_min = 0,
  .temp_max = 1023,

  .hygro_min = 0,
  .hygro_max = 1023,

  .pump_duration = 2000,
  .pump_wait = 5000,
};


void setup(void) {
  Serial.begin(9600);
  while (!Serial);
  screen::init();
  pump::setup();
  Serial.println("Je pense donc je suis !");
  pump::dispense(cfg.pump_duration, 0);
  ui = screen::UI();
}


void loop(void) {
  pump::loop();
  screen::loop();

  mqe::Message msg;
  if (mqe::receive(&msg)) {
    switch (msg.tag) {
      case mqe::MessageType::MESSAGE_CONFIG: {
        Serial.println("Updated config");
        cfg = msg.value.config;
        break;
      }
      case mqe::MessageType::MESSAGE_DISPENSE: {
        mqe::MessageDispense dispense = msg.value.dispense;
        uint32_t duration = dispense.duration;
        uint32_t wait = dispense.wait;

        if (duration > 0) {
          if (wait == 4294967295) {
            wait = cfg.pump_wait;
          }
          
          pump::dispense(duration, wait);
          Serial.print("Dispensing for ");
          Serial.print(duration, DEC);
          Serial.print("ms, then locking for ");
          Serial.print(wait, DEC);
          Serial.println("ms");
        } else {
          pump::dispense(cfg.pump_duration, cfg.pump_wait);
          Serial.println("Dispensing with default config");
        }
        
        break;
      }
    }

    msg.tag = (mqe::MessageType) 0xff;
  }

  if (analogRead(A2) < 500) {
    pump::dispense(cfg.pump_duration, cfg.pump_wait);
  }
}
