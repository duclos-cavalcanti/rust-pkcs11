#include "message.h"

ProtoMessage::ProtoMessage message(int id, 
                                   ProtoMessage::ProtoMessageType flag, 
                                   const std::vector<std::string>& data,
                                   int64_t integer) {
    ProtoMessage::ProtoMessage ret;
    ret.set_id(id);
    ret.set_flag(flag);
    ret.set_repeat(data.size());

    if (integer != 0) {
        ret.set_integer(integer);
    }

    for (int i=0; i< (int) data.size(); i++) { 
        ret.add_data(data[i]);
    }

    return ret;
}
