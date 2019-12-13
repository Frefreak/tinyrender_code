#ifndef TYPES_HH
#define TYPES_HH

#include <cstdint>
#include <tuple>

using u64 = uint64_t;
using u32 = uint32_t;
using u8 = uint8_t;

using Color = std::tuple<u8, u8, u8>;
using Coord = std::tuple<u32, u32>;

inline Color mkcolor(u8 r, u8 g, u8 b) {
  return static_cast<Color>(std::make_tuple(r, g, b));
}

inline Coord mkcoord(u32 x, u32 y) {
  return static_cast<Coord>(std::make_tuple(x, y));
}

#define get_r(c) (std::get<0>(c))
#define get_g(c) (std::get<1>(c))
#define get_b(c) (std::get<2>(c))

#define get_x(c) (std::get<0>(c))
#define get_y(c) (std::get<1>(c))

#endif /* ifndef TYPES_HH */
