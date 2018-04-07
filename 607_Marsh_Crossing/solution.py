#!/usr/bin/env python3

import math

refractive_indices = [1.0, 10/9, 10/8, 10/7, 10/6, 10/5]
delta_y = [100/math.sqrt(2)-50] + [10]*5
tdx = 100.0 / math.sqrt(2)

def slope(index_of_refraction, sin0):
    ior = index_of_refraction
    return 1/(sin0 / math.sqrt(1 - (sin0/ior)**2) / ior)

def total_delta_x(alpha0):
    sin0 = math.sin(alpha0)
    sum_delta_x = 0
    for i in range(6):
        dx, _ = delta_x_y(sin0, i)
        sum_delta_x += dx

    return sum_delta_x

def delta_x_y(sin0, idx):
    ior = refractive_indices[idx]
    dy = delta_y[idx]
    slope_idx = slope(ior, sin0)
    dx = dy / slope_idx
    return dx, dy

def velocity(idx):
    return 10 / refractive_indices[idx]

alpha_min = 0
alpha_max = math.pi/2

step = math.pi/8
alpha = math.pi/4

dx = 200
n = 0
while abs(dx - tdx) > 1e-14:
    n += 1
    dx = total_delta_x(alpha)
    dx_plus = total_delta_x(alpha+step)
    dx_min = total_delta_x(alpha-step)
    diff_plus = abs(tdx - dx_plus)
    diff_min = abs(tdx - dx_min)
    if diff_plus < diff_min:
        alpha += step
    else:
        alpha -= step
    step /= 2

sin = math.sin(alpha)
total_t = 0
for region in range(6):
    v = velocity(region)
    dx, dy = delta_x_y(sin, region)
    s = math.sqrt(dx**2 + dy**2)
    t = s/v
    total_t += t

print(f"{total_t:.10f}")
