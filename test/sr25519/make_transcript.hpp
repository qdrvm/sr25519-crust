//
// Created by iceseer on 10/19/22.
//

#ifndef SCHNORRKEL_CRUST_MAKE_TRANSCRIPT_HPP
#define SCHNORRKEL_CRUST_MAKE_TRANSCRIPT_HPP

#include "strobe.hpp"
#include "transcript.hpp"

extern "C" {
#include <schnorrkel/schnorrkel.h>
}

template <size_t MsgBufSize>
inline Strobe128 makeTranscript(std::string_view message) {
    primitives::Transcript t;
    t.initialize({'B', 'A', 'B', 'E'});
    char msg_buf[MsgBufSize];
    std::fill_n(msg_buf, MsgBufSize, 0);
    std::memcpy(msg_buf, message.data(), message.size());
    t.append_message({'m', 'e', 's', 's', 'a', 'g', 'e'}, msg_buf);

    Strobe128 strobe;
    std::memcpy(strobe.state, t.data().data(), t.data().size());
    strobe.pos = t.state().current_position;
    strobe.cur_flags = t.state().current_state;
    strobe.pos_begin = t.state().begin_position;
    return strobe;
}

#endif //SCHNORRKEL_CRUST_MAKE_TRANSCRIPT_HPP
