#include "ppm.hh"
#include <fstream>
#include <iostream>

PPM::PPM(u32 width, u32 height): width(width), height(height)
{
  data = new u8[width * height * 3];
  if (data == nullptr) {
    perror("creating image buffer");
    exit(1);
  }
  // printf("%d x %d\n", width, height);
}

void PPM::set(u32 x, u32 y, const Color &c)
{
  set(x, y, get_r(c), get_g(c), get_b(c));
}

void PPM::set(const Coord &pos, const Color &c)
{
  set(get_x(pos), get_x(pos), get_r(c), get_g(c), get_b(c));
}

void PPM::set(u32 x, u32 y, u8 r, u8 g, u8 b)
{
  u64 index = (y * width + x) * 3;
  data[index] = r;
  data[index+1] = g;
  data[index+2] = b;
}

void PPM::set(const Coord &pos, u8 r, u8 g, u8 b)
{
  set(get_x(pos), get_y(pos), r, g, b);
}

void PPM::write(const std::string &filepath)
{
  std::ofstream out;
  out.open(filepath, std::ios::binary | std::ios::out);
  out << "P6\n" << width << ' ' << height << '\n' << 255 << '\n';
  out.write((char *)data, width * height * 3);
  out.close();
}

PPM::~PPM()
{
  delete []data;
}

// <->
void PPM::flip_horizontal()
{
  // one row at a time
  for (u32 i = 0; i < height; i++) {
    u64 index = i * width * 3;
    u64 index2 = (i * width + width - 1) * 3;
    u64 offset = 3;
    while (index < index2) {
      std::swap(data[index], data[index2]);
      std::swap(data[index+1], data[index2+1]);
      std::swap(data[index+2], data[index2+2]);
      index += offset;
      index2 -= offset;
    }
  }
}

// ^
// |
// V
void PPM::flip_vertical()
{
  // one column at a time
  for (u32 i = 0; i < width; i++) {
    u64 index = i * 3;
    u64 index2 = ((height - 1) * width + i) * 3;
    u64 offset = 3 * width;
    while (index < index2) {
      std::swap(data[index], data[index2]);
      std::swap(data[index+1], data[index2+1]);
      std::swap(data[index+2], data[index2+2]);
      index += offset;
      index2 -= offset;
    }
  }
}
