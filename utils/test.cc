#include "ppm.hh"
#include <cstdio>
#include <ctime>

void random_dot(PPM &image, Color c)
{
  u32 dotx = rand() % image.width;
  u32 doty = rand() % image.height;
  printf("(%d, %d, %d) @ (%d, %d)\n", get_r(c), get_g(c), get_b(c),
         dotx, doty);
  image.set(dotx, doty, c);
}

int main(int argc, char *argv[])
{
  if (argc != 3) {
    fprintf(stderr, "Usage: %s <width> <height>\n", argv[0]);
    exit(1);
  }
  u32 width = atol(argv[1]);
  u32 height = atol(argv[2]);
  if (width <= 0 || height <= 0) {
    fprintf(stderr, "width and height must be positive integers (32 bit maximum)\n");
    exit(1);
  }
  PPM image = PPM(width, height);
  srand(time(0));
  random_dot(image, mkcolor(0xFF, 0, 0));
  random_dot(image, mkcolor(0, 0xFF, 0));
  random_dot(image, mkcolor(0, 0, 0xFF));

  if (width > 100 && height > 100) {
    printf("creating purple line\n");
    for (int i = 6; i < 12; i++)
      for (int j = 4; j < 30; j++)
        image.set(i, j, mkcolor(0xFF, 0, 0xFF));
  }
  // image.flip_vertical();
  image.flip_horizontal();
  image.write("test.ppm");
  return 0;
}
