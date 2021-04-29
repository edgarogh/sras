#include "mqe.hpp"
#include <Arduino.h>


enum class ReadState {
  None,
  Plus,
  Tagged,
  Done,
};


namespace mqe {

static ReadState state = ReadState::None;
static Message current;


bool receive(Message* dest) {
  int available = Serial.available();
  
  switch (state) {
    case ReadState::None: {
      if (available < 1) return false;
      else {
        if (Serial.read() == '+') {
          state = ReadState::Plus;
        } else {
          state = ReadState::Done;
        }
      }
      break;
    }
    case ReadState::Plus: {
      if (available < 1) return false;
      else {
        switch (Serial.read()) {
          case 'c': {
            current.tag = MessageType::MESSAGE_CONFIG;
            state = ReadState::Tagged;
            break;
          }
          case 'd': {
            current.tag = MessageType::MESSAGE_DISPENSE;
            state = ReadState::Tagged;
            break;
          }
          default: {
            state = ReadState::Done;
          }
        }
      }
      break;
    }
    case ReadState::Tagged: {
      switch (current.tag) {
        case MessageType::MESSAGE_CONFIG: {
          if (available < sizeof(config::Config)) return false;
          Serial.readBytes((char*) &current.value, sizeof(config::Config));
          *dest = current;
          state = ReadState::Done;
          return true;
        }
        case MessageType::MESSAGE_DISPENSE: {
          if (available < sizeof(MessageDispense)) return false;
          Serial.readBytes((char*) &current.value, sizeof(MessageDispense));
          *dest = current;
          state = ReadState::Done;
          return true;
        }
        default: {
          state = ReadState::Done;
        }
      }
      break;
    }
    case ReadState::Done: {
      if (available && (Serial.read() == '\n')) {
        state = ReadState::None;
      }
      break;
    }
  }

  return false;
}

}
