#define PIN_LIGHT 0
#define PIN_TEMP 0
#define PIN_HUMI 0

#include "pump.hpp"
#include "screen.hpp"

#include <Arduino.h>

screen::UI ui;


void setup(void) {
  Serial.begin(9600);
  screen::init();

  do {
    screen::Setup se = screen::init_setup();
    screen::SetupTask t_screen = se.add_task("Ecran marche");
    screen::SetupTask t_internet = se.add_task("Internet");
    screen::SetupTask t_pump = se.add_task("Pompe");

    t_screen.set_status(screen::SetupStatus::OK);
    delay(1000);
    t_internet.set_status(screen::SetupStatus::OK);

    t_pump.set_status(screen::SetupStatus::OK);
  } while(0);

  pump::setup();

  screen::init();

  Serial.println("Je pense donc je suis !");

  pump::dispense(1000);

  ui = screen::UI();
}


void loop(void) {
  pump::loop();
  screen::loop();
  delay(1000);
}
