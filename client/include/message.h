#ifndef __MESSAGE__H
#define __MESSAGE__H

#include "message.pb.h"
#include <string>

ProtoMessage::ProtoMessage message(int id, 
                                   ProtoMessage::ProtoMessageType flag, 
                                   const std::vector<std::string>& data,
                                   int64_t integer = 0);

#endif /* __MESSAGE__H */
