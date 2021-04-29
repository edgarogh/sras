#ifndef _MQE_H_
#define _MQE_H_

#include "config.hpp"

namespace mqe {
  enum MessageType { MESSAGE_CONFIG, MESSAGE_DISPENSE };

  struct MessageDispense {
    uint32_t duration;
    uint32_t wait;
  };
  
  struct Message {
    MessageType tag;
    union {
      config::Config config;
      MessageDispense dispense;
    } value;
  };

  bool receive(Message* dest);
}

#endif
