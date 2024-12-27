#include "message.h"

ProtoMessage::ProtoMessage make(int i, const std::string& str) {
    ProtoMessage::ProtoMessage ret;
    ret.set_id(i);
    ret.set_data(str);
    return ret;
}
