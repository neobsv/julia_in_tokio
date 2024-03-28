## Julia in Tokio

### Introduction

The complex equation for it is:
f(z) = z^2 + c

Where f(z) is the n+1th iteration of a complex number, and z represents the nth iteration of it with c being a fixed value
complex number that specifies the structure of the fractal.

### Escape Radius

The `escape radius` is a parameter used in iterating a point under the Mandelbrot Set and Julia Set functions. 
It is the radius of a circle on the Complex Plane used as a boundary to determine when iteration can stop. 
The circle is centered at the origin, and has a radius of at least 2.0.

### Generator Pseudocode

The Julia Fractal is generated using the following equations/code, given a rectangular window bound:
(source: Wikipedia)

```python

R = escape radius  # choose R > 0 such that R**2 - R >= sqrt(cx**2 + cy**2)


for each pixel (x, y) on the screen, do:   
{
    zx = scaled x coordinate of pixel; # (scale to be between -R and R)
       # zx represents the real part of z.
    zy = scaled y coordinate of pixel; # (scale to be between -R and R)
       # zy represents the imaginary part of z.

    iteration = 0;
    max_iteration = 1000;
  
    while (zx * zx + zy * zy < R**2  AND  iteration < max_iteration) 
    {
        xtemp = zx * zx - zy * zy;
        zy = 2 * zx * zy  + cy;
        zx = xtemp + cx;
    
        iteration = iteration + 1;
    }
  
    if (iteration == max_iteration)
        return black;
    else
        return iteration;
}
```
