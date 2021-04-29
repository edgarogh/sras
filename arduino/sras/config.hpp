#ifndef _CONFIG_H_
#define _CONFIG_H_

#include <stdint.h>

namespace config {
  typedef struct {
    uint32_t soil_min;
    uint32_t soil_max;

    uint32_t temp_min;
    uint32_t temp_max;
    
    uint32_t hygro_min;
    uint32_t hygro_max;
    
    uint32_t pump_duration;
    uint32_t pump_wait;
  } Config;
}

#endif
