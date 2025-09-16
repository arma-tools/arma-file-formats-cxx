#pragma once

#include <iostream>

#include <rust/cxx.h>

namespace ArmaPlace::ArmaFileFormats
{
    class Seeker
    {
    public:
        Seeker() {}
        ~Seeker() {}

        virtual void seek(uint64_t off, std::ios_base::seekdir dir) = 0;
    };

    class Reader : public Seeker
    {
    public:
        Reader() {}
        ~Reader() {}

        virtual size_t read(rust::Vec<uint8_t> buf) = 0;
    };
};