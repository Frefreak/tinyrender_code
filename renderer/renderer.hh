#ifndef RENDERER_H
#define RENDERER_H

#include "ppm.hh"

void line(int x0, int y0, int x1, int y1, PPM &image, Color &color);
void line(Coord &pos0, Coord &pos1, PPM &image, Color &color);
void line(int x0, int y0, Coord &pos1, PPM &image, Color &color);
void line(Coord &pos0, int x1, int y1, PPM &image, Color &color);

#endif /* ifndef RENDERER_H */
