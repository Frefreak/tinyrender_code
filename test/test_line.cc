#include "renderer.hh"
#include "constant.hh"

int main(int argc, char *argv[])
{
  PPM image = PPM(600, 400);
  line(20, 345, 99, 123, image, red);
  line(20, 345, 333, 345, image, blue);
  line(99, 123, 333, 345, image, green);
  image.write("test_line.ppm");
  return 0;
}
