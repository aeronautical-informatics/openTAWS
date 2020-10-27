#ifndef OTAWS_H
#define OTAWS_H

#include <bits/stdint-uintn.h>

struct otaws_state {
	void* taws_ptr;
	struct otaws_alarm* alarms;
	uint8_t alarms_count;
};

struct otaws_alarm {
	char const * name;
	uint8_t is_important;
};


/// Push new aircraft state to the TAWS
///
/// @param otaws_ptr A ptr to one otaws_state struct
/// @param position_lat The current latitude
/// @param position_lon The current longitude
/// @param altitude The current altitude above sea-level in feet
/// @return Returns the number of active alarms as determined after processing the new aircraft state
uint8_t otaws_push(struct taws_state* otaws_ptr , double position_lat, double position_lon, double altitude);

/// Create a new otaws_state struct
///
/// @return A pointer to the newly created otaws_state struct
struct otaws_state* otaws_create();

/// Deallocates the a otaws_state struct
/// Must be called exactly once on each otaws_state struct
///
/// @param otaws_ptr A pointer to the otaws_state struct to be deallocated
void otaws_delete(struct otaws_state* otaws_ptr);

#endif
