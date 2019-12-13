#ifndef PPM_H
#define PPM_H

#include <vector>
#include "types.hh"

struct PPM {
  u32 width;
  u32 height;
  u8 *data;

  PPM(u32 width, u32 height);
  ~PPM();
  // set pixel
  void set(u32 x, u32 y, const Color &c);
  void set(const Coord &pos, const Color &c);
  void set(u32 x, u32 y, u8 r, u8 g, u8 b);
  void set(const Coord &pos, u8 r, u8 g, u8 b);
  void write(const std::string &filepath);

  void flip_horizontal();
  void flip_vertical();
};

#endif /* ifndef PPM_H */

